# Objective

UI0D1 implements the internal conversion-ready boundary described by the
export-handoff design. It consumes one UI0C4B `FreshValidatedSequence` and
produces owned MIDI-domain values suitable for the existing pure
`MultitrackSequenceInput` assembler.

# Implemented boundary

`build_conversion_ready_sequence` reparses the exact owned bytes carried by
the fresh validation result, locates its structural sequence ordinal, checks
that the parsed structure still agrees with the retained evidence, and adapts
the authenticated resolved policy into `ConversionReadySequence`.

The handoff is Core-internal. It does not accept paths, display names, parser
offsets, or caller-supplied policy.

# Owned conversion representation

`ConversionReadySequence` owns sequence metadata and an ordered vector of
`ConversionReadyTrack` values. Each track owns its context, raw label bytes,
decoded export events, authenticated channel assignment, and strict Patch
policy. `with_multitrack_input` creates a short-lived borrow of those owned
values in the existing `MultitrackSequenceInput` shape; the handoff itself
does not serialize or write output.

# Fresh source and policy authority

The source bytes, structural ordinal, evidence, and resolved policy are taken
from the same `FreshValidatedSequence` produced by UI0C4B. The handoff never
rereads a path or consults the inspection-time session snapshot. It uses the
validated Descriptor166 parser and exact event bounds, then reuses the bounded
mixed-event walker for event adaptation.

# Policy application

Every parser track association must be ordinal and every track must have one
matching policy/evidence row. Authenticated MIDI channels are validated through
the existing `MidiChannel` type. Patch transitions consume the corresponding
fresh Patch evidence and the authenticated `ProgramOnly` or
`BankSelectAndProgram` translation. There are no defaults, inferred channels,
or fallback translations.

Parser/descriptor structural order is the sole musical-track ordering
authority. Resolved policy rows are matched by structural key, so changing
their vector order cannot reorder output tracks. Missing, extra, duplicate, or
structurally inconsistent policy coverage fails the handoff.

# Failure boundaries

The internal `ConversionReadyError` distinguishes sequence identity drift,
track coverage, policy mismatch, metadata decoding, invalid exact bounds,
bounded-walk failure, and parsing failure. The current closed mixed-event model
is converted exhaustively.
Unresolved associations, missing/duplicate policy rows, mismatched ranges, and
incomplete Patch coverage fail before a conversion-ready value is returned.

# Reuse of proven conversion logic

The adapter uses the established Descriptor166 parser, validated track-event
bounds helper, bounded mixed-event walker, existing decoded-event constructors,
channel assignment provenance, and Patch policy types. It creates no second
event parser and does not copy target-specific proof constants into generic
code.

# Tests

Portable in-memory Descriptor166 tests verify correct and incorrect structural
identity, complete/missing/extra/duplicate/inconsistent policy coverage,
parser-authoritative ordering under reordered policy rows, authenticated
channel attachment, exact program-only and banked Patch translation, Patch
coverage refusal, sequence/track names, Tempo/Meter propagation, owned lifetime,
and operation after the originating path has been removed.

When available, the external authenticated fixture adds a complete Ode
nine-track regression with authenticated channel ordering, four Patch
observations, exact event counts, and the borrowed `MultitrackSequenceInput`
view. It is not required for portable UI0D1 correctness and skips cleanly when
absent. Neither test layer proves UI0D2 orchestration, SMF assembly/equivalence,
destination writing, or UI0D3 reporting.

# Explicit exclusions

UI0D1 does not add an AppService export operation, readiness changes, source
revalidation, filesystem output, collision handling, serialization orchestration,
batch export, or UI-facing DTOs. UI0D2 remains responsible for service
orchestration and invoking this handoff after explicit fresh revalidation.

# UI0D1 gate

The owned conversion-ready handoff is implemented and reviewable. The next
step is UI0D2 service orchestration, which must keep UI0C4B revalidation as the
only export-authorization path.

# Single recommended next step

Implement UI0D2 `AppService` export orchestration around fresh revalidation and
this owned handoff, without adding filesystem output yet.
