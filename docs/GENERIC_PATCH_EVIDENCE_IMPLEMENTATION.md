# Objective

Populate only the generic Patch facts justified by the UI0C2D provenance audit.

# Scope

Descriptor166 tracks with validated bounds and a successful existing bounded
mixed-event walk are eligible. The unsupported 120-byte profile is excluded.

# Audit authority

`docs/GENERIC_PATCH_EVIDENCE_AUDIT.md` establishes the shared representation
across four authentic events and controlled direct-program evidence.

# Permitted Patch facts

Each `PatchToNote` observation contributes its deterministic bounded-walk item
ordinal, decoder-provided absolute representation range, and direct decoded
program byte. Bank fields remain `None`.

# Prohibited inferences

No bank semantics, channel, instrument, destination program translation,
compatibility policy, or Patch container ownership is inferred.

# Walker provenance

Patch evidence is extracted from the same successful `MixedEventWalk` used for
family inventory. There is no second parser or source scan.

# Patch evidence extraction

The adapter copies `BoundedPatchRepresentation::representation_range` and
`program_change.value` into the owned `PatchEvidence` DTO. The ordinal is the
source item order in the bounded walk and remains deterministic.

# Source ordering

Multiple observations retain bounded-walk source order. No unordered set or
value-based sorting is used.

# Duplicate observations

Repeated equal program values remain separate observations with separate source
ordinals and ranges.

# Failure semantics

If bounds or walking fails, no partial Patch evidence is retained. A validated
exact range may remain recorded, while family/count/Patch evidence remain empty
and evidence remains incomplete.

# Patch-to-Note semantics

A Patch-to-Note item contributes Patch and Note family facts and two logical
events through the existing walker count. Only the Patch component receives a
PatchEvidence record.

# Channel non-inference

`observed_channel` remains `None` for every event family.

# Evidence completeness

`evidence_complete` remains false because routing evidence and a complete
compatibility policy are not available. Registry assessment and readiness are
unchanged.

# Synthetic tests

Existing walker tests remain authoritative for Patch grammar and transactionality.
AppService inventory tests cover empty walks, repeated Notes, and conservative
ownership. The shared decoder's synthetic and controlled matrix remains green.

# Authentic regression

The optional Experiment 007 AppService test confirms Patch evidence is present
for at least one naturally decoded event, ranges are contained, bank fields and
channels remain absent, and readiness/export capability remain unchanged.

# Compatibility/readiness isolation

No registry call, profile migration, readiness change, or export capability is
introduced. Complete synthetic registry matching is unchanged.

# Broader-profile limitation

No Patch evidence is emitted for unvalidated profiles.

# Explicit exclusions

No decoder grammar, channel inference, bank translation, UI, FFI, serialization,
dependency, or CLI behavior changed.

# UI0C2D gate

The bounded generic subset passes when source range, ordinal, and direct program
facts come from the successful shared walk, repeated observations are retained,
and all unresolved semantics remain conservative.

# Historical next step (completed)

Keep bank fields opaque until a future controlled optionality experiment proves
their generic semantics; do not change readiness or compatibility matching.

# Current status

The bounded generic Patch subset described here is now included in the complete
UI0C2 adapter. Bank optionality, routing, and profile policy remain deferred.
