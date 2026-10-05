# Bounded 120-layout AppService integration

AppService exposes observed 120-layout candidates through the existing Sequence,
readiness and single-Sequence export contracts. Only successful `recovery120`
results grant authority. Association or structural observation alone grants none.
Blocked candidates remain visible with a neutral limitation and technical refusal
in diagnostics. Empty and duplicate names remain unchanged.

The private inspected authority binds a session-scoped Sequence ID to its source
ordinal, source range, exact name bytes and deterministic MIDI SHA-256. The session
retains the exact inspected source identity and path. Export re-reads that path,
checks source size and SHA-256, reruns the complete production recovery builder,
and checks Sequence identity and MIDI hash before destination preparation. There
is no export from cached MIDI and no client-supplied recovery authority.

The existing bounded capability fields carry `source_layout120_bounded_sequence`,
revision 1. The JSON schema and C ABI are unchanged. Swift recognizes this ID only
at revision 1; its existing Ready/capability eligibility checks still apply.

Both recovery paths use the existing temporary-file, sync, no-overwrite hard-link,
collision, cleanup and completion transaction. Terminal provenance survives in
inspection diagnostics and export warnings. No batch exporter or alternate
recovery interpretation is introduced. All authority120 and recovery120 guards
remain necessary. This integration grants no authority to unknown musical data.

External authentic SCHOOL validation: 15 visible candidates, eight Ready and
seven blocked. All eight AppService exports are byte-identical to the previously
validated persistent artifacts; each fresh builder result has passed mechanical
readback. Validation writes only disposable destinations. Portable tests cover
mutation, unknown session/Sequence, mismatched ordinal binding, occupied output,
and attempts to manufacture readiness on a blocked candidate. Existing
Descriptor166, transport and native bridge regressions remain required.
