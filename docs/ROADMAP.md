# Roadmap

This roadmap is intentionally provisional. Evidence gathered during research
may change priorities.

## Milestone 1: format-neutral inspection

- [x] Accept one file path.
- [x] Report canonical identity and size.
- [x] Calculate SHA-256.
- [x] Display a 256-byte hexadecimal and ASCII preview.
- [x] Reject missing, unreadable, and empty inputs clearly.
- [x] Add automated tests using synthetic files.

## Milestone 2: Inspect the first real Studio Vision project — completed

- [x] Inspect the local sample identified as `newest-stuff-001`.
- [x] Record its size and SHA-256 digest.
- [x] Record direct observations without interpreting file structures.

## Milestone 3: Discovery Inspector — completed

- [x] Scan complete files for printable ASCII strings of at least four bytes.
- [x] Report string offsets, lengths, and literal values in file order.
- [x] Summarize string counts, the longest string, the percentage of bytes in
      reported printable strings, and whole-file Shannon entropy.
- [x] Cover discovery analysis and existing CLI behavior with automated tests.

## Milestone 4: research workflow

- [x] Add conservative, evidence-based Studio Vision identification from
      classic Mac Finder type and creator metadata.
- [x] Keep identification independent of filenames and structural parsing.
- [x] Report observation, evidence, and confidence separately.

- [ ] Define a lawful, redistributable fixture policy.
- [ ] Record observations from independently obtained samples.
- [ ] Add comparison-oriented inspection only where evidence supports it.
- [ ] Document repeatable research methods and confidence levels.

## Issue #6: Inventory Studio Vision Pro and OMS artifacts for file-format evidence

- [x] Catalog the Studio Vision Pro 4.5 and OMS 2.3.8 installers.
- [ ] Catalog the Studio Vision application, Studio Vision Setup, and OMS
      applications.
  - The Studio Vision application and Studio Vision Setup inventories are
    complete. OMS application inventory is blocked because no extracted
    payload is available and no safe extraction method has been identified.
- [ ] Catalog manuals, templates, plug-ins, and example projects.
- [ ] Record filenames, sources, sizes, and SHA-256 digests for each artifact.
- [ ] Record Finder type and creator metadata, resource information, printable
      strings, and evidence-based notes for each artifact.

## Later possibilities

Only after sufficient evidence exists, consider format recognition, structured
parsing, richer validation, or a graphical interface. None is currently
committed or specified.

## Milestone 5: bounded Track 7 event-chain parser spike

- [x] Add a bounded 7-bit big-endian VLQ decoder with focused malformed-input
      tests.
- [x] Decode the evidence-backed timing/property/duration structure only within
      an explicit caller-supplied range.
- [x] Validate the baseline fixture and position-control fixtures.
- [x] Validate the authentic Experiment 007, 019, 020, and 022 artifacts
      read-only.
- [x] Verify the formerly provisional third structure against the documented
      sixth List Window event.
- [x] Manually verify preregistered rows 7–11 against the screenshot ground
      truth before extending the parser's evidence range.
- [x] Validate the supplied complete visible Track 7 List Window against the
      bounded chain: 143 rows, 140 aligned candidates, 560/560 fields.
- [x] Account locally for rows 1–3 property structures and characterize the
      conservative post-row-143 stop.
- [ ] Establish complete Track 7 framing before considering broader parsing.
- [x] Survey nearby repeated candidate event-chain/container contexts without
      implementing automatic discovery.
- [x] Correct Track 3 #2 to the `0x318b5` chain and validate all 84 note rows
      plus 83 note-to-note intervals against complete List Window ground truth.
- [x] Reconcile 85 UI events as one Patch plus 84 notes and withdraw the exact
      event-count interpretation because the analogous binary value is 86.
- [ ] Complete the first Patch event's start/type framing and field ownership
      before implementing general mixed-event decoding; Experiments 025–027
      identify position, variable-length name, PC, and local size behavior.
- [x] Confirm the Track 3 #2 direct Program Change byte with Experiment 023:
      `PC 23 -> PC 24` produces `0x318a5: 17 -> 18` with notes unchanged.
- [x] Replicate the Program Change field using a deliberately non-adjacent
      value: Experiment 024 confirms `PC 23 -> PC 100` as aligned
      `0x17 -> 0x64`, with Patch name and all 84 notes unchanged.
- [x] Isolate the Patch-event absolute timing representation with a
      position-only change: Experiment 025 confirms `84 12` = 530 to
      `84 13` = 531 at `0x31886–0x31887`.
- [x] Change only the Patch name to a same-length value: Experiment 026
      confirms the direct ASCII payload at `0x31891–0x3189c`.
- [x] Change only the Patch name to a shorter value: Experiment 027 confirms
      `0c -> 07`, variable-length storage, `-5` relocation, and size updates.
- [ ] Implement a bounded diagnostic-only Patch decoder spike for the known
      Track 3 #2 representation and validate Experiments 007 and 023–027.

This milestone is diagnostic only. It does not establish general Studio Vision
parsing or emit MIDI.

## Long-term product vision

Subject to evidence-based format research, Phoenix aims to:

- Recover raw MIDI and audio.
- Reconstruct project structure and routing.
- Offer user-approved modern instrument mappings.
- Export to modern DAW workflows.

Automatic instrument substitution must never occur without user approval.
