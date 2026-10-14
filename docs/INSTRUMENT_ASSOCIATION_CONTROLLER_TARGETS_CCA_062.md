# Instrument-association Controller targets — CCA-062

The owned Studio Vision 120-layout export path resolves Bn Controller status
nibbles as slots in the owning Track's counted Instrument association. Each
slot selects an agreeing 36-byte type-0x10 row; payload+26 identifies its unique
40-byte type-0x2a device and payload+27 supplies its zero-based MIDI channel.
The raw status is retained. No channel is guessed from a Track name, project
hash, reference MIDI, or a blanket Track assignment.

## Scope and implementation

`controller_target120` supplies a private lookup shared by owned Single routing
and the existing experimental GS/Multi handoff. Existing association shapes,
Instrument identities, unique device matches, channel bounds and ownership
proofs remain prerequisites. A Single list admits only slot0; Multi Controllers
resolve individually. Invalid selectors, invalid lists/rows/devices/channels,
missing or surplus handoff provenance and inconsistent bindings refuse
transactionally. The strict Patch-only Multi decoder is not broadened to new
mixed event bodies.

The generic walker, generic raw Controller handoff and SMF adapter keep their
existing behavior. Complete recovery retains its raw handoff; a cloned handoff
receives only proven Controller channel substitutions before the unchanged
Instrument transposition step. Explicit partial recovery uses the same lookup
and adaptation. Note pitches, durations, releases, Programs, Bends, Pressure,
SysEx, event clocks and ordering remain unchanged. FF41 and actual MIDI/SysEx
bytes are not reinterpreted. CCA-057's Pipe timing witness and CCA-059's guarded
length-7 decoder remain unchanged; no length-6 authority is added.

Every resolved Controller records its source ordinal/range/tick, encoded status
and status offset, explicit/compact flag, selector, number/value and their
offsets, association/list-entry ranges, Instrument row/record/identity offset,
device identity/record range, source channel offset and resolved channel.
Compact continuations retain their original inherited status offset. Existing
FF60 context/timing provenance is retained separately without alteration.

## Evidence reconciliation and authentic checks

The authentic reference study and CCA-061 established the mapping. Assignment
Experiments033/034 support assignment-mediated output channels, not a direct
edit of this selector field. CCA-058 supplies independent pitch-processing
evidence; CCA-059 independently establishes only the length-7 Controller bridge.
Those format rules are not expanded by this change. Reference files are read
only in validation and post-conversion comparison, never by runtime resolution.

Sweeper source COMIC BOOK is452284 bytes, SHA-256
`22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486`.
Native reference sweeper is74647 bytes, SHA-256
`e83cfc31971d92160ec4786a93c6b2a2730a4d6f6b7b54843978613563ed72dc`.
All5861 recovered Controllers match native tick/channel/number/value tuples in
their corresponding Track streams, including Multi's30 per-event targets.
The CCA-059 comparison changes exactly5861 Controller status bytes; every other
MIDI byte remains identical. Twelve musical chunks change only those channel
bytes; the other13 chunks remain identical. All2294 CCA-058 pitch corrections,
Pipe's151 Notes/993 Controllers/22 Bends and its ending, FF60 provenance,
source-object ledgers, event counts and refusal boundaries are retained.

Independent SCHOOL PROJECTS is343875 bytes, SHA-256
`bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331`.
M-vegetarian del SETUPS has association222047..222080 and bounded musical body
240451..240759. Its slots0/1/2 select rows21/76/78, whose resolved channels are
2/9/11. All six CC91/93 events match existing native M-vegetarian del:
21941 bytes, SHA-256
`59a14a9313bea0259ec964f9219aac3b1e0b82cbf6e422a78f5a558b4991378f`.
Production lookup/application is tested against this independently owned body;
the test does not confer broader mixed-Multi export authority. The existing
700-note SCHOOL transposition check also passes.

## Incomplete coverage and publication

CCA-062 retains3921 Notes,5861 Controllers,373 Bends,15 Programs,1 GS Reset,
26 Tempos,initial meter, and6641 unresolved source objects. Native comparison
establishes492 missing Notes and6112 missing Controllers:

| Source Track / unresolved range | Missing Notes | Missing Controllers | Obstacle |
|---|---:|---:|---|
| 7 Square Wave /46644..64556 | 455 | 4787 | length-6 FF60 Controller bridge |
| 11 Atmosphere /79617..81802 | 32 | 646 | length-6 FF60 Controller bridges |
| 17 Seashore /98506..100653 | 5 | 679 | SysEx framing, then length-6 FF60 |

Seashore additionally lacks three SysEx messages. Piano's terminal context,
Track14's FF68 object and conductor termination remain unresolved. Missing
musical-event counts are not the raw-object ledger count. Published limitations
retain the source-only converter report plus a clearly separate
`research_validation` annotation calculated after conversion from the existing
native reference; it cannot select or supply output events. The label remains
EXPERIMENTAL INCOMPLETE. Publication creates a fresh CCA-062 directory and uses
exclusive/no-overwrite file creation.

## Regression and preservation

Six portable tests cover list permutations, Multi per-event targets, compact
status inheritance, invalid selectors, identity/device/channel/list guards,
transactional provenance validation, preserved non-Controller events, unchanged
generic adaptation, complete owned recovery and retained FF60 accounting.
The full suite passes671 tests with0 failures and21 opt-in tests ignored.
Selected authentic checks cover Sweeper, six SCHOOL Controllers,700 SCHOOL
Notes, Reservist's length-7 bridges, and four original Pipe/prefix/refusal tests.
Eight earlier SCHOOL artifacts retain classification and differ only in proven
Controller destination bytes; prior artifacts are not replaced.

Formatting, strict all-target/all-feature Clippy and Git whitespace checks pass.
The pre-edit503-file snapshot protects original projects, references, prior
exports and unrelated uncommitted work. No commit, push, reset, discard or
SheepShaver experiment occurs.
