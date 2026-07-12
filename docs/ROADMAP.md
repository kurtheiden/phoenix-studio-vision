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

- [ ] Define a lawful, redistributable fixture policy.
- [ ] Record observations from independently obtained samples.
- [ ] Add comparison-oriented inspection only where evidence supports it.
- [ ] Document repeatable research methods and confidence levels.

## Later possibilities

Only after sufficient evidence exists, consider format recognition, structured
parsing, richer validation, or a graphical interface. None is currently
committed or specified.

## Long-term product vision

Subject to evidence-based format research, Phoenix aims to:

- Recover raw MIDI and audio.
- Reconstruct project structure and routing.
- Offer user-approved modern instrument mappings.
- Export to modern DAW workflows.

Automatic instrument substitution must never occur without user approval.
