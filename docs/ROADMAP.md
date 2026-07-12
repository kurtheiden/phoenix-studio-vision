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

## Milestone 2: research workflow

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
