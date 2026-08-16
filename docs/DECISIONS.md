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

## 2026-08-09: keep Track 7 boundary claims conservative

**Status:** Accepted

Local bytes match the property fields of all 143 visible rows. Rows 2–143
also have established timing prefixes; row 1's prefix is not assigned because
backward VLQ boundaries are ambiguous. The post-row-143 bytes begin with a
syntactically consumable VLQ but an out-of-range property byte, so the bounded
note model stops without calling the bytes a footer, terminator, or complete
track framing.
