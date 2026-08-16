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
- [x] Implement a bounded diagnostic-only Patch decoder spike for the known
      Track 3 #2 representation and validate Experiments 007 and 023–027.
- [x] Validate the explicitly bounded representation on an independently
      identified real Patch event: Track 3 / JV-1080 repeats core semantic
      fields but differs in local context, so the unchanged decoder rejects it.
- [x] Compare a third independently identified Patch event: Track 1 /
      Juno-106 confirms the common semantic core and distinguishes variable
      framing, bank-tail data, and compound pre-Note timing context.
- [x] Design a bounded representation-oriented Patch contract with explicit
      start/post-Note bounds, payload-derived PC, preserved opaque context, and
      a predefined authentic/controlled/malformed test matrix.
- [x] Implement the bounded representation decoder and authentic, controlled,
      malformed test matrix while preserving the independent strict Track 3
      #2 decoder.
- [x] Validate Track 2 / JV-1080 / PC 37 with independently established bounds;
      it passes unchanged, matches compact Track 3 framing, and strengthens the
      bank-tail correlation.
- [x] Isolate the bank-correlated post-name tail with a controlled CC32-only
      save: Experiment 028 confirms aligned `ff 51 01 -> ff 51 02` while the
      Patch core and complete Track 2 Note chain remain stable.
- [x] Change only Track 2 CC0/MSB in a fresh baseline duplicate: Experiment
      029 confirms aligned `ff 51 01 -> ff 52 01`, independently establishing
      direct CC0 storage alongside Experiment 028's direct CC32 result.
- [ ] Remove Bank Select from Track 2 in one controlled save to test whether
      `ff 51 01 -> ff ff ff` before designing optional bank semantics.
- [x] Inventory MIDI event families across all seven available Project 001
      exports using a non-duplicative three-export-set view; document 6,109
      Notes, 5,112 Controllers across 14 CC numbers, 440 Pitch Bends, 38
      Program Changes, and two SysEx events, with source provenance tracked
      separately.
- [x] Correct export provenance from authoritative Studio Vision UI evidence:
      only `Ode to Clarke` is mapped to `newest STUFF`; `ANALOG.MID #2` and
      `BATTL2GS.MID` remain source-unresolved.
- [x] Create one provenance-controlled multitrack export from a known,
      controller-rich active `newest STUFF` sequence, recording sequence and
      track identity at export time: `Bells for her` supplies 395 ordinary
      Controllers after excluding ten Patch-derived bank messages.
- [x] Investigate one shared Controller-event representation using that
      provenance-controlled mixed-controller region: CC1 and CC7 share `timing
      VLQ | ff 41 | 05 | opaque context[3] | number | value` across all 395
      natural records.
- [x] Implement the caller-bounded ordinary Controller decoder and authentic
      regression fixtures, with exact consumption, opaque context, and no
      scanning or timeline reconstruction.
- [x] Design caller/container integration that supplies established Controller
      bounds and event-start accumulation without heuristic discovery.
- [x] Correlate the already-bounded Track 9 mixed stream event-by-event to
      identify the first expected Channel Pressure project record and test its
      current-cursor discriminator/length using natural evidence only: 32/32
      timing/value matches establish one `d0`-entered stateful run.
- [x] Design an exact-bounded, state-aware Channel Pressure run contract that
      preserves provenance and never classifies continuations statelessly.
- [x] Implement the bounded Channel Pressure run decoder and authentic plus
      malformed fixtures without adding mixed-stream discovery.
- [ ] Correlate the provenance-controlled `Bells for her` Pitch Bend population
      in a naturally bounded/aligned project region before designing another
      stateful family decoder or revisiting mixed walking.
- [ ] Defer the source-unresolved `ANALOG.MID #2` Pitch Bend curves until their
      project/sequence provenance is established.
- [ ] Defer the controlled no-bank/sentinel experiment until it outranks
      Controller, Pitch Bend, Tempo/Meter, or SysEx recovery value.

This milestone is diagnostic only. It does not establish general Studio Vision
parsing or emit MIDI.

## Long-term product vision

Subject to evidence-based format research, Phoenix aims to:

- Recover raw MIDI and audio.
- Reconstruct project structure and routing.
- Offer user-approved modern instrument mappings.
- Export to modern DAW workflows.

Automatic instrument substitution must never occur without user approval.
