#!/usr/bin/env python3
"""Deterministic synthetic acceptance tests; no reserve paths."""
import hashlib
import json
import pathlib
import os
import stat
import subprocess
import tempfile
import unittest

from accept_inspect import Rejected, accept, validate_rows


def hash_file(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class AcceptanceTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="os1-acceptance-")
        self.addCleanup(self.tmp.cleanup)
        self.base = pathlib.Path(self.tmp.name)
        self.root = self.base / "source"
        self.root.mkdir()
        (self.root / "synthetic").write_bytes(b"synthetic bytes")
        self.binary = self.base / "fake-helper"
        self.binary.write_text("""#!/usr/bin/env python3
import hashlib, json, pathlib, sys
args = sys.argv
inventory = json.loads(pathlib.Path(args[args.index('--map') + 1]).read_bytes())
dest = pathlib.Path(args[args.index('--record-output') + 1])
rows = []
for entry in inventory['entries']:
    rows.append({'schema_version':'OS1_METADATA_HELPER_RECORD_V1',
      'helper_binary_sha256':inventory['helper_binary_sha256'],
      'helper_source_sha256':inventory['helper_source_sha256'],
      'artifact_id':entry['artifact_id'], 'kind':'REGULAR_FILE',
      'status':'INSPECTED', 'integrity_status':'VERIFIED',
      'size_bytes':(pathlib.Path(inventory['root_path']) / 'synthetic').stat().st_size,
      'eligibility_hint':'NONE', 'atime_changed':'NO',
      'sha256':hashlib.sha256((pathlib.Path(inventory['root_path']) / 'synthetic').read_bytes()).hexdigest(),
      'finder_info':{'state':'ABSENT'}, 'resource_fork':{'pre':'ABSENT','post':'ABSENT','unchanged':True}})
dest.write_text(''.join(json.dumps(row) + '\\n' for row in rows))
""")
        self.binary.chmod(0o700)
        self.map = self.base / "map.json"
        self.inventory = {"schema_version": "OS1_METADATA_HELPER_MAP_V1", "helper_binary_sha256": hash_file(self.binary),
                          "helper_source_sha256": "a" * 64, "root_path": str(self.root), "entry_count": 1,
                          "root_device": str(self.root.stat().st_dev), "root_inode": str(self.root.stat().st_ino),
                          "entries": [{"artifact_id": "R-000001", "basename_encoding": "c3ludGhldGlj", "kind": "REGULAR_FILE"}]}
        self.save_map()
        self.record = self.base / "record.jsonl"
        self.expect = {"executable_sha256": hash_file(self.binary), "map_sha256": hash_file(self.map)}

    def save_map(self):
        self.map.write_text(json.dumps(self.inventory))

    def check_rejected(self):
        with self.assertRaises(Rejected):
            accept(str(self.binary), str(self.map), str(self.record), self.expect)

    def test_valid(self):
        result = accept(str(self.binary), str(self.map), str(self.record), self.expect)
        self.assertEqual(result["status"], "ACCEPTED")
        self.assertEqual(result["record_sha256"], hash_file(self.record))
        self.assertEqual(result["source_before_sha256"], result["source_after_sha256"])

    def test_wrong_executable_hash(self):
        self.expect["executable_sha256"] = "0" * 64
        self.check_rejected()

    def test_altered_map_even_with_new_internal_digest(self):
        self.inventory["canonical_map_sha256"] = "f" * 64
        self.save_map()
        self.check_rejected()

    def test_altered_record(self):
        self.assertEqual(accept(str(self.binary), str(self.map), str(self.record), self.expect)["status"], "ACCEPTED")
        original = hash_file(self.record)
        self.record.write_bytes(self.record.read_bytes() + b"x")
        self.assertNotEqual(hash_file(self.record), original)
        with self.assertRaises((Rejected, ValueError)):
            validate_rows(self.inventory, self.record.read_bytes(), hash_file(self.binary))

    def test_nonzero_exit(self):
        self.binary.write_text("#!/usr/bin/env python3\nraise SystemExit(7)\n")
        self.binary.chmod(0o700)
        self.refreeze()
        self.check_rejected()

    def refreeze(self):
        self.inventory["helper_binary_sha256"] = hash_file(self.binary)
        self.save_map()
        self.expect = {"executable_sha256": hash_file(self.binary), "map_sha256": hash_file(self.map)}

    def test_missing_or_duplicate_ids(self):
        for value in ("R-000003", "R-000001"):
            self.inventory["entries"].append(dict(self.inventory["entries"][0], artifact_id=value))
            self.inventory["entry_count"] = 2
            self.save_map()
            self.expect["map_sha256"] = hash_file(self.map)
            self.check_rejected()
            self.inventory["entries"].pop()

    def test_partial_record_and_error(self):
        self.assertEqual(accept(str(self.binary), str(self.map), str(self.record), self.expect)["status"], "ACCEPTED")
        with self.assertRaises(Rejected):
            validate_rows(self.inventory, b"", hash_file(self.binary))
        self.record.unlink()
        script = self.binary.read_text().replace("'status':'INSPECTED'", "'status':'ERROR'")
        self.binary.write_text(script)
        self.refreeze()
        self.check_rejected()

    def test_changed_source(self):
        script = self.binary.read_text().replace("dest.write_text", "(pathlib.Path(inventory['root_path']) / 'synthetic').write_bytes(b'changed')\ndest.write_text")
        self.binary.write_text(script)
        self.refreeze()
        self.check_rejected()

    def test_missing_or_ambiguous_expectations(self):
        for expectations in ({}, {**self.expect, "other": "0" * 64}):
            self.expect = expectations
            self.check_rejected()

    def test_malformed_map_name_and_source_symlink(self):
        self.inventory["entries"][0]["basename_encoding"] = "Li4vZXNjYXBl"
        self.save_map()
        self.expect["map_sha256"] = hash_file(self.map)
        self.check_rejected()
        self.inventory["entries"][0]["basename_encoding"] = "c3ludGhldGlj"
        self.save_map()
        self.expect["map_sha256"] = hash_file(self.map)
        (self.root / "synthetic").unlink()
        (self.root / "synthetic").symlink_to(self.binary)
        self.check_rejected()

    def test_inconsistent_inspected_row(self):
        result = accept(str(self.binary), str(self.map), str(self.record), self.expect)
        self.assertEqual(result["status"], "ACCEPTED")
        rows = [json.loads(line) for line in self.record.read_text().splitlines()]
        rows[0]["sha256"] = "0" * 64
        self.record.write_text(json.dumps(rows[0]) + "\n")
        from accept_inspect import source_state
        with self.assertRaises(Rejected):
            validate_rows(self.inventory, self.record.read_bytes(), hash_file(self.binary), source_state(self.inventory))

    def test_private_exclusive_acceptance_output(self):
        expectations = self.base / "expectations.json"
        expectations.write_text(json.dumps(self.expect))
        output = self.base / "accepted.json"
        cmd = ["python3", str(pathlib.Path(__file__).with_name("accept_inspect.py")),
               "--binary", str(self.binary), "--map", str(self.map),
               "--record-output", str(self.record), "--expectations", str(expectations),
               "--acceptance-output", str(output)]
        subprocess.run(cmd, check=True, capture_output=True)
        self.assertEqual(stat.S_IMODE(output.stat().st_mode), 0o600)
        original = output.read_bytes()
        repeated = subprocess.run(cmd, capture_output=True)
        self.assertNotEqual(repeated.returncode, 0)
        self.assertEqual(output.read_bytes(), original)


if __name__ == "__main__":
    unittest.main()
