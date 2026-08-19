# Objective

UI0C4B adds a Core-only, one-shot freshness check between inspection-time
assessment and any future export operation. It does not export or project
readiness.

# Scope

`AppService::revalidate_sequence_policy` rereads the session's stored path,
checks exact content identity, reparses, rebuilds generic evidence, and reruns
the immutable compatibility registry for the original structural ordinal.

# Inspection-time assessment versus fresh authorization

The UI0C4A assessment is historical inspection state. A stored policy is not
authorization. Only a successful revalidation result may be handed to a later
Core export operation.

# Revalidation API

The public Core-facing status reports `Validated` and a safe capability/hash;
the internal `revalidated_policy_for_sequence` handoff additionally owns the
fresh bytes and resolved policy. No app DTO carries policy internals.

# Source reread

The stored path is opened again with standard file APIs. Missing or unreadable
paths return `FileUnreadable/source_revalidation_failed`; retained session bytes
are never used as authority.

# Source identity

Fresh byte size and SHA-256 must both equal the inspection snapshot. A same-size
mutation, replacement, or any hash mismatch returns
`ExportValidationFailed/source_identity_changed`. Modification time is not
authoritative.

# Fresh parsing

After identity equality, the established Descriptor166 parser and the shared
structural snapshot/evidence adapter are run again. No parser grammar is
duplicated or broadened.

# Fresh ProfileEvidence

The fresh snapshot is converted through the same owned `ProfileEvidence`
builder used by inspection. The selected opaque `SequenceId` is mapped through
its retained structural ordinal; names are never used for remapping.

# Sequence identity

The ordinal must still exist in the fresh evidence. Its absence returns
`source_sequence_identity_changed`; no alternate sequence is selected.

# Fresh registry assessment

The same immutable registry must return `Matched` for that ordinal. `NoMatch`,
`Rejected`, ambiguity, and registry configuration failures refuse revalidation
with bounded diagnostic codes.

# Profile identity consistency

Fresh `ProfileCapability` identity/version must equal the inspection match.
Profile substitution is refused.

# Resolved policy consistency

The fresh immutable policy must equal the stored policy, including sequence
identity, channels, and Patch translations. The stale policy is never returned
as a validated result.

# Missing source

Deleting or moving the original path is an unreadable revalidation failure.
Phoenix does not search directories, hashes, aliases, or renamed copies.

# Mutated/replaced source

Replacing the path, including a same-length one-byte mutation, fails before
policy use. Restoring byte-identical content permits a fresh validation again.

# No-policy behavior

NoMatch, Rejected, or RegistryError inspection states have no authority and
return `no_validated_profile_policy` without rereading for a nonexistent policy.

# Fresh validated source lifetime

Successful internal state owns the freshly read bytes, SHA-256, structural
ordinal, and resolved policy together. UI0D should consume these exact bytes,
avoiding a second source read.

# TOCTOU strategy

The returned owned bytes minimize the time-of-check/time-of-use window. A later
export must invoke revalidation immediately before conversion; no validated
state persists as a permanent flag.

# Session isolation

Both `SessionId` and `SequenceId` are required. Unknown sessions/sequences are
typed deterministic errors, and identities cannot cross sessions.

# Readiness freeze

Revalidation never changes `SequenceSummary`, project readiness, or assessment
history. UI0C4C remains responsible for any future readiness projection.

# Export isolation

No serializer, file writer, MIDI assembly, or app-facing export capability is
introduced here.

# Tests

Focused service tests cover unchanged-source success, same-size mutation,
missing source, unmatched sequences, and cross-session identity isolation.

# Target-specific isolation

AppService contains no authenticated profile constants; those remain isolated
in the compiled-in compatibility profile module.

# Explicit exclusions

No readiness, export, app-contract, parser/decoder, dependency, FFI,
serialization, or UI changes are part of UI0C4B.

# UI0C4B gate

The gate passes when fresh identity, structure, evidence, profile identity, and
resolved policy all agree, and every stale/missing/mutated/no-policy case is
refused conservatively.

# Single recommended next step

Design the UI0D export handoff around immediate UI0C4B revalidation and fresh
owned source bytes.
