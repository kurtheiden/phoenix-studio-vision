# OS1 inspect evidence acceptance

This small runner accepts one saved helper MAP and executes one helper `inspect`
command. Use only synthetic material until separate authentic-intake authorization.
The independent review's BLOCKED verdict is unchanged.

First freeze, outside this runner, a private JSON file with exactly
`executable_sha256` and `map_sha256`, each a lowercase 64-character SHA-256.
Compute these from the exact reviewed executable and saved MAP using an
independent digest tool. Do not copy `helper_binary_sha256` or
`canonical_map_sha256` from the MAP into the expectations. Keep the executable,
MAP, expectations, RECORD destination, and acceptance output in reviewed,
resolved private locations. The MAP must already have been produced by a
separately recorded successful `enumerate` command; this runner does not
authenticate that earlier command.

```text
python3 tools/os1-metadata-helper/acceptance/accept_inspect.py \
  --binary /private/helper --map /private/map.json \
  --record-output /private/record.jsonl \
  --expectations /private/frozen-expectations.json \
  --acceptance-output /private/accepted.json
```

The runner checks exact executable and MAP bytes, compares MAP executable
identity to the independently hashed binary, snapshots each mapped source
before and after execution, records the exact inspect argv and zero exit,
requires a complete matching set of neutral IDs and valid row outcomes, and
independently hashes and rereads the saved RECORD. Its exclusive acceptance
output is created mode 0600 in a pre-existing private parent; the runner rejects
an existing destination, a nonprivate immediate parent, or an output parent
inside the source root. It uses no-follow opens for mapped regular files and
checks the MAP root and entry identities. Keep the RECORD and acceptance output private
and freeze their bytes separately for later human review. A rejected run may
leave a partial RECORD; preserve it as failure evidence and do not silently
retry or treat file existence as acceptance.

The source snapshot checks identity, mode, size, modification/change times,
complete regular-file data-fork SHA-256, Finder Type/Creator bytes, and
resource-fork presence. It excludes access time. It does not prove historical
source identity, eliminate concurrent filesystem races or every symlinked
ancestor, attest candidate-only
root contents, classify archives or disk images, establish path resolution or
quarantine, or authenticate the earlier enumerate run. Those remain separate
conditions for authentic intake. The unit suite uses only generated synthetic
files and a fake helper command; it tests acceptance logic, not the real
helper's metadata collection:

```text
python3 -m unittest discover -s tools/os1-metadata-helper/acceptance -p 'test_*.py'
```
