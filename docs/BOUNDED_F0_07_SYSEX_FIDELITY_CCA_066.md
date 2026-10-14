# CCA-066 — bounded F0/07 SysEx export fidelity

Implemented and verified the CCA-065 GO recommendation. Only the owned
Descriptor166 handoff changes: a zero-framed source record emits its six MIDI
payload bytes, while all seven counted source bytes remain preserved.

## Scope and preservation

`sysex166::qualify_track` reparses the same source's Descriptor166 ownership,
requires exhaustive ordinal descriptor/pair correspondence and validated full
Track event bounds, and checks that the supplied walk exactly matches the
unchanged decoder. It transactionally qualifies only explicit F0/07 records
with counted first byte00, valid remaining seven-bit data, final F7 and an
explicit F0 or90 successor at the current next-event cursor. Terminal SysEx,
nonzero framing, malformed/truncated records, unsupported successors, ambiguous
ownership/bounds and inconsistent walks remain refusals. Nothing is scanned or
resynchronized. Existing source count fields are retained, not rewritten.

The privately constructed `SourceFramedSysEx07` retains primary/secondary/body
ranges, Sequence/descriptor/pair identity, complete representation bytes/range,
all seven counted bytes/range, framing offset/value, status/stored-length offsets,
timing bytes/range/value, absolute tick, logical source ordinal, raw source-object
ordinal and explicit successor timing/status provenance. Its immutable six-byte
MIDI payload is source-derived. `bounded_sequence` consumes these proofs only
after existing ownership, bounds, mute, routing and applicability checks.

A separate decoded-event variant binds adaptation to that proof's tick, ordinal
and source range. Its private SMF payload constructor supplies six already-proved
bytes to the existing writer. Existing generic walker, raw SysEx handoff,
ordinary MIDI adapter payload behavior, public payload constructor and all
serialize_* function bodies remain unchanged. There is no general zero stripping.
The SMF length changes from07 to06 only for the qualified source event.

Owned120 explicitly rejects this new proof variant. CCA-064 F0/09 and F0/0B,
120-layout F0/07 refusal, authenticated-profile SysEx refusal/omission policy,
selected166 Note-only authority and length-6 FF60 refusals are unchanged.
No project names, source hashes or reference MIDI influence the runtime rule.

## Authentic verification

Experiment007 `newest STUFF baseline`:211468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
This is the preserved normalized baseline used by CCA-017, not the older
203422-byte original. References are read only by tests, never converter input.
Ranges below are hexadecimal and half-open.

| Owned Track | Body | SysEx / Notes | Declared raw objects |
| --- | --- | ---: | ---: |
| happyone Track6, pair5 | [21236,21714) | 26 /138 | 166 |
| happyone Track7, pair6 | [217DF,22375) | 2 /519 | 521 |
| xForm Track2, pair1 | [79B6,7FB2) | 2 /242 | 247 |

All30 SysEx match exact payload, tick and joint Note-start/SysEx source order
against native Multitrack references; their SysEx streams also match the merged
exports. All899 Note starts match tick, channel, pitch and attack. Source counts
remain logical165+one initial context,521, and246+one context respectively.
Before/after decoded non-SysEx events are identical, including durations,
releases, source ordinals/ranges, Controllers and Patches. Scheduled non-SysEx
messages, Note provenance and adapter counts are identical.

Representative exact source records:

- happyone first [21476,21481), delta540, tick33900; last [21702,2170C), tick61812.
- happyone Track7 [219DB,219E5), tick12092; [219EC,219F6), tick12246.
- xForm [7A25,7A30), delta961, tick8625; [7A30,7A3A), delta19, tick8644.

Every record keeps `00 41 32 00 xx yy F7` as counted source data and emits
`41 32 00 xx yy F7`. CCA-017 raw golden assertions remain unchanged. CCA-063/065
native fidelity evidence supplies the correction; CCA-064 remains a separate
120-layout rule. Experiment007's untouched save is not a controlled edit of the
framing field. These checks confer no new conductor or complete-project authority;
happyone still refuses at conductor prelude, and xForm's unrelated unsupported
context remains refused.

Pinned native reference SHA-256:

- happyone Multitrack: `0061667c7bcfd7ba37478266af5fb4154905651f2ca845dc16c00cf76bcd13c7`
- happyone merged: `d19342947c74e5957133a95006d9f4249f2a49d6f637c19c0502cbde83209a29`
- xForm Multitrack: `872272ce43c389474fdb408416ad2406fa9a63de5e1d8bb94e0f6bbced7a1241`
- xForm merged: `5e737f1536e0eabe90c7fecfb8d2d5b795fb4ac7ecee5219b1bedeb7486e4f8f`

## Regression result and publication

Synthetic checks cover zero/nonzero framing, both successor forms, nonzero VLQ
intervals, same-tick ordering, malformed/truncated data, missing/unsupported
successors, terminal records, ambiguous bounds/ownership, proof rebinding and
late-record transactional refusal. Owned handoff output assertions now expect
six-byte payloads; generic MIDI adapter and raw source assertions remain intact.

- Full regression suite:678 passed, zero failures,24 opt-in checks ignored.
- Explicit authentic checks:13 passed, zero failures, including the30-message /
  899-Note comparison, independent SCHOOL and Reservist checks and prior refusal
  regressions. These tests write no new authentic/Sweeper exports.
- Strict all-target/all-feature Clippy, formatting and Git whitespace checks pass.

Sweeper was rebuilt only in memory. Every MIDI byte and the complete partial120
ledger match published CCA-064. MIDI SHA-256 remains
`267e85a77e44daf1c7a69f595b89d8a02c6517adf4ae706ae739e4eadc06f075`.
Seashore's next refusal remains98578; Square Wave/Atmosphere boundaries, Pipe's
ending, percussion pitches, Controller routing and FF60 provenance are intact.
No new Sweeper MIDI or replacement artifact is published.

This report is the CCA-066 publication. Original sources, references, all earlier
exports and unrelated uncommitted work are preserved. No commit, push, reset,
discard, SheepShaver experiment or Logic action was performed.
