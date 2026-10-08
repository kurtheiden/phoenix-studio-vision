# Prologue recovery research checkpoint through PM-R13

## Scope and frozen baseline

Documentation checkpoint, 2026-10-08, against frozen GameSoundCon RC2 commit
11b4f3bc4796c7a177401f0a88e8d8564485a934 (Correct Phoenix RC2 app icon transparency).

This records completed research, with no parser, export, compatibility-profile
or readiness-policy change. **Observational blocker removal** means a bounded
rule tested in temporary research tooling, not an implemented product capability.
Frozen RC2 remains unchanged. **No Prologue Sequence is declared recoverable.**

This extends the [post-freeze checkpoint](OS1_PROLOGUE_MASTER_POST_FREEZE_RESEARCH_CHECKPOINT.md),
[normalization checkpoint](OS1_PROLOGUE_SVP45_NORMALIZATION_RESEARCH_CHECKPOINT_2026_09_16.md),
[reference-field trace](OS1_PROLOGUE_SVP45_REFERENCE_FIELD_TRACE_CHECKPOINT_2026_09_17.md)
and [two-marker checkpoint](OS1_PROLOGUE_120_LAYOUT_TWO_MARKER_IMPLEMENTATION_CHECKPOINT_2026_10_02.md).
Later evidence does not rewrite historical refusals or inconclusive experiments.

## Source identity and provenance

Original authentic data fork, read-only:

- Path: /Users/kurtheiden/Documents/Phoenix Research/Authentic Studio Vision Projects/Prologue Caddys and Dinos/Prologue Master
- Size: **33,057 bytes**
- SHA-256: b3a3efd8179d3ffe2a5026b3a40eab74da3ce6e674bb06162a176df07389b733
- Evidenced layout: 120-byte Track-slot geometry.
- Historical Finder identity recovered from FINDER.DAT: Type MID2, Creator MIDI.
  Earlier metadata restoration/opening concerned a separate research copy;
  it does not imply alteration of the archived source or current attributes.

Existing normalized research data fork:

- Path: /Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/Prologue SVP Transfer
- Size: **58,119 bytes**
- SHA-256: 0618cd5087987d7f694ca7532e1cc8a01b4adf7e3043ec648783d71e7e5ad714
- Provenance: earlier owner-performed Studio Vision Pro 4.5 no-intentional-edit
  save, separately written directly to Shared with Save a Copy As.
- Evidenced layout: 166-byte Track-slot geometry. This is not the guest-native
  Prologue SVP Save file or a new normalization produced by this work.

Shared-folder data-fork evidence does not prove preservation of classic Finder
metadata/resource forks. Normalization is a structural comparator, not proof
that every original object survived unchanged: the serialized Prologue object's
fate and some normalized pair additions remain unresolved.

**Owner-visible observation:** Studio Vision Pro opened the authentic project
after Finder metadata restoration. The project window showed seven Sequences:
Frame 1, Frames 2-4, Frames 5-6, Frames 7-8, Frames 9-10, Frame 11, Frame 12;
seven same-named Segments; Templates appeared empty.

A serialized record named **Prologue** exists, but its semantic UI role is not
established. Do not label Phoenix's duplicate serialized Frame-name group as
Segments. Earlier Key-edit evidence about directly edited objects does not
establish a complete category-to-group mapping.

## Initial Phoenix observation: PM-R01 / PM-R02

Frozen RC2 showed **15 sequences found, 0 recoverable, 15 not yet recoverable**.
Serialized order is seven Frame-name records, Prologue, then the repeated seven.

All fifteen initially refused with IncompleteNeighborhood. Fourteen Frame
records shared additional type-07 interruptions in the supported pairing
neighborhood. Record 8, Prologue, instead encountered type-02/type-05 structure;
that separate blocker remains outside this Frame work.

PM-R02 counted **64 additional type-07 records and 20 distinct payload forms**.
Recognizable strings and visible-comment correlations did not prove safe
ownership/playback independence. PM-R02 required more evidence.

## Controlled Track Comments: PM-R03 through PM-R10

Owner-controlled Experiment 007 supplied the controlled lineage, not Prologue
or another reserved authentic validation specimen:

- Native source: MacOS9HD:newest STUFF baseline copy
- Data fork: **211,468 bytes**
- SHA-256: e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132
- Target: Ode to Clarke / Track 7 / Comments initially **EMPTY**.

The guest copy was bound to Experiment 007 by a host-accessible data-fork hash,
not recollection or filename alone. Existing PMR03 CTRL exactly matched baseline.

| Stage | Evidence and historical result |
|---|---|
| PM-R03 | Intended EMPTY -> PHX-CMT-R03 edit also accidentally created Track 8, subsequently confirmed by the owner. Remains INCONCLUSIVE; a fresh clean repeat superseded it scientifically, without retroactive reclassification. |
| PM-R03R | Clean repeat inserted exactly one bounded 16-byte type-07 comment record; no Track 8, extra pair or extra empty type-07. All 168 original event bodies and 168 secondary payloads stayed byte-identical. Descriptor/reference changes left the result INCONCLUSIVE / Decision C. |
| PM-R04 | Of 1,331 changed byte positions, classified 1,232 SAVE-VARIANT and 94 routing-selector changes preserving decoded relationships; five bytes remained UNKNOWN. Decision B. |
| PM-R05 | Descriptor +42 was unresolved after 7,436 comparable descriptor instances in 44 existing artifacts. Decision C. |
| PM-R06 | Fresh EMPTY -> PHX-CMT-R06 edit reproduced bounded comment insertion and preserved all 168 bodies/payloads. Meter +42 values were CONTROL88 / PM-R03R B5 / PM-R06 88, not a fixed comment-state pattern. Track 7 +88 added a sixth unresolved byte. INCONCLUSIVE / Decision C. |
| PM-R07 | Six bytes excluded from established framing, stride and local event-field calculations; full playback/context independence remained unproven. Decision B. |
| PM-R08 | Causal playback/context evidence narrowed the remaining component to saved mute. Track 7's 04-prefixed applicability guard returned Unknown(GuardMismatch). Decision B. |
| PM-R09 | Exact 04-prefixed geometry compatible, but earlier controlled mute transitions tested only 00-prefixed forms. Applicability unproven. Decision B. |
| PM-R10 | Owner-observed clean visible Mute OFF -> ON caused candidate 0x80 -> 0x88; exact guard 04 04 00 00 04 01 00 unchanged. All 168 original bodies and 168 secondary payloads byte-identical; routing graph equivalent, established ownership intact. Decision A closed the evidenced playback/context gap. |

Custody: the clean PM-R03R result was transferred under host filename PMR03 EDIT,
replacing the earlier Shared-folder accidental result. The original PM-R03
report's identities/results remain historical evidence; filename alone does
not identify an arm. Both comment literals have **11 ASCII bytes**; older
preregistration length discrepancies remain historical. The 16-byte insertion
includes five framing bytes.

PM-R10 established within the evidenced Descriptor166 grammar:
**Studio Vision Track Comments are preservation metadata, not MIDI performance
data.** Phoenix should not invent a Standard MIDI File meaning for Comments.
A future Recovery Report may preserve Comments separately.

This did not transfer Track association automatically to Prologue's 120 layout,
accept arbitrary type-07 records, or clear a Prologue blocker.

## Sparse type-07 association: PM-R11 / PM-R12 / PM-R12A

All 64 additional records are independently bounded. In every affected Frame,
Track slots with a **nonzero descriptor-relative +85..88 window** exactly equal
the additional type-07 count. The predicate is any nonzero byte in that
four-byte window; numeric pointer/reference semantics were not established.

PM-R11 Decision C retained the per-Track association gap. PM-R12 showed across
all fourteen original120/normalized166 Frame correspondences that Track order,
sparse-mask membership/order and all **64 ordered payloads** were preserved,
with no counterexample. Count equality, adjacency and text alone did not prove
list ordering; PM-R12 Decision B retained one independent ordering link.

**Prospective discriminator preserved before owner inspection:**

- Frames 7-8 / Strings: ascending sparse Track order predicts **#90**.
- Tested alternative swapping the adjacent sparse owners predicts **#49**.

The owner subsequently inspected Studio Vision read-only and observed Strings
Comments **#90**, with Instrument Sound Canvas-7. The same UI view corroborated
String 2 -> **#49**, Piccollo -> **#73**, Marimba #3 -> **#13**, agreeing with
the already-derived mapping. These are corroborating rows from one UI view,
not four independent prospective experiments. The audit used the owner's
screenshot description, without claiming independent pixel inspection.

PM-R12A Decision A established ordering within the evidenced Prologue120 grammar.
**All 64 records are Track-associated under the evidenced rule.**

### Rule and classification limits

Independently inventory Track slots (start at candidate +106, stride120,
label at slot +15). Additional bounded type-07 records form a sparse list in
physical order corresponding to ordinary nonzero +85..88 slots in ascending
structural Track order. Framing is type byte plus four-byte big-endian payload
length; observed extra payload lengths are 2, 3, 6, 8, 10, 12 and 15 bytes.
The supported initial ten-zero prelude is separate from these 64 extras.

Require exact bounds, mask/list count, unique inventory and complete framed
accounting. Preserve independent state exclusions, pair framing/cardinality,
routing/event decoding and later authority gates. Refuse extra/missing records,
special-slot presence, invalid bounds, ambiguous association and unsupported
forms. Payload text does not select a Track or event pair.

PM-R12A occurrence classification was **A14 / B50 / C0**:

- A14: Track-associated bounded preservation metadata in the three Frames with
  reconciled independent consuming-slot/event-pair correspondence.
- B50: Track association proven, but complete protected event ownership remained
  blocked by cardinality in the other eleven Frames.
- C0: no remaining unresolved sparse-order association.

These are occurrence counts, not recoverable Sequences. All 64 gained Track
association; not all 64 gained complete musical ownership.

**THE TYPE-07 INTERRUPTION EXPORT BLOCKER WAS REMOVED OBSERVATIONALLY FOR
14/14 FRAME RECORDS.** Next gates were C0 decoding in three and cardinality
in eleven. No Sequence was declared recoverable.

## PM-R13 Program Change result

PM-R13 handled only the three C0 failures, leaving eleven cardinality failures
untouched. Offsets below are half-open data-fork ranges. Local pair/Track slot2
includes Meter and Tempo slots0/1 and is the first ordinary consuming Track.

| Serialized ordinal / Frame | Track | Primary / secondary records | Event body | C0 offset | Program |
|---|---|---|---|---:|---:|
| 1 / Frame 1 | Low Strings | [2487,2863) / [2863,2894) | [2506,2856) | 2513 | 96 |
| 4 / Frames 7-8 | Piano l Hand | [14283,14466) / [14466,14497) | [14302,14459) | 14311 | 1 |
| 6 / Frame 11 | Low Strings | [18515,19438) / [19438,19469) | [18534,19431) | 18541 | 96 |

C0 occupied the established status position after timing and a decoded Note,
followed by one bounded 7-bit Program byte. **Nine explicit C0 entries** occurred
in these three initial bodies:

- Frame 1: 69 Notes +1 Program = declared70, exact350-byte consumption.
- Frames 7-8: 20 Notes +4 Programs = declared24, exact157-byte consumption.
- Frame 11: 176 Notes +4 Programs = declared180, exact897-byte consumption.

The existing normalized representation supplied the decisive bridge: each
C0 p becomes a bounded Patch core with bytes

    FF 7C 0C 00 00 p 00 00 00 00 04 FF FF FF p

carrying the same Program, with preceding timing retained. Each adds13 bytes.
Only those substitutions make all three corresponding bodies byte-identical;
no timing/Note bytes change. The unchanged production Patch-core decoder accepts
all nine normalized cores. Controlled FF7C Program experiments independently
established direct values (displayed23 ->24/100 maps17 ->18/64).

Independent Track/event ownership and original-to-normalized routing joins
remained valid. Low Strings routes to Sound Canvas/channel3; Piano l Hand to
Sound Canvas/channel1. In this evidenced project grammar C0 is a canonical
Program-entry tag, not permission to override routing with literal wire-MIDI
channel1. Do not generalize to C1..CF or infer Patch names/banks from Comments.

The bounded observational rule consumes existing timing VLQ / explicitC0 /
one Program byte0..127, preserves provenance, counts one event, retains
independent single-Instrument routing and resets continuation state. A successor
must re-establish supported explicit status (90 and E0 were observed in the
wider three-Frame work). Compact Program running status was not established.
Missing/high-bit data, unknown statuses, ambiguous ownership/routing, count
deficits and trailing unexplained bytes remain refusals.

All18 ordinary consuming pairs in these three Frames then passed exact body
consumption, declared logical/context reconciliation and independent
routing/context checks, retaining established zero-state exclusions.
No MIDI was generated.

**PM-R13 Decision B: THE BOUNDED C0 PROGRAM CHANGE FORM IS STRUCTURALLY PROVEN.**
The explicit project representation was new relative to frozen walker coverage,
although Program semantics and FF7C parsing already existed.

**THE C0 EXPORT BLOCKER WAS REMOVED OBSERVATIONALLY FOR ALL THREE AFFECTED
FRAME RECORDS.** Each then reached **conductor Envelope** at the next checked
existing gate. No Sequence was declared recoverable. General bounded
implementation is eligible for separate review; none was implemented.

## Current blocker frontier after PM-R13

This is the **observational research frontier**, not changed RC2 readiness.
Ordinals disambiguate repeated names. Cardinality counts are consuming
slots/pairs after only already-established zero-state exclusions.

| Ordinal | Serialized name | Current research blocker |
|---:|---|---|
| 1 | Frame 1 | conductor Envelope |
| 2 | Frames 2-4 | Cardinality: slots6 / pairs4 |
| 3 | Frames 5-6 | Cardinality: slots7 / pairs5 |
| 4 | Frames 7-8 | conductor Envelope |
| 5 | Frames 9-10 | Cardinality: slots5 / pairs2 |
| 6 | Frame 11 | conductor Envelope |
| 7 | Frame 12 | Cardinality: slots9 / pairs8 |
| 9 | Frame 1 | Cardinality: slots5 / pairs0 |
| 10 | Frames 2-4 | Cardinality: slots6 / pairs0 |
| 11 | Frames 5-6 | Cardinality: slots7 / pairs0 |
| 12 | Frames 7-8 | Cardinality: slots10 / pairs0 |
| 13 | Frames 9-10 | Cardinality: slots5 / pairs0 |
| 14 | Frame 11 | Cardinality: slots9 / pairs0 |
| 15 | Frame 12 | Cardinality: slots9 / pairs0 |

Three Frames advanced through type-07/C0 to conductor Envelope; eleven through
type-07 to cardinality. Record8 Prologue remains separately blocked by
type-02/type-05, outside those fourteen. Its UI role remains unassigned.
Later terminal/order/translation/export-authority gates remain uncertified.

## Safety invariants and resume boundary

Unknown data may cease to block export only when Phoenix proves it does not
overlap or affect:

- MIDI channel/routing;
- Instrument/Patch/Program/bank/playback state;
- timing/order;
- Track/event ownership;
- musical/performance data.

Do not silently discard or invent music. Saved mute is historical playback
state, not permission to discard Track data. Legitimate no-event-data Tracks
exist, but must **not** relax the eleven cardinality failures without evidence.
Descriptor166 metadata proof does not transfer to arbitrary layouts/type-07.

**NEXT RESEARCH MUST NOT BEGIN IN THIS CHECKPOINT TASK.**

Future choices: (A) investigate conductor Envelope in the three advanced Frames;
or (B) investigate the eleven cardinality failures. Recommend conductor Envelope
first unless later evidence provides a stronger reason otherwise. PM-R13 left
the eleven cardinality failures untouched. Neither branch nor PM-R14 begins here.

## Evidence and preservation record

This document preserves essential identities, bounded rules, causal chain,
classification meanings and exact frontier without requiring /tmp to recover
the stopping point. Original detailed report identities follow as supplemental
provenance. Existing repository checkpoints retain earlier limits and chronology.

This task commits only this new document. Pre-existing DECISIONS/ROADMAP edits,
the two untracked owner research documents and Xcode user state are preserved.
Production source, controlled artifacts, original/normalized Prologue, historical
reports, RC1/RC2 and conference master remain unchanged. No new reverse
engineering, MIDI export, experiment, reset/clean/restore or push is performed.

### Supplemental completed reports

| Report | SHA-256 |
|---|---|
| /tmp/phoenix-prologue-120-type07-track-association-pm-r11.txt | 1f6b5e05eb4578483111d6b4b447e0517a428be5377f14dce37778dd6bf0f38a |
| /tmp/phoenix-prologue-descriptor-plus42-resolution-pm-r05.txt | 2f824d96cfb025cd575994c6ed9ef8e20a1cfce91838f5564f1fc9f015666667 |
| /tmp/phoenix-prologue-descriptor-reference-classification-pm-r04.txt | 7e6f009e57c35b12245117dd8ef84bf75550f958b5b545135dc5d06dcedb505f |
| /tmp/phoenix-prologue-master-blocker-decomposition-pm-r01.txt | 0a59c0423dfb0b372b406f2c754d8596d03de71b65b09c3648b10cb3e091fa82 |
| /tmp/phoenix-prologue-master-type07-analysis-pm-r02.txt | b05374113c9f717561aa6f181c14a4c753972e810df2adb7d75d285cc94695e2 |
| /tmp/phoenix-prologue-playback-context-dependency-pm-r08.txt | 2c441c7342b4731b3753b19272274daac7b43bc93402bb3bb10615ac2e40f503 |
| /tmp/phoenix-prologue-program-change-pm-r13.txt | 3facc62533e6f03f02b660dbbdbbeb534d35b62bfec2f03fdba4072f3438f2da |
| /tmp/phoenix-prologue-saved-mute-04-guard-pm-r09.txt | f28b5dab435bb754ee267b05197ffa62854cce4ae3d2d503078fee12f098b9bb |
| /tmp/phoenix-prologue-saved-mute-04-guard-pm-r10-analysis.txt | 8cd353932d6b3fc153f0d08e6d088aa4f556bb136dbbb4478a179d797b9e37cb |
| /tmp/phoenix-prologue-sparse-type07-ordering-pm-r12.txt | 0cae344cffe8c0e255228fcf60eb9219d7afbc4e1e71aad2e349d74e7c455ecf |
| /tmp/phoenix-prologue-sparse-type07-ordering-pm-r12a.txt | e25f050edaae31a4f1e4c5f6e0e90abe5cbd86f3ff554cfae5e7b69938cca9f8 |
| /tmp/phoenix-prologue-track-comment-experiment-pm-r03r-analysis.txt | d22c538b194b8e9891a2f630339f810aa53968c7ddde6a3ad9ce6eea09696fca |
| /tmp/phoenix-prologue-track-comment-linkage-pm-r06-analysis.txt | 0f9ed3cc36a1c489fd77985724b7bbb350a899e92abf1f77fec61c622283e27d |
| /tmp/phoenix-prologue-unresolved-descriptor-dependency-pm-r07.txt | 8b78b8806724c298e3c3e8237ce21e3ba78cbffefb8d225b4e05952fb494dbd9 |
