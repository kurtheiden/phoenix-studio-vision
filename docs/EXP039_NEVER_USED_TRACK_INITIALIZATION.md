# EXP039: bounded never-used track correspondence

## Question and owner evidence

Can a legitimate Studio Vision track retain metadata without an event-data
record pair? This evidence supports a bounded answer for the existing 120-layout
source correspondence path, not a universal Studio Vision invariant.

The owner identified the source's unnamed candidate #13 as **Sequence M**. Studio
Vision showed exactly one unnamed ordinary track, no Instrument, no Patch and no
visible musical material (4/4, tempo 110, sequence length 4). The owner said:

> I'm pretty certain that empty track is just one I created and never put
> anything into it. It's not a glitch or fault in the file.

The owner identified candidate #8 as **Sequence H**. Its 12 ordinary rows match
source metadata order: Track 1, Track 1 #2, Track 3, Track 4, Track 5, Track 6,
Track 7, four blank rows, then Track 2. The blank rows are real visible tracks.
All four have the observed ZERO-STATE composition, and the source has four fewer
ordinary event-data pairs than ordinary metadata slots. The earlier suggestion
of a one-pair shortage was contradicted by the authenticated structural count.
These UI names are owner evidence; the original recovered sequence-name bytes
are empty.

## State and negative controls

ZERO-STATE is neutral terminology for the observed eight-byte context at
label-start minus 39:

```text
00 00 04 00 00 04 00 00
```

The first byte is the observed state byte. It is distinct from the Instrument
list-count byte at label-start minus 33. No additional opaque-field semantics
are claimed. Twelve accepted event-empty ordinary tracks in the same original
source retain pairs and have state `80`, not ZERO-STATE. Therefore event emptiness
alone does not authorize omitting a pair. Saved mute state is separate; this
rule changes no mute behavior.

## Controlled design and observations

The owner supplied three files for EXP039, using the existing lone Sequence M
track, without intentional changes elsewhere:

| Condition | Intentional action | State | Ordinary pairs | Saved label |
|---|---|---:|---:|---|
| CTRL | Finder duplicate; reportedly not opened or saved | `00` | 0 | empty |
| INST | Independently assign JV-1080-1; no musical event; ordinary Save | `80` | 1 | Track 1 |
| NOTE | Independently assign JV-1080-1 and add one Note; ordinary Save | `80` | 1 | Track 1 |

Instrument assignment followed by ordinary Save is sufficient for the observed
initialization; the first Note is not necessary to create a pair. This does not
establish the precise UI moment of allocation separately from Save. The saved
label change was observed, not attested as an intentional rename.

The added seven-byte Note representation was decoded by the existing bounded
walker as exactly one Note: position 0, pitch 60, attack 127, release 64,
duration 240. Its remaining primary suffix matches INST. Both saved pair
envelopes still refuse the existing full event-bound terminal predicate because
of a zero terminal tail. The isolated Note confirmation is not full event-region
certification; this correspondence change does not repair that grammar.

### Mandatory caveats and identities

CTRL was **not byte-identical** to the authenticated original. The relevant
Sequence M baseline remains structurally comparable, but the raw neighborhood
and other sequences differ. It must not be described as an authenticated exact
copy of the original.

| Input | Bytes | SHA-256 |
|---|---:|---|
| Authenticated original | 343875 | `bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331` |
| EXP039 CTRL | 341970 | `d7da247b45e014c9b0163a111369b7580cada9e1dd5881493b8a0f1f6fa91720` |
| EXP039 INST | 389052 | `6ca80339a800c3220e939f71cea94f2654d8c4b5c6c34a76caa50b6b5671dfd0` |
| EXP039 NOTE | 389059 | `51556b0ca2df65bad759d63e23e26d114eb6a2c9d7cadaf95ed53498b9d96747` |

Ordinary Save normalized INST/NOTE to a recognizable 166-layout. The local
Sequence M transition remains comparable through the existing candidate parser;
the whole-project parser refuses an earlier unrelated candidate. Sequence H's
four zero-state slots and eight ordinary pairs remain unchanged in both saves.
Thus normalization did not indiscriminately initialize all zero-state slots.
Neither this evidence nor implementation changes the 166-layout parser.

## Adopted bounded production rule

All existing 120 discovery, marker, neighborhood, trailer, framing, label,
leading-special and overlap guards remain required. Only ordinary metadata slots
with the exact eight-byte composition above consume no pair. Every other
ordinary slot consumes the next pair, without searching or backtracking. Two
leading Meter/Tempo slots always consume their own pairs. Adjusted consuming-slot
count must equal total available pairs exactly; otherwise the entire association
refuses. Diagnostic `Cardinality.slots` reports this adjusted consuming count.

The model retains paired ordinary bindings and a separate `no_event_data`
collection containing the complete slot view/label and state-context bytes/range.
Merge by slot ordinal to reconstruct source order. No-data tracks are legitimate
zero-event tracks, not corruption or silently deleted slots. No MIDI event,
Instrument, routing or Patch information is fabricated. Diagnostics expose them
explicitly. This internal bridge still grants no event/conductor export authority,
readiness or publication permission.

## Verification and remaining scope

Synthetic tests cover interleaving, consecutive/all no-data slots, exact-context
mutations, shortage/surplus, retained labels and provenance, unchanged special
bindings and existing equal-count/nonzero behavior. Existing 166 tests remain
unchanged. The authorized original-source check verifies:

- 12 certified correspondences (previously eight), 129 ordinary pair bindings;
- seven retained zero-event tracks without pairs;
- 128 successful existing event-bound probes and one unchanged terminal refusal;
- Sequence H: four retained slots and deterministic mapping through the final Track 2;
- Sequence M: one retained ordinary zero-event track, no ordinary pair;
- DANCING.MID and NOTHING.MID also gain correspondence;
- experimental and ultra change still refuse their trailer guards, and independent
  adjusted cardinality remains short by one and two pairs respectively.

Four of six previous count-mismatch candidates gain correspondence. That is not
an exportability claim. No additional rules were invented for the remaining two.
