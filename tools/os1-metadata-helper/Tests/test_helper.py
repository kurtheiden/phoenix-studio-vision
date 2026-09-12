#!/usr/bin/env python3
"""Synthetic-only black-box tests for the OS1 metadata helper."""

import base64
import ctypes
import hashlib
import json
import os
import pathlib
import shutil
import stat
import subprocess
import sys
import tempfile


DATA_SENTINEL = b"SYNTHETIC_DATA_SENTINEL_NEVER_EMIT"
TAIL_SENTINEL = b"TAIL_SENTINEL_NEVER_EMIT_123"
FORK_SENTINEL = b"RESOURCE_FORK_SENTINEL_NEVER_EMIT"
XATTR_SENTINEL = b"UNRELATED_XATTR_VALUE_NEVER_EMIT"
PRIVATE_NAME = "private_filename_sentinel.project"


def set_xattr(path, name, value):
    libc = ctypes.CDLL("/usr/lib/libSystem.B.dylib", use_errno=True)
    function = libc.setxattr
    function.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_void_p, ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int]
    function.restype = ctypes.c_int
    buffer = ctypes.create_string_buffer(value)
    result = function(os.fsencode(path), name.encode(), buffer, len(value), 0, 0)
    if result != 0:
        raise OSError(ctypes.get_errno(), "synthetic xattr setup failed")


def get_xattr(path, name):
    libc = ctypes.CDLL("/usr/lib/libSystem.B.dylib", use_errno=True)
    function = libc.getxattr
    function.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.c_void_p,
                         ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int]
    function.restype = ctypes.c_ssize_t
    size = function(os.fsencode(path), name.encode(), None, 0, 0, 0)
    if size < 0:
        raise OSError(ctypes.get_errno(), "synthetic xattr read failed")
    buffer = ctypes.create_string_buffer(size)
    actual = function(os.fsencode(path), name.encode(), buffer, size, 0, 0)
    if actual != size:
        raise OSError("synthetic xattr changed during read")
    return buffer.raw[:actual]


def run(binary, *args):
    return subprocess.run([binary, *map(str, args)], capture_output=True, check=False, timeout=20)


def require(condition, label):
    if not condition:
        raise AssertionError(label)


def records(path):
    return [json.loads(line) for line in path.read_text().splitlines()]


def accepted(completed, map_path, record_path, expected_digest):
    """Test-side acceptance check, not a new helper schema or production validator."""
    if completed.returncode != 0 or completed.stdout or completed.stderr:
        return False
    try:
        saved = record_path.read_bytes()
        inventory = json.loads(map_path.read_text())
        rows = records(record_path)
        entries = inventory["entries"]
        if hashlib.sha256(saved).hexdigest() != expected_digest:
            return False
        if len(rows) != inventory["entry_count"] or len(rows) != len(entries):
            return False
        if [r["artifact_id"] for r in rows] != [e["artifact_id"] for e in entries]:
            return False
        return all(
            (r["status"] == "INSPECTED" and r["integrity_status"] == "VERIFIED") or
            (r["status"] in {"REJECTED_NO_FOLLOW", "REJECTED_DIRECTORY",
                             "REJECTED_UNSUPPORTED_KIND", "REJECTED_ALIAS_TYPE"}
             and r["integrity_status"] == "NOT_INSPECTED")
            for r in rows
        )
    except (OSError, ValueError, KeyError, TypeError):
        return False


def preserved_state(path, attributes=()):
    # Test-owned regular files only; intentionally excludes access time.
    return (path.read_bytes(), path.stat().st_size,
            {name: get_xattr(path, name) for name in attributes})


def invoke(root, out, binary):
    map_path = out / "map.json"
    record_path = out / "record.jsonl"
    first = run(binary, "enumerate", "--root", root, "--map-output", map_path)
    require(first.returncode == 0 and first.stdout == b"" and first.stderr == b"", "enumerate success channel")
    second = run(binary, "inspect", "--map", map_path, "--record-output", record_path)
    require(second.returncode == 0 and second.stdout == b"" and second.stderr == b"", "inspect success channel")
    return map_path, record_path, records(record_path), second


def main():
    require(len(sys.argv) == 2, "binary argument")
    binary = os.path.abspath(sys.argv[1])
    test_root = pathlib.Path(tempfile.mkdtemp(prefix="phoenix-os1-synthetic-"))
    os.chmod(test_root, 0o700)
    checks = []
    unavailable = []

    def skipped(label):
        unavailable.append(label)

    def checked(label, condition=True):
        require(condition, label)
        checks.append(label)

    try:
        source = test_root / "source"
        output = test_root / "private-output"
        source.mkdir(mode=0o700)
        output.mkdir(mode=0o700)
        regular = source / PRIVATE_NAME
        regular.write_bytes(DATA_SENTINEL)
        (source / "empty.synthetic").write_bytes(b"")
        (source / "duplicate.synthetic").write_bytes(DATA_SENTINEL)
        (source / "different.synthetic").write_bytes(b"different synthetic bytes")
        nested = source / "nested_directory_sentinel"
        nested.mkdir()
        (nested / "nested_content_sentinel").write_bytes(b"NESTED_SENTINEL_NEVER_EMIT")
        os.symlink(regular, source / "symlink_sentinel")

        finder_supported = True
        finder_value = bytes([0, 31, 127, 128, 65, 66, 67, 68]) + TAIL_SENTINEL[:24]
        try:
            set_xattr(regular, "com.apple.FinderInfo", finder_value)
        except OSError:
            finder_supported = False
        xattr_supported = True
        try:
            set_xattr(regular, "user.synthetic.unrelated_name_never_emit", XATTR_SENTINEL)
        except OSError:
            xattr_supported = False

        resource_supported = True
        try:
            set_xattr(regular, "com.apple.ResourceFork", FORK_SENTINEL)
        except OSError:
            resource_supported = False

        installed = []
        for supported, name in [(finder_supported, "com.apple.FinderInfo"),
                                (resource_supported, "com.apple.ResourceFork"),
                                (xattr_supported, "user.synthetic.unrelated_name_never_emit")]:
            if supported:
                installed.append(name)
        before = {path: preserved_state(path, installed if path == regular else ())
                  for path in [regular, source / "empty.synthetic", source / "duplicate.synthetic",
                               source / "different.synthetic", nested / "nested_content_sentinel"]}
        map_path, record_path, result, completed = invoke(source, output, binary)
        saved_digest = hashlib.sha256(record_path.read_bytes()).hexdigest()
        checked("complete intake acceptance", accepted(completed, map_path, record_path, saved_digest))
        for path, state in before.items():
            checked("source bytes size and installed metadata preserved: " + path.name,
                    preserved_state(path, installed if path == regular else ()) == state)
        private_map = json.loads(map_path.read_text())
        by_id = {row["artifact_id"]: row for row in result}
        name_to_id = {base64.urlsafe_b64decode(e["basename_encoding"] + "=" * (-len(e["basename_encoding"]) % 4)).decode(): e["artifact_id"] for e in private_map["entries"]}
        source_file = pathlib.Path(__file__).parents[1] / "Sources" / "main.swift"
        checked("embedded source identity", private_map["helper_source_sha256"] == hashlib.sha256(source_file.read_bytes()).hexdigest())
        checked("embedded binary identity", private_map["helper_binary_sha256"] == hashlib.sha256(pathlib.Path(binary).read_bytes()).hexdigest())

        ordinary = by_id[name_to_id[PRIVATE_NAME]]
        empty = by_id[name_to_id["empty.synthetic"]]
        duplicate = by_id[name_to_id["duplicate.synthetic"]]
        different = by_id[name_to_id["different.synthetic"]]
        checked("ordinary regular file", ordinary["status"] == "INSPECTED" and ordinary["size_bytes"] == len(DATA_SENTINEL))
        checked("empty regular file", empty["sha256"] == hashlib.sha256(b"").hexdigest() and empty["eligibility_hint"] == "EMPTY_REGULAR_FILE")
        checked("symlink rejection", by_id[name_to_id["symlink_sentinel"]]["status"] == "REJECTED_NO_FOLLOW")
        checked("directory rejection", by_id[name_to_id["nested_directory_sentinel"]]["status"] == "REJECTED_DIRECTORY")
        checked("no recursion", "nested_content_sentinel" not in map_path.read_text())
        checked("FinderInfo absent", by_id[name_to_id["different.synthetic"]]["finder_info"]["state"] == "ABSENT")
        if finder_supported:
            checked("FinderInfo present", ordinary["finder_info"] == {"state": "PRESENT", "type_hex": "001f7f80", "creator_hex": "41424344"})
            checked("FinderInfo tail concealed", finder_value[8:] not in record_path.read_bytes())
            checked("nonprintable Type Creator encoding", ordinary["finder_info"]["type_hex"] == "001f7f80")
        else:
            skipped("FinderInfo present/tail/nonprintable preservation: setup unavailable")
        checked("resource fork absent", by_id[name_to_id["different.synthetic"]]["resource_fork"]["pre"] == "ABSENT")
        if resource_supported:
            checked("resource fork present", ordinary["resource_fork"]["pre"] == "PRESENT")
        else:
            skipped("resource fork presence/privacy/preservation: setup unavailable")
        visible = record_path.read_bytes()
        if xattr_supported:
            checked("unrelated xattr nonexposure", b"unrelated_name_never_emit" not in visible and XATTR_SENTINEL not in visible)
        else:
            skipped("unrelated xattr privacy/preservation: setup unavailable")
        checked("duplicate digests", ordinary["sha256"] == duplicate["sha256"])
        checked("different digests", ordinary["sha256"] != different["sha256"])
        checked("filename privacy", PRIVATE_NAME.encode() not in visible)
        checked("closed record schema", all("root_path" not in row and "basename_utf8" not in row and "root_inode" not in row for row in result))
        checked("output permissions", stat.S_IMODE(map_path.stat().st_mode) == 0o600 and stat.S_IMODE(record_path.stat().st_mode) == 0o600)

        prior_map_bytes = map_path.read_bytes()
        collision = run(binary, "enumerate", "--root", source, "--map-output", map_path)
        checked("output collision no overwrite", collision.returncode != 0 and collision.stderr == b"OS1_METADATA_HELPER_ERROR:OUTPUT_EXISTS\n")
        checked("existing MAP bytes preserved", map_path.read_bytes() == prior_map_bytes)
        prior_record_bytes = record_path.read_bytes()
        record_collision = run(binary, "inspect", "--map", map_path, "--record-output", record_path)
        checked("existing RECORD bytes preserved", record_collision.returncode != 0 and
                record_collision.stderr == b"OS1_METADATA_HELPER_ERROR:OUTPUT_EXISTS\n" and
                record_path.read_bytes() == prior_record_bytes)
        checked("collision not accepted", not accepted(record_collision, map_path, record_path, saved_digest))
        for label, payload in [
            ("truncated", prior_record_bytes[:len(prior_record_bytes) // 2]),
            ("missing row", b"\n".join(prior_record_bytes.splitlines()[:-1]) + b"\n"),
            ("altered", prior_record_bytes.replace(b'"VERIFIED"', b'"FAILED"', 1)),
        ]:
            damaged = output / (label.replace(" ", "-") + ".jsonl")
            damaged.write_bytes(payload)
            checked(label + " saved record refused", not accepted(completed, map_path, damaged, saved_digest))
            if label != "truncated":
                checked(label + " accounting/status refused even with new digest",
                        not accepted(completed, map_path, damaged, hashlib.sha256(payload).hexdigest()))
        checked("missing saved record refused", not accepted(completed, map_path, output / "absent", saved_digest))
        malformed = run(binary, "bad-mode", "--root", source, "--map-output", output / "unused")
        checked("bounded error grammar", malformed.stderr == b"OS1_METADATA_HELPER_ERROR:INVALID_ARGUMENTS\n" and PRIVATE_NAME.encode() not in malformed.stderr)

        tampered = output / "tampered-map.json"
        tampered_data = json.loads(map_path.read_text())
        tampered_data["entry_count"] += 1
        tampered.write_text(json.dumps(tampered_data))
        os.chmod(tampered, 0o600)
        tamper_result = run(binary, "inspect", "--map", tampered, "--record-output", output / "tampered-record")
        checked("map tampering", tamper_result.returncode != 0 and tamper_result.stderr.startswith(b"OS1_METADATA_HELPER_ERROR:MAP_"))

        replacement_source = test_root / "replacement-source"
        replacement_out = test_root / "replacement-output"
        replacement_source.mkdir(mode=0o700); replacement_out.mkdir(mode=0o700)
        (replacement_source / "one").write_bytes(b"one")
        replacement_map = replacement_out / "map.json"
        require(run(binary, "enumerate", "--root", replacement_source, "--map-output", replacement_map).returncode == 0, "replacement enumerate")
        moved = test_root / "old-root"
        replacement_source.rename(moved)
        replacement_source.mkdir(mode=0o700)
        root_change = run(binary, "inspect", "--map", replacement_map, "--record-output", replacement_out / "record")
        checked("root replacement", root_change.stderr == b"OS1_METADATA_HELPER_ERROR:MAP_ROOT_IDENTITY_CHANGED\n")

        # Replace timing guesses with deterministic between-observation changes.
        # These are test mutations, NOT helper-induced changes or in-read race proof.
        changed_source = test_root / "changed-source"
        changed_out = test_root / "changed-output"
        changed_source.mkdir(mode=0o700); changed_out.mkdir(mode=0o700)
        changed_file = changed_source / "one"
        changed_file.write_bytes(b"AAAA")
        _, _, initial_rows, _ = invoke(changed_source, changed_out, binary)
        changed_map = changed_out / "map.json"
        changed_file.write_bytes(b"BBBB")
        changed_record = changed_out / "after-mutation.jsonl"
        change_result = run(binary, "inspect", "--map", changed_map, "--record-output", changed_record)
        checked("test-induced same-size change detected by pre/post identity comparison",
                change_result.returncode == 0 and records(changed_record)[0]["sha256"] != initial_rows[0]["sha256"])
        replacement = changed_source / "replacement"
        replacement.write_bytes(b"CCCC")
        os.replace(replacement, changed_file)
        replaced_record = changed_out / "after-replacement.jsonl"
        replace_result = run(binary, "inspect", "--map", changed_map, "--record-output", replaced_record)
        checked("test-induced replacement detected by pre/post identity comparison",
                replace_result.returncode == 0 and records(replaced_record)[0]["sha256"] != initial_rows[0]["sha256"])
        skipped("in-read mutation/replacement races: overlap not established; not claimed")

        special_source = test_root / "special-source"
        special_out = test_root / "special-output"
        special_source.mkdir(mode=0o700); special_out.mkdir(mode=0o700)
        alias = special_source / "alias"
        alias.write_bytes(b"SYNTHETIC_ALIAS_BODY_NEVER_EMIT")
        alias_supported = True
        try:
            set_xattr(alias, "com.apple.FinderInfo", b"alisTEST" + bytes(24))
        except OSError:
            alias_supported = False
            skipped("Finder alias refusal: metadata setup unavailable")
        fifo_supported = True
        try:
            os.mkfifo(special_source / "fifo", 0o600)
        except (OSError, AttributeError):
            fifo_supported = False
            skipped("FIFO refusal: setup unavailable")
        alias_before = preserved_state(alias, ["com.apple.FinderInfo"] if alias_supported else [])
        _, special_record, special_rows, special_result = invoke(special_source, special_out, binary)
        if alias_supported:
            alias_row = special_rows[0]
            checked("Finder alias refused without successful observation",
                    alias_row["status"] == "REJECTED_ALIAS_TYPE" and
                    alias_row["integrity_status"] == "NOT_INSPECTED" and "sha256" not in alias_row)
            checked("alias body concealed", b"SYNTHETIC_ALIAS_BODY_NEVER_EMIT" not in
                    special_record.read_bytes() + special_result.stdout + special_result.stderr)
        checked("alias fixture bytes size and installed metadata preserved",
                preserved_state(alias, ["com.apple.FinderInfo"] if alias_supported else []) == alias_before)
        if fifo_supported:
            checked("FIFO refused within timeout", special_rows[-1]["kind"] == "FIFO" and
                    special_rows[-1]["status"] == "REJECTED_UNSUPPORTED_KIND")

        unreadable_source = test_root / "unreadable-source"
        unreadable_out = test_root / "unreadable-output"
        unreadable_source.mkdir(mode=0o700); unreadable_out.mkdir(mode=0o700)
        (unreadable_source / "a-good").write_bytes(b"good synthetic")
        unreadable = unreadable_source / "b-denied"
        unreadable.write_bytes(b"denied synthetic")
        (unreadable_source / "c-unvisited").write_bytes(b"unvisited synthetic")
        unreadable.chmod(0)
        unreadable_map = unreadable_out / "map.json"
        require(run(binary, "enumerate", "--root", unreadable_source, "--map-output", unreadable_map).returncode == 0, "unreadable enumerate")
        denied = run(binary, "inspect", "--map", unreadable_map, "--record-output", unreadable_out / "record")
        if denied.returncode == 0:
            skipped("unreadable/partial ERROR record: host bypasses file permissions")
        else:
            checked("unreadable closed error", denied.stderr == b"OS1_METADATA_HELPER_ERROR:OPEN_FAILED\n")
            partial_path = unreadable_out / "record"
            partial = records(partial_path)
            checked("nonzero helper leaves partial ERROR evidence", len(partial) == 2 and
                    partial[0]["status"] == "INSPECTED" and partial[1]["status"] == "ERROR" and
                    partial[1]["integrity_status"] == "FAILED")
            checked("partial file existence is not completion",
                    not accepted(denied, unreadable_map, partial_path,
                                 hashlib.sha256(partial_path.read_bytes()).hexdigest()))
        unreadable.chmod(0o600)

        # atime behavior may be filesystem-policy dependent; the schema must report it separately.
        checked("access time separate", ordinary["atime_changed"] in {"YES", "NO", "UNAVAILABLE"} and ordinary["integrity_status"] == "VERIFIED")
        checked("two-pass digest result", ordinary["sha256"] == hashlib.sha256(DATA_SENTINEL).hexdigest())
        checked("no helper subprocess surface", b"Process" not in source_file.read_bytes())

        all_visible = visible + special_record.read_bytes()
        for outcome in [completed, collision, record_collision, malformed, tamper_result,
                        root_change, change_result, replace_result, special_result, denied]:
            all_visible += outcome.stdout + outcome.stderr
        if denied.returncode != 0:
            all_visible += (unreadable_out / "record").read_bytes()
        forbidden = [DATA_SENTINEL, PRIVATE_NAME.encode(), str(source).encode(), b"nested_content_sentinel"]
        if finder_supported: forbidden.append(finder_value[8:])
        if resource_supported: forbidden.append(FORK_SENTINEL)
        if xattr_supported: forbidden.append(XATTR_SENTINEL)
        checked("forbidden-output sentinel scan", all(value not in all_visible for value in forbidden))
        checked("temporary filename privacy", all(PRIVATE_NAME not in item.name for item in output.iterdir()))
        for path, state in before.items():
            checked("source preserved after failure/collision checks: " + path.name,
                    preserved_state(path, installed if path == regular else ()) == state)

        print(json.dumps({"status": "PASS", "passed": len(checks), "failed": 0,
                          "unavailable": unavailable, "unavailable_count": len(unavailable)}, sort_keys=True))
    except Exception as error:
        print(json.dumps({"status": "FAIL", "passed": len(checks), "failed": 1,
                          "failure": "helper timeout" if isinstance(error, subprocess.TimeoutExpired) else str(error),
                          "unavailable": unavailable}, sort_keys=True))
        raise
    finally:
        shutil.rmtree(test_root)


if __name__ == "__main__":
    main()
