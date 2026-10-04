# Bounded compact initial conductor recovery

`bounded_conductor::decode_compact_conductor` proves coverage of four contiguous,
same-source framed records supplied by an established sequence association:
Meter primary/secondary and Tempo primary/secondary. The caller must establish
sequence ownership and absence of additional conductor records/preludes.
The helper is independent of readiness and export authorization.

The supported primaries have 14-byte headers, counted initial events and exact
seven-byte terminal tails. Meter has one six-byte initial Key annotation followed
by one eight-byte initial Meter event (35-byte payload); Tempo has one seven-byte
initial Tempo event (28-byte payload). Four opaque header bytes remain retained
context. The exact compact secondary framing, counts, bounded lengths and final
tag/value copies must agree. There is no scanning or acceptance of trailing events.

Existing Tempo/Meter decoders and the strict Identity480/KnownHistoricalOnly adapter
remain authoritative. Invalid values, malformed framing, unknown event tags,
additional content or contradictory copies refuse. The key uses standard signed
accidentals (-7 through 7) and major/minor (0/1), retaining its source range.
No source-specific names, channels, offsets, tempo or meter values are predicates.

The existing padded Descriptor166 conductor guard is unchanged. The optional
`smf::serialize_conductor_track_with_key` emits initial name, tempo, key, meter,
then EOT; the original serializer delegates with no key and retains identical
output. No inferred timecode offset, opaque context or source terminal sentinel
becomes a MIDI event. Native export end padding is not synthesized.

Six synthetic tests cover varied values, provenance, exact serialization, opaque
context independence, framing mutations, truncations, invalid values, additional
well-formed conductor content and unchanged older serializer output. Existing
manifest tests retain coverage of the older padded envelope.

The private experimental ORIGINAL SCHOOL PROJECTS Nothing FINAL gate passes:
15/15 ordinary tracks, 18/18 Patches, 70/70 source-channel Controllers,
406 Pitch Bends and 5,109 Notes (including five context-mediated Notes).
The full experimental Format-1 artifact uses 480 PPQN and separate source tracks.
Independent readback matches every channel message's timing/status/data against
the source handoff. Key metadata is retained in MIDI. Zero native timecode metadata
and native trailing end padding are not reproduced. Saved source mute states are
reported separately; the listening artifact includes all recovered tracks,
including initialization tracks. This neither changes production mute behavior
nor confers production readiness or historical external-hardware equivalence.
