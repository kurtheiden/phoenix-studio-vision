# Guarded owned-120 SysEx framing — CCA-064

CCA-063 reconciled CCA-017, native Sweeper comparisons, and existing controlled
saves. This implementation admits only explicit F0 records with single-byte
stored lengths 09 or 0B in independently associated, bounded 120-layout bodies.
The first counted byte must be 00. It is retained as source framing; the other
length-minus-one bytes are seven-bit MIDI payload ending in F7. A supported
explicit B0..BF Controller must follow at the next event cursor. Terminal,
compact, consecutive SysEx/context and unrelated successors remain refused.
There is no scan, resynchronization, new FF60/06 authority or source selector.

The private owned walker retains timing/ranges and the framing offset separately
from the payload. Full Single authority reconciles each SysEx as one raw source
object. Prefix recovery requires semantic ownership, bounded consumption,
consistent counts and independent routing. Existing CCA-062 Instrument-list
resolution applies to every Controller, including compact continuations.
Handoff preserves logical ordinal/range/tick. The partial ledger also preserves
stored length, framing value/offset, complete source bytes, timing bytes/range,
MIDI payload/range and logical source ordinal. The owned adapter constructs only
8/10-byte validated SMF payloads; generic adaptation still requires the original
7-byte payload constructor. Serialization preserves source order and F7.
No general SMF parser changes are necessary.

## Evidence and expected delta

Original COMIC BOOK: 452284 bytes, SHA-256
22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486.
Existing native sweeper: 74647 bytes, SHA-256
e83cfc31971d92160ec4786a93c6b2a2730a4d6f6b7b54843978613563ed72dc.
Seashore source descriptor17 (`major noise sweep`) has bounded body
[98334,100653). Three records [98506,98518), [98531,98545), [98555,98569)
export respectively 8,10,10 payload bytes, all at tick0. Native chunk13 confirms
these payloads and source order. Nine newly recovered Controllers match native
channel7/number/value/tick; final two are CC10=127 at76710 and CC10=125 at76739.
The prefix grows from57 to69 raw objects: 66 Controllers and3 SysEx. It ends at
98578, leaving677 of746 declared objects unresolved. The next FF60/06 record
remains refused in its entirety; no Note is newly recovered.

Independent SCHOOL PROJECTS (SHA-256
bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331)
owns minuet.mid `program chg`, primary [214118,214584), body [214137,214577).
Its F0/0B [214137,214151) exports GS Reset at0, followed by an explicit
Controller interval1. Existing minuet.mid reference (20993 bytes, SHA-256
641c29017f64f5c9fc285c5a5f9433db09c8953a03e63341d5b20c91827e49b8)
chunk10 matches all28 ordered musical messages: one SysEx,14 Controllers and
13 Programs. Owned framing is exercised independently; existing Multi suffix
routing/translation supplies the comparison. Strict Patch-only Multi authority
is unchanged. No reference file is converter input.

The 74-project deduplicated CCA-063 survey found210 candidate F0/0B records,
all with counted00 framing. Eight F0/09 occurrences belong only to COMIC BOOK
versions, so length09 lacks an unrelated exact-form witness. Its supported
interpretation follows the independently corroborated counted framing principle,
not independent provenance for Sweeper's particular payload.

## Separate F0/07 follow-up

CCA-017 remains byte-for-byte unchanged in decoding and serialization. It emits
all seven counted bytes, including the leading00. CCA-063 compared all28
happyone SysEx messages with authentic happyone Multitrack: omitting that first
counted00 produces exact native payload/tick matches. This is a newly established
payload discrepancy, separate from CCA-017's valid bounds/consumption proof.
Any correction requires separately scoped compatibility and regression work;
CCA-064 must not silently normalize F0/07.

## Validation and limitations

Synthetic tests cover both lengths, nonzero event/successor intervals, inherited
Controller status, source ranges/framing, same-tick order, Note re-entry,
Instrument routing, exact raw counts, invalid selectors, truncation, malformed
length/framing/data/F7, missing/unsupported/consecutive successors and overflow.
Generic walker/adapter and F0/07 preservation are explicitly checked.

Authentic checks compare Seashore's69 ordered SysEx/Controller messages with
native MIDI, all5870 recovered Controller tuples, and all previous CCA-062
musical messages unchanged. All other Track ledgers, FF60 provenance,2294
percussion pitch corrections and Pipe's ending remain checkpoint-equivalent.
The output remains EXPERIMENTAL INCOMPLETE:3921 Notes,5870 Controllers,
373 Bends,15 Programs,4 SysEx,26 Tempos,initial meter;6629 raw unresolved objects.
Native totals4413 Notes/11973 Controllers establish492 missing Notes and6103
missing Controllers. Square Wave and Atmosphere remain blocked at46644 and
79617; Seashore remains blocked at98578. MIDI EOT is not historical duration.

Final validation: 674 default tests passed, zero failures, 22 opt-in tests ignored;
12 explicitly selected authentic checks passed. Formatting, strict all-target/
all-feature Clippy and Git whitespace checks pass. All24 non-Seashore MIDI chunks
are byte-identical to CCA-062. Publication uses a fresh exclusive directory and
no-overwrite files under `~/Documents/Phoenix Exports/CCA-064 Sweeper/`.
