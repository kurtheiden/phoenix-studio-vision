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
- [x] Correlate the provenance-controlled `Bells for her` Pitch Bend population
      in a naturally bounded/aligned project region before designing another
      stateful family decoder or revisiting mixed walking: 102 events form nine
      exact `e0`-entered runs with direct LSB/MSB storage and 102/102 agreement.
- [x] Design an exact caller-bounded, state-aware Pitch Bend run decoder that
      preserves both value bytes and does not discover run ends.
- [x] Implement the bounded Pitch Bend run decoder with all nine authentic
      ranges and focused malformed fixtures, without mixed-stream discovery.
- [x] Correlate the provenance-controlled `Bells for her` Tempo event with the
      corrected controlled tempo-save evidence: the exact initial form is
      `00 ff 51 03 | unsigned 24-bit big-endian MPQN`.
- [x] Design an exact caller-bounded seven-byte initial Tempo decoder with
      byte provenance, no scanning, and no general position or Tempo-map claim.
- [x] Implement the bounded initial Tempo decoder with fixed natural and
      controlled fixtures, absolute provenance for all seven bytes, derived
      MPQN/optional BPM, exact-bound failures, and no scanning.
- [x] Correlate the adjacent Meter structure using natural 4/4/6/8/10/8,
      controlled 7/8, and provenance-controlled exports: establish the exact
      initial `00 ff 58 04 nn dd xx yy` form and bounded historical `cc`
      mappings without requiring another Meter experiment.
- [x] Design an exact caller-bounded eight-byte initial Meter decoder that
      preserves every byte, derives denominator safely, does not scan, and
      keeps SMF export policy separate.
- [x] Implement the bounded initial Meter decoder with fixed 4/4, 6/8, 7/8,
      and project-only 10/8 fixtures plus focused malformed/no-scanning tests.
- [x] Correlate all 18 sequence/container structures: establish the 208-byte
      preamble, descriptor count/stride, Pascal name, big-endian record lengths,
      Meter/Tempo bounds, track primary containers, and terminal-to-next-start
      chain without changing production code.
- [x] Resolve the project-root entry point: preserve the opaque eight-byte root
      header, then use checked top-level record lengths to reach the first
      type-`0x01` sequence preamble and exact EOF without signature scanning.
- [x] Design the narrow read-only root-record/sequence-container parser with
      deterministic errors, absolute provenance, a mismatch-safe Sequence I
      representation, and an explicit 166-byte descriptor profile.
- [x] Implement that bounded design with generic exact-EOF framing, strict
      166-byte sequence validation, authentic 527/18 and older 495-record
      fixtures, mismatch-safe Sequence I handling, and no scanning.
- [x] Correlate exact track-local event termination and family transitions:
      establish the validated seven-byte tail and exact 166-profile event end,
      while retaining partial Note and Pressure/Bend state exits.
- [x] Resolve the Patch-to-first-Note handoff with Experiment 031: confirm
      `81 25 -> 81 26`, separate the length-framed `ff 60 07` context from the
      final timing VLQ, and establish direct/extended bounded navigation.
- [x] Correlate current-cursor state exit after timing VLQs across Note,
      Channel Pressure, and Pitch Bend: 785 transitions establish data-byte
      continuation, high-bit tagged/status exit, and exact Track 9/14 walks.
- [x] Design the first exact-bounded mixed-event walker for the established
      Note, Patch, Controller, Channel Pressure, and Pitch Bend grammar with a
      transactional exact-range API, lossless coupled Patch transitions, and
      strict unsupported-branch rejection.
- [x] Implement the designed mixed-event walker and validate exact no-scan
      walks of all 184 Bells Track 9 and 601 Bells Track 14 events before
      beginning MIDI writer/export integration.
- [x] Design MIDI writer/export integration over the proven sequence,
      Meter/Tempo, and mixed-event representations without broadening binary
      parsing or adding unsupported event families.
- [x] Correlate authenticated `Ode to Clarke` export channels with bounded
      descriptor/routing fields: no direct descriptor byte/nibble survives all
      tracks; preserve a complete hash-and-range-locked nine-track manifest as
      proof policy, not Studio Vision format knowledge.
- [x] Implement Phase A pure SMF Format 1 primitives with validated MIDI-domain
      types, explicit statuses, bounded VLQs/lengths, deterministic same-tick
      ordering, automatic EOT, conductor construction, and independent
      synthetic parsing. No parser, manifest, CLI, or artifact integration.
- [x] Implement Phase B pure decoded-event adaptation with explicit channel and
      identity-480 timing policy, deterministic release-velocity Note Off
      scheduling, direct Controller/Pressure/Bend mapping, strict classified
      Patch handling, Meter/Tempo policy, transactional reports/errors, and
      synthetic in-memory Format 1 integration only.
- [x] Complete the automated Phase C authentic Ode Track 3 proof: validate the
      hash-and-range manifest, walk one Patch plus 84 Notes exactly, serialize
      a two-track Format 1 SMF, independently compare every Note and supported
      conductor/Patch field, and re-open the research artifact from disk.
- [x] Validate the Ode Track 3 Phoenix proof manually in Logic Pro 12: the
      expected single `Track 3` opened and played without audible glitches,
      obvious timing errors, hanging notes, misplaced notes, or interruption
      while changing instrument patches. Phase C is complete.
- [x] Design strict Ode multitrack Phase D: all nine ranges walk, all 1,308
      Notes match the reference, four Patches have explicit safe policy, and no
      required unsupported musical event exists. Keep the complete manifest as
      caller-supplied proof policy rather than channel-format knowledge.
- [x] Implement Phase D1 pure transactional multitrack sequence assembly with
      ordered/empty/duplicate tracks, all supported event families, aggregate
      reports, whole-sequence failure, and independent synthetic SMF parsing;
      no Ode manifest or authentic artifact is coupled to production code.
- [x] Implement Phase D2 authenticated Ode manifest validation and nine-track
      exact walk/flatten integration as proof policy feeding D1; produce and
      structurally validate a complete ten-track Format 1 result in memory.
- [x] Implement Phase D3 independent full ten-track normalized comparison
      against the authenticated Studio Vision MIDI export: all 1,308 Notes,
      1,291 comparable releases, four Patch translations, conductor state,
      track identities/channels, and supported-family inventories pass.
- [x] Implement Phase D4 explicit proof write and independent disk re-open:
      persist the exact D3 buffer at the approved external path, verify byte
      identity and SHA-256, and repeat the complete comparison from disk while
      normal tests remain write-free.
- [x] Complete D5 user-owned Logic Pro 12 multitrack validation: all nine
      expected musical tracks appeared and the sequence looked and sounded
      correct during playback, with no problem reported. The bounded Ode
      multitrack proof cycle is complete.
- [x] Design the macOS desktop prototype as a native SwiftUI/AppKit thin client
      over a versioned Phoenix Core application-service boundary, with one-file
      inspection and export restricted to exact validated compatibility
      profiles.
- [x] Design UI0's owned/versioned Core service contract: session/sequence
      identities, readiness/reason codes, diagnostics, compatibility-profile
      isolation, export request/report, errors, JSON-over-C ABI direction,
      cancellation/progress seam, and sandbox-safe path ownership.
- [x] Implement UI0A: owned app-facing Core DTOs and deterministic contract
      tests before creating the desktop shell or FFI.
- [x] Implement UI0B: synchronous path-based AppService inspection, opaque
      sessions, conservative sequence readiness, bounded diagnostics, and
      source-identity retention without export or profile guessing.
- [x] Design UI0C: isolate the Core-only compatibility-profile registry before
      enabling any application-facing export operation; use exact provenance,
      structural manifests, immutable policy handles, and export-time
      revalidation without exposing internals to the UI.
- [x] Implement UI0C1: generic registry/profile/match types and synthetic
      tests before migrating the authenticated Ode policy.
- [x] Implement UI0C2: adapt owned AppService structural snapshots into generic
      ProfileEvidence without registry assessment, channel inference, or
      readiness changes; retain incomplete exact-event evidence explicitly.
- [ ] Establish a reusable exact event-range/evidence seam before UI0C3 profile
      migration.
- [x] Investigate UI0C2A exact event termination: the seven-byte suffix rule is
      confirmed for the established 166-byte profile, while broader-format
      generalization remains open.
- [ ] Implement the scoped validated track-event-bounds parser helper before
      completing UI0C2 evidence families and Patch facts.
- [x] Implement the scoped Descriptor166 validated track-event-bounds helper;
      retain incomplete evidence until UI0C2C supplies event-family facts.
- [x] Populate deterministic decoded event-family inventory from validated
      Descriptor166 ranges without changing readiness or policy.
- [x] Audit and populate only evidence-backed generic Patch observations;
      retain bank semantics and routing as unresolved.
- [x] Complete the UI0C2 ProfileEvidence adapter audit: provenance, structural
      identity, Descriptor166 bounds, deterministic decoded families, and the
      bounded generic Patch subset are complete without readiness or export
      integration.
- [x] Implement UI0C3's isolated authenticated Ode compatibility profile and
      exact-match regression without wiring AppService readiness or export.
- [x] Design UI0C4's multi-sequence Core-only assessment handoff, private
      resolved-policy lifetime, and export-time source revalidation boundary;
      defer implementation and readiness projection.
- [x] Implement UI0C4A per-sequence compatibility assessment storage with
      readiness/export behavior frozen.
- [x] Implement UI0C4B one-shot source revalidation and stale-policy refusal;
      keep readiness and export projection deferred.
- [x] Implement UI0C4C per-sequence readiness projection and mixed-project
      aggregation; keep UI0D export deferred.
- [x] Design UI0D's fresh-revalidated export handoff, conversion boundary, and
      destination/report ownership; defer implementation.
- [x] Implement UI0D1's owned conversion-ready handoff from one freshly
      revalidated sequence into the existing multitrack assembler input;
      defer service orchestration and filesystem output.
- [x] Implement UI0D2 crate-internal in-memory export preparation through
      operation-aware fresh revalidation, UI0D1 conversion, and transactional
      assembly; defer destination commit and public response to UI0D3.
- [x] Implement UI0D3 public single-sequence destination commit with checked
      response mapping, deterministic no-overwrite collision handling, and
      same-directory hard-link publication.
- [x] Design UI0E stable error/report identity, diagnostic-code policy, and an
      explicit unsupported v0 cancellation seam that preserves UI0D3's
      irreversible commit point.
- [x] Implement UI0E contract-invariant tests and the explicit
      `cancellation_not_supported` operation without adding transport.
- [x] Design UI0F's generic JSON dispatcher, explicit service-handle lifetime,
      separate ABI/application versioning, transport errors, and exact-length
      Rust-owned response-buffer contract; specify pointer preconditions,
      destroy/poison behavior, deterministic statuses, and UI0F header
      ownership.
- [x] Implement UI0F1 safe Rust JSON dispatch with strict request shapes,
      direct owned-DTO serialization, Core error/version authority, portable
      round trips, and session persistence without any C ABI.
- [x] Implement UI0F2 C service handles, buffers/freeing, public header, and
      static-library output under the finalized ownership contract.
- [x] Implement UI0F3 concurrency/lifecycle, panic/poison, layout/symbol, and
      external C-boundary hardening before UI0G.
- [x] Implement UI0G1 external C linkage and lifecycle smoke against the
      release static library; defer Swift interoperability and fixture/CI
      hardening to UI0G2/UI0G3.
- [x] Implement UI0G2 command-line Swift interoperability through the public
      C module map; defer fixture/CI hardening to UI0G3 and desktop integration
      to UI1.
- [x] Complete UI0G3 aggregate C/Swift external validation and semantic
      cross-language fixture policy; defer host expansion and desktop
      application integration to UI1.
- [x] Implement UI1A native SwiftUI shell, Core handshake, ownership,
      threading, development linkage, and three-state startup model; defer
      project opening and product workflows to UI1B and later.
- [x] Implement UI1B one-file Open, path-based `inspect_project` dispatch,
      compact Core-derived summary, nested AppError presentation, and repeated
      Open behavior; defer sequence presentation to UI1C.
- [x] Implement UI1C project summary, explicit Core counts, typed noninteractive
      sequence presentation, and truthful zero-sequence behavior; defer
      readiness/diagnostics presentation to UI1D and export to UI1E.
- [x] Define UI1D readiness, limitations, warnings, and lazy project-level
      diagnostics presentation; defer implementation and export to UI1E.
- [x] Implement UI1D Core-owned readiness and warning presentation, opaque
      sequence selection for limitation context, and lazy cached summary
      diagnostics with failure isolation; authentic-file UI validation was
      completed before the UI1D commit.
- [x] Implement UI1E one-selected-sequence MIDI export using Core-projected
      readiness/capability, directory selection, Core-owned unique naming,
      bounded result/failure state, and Reveal in Finder; authentic repeated-
      export, cancellation, eligibility, termination, and crash-report
      validation completed, while batch/audio resurrection remain deferred.
- [ ] Defer the source-unresolved `ANALOG.MID #2` Pitch Bend curves until their
      project/sequence provenance is established.
- [ ] Defer the controlled no-bank/sentinel experiment until it outranks
      Controller, Pitch Bend, Tempo/Meter, or SysEx recovery value.

This milestone remains compatibility-profile gated and does not establish
general Studio Vision parsing.

## Long-term product vision

Subject to evidence-based format research, Phoenix aims to:

- Recover raw MIDI and audio.
- Reconstruct project structure and routing.
- Offer user-approved modern instrument mappings.
- Export to modern DAW workflows.

Automatic instrument substitution must never occur without user approval.
