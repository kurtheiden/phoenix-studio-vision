# Objective

Determine whether the MIDI output channel used by Studio Vision for each
authenticated `Ode to Clarke` track is stored in a stable bounded sequence
descriptor/routing field. This is read-only correlation. It does not infer a
channel from performance-event status bytes, implement parsing/export, or
change either source artifact.

# Artifacts and provenance

**OBSERVED — project.** Experiment 007 untouched baseline:

`/Users/kurtheiden/Documents/Phoenix Research/Controlled Save Experiments/Experiment 007 - Untouched Baseline/newest STUFF baseline`

- size: 211,468 bytes;
- SHA-256: `e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`;
- authenticated `Ode to Clarke` sequence: `0x02ef6f..0x03202c`.

**OBSERVED — MIDI.** Authenticated Studio Vision export:

`/Users/kurtheiden/Documents/Phoenix Research/Studio Vision MIDI Exports/Project 001/Ode to Clarke Multi All`

- size: 12,141 bytes;
- SHA-256: `4f63b34ef92204d4bc5eeb78dbbe7b94d005c1f9ceb57ea0f9809533ad590f29`;
- SMF Format 1, ten chunks, division 480;
- exact chunk walk ends at byte 12,141.

Both artifacts were hashed and parsed/inspected directly during this task.
Earlier report summaries were not used as substitutes for the MIDI inventory.

# MIDI export channel inventory

The MIDI was independently decoded with running status and absolute ticks.
Counts below are channel voice messages; Note On includes velocity-zero note
endings where Studio Vision used that representation. Track 0 is conductor
metadata and has no channel messages.

| SMF track | Track Name | Channels | Channel families and counts | Program | Bank Select |
|---:|---|---|---|---|---|
| 0 | `Ode to Clarke` | none | none | none | none |
| 1 | `Track 1` | 1 | PC 1, Note On 93, Note Off 89 | tick 0: PC 61 | none |
| 2 | `Track 2` | 2 | CC 2, PC 1, Note On 215, Note Off 207 | tick 0: PC 37 | tick 0: CC0=81, CC32=1 |
| 3 | `sys100loops` | 10 | Note On 326, Note Off 318 | none | none |
| 4 | `Track 4` | 10 | Note On 180, Note Off 178 | none | none |
| 5 | `Track 5` | 10 | Note On 135, Note Off 133 | none | none |
| 6 | `Track 3` | 1 | CC 2, PC 1, Note On 86, Note Off 82 | tick 480: PC 29 | tick 480: CC0=81, CC32=2 |
| 7 | `Track 6` | 10 | Note On 61, Note Off 59 | none | none |
| 8 | `Track 3 #2` | 15 | PC 1, Note On 86, Note Off 82 | tick 530: PC 23 | none |
| 9 | `Track 7` | 10 | Note On 143, Note Off 143 | none | none |

**DERIVED.** Every non-conductor track has channel messages and uses exactly
one channel. The proposed track-level `ChannelAssignment` is therefore valid
for this proof target. No MIDI track contains conflicting channels.

# SVP descriptor/pair mapping

The sequence has eleven 166-byte descriptors: Meter and Tempo at ordinals 0
and 1, followed by nine track descriptors. Nine `0x02`/`0x29` pairs follow the
Meter/Tempo pairs. Equal counts establish ordinal descriptor-to-pair binding;
names are corroboration, not the association rule. Event ranges use the
validated 166-profile rule `primary payload + 14 .. primary end - 7`.

| SMF | Descriptor ordinal / range | Descriptor name | Pair ordinal | Primary range | Exact event range | Channel |
|---:|---|---|---:|---|---|---:|
| 1 | 2 / `0x02f18b..0x02f231` | `Track 1` | 0 | `0x02f820..0x02fa7a` | `0x02f833..0x02fa73` | 1 |
| 2 | 3 / `0x02f231..0x02f2d7` | `Track 2` | 1 | `0x02fb42..0x0300df` | `0x02fb55..0x0300d8` | 2 |
| 3 | 4 / `0x02f2d7..0x02f37d` | `sys100loops` | 2 | `0x0301b7..0x03097d` | `0x0301ca..0x030976` | 10 |
| 4 | 5 / `0x02f37d..0x02f423` | `Track 4` | 3 | `0x030a17..0x030e9f` | `0x030a2a..0x030e98` | 10 |
| 5 | 6 / `0x02f423..0x02f4c9` | `Track 5` | 4 | `0x030f31..0x03125b` | `0x030f44..0x031254` | 10 |
| 6 | 7 / `0x02f4c9..0x02f56f` | `Track 3` | 5 | `0x0312ed..0x03156b` | `0x031300..0x031564` | 1 |
| 7 | 8 / `0x02f56f..0x02f615` | `Track 6` | 6 | `0x03165b..0x031805` | `0x03166e..0x0317fe` | 10 |
| 8 | 9 / `0x02f615..0x02f6bb` | `Track 3 #2` | 7 | `0x031873..0x031b05` | `0x031886..0x031afe` | 15 |
| 9 | 10 / `0x02f6bb..0x02f761` | `Track 7` | 8 | `0x031bf5..0x031fa3` | `0x031c08..0x031f9c` | 10 |

The final descriptor overlaps the separately bounded Pascal sequence-name
location at its established tail; that existing profile behavior is preserved
and is not interpreted as routing.

# Descriptor byte comparisons

All nine complete bounded descriptors were compared at every relative offset
`+0x00..+0xa5` against the authenticated channel vector:

```text
1, 2, 10, 10, 10, 1, 10, 15, 10
```

**OBSERVED.** There is no relative byte offset whose complete vector equals:

- one-based channel `1..16`;
- zero-based channel `0..15`;
- the low nibble of either encoding; or
- the high nibble of either encoding.

Thus no direct byte or simple packed-nibble field explains every track.

Several repeated descriptor families instead group apparent output/device
contexts. For example, relative `+0x22/+0x24/+0x26` separates the Track
1/2/3/3 #2 family (`fe 00 83 00 02`) from most channel-10 tracks
(`ff 00 14 00 14`). This is not a channel field:

- Track 1 and Track 3 use channel 1;
- Track 2 uses channel 2 while sharing the same broad descriptor family;
- Track 3 #2 uses channel 15 while also sharing that family;
- several channel-10 descriptors vary elsewhere despite a shared channel.

Relative `+0x32`, among other plausible small values, yields
`05,09,03,03,03,09,03,05,05`; it is inconsistent with both channel encodings.
The comparison found other constant or clustered bytes, but none independently
maps to all nine channels. Numeric plausibility alone is rejected.

# Candidate fields considered

| Candidate | Test | Result |
|---|---|---|
| descriptor direct byte | all 166 relative offsets, one-/zero-based | rejected: no complete match |
| descriptor nibble | high/low nibble, one-/zero-based | rejected: no complete match |
| descriptor output/device family | compare equal/different channel groups | unresolved indirect reference; contradicted as channel itself |
| event status low nibble | compare stored `90`/Patch entry with SMF | rejected by existing evidence; source low nibble zero while channels vary |
| Patch pre-name bytes | four authenticated Patch tracks | rejected as direct channel; values differ between channel-1 tracks and do not encode 1/2/15 consistently |
| Patch post-name prefix | device/framing grouping | unresolved; channel and device/context covary incompletely |
| Patch bank/Program fields | compare authenticated CC0/CC32/PC | rejected as channel; these fields independently encode bank/program |
| sequence type-`0x09` prelude | payload `0x02f765..0x02f771` | rejected as a direct per-track table: 12 bytes cannot supply an ordinal channel value for all nine tracks, and no established descriptor references decode it |

The sole Ode pre-Meter `0x09` record is `0x02f760..0x02f771` with payload:

```text
00 01 00 02 00 0d 26 0f 00 0d 24 0f
```

**OBSERVED:** it is bounded sequence-local data. **UNKNOWN:** its semantics or
whether it participates indirectly in routing. No safe descriptor-to-entry
reference is established, so it cannot be used as channel input.

# Patch and routing observations

Four tracks contain established Patch representations:

- Track 1: Juno-106 metadata, channel 1;
- Track 2: JV-1080 metadata, channel 2;
- Track 3: JV-1080 metadata, channel 1;
- Track 3 #2: JD-800 metadata, channel 15.

**OBSERVED:** the same JV-1080 identity occurs on two different channels, so
device identity alone does not determine channel. Track 1 and Track 3 share a
channel despite different device identities, so channel is not merely a device
model property. Patch bank and Program bytes have already established meanings
independent of channel. Opaque Patch contexts do not form a direct channel
encoding across all four samples.

**INFERRED:** channel likely belongs to an output assignment combining device
and channel, or to another referenced routing structure, rather than the
decoded Patch event itself. **UNKNOWN:** the bounded reference/target encoding.
Classification among the requested alternatives is **D: another bounded
structure, or E: unresolved**; current evidence cannot choose safely.

# Bells cross-check

**NOT RUN.** No credible Ode descriptor field survived the complete direct
byte/nibble tests, so the prerequisite for broadening the check to `Bells for
her` was not met. Existing Bells channels remain independent evidence, not a
search space for manufacturing a candidate.

# Conclusion classification

## CHANNEL MANIFEST ONLY

No safe SVP channel field is established. A complete authenticated channel
assignment is nevertheless available for every `Ode to Clarke` musical track
needed by the first proof, and every track is single-channel.

Confidence is **HIGH** for the manifest: artifact hashes, SMF chunk parsing,
channel-message inspection, equal-count structural descriptor/pair binding,
and exact event ranges all agree. Confidence is **LOW** that any presently
opaque descriptor/output byte has channel semantics.

# Implementation consequence

The first exporter proof may use an immutable, provenance-controlled policy
manifest keyed by structural identity, not by filename or track name:

```text
project_sha256 = e5a70056...7e5132
sequence_range = 0x02ef6f..0x03202c
sequence_name_bytes_range = 0x02f753..0x02f760
entries = (descriptor_ordinal, descriptor_range, pair_ordinal,
           primary_range, event_range, midi_channel)
```

The nine exact entries are the rows in the mapping table above. The adapter
must verify the project hash, sequence range/name bytes, descriptor and pair
ranges, exact event range, and channel `1..=16` before use. Any mismatch or
missing row is `UnknownChannel`; there is no fallback by label and no default
channel. This manifest is **proof-target export policy**, not Studio Vision
format knowledge and not a parser field.

This resolves the channel blocker for the first authenticated `Ode to Clarke`
proof only. General channel derivation remains partial and production export
must not embed this manifest as a universal format rule.

# Unknowns

- Which descriptor field, if any, references an output assignment.
- Semantics of the sequence-local `0x09` payload.
- Whether the channel resides in a project-global device/output table, a
  sequence-local routing object, or another referenced structure.
- Encoding and validation of an output/device/channel composite.
- Generality beyond this exact project and sequence.

# Single recommended next step

Use the nine-entry, hash-and-range-locked channel manifest for the read-only
first-export proof adapter design/implementation. Defer general channel-field
parsing until independent routing evidence exists; no controlled experiment is
required for the proof target.
