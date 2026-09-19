#!/usr/bin/env python3
"""Synthetic rehearsal / separately authorized private OS1 inspect acceptance.

The expectation file must be frozen outside this command. Never derive its hashes
from the MAP or executable being accepted in the same run.
"""
import argparse
import base64
import binascii
import ctypes
import hashlib
import json
import os
import pathlib
import stat
import subprocess
import sys


class Rejected(Exception):
    pass


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_regular(path):
    if not stat.S_ISREG(os.lstat(path).st_mode):
        raise Rejected("evidence is not a regular file")
    return pathlib.Path(path).read_bytes()


def digest(value):
    return isinstance(value, str) and len(value) == 64 and all(c in "0123456789abcdef" for c in value)


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise Rejected("duplicate JSON key")
        result[key] = value
    return result


def parse_json(data):
    return json.loads(data, object_pairs_hook=unique_object)


def xattr(fd, name, length_only=False):
    libc = ctypes.CDLL("/usr/lib/libSystem.B.dylib", use_errno=True)
    get = libc.fgetxattr
    get.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_void_p,
                    ctypes.c_size_t, ctypes.c_uint32, ctypes.c_int]
    get.restype = ctypes.c_ssize_t
    size = get(fd, name, None, 0, 0, 0)
    if size < 0:
        code = ctypes.get_errno()
        if code == 93:  # ENOATTR on Darwin
            return None
        raise OSError(code, "source metadata probe failed")
    if length_only:
        return size
    buffer = ctypes.create_string_buffer(size)
    actual = get(fd, name, buffer, size, 0, 0)
    if actual != size:
        raise Rejected("source metadata changed during probe")
    return buffer.raw[:actual]


def source_state(inventory):
    root = inventory["root_path"]
    if not isinstance(root, str) or not os.path.isabs(root) or not isinstance(inventory.get("entries"), list):
        raise Rejected("invalid MAP root or entries")
    result = []
    seen_names = set()
    root_fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        root_info = os.fstat(root_fd)
        if str(root_info.st_dev) != inventory.get("root_device") or str(root_info.st_ino) != inventory.get("root_inode"):
            raise Rejected("MAP root identity mismatch")
        for entry in inventory["entries"]:
            if not isinstance(entry, dict) or not isinstance(entry.get("basename_encoding"), str):
                raise Rejected("invalid MAP entry")
            encoded = entry["basename_encoding"]
            try:
                name = base64.b64decode(encoded + "=" * (-len(encoded) % 4), altchars=b"-_", validate=True)
            except (binascii.Error, ValueError):
                raise Rejected("invalid MAP basename")
            if (not name or name in (b".", b"..") or b"/" in name or b"\0" in name or
                    base64.urlsafe_b64encode(name).decode().rstrip("=") != encoded):
                raise Rejected("invalid MAP basename")
            if name in seen_names:
                raise Rejected("duplicate MAP basename")
            seen_names.add(name)
            info = os.stat(name, dir_fd=root_fd, follow_symlinks=False)
            kind = ("REGULAR_FILE" if stat.S_ISREG(info.st_mode) else "SYMLINK" if stat.S_ISLNK(info.st_mode)
                    else "DIRECTORY" if stat.S_ISDIR(info.st_mode) else "FIFO" if stat.S_ISFIFO(info.st_mode)
                    else "SOCKET" if stat.S_ISSOCK(info.st_mode) else "CHARACTER_DEVICE" if stat.S_ISCHR(info.st_mode)
                    else "BLOCK_DEVICE" if stat.S_ISBLK(info.st_mode) else "OTHER")
            if kind != entry.get("kind"):
                raise Rejected("MAP source kind mismatch")
            if kind == "REGULAR_FILE":
                fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=root_fd)
                try:
                    opened = os.fstat(fd)
                    if (opened.st_dev, opened.st_ino, opened.st_mode) != (info.st_dev, info.st_ino, info.st_mode):
                        raise Rejected("source object changed")
                    hasher = hashlib.sha256()
                    while chunk := os.read(fd, 1024 * 1024):
                        hasher.update(chunk)
                    content = hasher.hexdigest()
                    finder_value = xattr(fd, b"com.apple.FinderInfo")
                    finder = finder_value[:8].hex() if finder_value is not None else None
                    fork_value = xattr(fd, b"com.apple.ResourceFork", length_only=True)
                    fork = fork_value is not None
                finally:
                    os.close(fd)
            else:
                content, finder, fork = None, None, None
            result.append((entry["artifact_id"], info.st_dev, info.st_ino, info.st_mode,
                           info.st_size, info.st_mtime_ns, info.st_ctime_ns,
                           content, finder, fork))
    finally:
        os.close(root_fd)
    return result


def validate_rows(inventory, saved, binary_hash, sources=None):
    if not saved or not saved.endswith(b"\n"):
        raise Rejected("missing or truncated RECORD")
    entries = inventory["entries"]
    if inventory["entry_count"] != len(entries):
        raise Rejected("MAP count mismatch")
    ids = [entry["artifact_id"] for entry in entries]
    if ids != [f"R-{n:06d}" for n in range(1, len(ids) + 1)]:
        raise Rejected("missing or duplicate MAP neutral ID")
    rows = [parse_json(line) for line in saved.splitlines()]
    if len(rows) != len(ids) or [row["artifact_id"] for row in rows] != ids:
        raise Rejected("missing or duplicate RECORD neutral ID")
    for index, (entry, row) in enumerate(zip(entries, rows)):
        if not isinstance(row, dict):
            raise Rejected("invalid RECORD row")
        if row.get("schema_version") != "OS1_METADATA_HELPER_RECORD_V1" or row.get("helper_binary_sha256") != binary_hash or row.get("helper_source_sha256") != inventory.get("helper_source_sha256"):
            raise Rejected("RECORD identity mismatch")
        kind = entry["kind"]
        status = row.get("status")
        if status == "ERROR":
            raise Rejected("ERROR row")
        if status == "INSPECTED":
            if (kind != "REGULAR_FILE" or row.get("kind") != kind or row.get("integrity_status") != "VERIFIED"
                    or not digest(row.get("sha256")) or row.get("resource_fork", {}).get("unchanged") is not True
                    or row.get("resource_fork", {}).get("pre") != row.get("resource_fork", {}).get("post")
                    or row.get("finder_info", {}).get("state") not in ("PRESENT", "ABSENT")
                    or row.get("eligibility_hint") != ("EMPTY_REGULAR_FILE" if row.get("size_bytes") == 0 else "NONE")
                    or row.get("atime_changed") not in ("YES", "NO")):
                raise Rejected("invalid inspected row")
            if sources is not None:
                source = sources[index]
                if row.get("sha256") != source[7] or row.get("size_bytes") != source[4]:
                    raise Rejected("inspected row disagrees with source evidence")
                finder = row["finder_info"]
                if (source[8] is None) != (finder["state"] == "ABSENT"):
                    raise Rejected("FinderInfo state mismatch")
                if source[8] is not None and (finder.get("type_hex") != source[8][:8] or finder.get("creator_hex") != source[8][8:16] or source[8][:8] == b"alis".hex()):
                    raise Rejected("FinderInfo identity mismatch")
                fork = row["resource_fork"]
                expected_fork = "PRESENT" if source[9] else "ABSENT"
                if fork.get("pre") != expected_fork or fork.get("post") != expected_fork:
                    raise Rejected("resource-fork state mismatch")
        else:
            expected = {"SYMLINK": "REJECTED_NO_FOLLOW", "DIRECTORY": "REJECTED_DIRECTORY"}.get(kind, "REJECTED_ALIAS_TYPE" if kind == "REGULAR_FILE" else "REJECTED_UNSUPPORTED_KIND")
            if status != expected or row.get("kind") != kind or row.get("error_code") != status or row.get("integrity_status") != "NOT_INSPECTED":
                raise Rejected("unexpected refusal status")
            if kind == "REGULAR_FILE" and sources is not None and (sources[index][8] is None or sources[index][8][:8] != b"alis".hex()):
                raise Rejected("alias refusal without alias evidence")


def accept(binary, map_path, record_path, expectations):
    if set(expectations) != {"executable_sha256", "map_sha256"} or not all(digest(v) for v in expectations.values()):
        raise Rejected("missing or ambiguous frozen expectations")
    binary_hash = sha(read_regular(binary))
    if binary_hash != expectations["executable_sha256"]:
        raise Rejected("executable hash mismatch")
    map_bytes = read_regular(map_path)
    if sha(map_bytes) != expectations["map_sha256"]:
        raise Rejected("MAP hash mismatch")
    inventory = parse_json(map_bytes)
    if inventory.get("schema_version") != "OS1_METADATA_HELPER_MAP_V1" or inventory.get("helper_binary_sha256") != binary_hash:
        raise Rejected("MAP executable identity mismatch")
    if os.path.lexists(record_path):
        raise Rejected("RECORD destination already exists")
    if not isinstance(inventory.get("entry_count"), int) or inventory["entry_count"] != len(inventory.get("entries", [])):
        raise Rejected("MAP count mismatch")
    ids = [entry.get("artifact_id") for entry in inventory["entries"]]
    if ids != [f"R-{n:06d}" for n in range(1, len(ids) + 1)]:
        raise Rejected("invalid MAP neutral IDs")
    before = source_state(inventory)
    command = [os.path.abspath(binary), "inspect", "--map", os.path.abspath(map_path), "--record-output", os.path.abspath(record_path)]
    completed = subprocess.run(command, capture_output=True, timeout=20, check=False)
    after = source_state(inventory)
    if before != after:
        raise Rejected("source evidence changed")
    if completed.returncode != 0 or completed.stdout or completed.stderr:
        raise Rejected("helper command failed")
    if sha(read_regular(binary)) != binary_hash or sha(read_regular(map_path)) != expectations["map_sha256"]:
        raise Rejected("input evidence changed")
    saved = read_regular(record_path)
    record_hash = sha(saved)
    validate_rows(inventory, saved, binary_hash, after)
    if sha(read_regular(record_path)) != record_hash:
        raise Rejected("RECORD changed during acceptance")
    return {"status": "ACCEPTED", "command": command, "exit_status": completed.returncode,
            "executable_sha256": binary_hash, "map_sha256": expectations["map_sha256"],
            "record_sha256": record_hash, "row_count": inventory["entry_count"],
            "source_before_sha256": sha(repr(before).encode()), "source_after_sha256": sha(repr(after).encode())}


def private_destination(path):
    parent = os.path.dirname(os.path.abspath(path))
    info = os.stat(parent, follow_symlinks=False)
    if not stat.S_ISDIR(info.st_mode) or info.st_mode & 0o077:
        raise Rejected("output parent is not private")
    if os.path.lexists(path):
        raise Rejected("output already exists")


def outside_source(path, root):
    target = os.path.realpath(os.path.dirname(os.path.abspath(path)))
    source = os.path.realpath(root)
    if os.path.commonpath((target, source)) == source:
        raise Rejected("output parent is inside source root")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", required=True)
    parser.add_argument("--map", required=True)
    parser.add_argument("--record-output", required=True)
    parser.add_argument("--expectations", required=True)
    parser.add_argument("--acceptance-output", required=True)
    args = parser.parse_args()
    try:
        private_destination(args.record_output)
        private_destination(args.acceptance_output)
        expectations = parse_json(read_regular(args.expectations))
        frozen_map = parse_json(read_regular(args.map))
        outside_source(args.record_output, frozen_map["root_path"])
        outside_source(args.acceptance_output, frozen_map["root_path"])
        result = accept(args.binary, args.map, args.record_output, expectations)
        output = pathlib.Path(args.acceptance_output)
        fd = os.open(output, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
        with os.fdopen(fd, "w") as stream:
            json.dump(result, stream, sort_keys=True)
            stream.write("\n")
            stream.flush()
            os.fsync(stream.fileno())
    except (Rejected, OSError, ValueError, KeyError, TypeError, subprocess.TimeoutExpired):
        print("OS1_ACCEPTANCE_REJECTED", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
