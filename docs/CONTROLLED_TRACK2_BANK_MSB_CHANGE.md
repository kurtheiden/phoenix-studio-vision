# Objective

Test whether the middle byte of the established Track 2 three-byte bank tail
directly stores Bank Select MSB / CC0 by changing only CC0 from 81 to 82 in a
fresh Experiment 007 duplicate.

# Controlled change

`Ode to Clarke` / `Track 2` / `JV-1080` retained Patch position `1·1·0`
(tick 0), name `Stereoww Bs`, CC32 1, PC 37, and all Note data. The only
intentional change was CC0 `81 -> 82`.

# Experiment lineage

Experiment 029 descends directly from a fresh Experiment 007 duplicate, not
Experiment 028 or another controlled save. Experiment 007 is therefore the
direct baseline; Experiment 028 is independent supporting evidence.

# Preregistered prediction

Baseline Track 2 stores `ff 51 01` for CC0=81 / CC32=1. Experiment 028 already
established the final byte as direct CC32 storage. The locked Experiment 029
prediction was `ff 51 01 -> ff 52 01`: leading `ff` stable, middle byte
`51 -> 52`, and final `01` stable.

# Artifact identity

| Artifact | Exact path | Size | SHA-256 | Finder type / creator |
|---|---|---:|---|---|
| Experiment 007 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline` | 211,468 | `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132` | `MID2` / `MIDA` |
| Experiment 029 | `/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 029 - Track 2 Bank Select MSB Change/newest STUFF baseline EXP29` | 211,468 | `81598e095c5aa05005ce6b91d7fba9cdb812b59d3ef54afe63bd752a28365b86` | `MID2` / `MIDA` |

The distinct hashes identify different uncompressed Studio Vision projects.

# Structural alignment

The length-prefixed Patch name, position, PC, first-note properties, complete
211-note chain, and surrounding Patch structure align at the same offsets.
The bounded Track 2 representation remains `0x2fb55..0x2fb75`, status remains
at `0x2fb74`, and neither the file nor region changes size or relocates.

# Controlled-data stability

Both bounded representations derive position 0, payload length 25, name
`Stereoww Bs`, PC 37, post-PC timing 1,920, empty pre-Note context, and Note
status `90` at `0x2fb74`. The complete binary Note chain
`0x2fb75..0x300d8` is byte-identical, establishing stability of all 211 stored
pitches, attacks, releases, and durations and all 210 note-to-note timing
fields. The known four SMF zero-velocity note-off/export differences remain an
export caveat only; they do not differ between these project files.

# Local Patch diff

Exactly one byte changes within the bounded Patch representation:

| Baseline offset | Experiment offset | Baseline | Experiment | Structural location | Assessment |
|---:|---:|---:|---:|---|---|
| `0x2fb6f` | `0x2fb6f` | `51` | `52` | middle bank-tail byte | direct CC0 value, high confidence |

Every other bounded Patch byte is identical. Immediately before the Patch
start, the known save-dependent marker-family bytes at `0x2fb52..0x2fb54`
change `c4 b2 8c -> c7 85 2c`. They are outside the decoder bounds, recur as
save noise in prior controls, and remain unexplained.

# Bank-tail prediction result

**CONFIRMED.** At aligned offsets `0x2fb6e..0x2fb70`, baseline
`ff 51 01` becomes `ff 52 01`. Only `0x2fb6f` changes `51 -> 52`; leading
`ff` and the controlled CC32 byte `01` remain stable.

# Existing decoder result

The unchanged `decode_bounded_patch_representation`, invoked with independently
established bounds `0x2fb55..0x2fb75`, returns:

- position `00` = 0 at `0x2fb55`;
- payload length 25 at `0x2fb58`;
- pre-name context `00 01 25 f8 a5`;
- name `Stereoww Bs`;
- post-name context `02 33 38 04 ff 52 01` at `0x2fb6a..0x2fb71`;
- PC 37 at `0x2fb71`;
- post-PC `8f 00` = 1,920;
- empty pre-Note context;
- Note status `90` at `0x2fb74`.

Result: **PASS unchanged**. No production decoder code or contract changed.

# Experiments 028 and 029 comparison

| State | CC0 | CC32 | Tail |
|---|---:|---:|---|
| baseline | 81 | 1 | `ff 51 01` |
| Experiment 028 | 81 | 2 | `ff 51 02` |
| Experiment 029 | 82 | 1 | `ff 52 01` |

The two independent single-variable saves establish middle-byte direct CC0
storage and final-byte direct CC32 storage for this Track 2 representation.
They do not establish the role of the leading `ff`.

# Known bank-state comparison

| State | Bank evidence | Project tail |
|---|---|---|
| Track 1 | no exported bank select | `ff ff ff` |
| Track 2 baseline | CC0=81, CC32=1 | `ff 51 01` |
| Track 2 Experiment 028 | CC0=81, CC32=2 | `ff 51 02` |
| Track 2 Experiment 029 | CC0=82, CC32=1 | `ff 52 01` |
| Track 3 | CC0=81, CC32=2 | `ff 51 02` |
| Track 3 #2 | no exported bank select | `ff ff ff` |

The last two bytes now have controlled value evidence. The leading `ff` and
the `ff ff ff` no-bank state remain correlational because bank removal has not
been controlled.

# Bank semantics assessment

- **A. Middle byte directly stores CC0: YES** for this Track 2 representation.
- **B. Final byte directly stores CC32: YES** for this representation.
- **C. CC0 and CC32 independently established as direct values: YES.**
- **D. Leading `ff` understood: NO.** It was stable but not manipulated.
- **E. `ff ff ff` proven to mean no bank select: PARTIAL.** Two authentic
  no-bank states correlate; no controlled removal exists.
- **F. Bank values sufficiently established for diagnostic decoding: YES.**
  Both value bytes have independent causal evidence, provided diagnostics keep
  raw provenance and do not invent optionality semantics.
- **G. Sufficient for semantic fields in the bounded representation: PARTIAL.**
  Values are established, but field presence, leading sentinel, and no-bank
  representation are not.

# Decoder-policy decision

Choose **C**: design first-class optional bank fields, but postpone
implementation until a controlled bank-removal experiment establishes the
leading `ff` and `ff ff ff` no-bank behavior. Until then the shared decoder
continues preserving the complete post-name context as opaque bytes. This
records the now-confirmed value subfields without weakening the evidence-bound
contract or prematurely defining optionality.

# Evidence supported

- The locked `ff 51 01 -> ff 52 01` prediction is uniquely confirmed.
- `0x2fb6f` directly stores CC0 for this Track 2 representation.
- Experiment 028 independently established `0x2fb70` as CC32.
- Position, payload framing, name, PC, timing, Note boundary, and all 211 Notes
  remain stable.
- Experiment 029 passes the shared bounded decoder unchanged and preserves the
  modified opaque context byte-exactly.

# Unknowns

The leading `ff`, bank-field presence/optionality, controlled meaning of
`ff ff ff`, behavior of partially specified banks, broader device/version
generality, and adjacent save-dependent fields remain unresolved.

# Single recommended next step

Perform one controlled removal of Bank Select from this Track 2 Patch in a
fresh Experiment 007 duplicate while holding position, name, PC, and notes
constant. This directly tests whether `ff 51 01 -> ff ff ff` and resolves the
remaining sentinel/optionality question before bank fields enter the shared
semantic API.
