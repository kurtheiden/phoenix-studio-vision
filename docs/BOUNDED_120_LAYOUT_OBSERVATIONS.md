# Bounded 120-layout structural observations

Phoenix can record bounded structural observations for the source layout
initially observed in the private SCHOOL PROJECTS research specimen, with a
second exact marker form established by later bounded Prologue research. This is candidate
and name-span discovery, **not** semantic sequence inventory or music recovery.
Authentic project bytes remain outside Git under the authentic-sample policy.

The positive predicate uses the existing complete root-record framing (eight
opaque header bytes, then type/u32-BE payload-length records consuming the
input). Each type-01 candidate must have a full framed size of
`75 + 120 × count`, where count is the byte at candidate +5. Local bytes
+15..+22 must be `00 00 00 00 00 00 01`, +22 must be `80` or `88`, and
+41..+43 must be exactly `fe ff` or `ff ff`. The immediately following framed record must be
type 07; its opaque payload is not interpreted. These are observed structural
guards, not decoded flags or track-count semantics. Requiring count >=2 is an
explicit conservative defensive guard, not evidence of reserved tracks.

An observed name span starts at +23 and ends at the first NUL in the half-open
window `[+23, +41)`. +41 is excluded. This ceiling does not establish the true
historical field capacity. Empty spans remain empty observations; bytes after
the first NUL remain opaque and need not be zero. Raw bytes are preserved,
with an optional UTF-8 view when valid. There is no Pascal length field, lossy
text repair, filename/hash predicate or name-based remapping.

The existing full 166-layout candidate validator is tested independently,
without changing its semantics. Dual-valid candidates and projects mixing 120/166 layouts refuse discovery.
The two supported marker forms may coexist within one structural-observation
collection; their raw bytes/ranges and `FeFf`/`FfFf` forms are preserved without
assigning either form a meaning. Unsupported marker values still refuse. A malformed candidate invalidates the complete
120-layout result. Generic 166 failure alone never selects the 120 layout.

AppService exposes only an aggregate human diagnostic alongside the original
166-profile rejection, including computed counts of each observed marker form.
No semantic sequence IDs, SequenceContainer,
SequenceName, track bindings, compatibility evidence or export authorities are
created from these observations. The semantic inventory remains empty and
its aggregate readiness remains Unknown. No MIDI conversion or publication
path has been added.

Synthetic tests cover the production rules and the service firewall. An
explicitly requested, ignored-by-default private evidence test authenticates
SCHOOL PROJECTS before checking 15 structural candidates, including three
empty observed name spans. These counts are evidence assertions only, not
production invariants. A separate explicitly authorized, ignored-by-default
Prologue evidence test checks 15 structural candidates (8 `fe ff`, 7 `ff ff`),
without semantic inventory, readiness promotion or export authority. This is
post-research implementation verification, not the original prospective test
or successful independent music recovery. COMIC BOOK remains reserved.
Ownership, musical events and recovery require separate research and authorization.

The original SCHOOL PROJECTS-derived observer was frozen at `cd1b8d6` and
prospectively refused Prologue at record 170. That refusal was durably recorded
and pushed before investigation. Only after the failed marker guard and the
seven-record cohort were reviewed was this two-marker extension designed and
implemented. The [original refusal checkpoint](OS1_PROLOGUE_120_LAYOUT_PROSPECTIVE_REFUSAL_CHECKPOINT_2026_10_02.md)
remains unchanged; see the [extension checkpoint](OS1_PROLOGUE_120_LAYOUT_TWO_MARKER_IMPLEMENTATION_CHECKPOINT_2026_10_02.md).
