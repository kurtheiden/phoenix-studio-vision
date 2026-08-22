# Decision Log

## 2026-07-10: begin with a minimal Rust CLI

**Status:** Accepted

Phoenix begins as one Rust binary crate named `phoenix`. This keeps the first
milestone portable and testable without committing to a multi-crate, GUI, or
FFI architecture.

## 2026-07-10: remain format neutral

**Status:** Accepted

The first implementation reports only generic byte-level facts. It does not
recognize Studio Vision files, assign signatures, or infer undocumented format
details.

## 2026-07-10: read-only input handling

**Status:** Accepted

Phoenix opens input through Rust's read-only `File::open` API and exposes no
operation that writes to the input path.

## 2026-07-10: use the MIT License provisionally

**Status:** Accepted, review required

The repository initially uses the MIT License. Licensing must be reviewed
before commercial distribution or before accepting significant outside
contributions. This decision does not settle contributor agreements, patent
terms, or the licensing of research fixtures.

## 2026-07-10: use SHA-256 from a focused dependency

**Status:** Accepted

The CLI uses the RustCrypto `sha2` crate rather than maintaining a custom
cryptographic implementation. Dependency scope will remain deliberately small.

## 2026-07-12: keep discovery inspection streaming and format neutral

**Status:** Accepted

The Discovery Inspector updates its SHA-256 digest, byte-frequency histogram,
printable-string scanner, and 256-byte preview during the same buffered,
read-only pass. Printable strings use only bytes from `0x20` through `0x7e`,
have a minimum length of four bytes, and remain ordered by starting offset.
Strings may span read-buffer boundaries.

Shannon entropy is calculated directly from the complete file's 256-bin byte
frequency histogram, avoiding another dependency. The `Bytes in reported
printable strings` percentage counts only bytes in strings that meet the
four-byte reporting threshold. If multiple strings share the greatest length,
the first one in file order is reported as the longest.

These values are byte-level observations only. They do not identify file
formats or assign semantic meaning to strings or surrounding bytes. Repeated
block detection remains outside this decision.

## 2026-07-17: identify Studio Vision from explicit Finder metadata evidence

**Status:** Accepted

Phoenix keeps identification separate from both format-neutral inspection and
Studio Vision structural parsing. Identification does not consider filename
extensions or infer a content signature from the available authentic sample.

Identification reports three distinct layers:

- **Observation:** the literal Finder type and creator codes, or an explicit
  absent, unsupported, malformed, or read-error state.
- **Evidence:** the documented relevance assigned to those observations. The
  `MID2` and `MIDA` pair is strong observed Studio Vision evidence; a single
  matching code with the other unavailable is provisional evidence.
- **Conclusion:** one confidence value from Very High, High, Medium, Low, or
  Unknown. The initial implementation assigns only High, Low, and Unknown.

The `MID2` type plus `MIDA` creator produces High confidence. One matching code
with the other absent or unavailable produces Low confidence. Conflicting,
missing, unsupported, malformed, or unreadable metadata produces Unknown.
Confidence is not proof that a file is Studio Vision or structurally valid.

On macOS, Phoenix reads the first eight bytes of `com.apple.FinderInfo` through
a small read-only platform adapter. Unsupported platforms report that state
without failing. AppleDouble sidecars remain outside this decision.

## 2026-08-09: keep the first event parser explicitly bounded and diagnostic

**Status:** Accepted

The first parser spike is confined to a caller-supplied Track 7 byte range and
the experimentally supported sequence `[timing VLQ][pitch][attack][release][duration VLQ]`.
It reports source offsets and provisional accumulated interval units. It does
not scan whole files, infer unsupported framing or channels, classify the
provisional third structure as a confirmed note, or emit MIDI. A command-line
interface is deferred until an evidence-backed region-selection contract is
available.

## 2026-08-09: preserve strict sequential alignment for Track 7 validation

**Status:** Accepted

Full-list validation compares screenshot row N only with binary candidate N
after the experimentally established fourth-event anchor. The first three
visible rows remain an explicitly unaligned prefix, and a parser failure is
reported rather than repaired by skipping or resynchronizing. This preserves
the distinction between screenshot ground truth, parser mechanics, and
unresolved track framing.

## 2026-08-09: defer automatic Track 7 discovery after container survey

**Status:** Accepted

Comparative scanning found a family of bounded candidate chains with recurring
marker and post-context bytes, but no validated track identity or metadata
reference selects Track 7. Phoenix therefore keeps the forensic offset
explicit and does not add heuristic discovery until one neighboring candidate
is independently matched to Studio Vision ground truth.

## 2026-08-11: retain bounded diagnostics after Track 3 #2 generalization

**Status:** Accepted

Track 3 #2 independently validates the Track 7 property and timing model and
was initially assigned to the wrong one of two duplicate 17-note regions.
Complete ground truth corrects the selected chain to `0x318b5`, validates all
84 notes, and explains the UI count as one Patch plus 84 notes. Its analogous
marker-relative value is 86 rather than 85, so Phoenix withdraws the exact
85/143 event-count interpretation and still does not add a general parser.
Patch parsing remains deferred until its local fields and timing are isolated.

## 2026-08-11: identify the Patch program byte without adding Patch parsing

**Status:** Accepted

Experiment 023 confirms the preregistered direct Program Change field at
`0x318a5`: displayed `PC 23 -> PC 24` produces `17 -> 18`, while the Patch
name and complete note stream remain unchanged. Phoenix records the field but
does not implement Patch parsing because event start/type/timing framing remains
partial. A non-adjacent-value replication is required before broader field-
behavior claims.

## 2026-08-13: accept direct Patch Program Change storage, retain parser boundary

**Status:** Accepted

Experiment 024 independently confirms the non-adjacent preregistered change:
displayed `PC 23 -> PC 100` produces aligned project byte `17 -> 64` at
`0x318a5`. Together with Experiment 023's `PC 23 -> PC 24` / `17 -> 18`, the
three controlled states establish that this Patch event's project byte directly
equals the Vision-displayed PC number. Literal `Ming Dynasty` and all known
note performance data remain unchanged.

Phoenix still does not add Patch parsing. Exact event start/end ownership,
type discrimination, and timing representation remain insufficiently
established. The next controlled experiment should change only Patch position.

## 2026-08-13: identify absolute Patch timing, retain incomplete framing

**Status:** Accepted

Experiment 025 changes only Patch position from `1·2·50` to `1·2·51` and
identifies `0x31886–0x31887` as its absolute-position field: 7-bit VLQ
`84 12` = 530 becomes `84 13` = 531 in the established Studio Vision timing
unit. A second component changes 8,908 to 8,907 and, with a stable 165 field,
supports the preregistered Patch-to-first-note interval change 9,073 to 9,072.
PC, literal name, and all note data remain stable.

Phoenix still does not implement Patch parsing. The primary Patch span is more
precise, but unresolved metadata, compound interval ownership, complete end
framing, and the exact Patch event-type discriminator remain incomplete. A
same-length Patch-name control is the next evidence step.

## 2026-08-13: confirm editable Patch-name payload, defer decoder

**Status:** Accepted

Experiment 026 changes only `Ming Dynasty` to equal-length `Phoenix Test` and
confirms the editable 12-byte ASCII payload at `0x31891–0x3189c`. The aligned
position, PC, timing components, note boundary, and complete note stream remain
stable. A preceding `0x0c` strongly suggests Pascal-style length framing, but
the equal-length experiment cannot establish variable/fixed width, padding,
relocation, or record-size behavior.

Phoenix still does not implement a Patch decoder. A hard-bounded diagnostic
extractor could report the confirmed fields in this known instance, but event
end ownership, name-length framing, compound interval ownership, and the exact
event-type discriminator remain incomplete. The next control should use a
deliberately shorter Patch name.

## 2026-08-13: accept variable-length Patch name and bounded decoder spike

**Status:** Accepted

Experiment 027 confirms the locked length prediction: `0x31890: 0c -> 07`,
followed immediately by seven ASCII bytes for `Phoenix`. The project and every
following Patch/Note anchor become five bytes shorter with no padding. PC and
all note data remain unchanged after relocation. A local payload length changes
27 to 22, a broader size candidate changes 653 to 648, and dependent offsets
also adjust by five.

A bounded, diagnostic-only Patch decoder spike is now justified for this known
Track 3 #2 representation. It must use explicit evidence bounds/anchors,
decode only confirmed position, length-prefixed name, and PC fields, report
source offsets and relocation, and validate against Experiments 007 and
023–027. This does not authorize general Patch discovery, a general Studio
Vision event grammar, or MIDI emission.

## 2026-08-13: keep the first Patch decoder explicitly bounded

**Status:** Accepted

The Track 3 #2 Patch spike requires an explicit known position-field start and
exclusive end bound. It decodes only confirmed absolute position,
one-byte-length-prefixed ASCII name, direct Program Change, and the transition
to Note status `0x90`. It validates Experiments 007 and 023–027, including the
short-name relocation, and performs no scanning or recovery.

The spike remains a library/test diagnostic. No CLI was added, existing Track
7 behavior was not refactored, and no general Patch discovery, event grammar,
unknown-field interpretation, interval ownership, or MIDI emission is implied.

## 2026-08-13: do not relax Patch context after independent Track 3 validation

**Status:** Accepted

The naturally occurring `Ode to Clarke` / `Track 3` / `JV-1080` event repeats
the core position, length-prefixed ASCII name, direct PC, post-PC VLQ shape,
and Note transition relationships. It differs in the local payload-length
relationship and pre-name, post-name, and pre-Note context. The existing Track
3 #2 decoder fails unchanged at the first difference, as its contract intends.

This supplies partial cross-track generalization evidence but does not justify
weakening context validation or renaming the decoder as a shared abstraction.
Investigate the exact local-context differences with another independently
identified Patch event before changing code.

## 2026-08-13: separate common Patch semantics from variable framing

**Status:** Accepted

The third authentic event, Track 1 / Juno-106 / `Empty Patch` / PC 61, confirms
that all three representations share a VLQ position, `ff 7c`, local payload
length through PC, length-prefixed ASCII name, direct PC, post-PC VLQ, and
transition to `0x90`. They do not share fixed widths or context: Track 1 uses a
one-byte position, Track 3 #2 has a wider post-name field and extra pre-Note
context, and bank-tail values vary with exported bank selection.

Future design work may define a bounded representation-oriented contract that
preserves opaque context and validates common measured relationships. The
existing Track 3 #2 decoder must remain strict until such a design is reviewed;
no checks are relaxed and no generalized parser is authorized here.

## 2026-08-13: bound the shared Patch design at the known Note transition

**Status:** Accepted design; not implemented

The future `decode_bounded_patch_representation` contract requires the caller
to supply the exact position start and an exclusive boundary immediately after
the expected `0x90` Note status. Payload length locates PC as the final payload
byte; all unknown pre-name, post-name, and pre-Note bytes are returned with
absolute provenance. The decoder performs no discovery or resynchronization.

Bank-correlated bytes remain opaque in v1, payload length is returned as
diagnostic framing metadata, and post-PC timing retains neutral component
terminology. After independent implementation tests pass, the current Track 3
#2 decoder should become a strict wrapper that preserves its controlled-file
checks and public diagnostic behavior.

## 2026-08-13: retain independent strict decoder after shared implementation

**Status:** Accepted

The shared bounded representation decoder now passes the authentic,
controlled, and malformed matrix. The Track 3 #2 decoder remains independent
temporarily: immediate wrapper conversion would require error translation and
failure-order equivalence work unrelated to the shared semantic result.

Existing controlled acceptance remains unchanged, and an explicit regression
test confirms that the strict API still rejects Track 1 and Track 3. Wrapper
conversion is deferred to a focused compatibility refactor, not abandoned.

## 2026-08-13: stop ordinary Patch replication after fourth authentic event

**Status:** Accepted

Track 2 / JV-1080 validates the shared bounded decoder unchanged and matches
the compact Track 3 framing family. Four authentic events now support the
common semantic core, while Track 2's `ff 51 01` supplies a second exact
bank-export correlation. Further ordinary Patch-event replication is lower
value than isolating the remaining bank-tail semantics with a controlled
single-variable change.

## 2026-08-13: keep bank bytes opaque after controlled CC32 confirmation

**Status:** Accepted

Experiment 028 confirms that changing only Track 2 CC32 from 1 to 2 changes
only the final bank-tail candidate from `01` to `02` inside the bounded Patch
representation. This establishes that subfield for this representation, while
the candidate CC0 byte has not been independently changed and the leading
`ff`/absent-bank behavior remains unresolved.

The shared decoder therefore continues returning the full post-name context
as opaque provenance. No first-class bank fields are added until CC0 and
absence semantics have comparable controlled support.

## 2026-08-13: defer semantic bank fields pending optionality evidence

**Status:** Accepted

Experiment 029 complements Experiment 028: independent CC0-only and CC32-only
changes establish the middle and final bank-tail bytes as direct values for
Track 2. This is sufficient to design diagnostic bank candidates, but not to
define optional semantic fields because the leading `ff` and `ff ff ff`
no-bank representation have not been controlled.

Keep the post-name context opaque in production. Design optional bank fields
only after a bank-removal experiment establishes presence/sentinel behavior.

## 2026-08-13: prioritize MIDI recovery by event family

**Status:** Accepted, with provenance scope corrected below

Available exports show 5,112 Control Changes across fourteen controller
numbers, with most traffic belonging to repeated NRPN/RPN sequences. This is
Studio Vision export evidence; it is not `newest STUFF` coverage because 5,108
events belong to source-unresolved export sets. Treat Controller as one
candidate structural event family containing timing, number, value, and
channel/instrument context. Do not create parameter-specific CC decoders
without evidence of distinct project grammars.

Provenance-controlled Controller ground truth precedes bank-removal optionality
work and additional ordinary Patch replication. The 440 Pitch Bend events are
also source-unresolved and cannot be mapped to this project yet. Export meta
events remain separate from project-event claims.

## 2026-08-13: require provenance before export-to-project correlation

**Status:** Accepted correction

Studio Vision UI ground truth shows that neither `ANALOG.MID #2` nor `Analog
Seq` is an active sequence in the untouched `newest STUFF` baseline. Withdraw
all mappings based only on export filenames, SMF track names, or readable
project strings. Only the `Ode to Clarke` export set is currently proven to
come from this project through independent musical and binary correlation;
the `ANALOG` and `BATTL2GS` sets remain source-unresolved.

Aggregate event-family counts remain useful evidence of what Studio Vision
exports can contain, but must not be presented as `newest STUFF` coverage.
Controller-region selection is blocked until a controller-rich active sequence
has provenance-controlled export or direct List Window ground truth.

## 2026-08-16: implement ordinary Controllers as one bounded record family

**Status:** Accepted

The later provenance-controlled `Bells for her` export satisfies the preceding
gate. It establishes 395 ordinary records using `timing VLQ | ff 41 | 05 |
opaque context[3] | number | value`: Track 9 matches all 120 number/value pairs
and event-start deltas, and Tracks 3, 4, 6, and 14 validate the family
independently across CC1 and CC7.

Implement one exact-bound, provenance-preserving Controller decoder rather
than parameter-specific decoders. Keep context opaque, leave absolute timeline
accumulation to the caller, and do no scanning. Patch-derived CC0/CC32 export
messages remain Patch state. No Experiment 030 is required; broader CC,
version, project, and device generality remains unproven.

## 2026-08-16: keep Controller implementation exact-bound and structural

**Status:** Accepted and implemented

`decode_bounded_controller_record` implements the preceding decision with
exact consumption and byte provenance. The binary layer accepts `u8` number
and value fields without inventing MIDI-range validity, preserves context as
opaque bytes, and returns only the encoded event-start delta. Fixed authentic
fixtures cover Tracks 3, 4, 6, 9, and 14. Patch bank tails fail structurally;
no discovery, timeline accumulation, or CLI integration was added.

## 2026-08-16: do not implement a generic mixed-event walker yet

**Status:** Accepted

Keep record decoding separate from stream walking. A walker may advance only
when the event family and end are justified at its current cursor; unsupported
bytes leave cursor and absolute-time state unchanged. Controller-only bounded
walking and explicitly asserted consecutive Note-chain walking are ready as
separate profiles. Generic mixed Note/Controller walking is blocked by the lack
of a collision-resistant Note discriminator, while Patch adds an externally
supplied first-Note boundary and compound timing handoff. Do not implement
fallback decoder trials, scanning, skipping, or resynchronization.

## 2026-08-16: design Channel Pressure as an exact-bounded stateful run

**Status:** Accepted

The provenance-controlled Track 9 correlation establishes one 32-event run:
`timing VLQ | d0 | value` entry followed by 31 `timing VLQ | value`
continuations, with 32/32 value and timing agreement. A bounded decoder may
require exact caller run bounds, validate `d0` at entry, and parse compact
continuations only under state established by that entry. It must not expose a
stateless continuation decoder, infer MIDI channel, discover the run end, or
accumulate absolute time. No controlled experiment is needed for this observed
contract. The following explicit `90` is evidence for that one transition and
does not authorize a generic mixed walker.

## 2026-08-16: implement the exact-bounded Channel Pressure run contract

**Status:** Accepted and implemented

`decode_bounded_channel_pressure_run` implements the observed stateful contract
with exact consumption and byte provenance. It requires `d0` on entry and
allows timing/value continuations only after that state is established. It
does not restrict pressure bytes at the binary layer, infer MIDI channel or
absolute time, discover bounds, scan, or integrate with mixed-event walking.
The authentic 32-entry Track 9 fixture and malformed bounds/data tests enforce
those limits.

## 2026-08-16: design Pitch Bend as an exact-bounded stateful run

**Status:** Accepted

The provenance-controlled Track 14 correlation establishes 102 Pitch Bend
events in nine exact runs with 102/102 timing and direct LSB/MSB value matches.
Each run uses `timing VLQ | e0 | LSB | MSB` at entry and `timing VLQ | LSB |
MSB` for continuations. Design one caller-bounded, provenance-preserving run
decoder that requires `e0`, consumes the supplied bound exactly, and keeps the
two stored bytes authoritative. Do not infer MIDI channel, discover run ends,
expose arbitrary continuation parsing, accumulate absolute time, or generalize
Channel Pressure and Pitch Bend into universal running status. No controlled
experiment is required for this observed contract.

## 2026-08-16: implement the exact-bounded Pitch Bend run contract

**Status:** Accepted and implemented

`decode_bounded_pitch_bend_run` implements the observed `e0` entry and
two-byte continuation state with exact consumption and byte provenance. The
two located stored bytes remain authoritative; `raw_value()` is derived. All
nine fixed Track 14 ranges and all 102 timing/value tuples are regression
fixtures. No channel inference, run discovery, generic running-status
abstraction, absolute-time accumulation, or mixed-walker integration was added.

## 2026-08-18: design Tempo only as an exact-bounded initial representation

**Status:** Accepted

Natural `Bells for her` evidence and the corrected Experiment 002/004
comparison establish one seven-byte form: `00 ff 51 03` followed by unsigned
24-bit big-endian MPQN. Design a decoder that requires an exact caller-supplied
seven-byte bound, preserves every located byte, requires the leading zero and
fixed tag/length, and derives MPQN without replacing its source bytes.

Keep the leading field semantically neutral as `initial_position_byte`; all
known examples are initial Tempo at sequence start, but absolute-versus-delta
meaning and nonzero forms are unproven. Keep Tempo in the sequence-level
Meter/Tempo structural layer, outside mixed performance-event walking. Do not
parse the correlated secondary copy, discover records, or imply general Tempo
map support. No controlled experiment is needed for this bounded initial form.

## 2026-08-18: implement the exact-bounded initial Tempo contract

**Status:** Accepted and implemented

`decode_bounded_initial_tempo` accepts only an exact caller-supplied seven-byte
range matching `00 ff 51 03 tt tt tt`. It returns every byte with absolute
provenance, derives unsigned 24-bit big-endian MPQN, and derives optional BPM
without dividing by zero. Length and structure failures are deterministic and
the decoder never scans.

Keep `initial_position_byte` restricted to zero without assigning absolute or
delta semantics. Do not parse the secondary copy, discover sequence structure,
construct Tempo maps, enter the performance-event walker, or emit MIDI. Fixed
natural and controlled fixtures establish only the bounded initial form.

## 2026-08-18: design Meter only as an exact-bounded initial representation

**Status:** Accepted

Natural 4/4 and 6/8 evidence plus controlled Experiment 030 7/8 establish the
eight-byte primary `00 ff 58 04 nn dd xx yy`. Design one decoder requiring an
exact caller-supplied bound, zero initial-position byte, fixed tag/length, and
absolute provenance for all eight bytes. Preserve numerator, denominator
exponent, third payload, and fourth payload directly. Derive `2^dd` only as an
overflow-safe convenience and do not reject framing-valid payload values.

Keep the third payload semantically neutral. Historical exports correlate
`xx 08 -> cc 18` and `xx 06 -> cc 0c`, but SMF policy is separate from binary
decoding and the universal rule remains partial. Do not parse secondary copies,
discover Meter, infer nonzero position semantics, enter the performance-event
walker, or imply general Meter-map support. No further Meter experiment is
needed for the bounded representation or current converter goal.

## 2026-08-18: implement the exact-bounded initial Meter contract

**Status:** Accepted and implemented

`decode_bounded_initial_meter` accepts only an exact caller-supplied eight-byte
range matching `00 ff 58 04 nn dd xx yy`. It preserves every source byte with
absolute provenance and derives `2^dd` only through an overflow-safe optional
convenience. Arbitrary numerator, exponent, third payload, and fourth payload
bytes remain structurally preservable after framing validation.

Keep the initial-position byte restricted to zero without assigning position
semantics. Do not implement historical `xx -> cc` conversion, Meter discovery,
secondary copies, Meter maps, sequence/container integration, mixed-event
walking, or MIDI emission. Authentic and synthetic tests enforce these bounds.

## 2026-08-18: enter sequence parsing through the checked root record stream

**Status:** Accepted

Treat the first eight bytes as an opaque fixed root header, then walk top-level
records from offset eight with checked `type | big-endian u32 length | payload`
framing. This consumes Experiment 007 exactly through EOF and reaches the first
type-`0x01` sequence preamble at `0x006abc` without searching for names,
descriptor labels, Meter/Tempo tags, or event signatures. Require the full
sequence-local invariant set before admitting a type-`0x01` record as a
sequence. Preserve root-header words and unknown record payloads without
assigning semantics.

The fixed header boundary and exact-to-EOF record walk are cross-validated in
the older authentic sample and controlled project population. Do not invent a
root pointer, sequence count, or parent payload. `Sequence I` inactive
descriptor mapping and track-local event ends remain partial; keep those and
mixed-event/family-run interpretation separate from root/container traversal.

The first semantic parser profile supports only the proven 208-byte
preamble/166-byte descriptor form. Keep generic root framing available for the
older authentic sample, but reject its 120-byte sequence form under this
profile rather than deriving a width from opaque root word 3. When descriptor
and track-pair counts differ, preserve both collections with unresolved
associations; do not infer which descriptor is inactive. A malformed
type-`0x01` candidate fails semantic project classification at that cursor and
must not be skipped in search of a later candidate.

This contract is now implemented in the `sequence_container` module. Keep the
generic `parse_root_record_stream` guarantee distinct from strict
`parse_project_166` classification. The implemented parser may supply exact
initial Meter/Tempo bounds and track-primary containing ranges, but it must not
promote those containing ranges to exact event ranges or absorb mixed-event
walking.

## 2026-08-18: terminate 166-profile track events at the validated seven-byte tail

**Status:** Accepted as read-only structural evidence

For an exact track-primary payload supplied by the sequence-container parser,
validate the repeated final grammar `ff aa bb cc ff 2f 00`. In the established
166-byte profile, define performance-event bytes as payload `+14` through
payload end minus seven. This is supported by all 132 authenticated track
primaries and by 15 zero-count tracks whose post-header region contains only
the tail. Preserve `aa bb cc` without semantic interpretation and do not apply
the rule to the unsupported 120-byte profile without evidence.

Keep family/run termination separate. A Controller has an exact next cursor,
but internal Note-run exit and Channel Pressure/Pitch Bend state exits remain
partial. Experiment 031 subsequently resolves Patch-to-first-Note navigation
for the established bounded grammar.

## 2026-08-18: admit only the validated direct or extended Patch transition

**Status:** Accepted as read-only structural evidence

A future bounded mixed walker may advance from an established Patch start by
validating `position VLQ | ff 7c | payload length | payload ending in PC |
post-PC timing VLQ`, then accepting either immediate `90` or exactly one
established `ff 60 | one-byte length | payload | final timing VLQ | 90`
extension. It must derive every boundary at the current cursor and reject
unknown tags, malformed lengths, repeated optional records, or missing status;
it must not scan for `90`.

For direct forms, the post-PC value owns the complete Patch-to-first-Note
interval. For established extended forms, add the post-PC and final timing
values. Experiment 031 proves final timing `81 25 -> 81 26` owns a +1 first-Note
edit while `c5 4c` and the framed context remain unchanged. This decision does
not authorize the mixed walker: current-cursor Note/Pressure/Bend state exit is
still unresolved.

## 2026-08-18: dispatch bounded mixed events by post-VLQ byte class

**Status:** Accepted as design-enabling structural evidence

Within an exact 166-profile event region, test `cursor == event_end` before
decoding. Otherwise decode one bounded timing VLQ and classify its first
following byte. Under active Note, Channel Pressure, or Pitch Bend state,
`00..7f` selects that family's established continuation width. `ff` selects a
strict known tagged/context branch. Observed `80..ef` status branches are only
`90`, `d0`, and `e0`. Unknown high-bit branches and data bytes without active
state are deterministic unsupported errors, never fallback or scan triggers.

This rule reproduces the complete authenticated Track 9 and Track 14 event
populations and dynamically derives Pressure/Bend run ends. It permits design,
not implementation, of one exact-bounded walker limited to Note, Patch,
ordinary Controller, Channel Pressure, and Pitch Bend. Do not generalize to
SysEx, unknown tags/statuses, other Patch contexts, the 120-byte profile, or
MIDI emission.

## 2026-08-18: make the first mixed walker exact-bounded and transactional

**Status:** Accepted design

The first mixed walker receives one absolute `event_start..event_end` range
already derived and tail-validated by the 166-profile container layer. It owns
only cursor dispatch, accumulated position, and the compact active states None,
Note, Channel Pressure, and Pitch Bend. Controller remains an individually
tagged record; Patch and the established `ff 60` syntax are strict transition
forms rather than persistent states.

Return decoded output only after exact complete consumption. Preserve a coupled
Patch-to-Note result because its source grammar and existing bounded Patch
representation cross the logical event handoff. Reuse the bounded Controller
decoder directly, wrap the bounded Patch decoder after deriving its transition
bound, and factor shared single-event primitives for Note, Channel Pressure,
and Pitch Bend without weakening their current bounded APIs. Unknown branches
fail at the current cursor; no decoder probing, scan-ahead, backtracking, or
partial-success recovery is allowed.

**Implementation:** `mixed_event::walk_bounded_mixed_events` now enforces this
contract. Authentic Track 9 and Track 14 consume exactly with the established
184/601 counts; one Pressure entry and nine Bend entries dynamically establish
their runs. Shared single-event primitives preserve the previous exact-run
decoder APIs and tests.

## 2026-08-19: design first MIDI export as Format 1 with an explicit adapter

**Status:** Accepted design

Use SMF Format 1 for one Studio Vision sequence, with a conductor track for
sequence name, initial Tempo, and initial Meter and one SMF track per included
validated musical track. An export adapter, not the parser or serializer,
converts decoded Phoenix representations, schedules explicit release-velocity
Note Offs, resolves channel/text/Meter/Patch policy, and produces a report.
Use the authenticated 480-position-unit to 480-PPQN identity mapping; do not
generalize it to other profiles.

Choose `Ode to Clarke` as the first authentic target and direct SMF
serialization as the initial implementation approach. The implementation gate
remains partial because current decoded representations do not expose MIDI
channel. Missing channel is an error; no default channel is permitted. A
proven per-track override can support the authentic proof without becoming a
parser fact. Unsupported nonempty structures or events fail the first export,
and no omission is silent.

## 2026-08-19: use a provenance-locked Ode channel manifest for the proof

**Status:** Accepted research consequence

Independent parsing establishes that all nine `Ode to Clarke Multi All`
musical tracks use exactly one channel and maps them ordinally to the nine
authenticated descriptor/pair bindings. Exhaustive relative-byte and nibble
tests find no direct channel field in the 166-byte descriptors; Patch/device
and sequence-local routing relationships remain unresolved.

For the first authentic proof only, permit an immutable channel manifest keyed
by the exact project SHA-256, sequence range/name, descriptor ordinal/range,
pair ordinal, primary range, and event range. Never select it by filename or
track label, never default a missing channel, and never describe the manifest
as parser knowledge. This resolves the first-target channel gate without
claiming general Studio Vision routing support.

## 2026-08-19: implement Phase A as pure direct SMF serialization

**Status:** Implemented

Extend the existing `smf` module with MIDI-domain Format 1 primitives rather
than adding a MIDI dependency or a second conflicting module. Public channels
are human-numbered 1 through 16; seven-bit data is validated by type. Musical
tracks accept absolute scheduled channel messages, apply the explicit same-tick
priority plus stable ordinal, convert checked deltas, and automatically append
one final EOT. A separate conductor helper fixes tick-zero name/Tempo/Meter/EOT
order.

Keep `SerializedTrack` construction private so EOT cannot be omitted or
duplicated. Emit explicit statuses only. The serializer imports no Studio
Vision modules, performs no channel inference or file I/O, and contains no Ode
manifest values. Synthetic byte-exact tests include an independent SMF parser.

## 2026-08-19: implement Phase B as transactional decoded-event adaptation

**Status:** Implemented

Add a focused `midi_export` module between existing decoded values and the SMF
serializer. Require an explicit human-facing channel assignment and explicit
`Identity480` timing policy. Encode stable source ordinals as even values and
generated Note Off ordinals as the corresponding odd values; reject duplicate
or overflowing ordinals. Preserve attack and release velocity and schedule
each Note ending independently.

Map Controller, Pressure, and Bend directly through validated MIDI data bytes.
Require Patch bank/no-bank safety to be classified upstream; never inspect
opaque Patch bytes in the adapter. Implement the documented historical/fallback
Meter policy, initial Tempo validation, counts/warnings/report aggregation, and
transactional failure. Preserve valid UTF-8 text and reject invalid legacy text
as `MacRomanDeferred` pending an explicit encoding implementation decision.
Synthetic tests alone cover all mappings and complete in-memory Format 1
integration; no authentic artifact or manifest is used.

## 2026-08-19: isolate the first authentic proof in provenance-locked test integration

**Status:** Implemented; automated and user-observed Logic Pro 12 validation pass

Keep the Ode Track 3 project hash, structural ranges, channel override, and
Patch classification in focused authentic integration coverage, outside the
generic parser, adapter, and serializer. Validate every manifest key before
adaptation and fail on any unsupported event or comparison mismatch.

Permit the general SMF serializer to add a caller-supplied Track Name at tick
zero to a musical track; this is MIDI infrastructure and contains no Studio
Vision policy. Write the permanent research proof only through an explicit
test-owned action after the in-memory normalized comparison passes. Do not
turn that action into CLI behavior.

## 2026-08-19: supply Phase D proof policy to reusable sequence assembly

**Status:** Accepted; reusable D1 assembly implemented

Build Phase D around a pure transactional multitrack assembler that consumes a
caller-supplied manifest/policy. Keep the concrete Ode project/reference
hashes, nine structural rows, channel overrides, and Patch classifications in
authenticated proof integration. This permits parsed routing to replace the
manifest later without changing sequence assembly or MIDI serialization.

Emit musical tracks in validated descriptor order. For Ode this exactly equals
pair order and the authenticated Studio Vision export order. Fail the complete
sequence on a missing, extra, mismatched, unsupported, or failed row; never
return a successful partial multitrack file.

The implemented `multitrack_export` module follows this boundary. It accepts
only decoded events and explicit MIDI-domain policy, preserves caller track
order, and returns complete Format 1 bytes plus adapter-derived reports only
after every conductor/track step succeeds. It contains no authenticated proof
constants or file I/O.

The D2 concrete manifest and Patch expectations remain entirely in focused
authenticated integration coverage. That integration validates all nine rows
before invoking D1 once; no Ode value or authenticated channel is promoted into
generic production code.

D3 reuses that integration path and compares both complete SMFs through a
running-status-aware test-side parser. Success is transactional across all ten
tracks and requires normalized musical timing/value equality; optional
metadata, historical Note-end representation, raw encoding/order, and EOT
padding remain explicit policy differences rather than serializer changes.

D4 exposes persistence only as an ignored test-owned research action. It
refuses to overwrite a differing final file, writes through a same-directory
temporary file, and validates byte identity plus the complete comparison from
disk. Normal tests remain proof-write-free; no general CLI/file-writing policy
is inferred from this one approved artifact.

D5 records manual validation separately from automated equivalence. The
user-observed Logic Pro 12 result establishes that the exact proof opened with
all nine expected musical tracks and looked and sounded correct during
playback, with no problem reported. Logic's nine top-level Event List/filter
entries are track/region observations, not an independent 1,308-Note count.
The bounded Ode cycle is complete without broadening claims to arbitrary
projects, general routing, unsupported families, original sound recreation,
or a user-facing export workflow.

## 2026-08-19: use a native macOS thin client over Phoenix Core

**Status:** Designed; implementation deferred

Use SwiftUI with focused AppKit interop for the first macOS desktop prototype,
calling Phoenix Core through a small versioned C-compatible application-service
boundary. Swift owns user-selected URLs, window state, presentation, standard
panels, accessibility, and Finder interaction. Rust owns identification,
inspection, readiness, conversion policy, transactional export, validation,
reports, and technical errors.

Inspect one file at a time. Permit real export only for an explicitly labeled
compatibility profile whose complete provenance policy validates in Core;
arbitrary projects remain inspectable but receive no enabled Export action.
Authenticated offsets/channels never enter Swift or generic parser modules.
General channel derivation is not a prerequisite for the shell architecture.

## 2026-08-19: define UI0 as an owned versioned Core service contract

**Status:** Designed; implementation deferred

Use a session-scoped opaque Core handle and session-scoped opaque SequenceId;
never expose offsets, borrowed parser values, or names as identity. Return owned
project/sequence summaries, four-state readiness plus machine reason codes,
warnings, on-demand diagnostics, and transactional export reports.

For the first ABI, keep Rust calls synchronous and run them off the Swift main
actor. Prefer JSON payloads over a tiny C ABI with Rust-owned length-delimited
response buffers and an explicit free function; implementation may add a JSON
serialization dependency only after separate approval. Swift passes authorized
POSIX paths while security-scoped access is active; Core owns collision checks
and final atomic output writing.

Core alone matches explicit compatibility profiles and selects channel/Patch
policy. Arbitrary projects remain inspectable but do not receive Export. All
processing is local and diagnostics never expose arbitrary raw bytes.

## 2026-08-19: implement UI0B as conservative inspection-only service

**Status:** Implemented; export/profile matching deferred

`AppService` owns path-based inspection sessions and translates existing Finder
identification plus the established 166-byte parser into owned UI0A DTOs. A
readable unrecognized file returns a safe assessment rather than an operation
failure; unreadable files return typed `FileUnreadable`. Parsed sequences are
reported in structural order but remain `PartiallySupported` with
`MissingChannelRouting` until a complete Core compatibility profile exists.
Sessions retain path, bytes, size, and SHA-256 for future revalidation. No Ode
constants, channel inference, export writing, FFI, or serialization dependency
was introduced.

## 2026-08-09: keep Track 7 boundary claims conservative

**Status:** Accepted

Local bytes match the property fields of all 143 visible rows. Rows 2–143
also have established timing prefixes; row 1's prefix is not assigned because
backward VLQ boundaries are ambiguous. The post-row-143 bytes begin with a
syntactically consumable VLQ but an out-of-range property byte, so the bounded
note model stops without calling the bytes a footer, terminator, or complete
track framing.

## 2026-08-19: keep compatibility profiles Core-only and declarative

**Status:** Designed; UI0C implementation deferred

Use an immutable compiled-in registry queried only after generic inspection.
Profiles match exact project hash/size, parser profile, sequence identity, and
complete structural track manifests before returning a capability. Authenticated
channel and Patch policy remains in a private resolved handle; names never
trigger Ready and no fallback policy is allowed. Multiple matches are an
ambiguity error, and export must re-read, re-hash, reparse, and re-match. The
existing UI0A `ProfileCapability`/readiness DTOs are sufficient; no policy
internals cross into the app contract.

## 2026-08-19: keep UI0C2 evidence conservative

**Status:** Implemented; profile assessment deferred

AppService stores an owned structural snapshot and can build generic
`ProfileEvidence` for parsed sessions. It copies raw sequence names, ranges,
descriptor/pair identities, and source provenance. Because the reusable parser
does not expose the proof-specific exact event-tail derivation, event ranges,
decoded families, Patch facts, and channels remain incomplete rather than
guessed. UI0B readiness and export capability are unchanged.

## 2026-08-19: scope exact event bounds to the validated 166-byte profile

**Status:** Evidence accepted; helper implementation pending

The all-sequence 132-primary census, empty-track controls, and independent
Bells/Ode observations confirm `ff aa bb cc ff 2f 00` at every established
166-profile primary payload end. A future parser helper may validate that tail
and derive payload `+14 .. payload.end - 7` with checked arithmetic. This does
not generalize to the unsupported 120-byte profile, does not give the walker
terminal discovery, and does not resolve internal Pressure/Bend run exits.

## 2026-08-19: implement validated Descriptor166 event bounds

**Status:** Implemented; UI0C2C remains pending

The parser now exposes `TrackEventBounds` for a track pair. It checks the
already-established 14-byte prefix relationship, validates `ff ?? ?? ?? ff 2f
00` at the payload end, and derives exact half-open event and tail ranges with
checked arithmetic. Empty event ranges are valid. The containing range remains
unchanged, and AppService records an exact range without marking evidence
complete or changing readiness. The helper is not applied to the 120-byte
profile or used for channel/Patch policy.

## 2026-08-19: keep UI0C2C family inventory factual

**Status:** Implemented; compatibility evidence remains incomplete

AppService reuses the existing bounded mixed-event walker over validated
Descriptor166 event ranges. It records canonical family presence and logical
counts, including Patch and its coupled Note as two logical facts, but does not
populate Patch evidence, channels, readiness, or export capability. A walk
failure yields no partial inventory.

## 2026-08-19: permit only bounded generic Patch evidence

**Status:** Implemented; bank semantics and routing remain unresolved

Four authentic Patch representations validate a shared bounded semantic core,
and controlled program changes confirm the direct program byte. AppService may
therefore record the walker item ordinal, decoded representation range, and
direct program value from a successful `PatchToNote` walk. Bank fields remain
opaque, and no channel, translation, readiness, or compatibility policy is
inferred.

## 2026-08-19: close the UI0C2 evidence adapter milestone

**Status:** Complete; UI0C3 deferred

The accumulated UI0C2A–UI0C2D work now provides the complete conservative
ProfileEvidence bridge intended for this milestone: exact Descriptor166 bounds,
bounded-walker family/count inventory, and the audited generic Patch subset.
`evidence_complete` remains false in generic AppService evidence because that
service does not assert authenticated policy completeness. The profile matcher
validates explicit structural/Patch fields while the resolved profile supplies
authenticated routing. Channel routing, bank semantics, the unsupported
120-byte profile, authenticated profile migration, registry assessment, and
export remain intentionally later work.

## 2026-08-19: isolate the authenticated Ode profile

**Status:** Implemented; AppService handoff deferred

The exact Ode research manifest, channels, and four authenticated Patch
translations live only in `compatibility_profiles`. Generic modules remain
target-independent. A built-in registry match returns a resolved policy, but
inspection readiness and export remain unchanged until a separately designed
handoff adds source revalidation.

UI0C4A now stores these results privately per `SequenceId`; it deliberately does
not project a capability or Ready state into app-facing summaries.

The banked-policy matcher has explicit regressions for absent, matching,
partial, and contradictory observed bank values; absent generic evidence never
acts as permission to ignore a contradictory observed value.

## 2026-08-19: design per-sequence profile assessment handoff

**Status:** Designed; UI0C4A implementation deferred

AppService will own an immutable registry and assess every discovered sequence
against retained evidence, storing `SequenceId`-keyed resolved policies
privately. UI0C4A will not change readiness; a later projection may do so only
for exact matches. Export must reread, rehash, reparse, remap, and reassess the
selected sequence before using any stored policy.

## 2026-08-19: implement one-shot source revalidation

**Status:** Implemented; export and readiness projection deferred

UI0C4B requires fresh path bytes, exact size/SHA-256 identity, fresh parsing and
evidence, same structural ordinal, registry reassessment, and equivalent
resolved policy before a future exporter may use policy. The successful handoff
owns the freshly checked bytes; stale inspection policy is never authorization.

## 2026-08-19: project exact compatibility matches into readiness

**Status:** Implemented; export remains deferred

UI0C4C projects only complete per-`SequenceId` matches to `Ready`, the stable
validated-profile reason, and safe capability. Non-matches retain generic
readiness. Project `Ready` requires every discovered sequence to be Ready;
mixed projects remain `PartiallySupported`. UI0C4B freshness remains mandatory
before export.

## 2026-08-19: design the UI0D export handoff

**Status:** Designed; implementation deferred

The existing `ExportSequenceRequest`/`ExportSequenceResponse` contract is the
service boundary. Export must invoke UI0C4B immediately, convert from its same
fresh owned bytes and resolved policy through `assemble_multitrack_sequence`,
then commit one complete SMF under the caller's destination/collision policy.
Readiness never authorizes export by itself, and sibling sequences remain
independent.

## 2026-08-19: implement the UI0D1 conversion-ready handoff

**Status:** Implemented; service orchestration and output remain deferred

`ConversionReadySequence` owns the fresh sequence metadata, ordered decoded
events, authenticated channel assignments, and strict Patch policy needed by
the existing `MultitrackSequenceInput`. It is built only from the same owned
bytes, evidence, structural ordinal, and resolved policy returned by UI0C4B;
the handoff performs no path reread, readiness projection, serialization, or
filesystem write.

## 2026-08-19: separate UI0D2 preparation from UI0D3 public commit

**Status:** Designed; implementation deferred

UI0D2 returns a crate-internal owned `PreparedExportSequence` containing the
selected session/sequence identity, display name, concrete safe capability,
and `MultitrackExportResult`. Its crate-internal `prepare_export_sequence`
accepts the existing `ExportSequenceRequest` by reference, but destination,
filename, collision, and operation-identifier fields remain untouched for
UI0D3. UI0D2 ends after fresh UI0C4B authorization, UI0D1 conversion, and
transactional in-memory assembly.

UI0D3 alone exposes public `AppService::export_sequence`, commits the prepared
SMF under the request's destination policy, and constructs
`ExportSequenceResponse` with a real `output_path`. Export preparation errors,
including UI0C4B failures, must carry `AppOperation::ExportSequence`; implement
this by parameterizing the private revalidation path with its calling operation
rather than duplicating revalidation or rewriting errors afterward.
