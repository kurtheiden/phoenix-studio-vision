# Objective

Determine whether the established Phoenix parser evidence supports a reusable
exact performance-event range for the supported 166-byte sequence profile, so
UI0C2 can later complete generic compatibility evidence without copying proof
logic or guessing boundaries.

# Current generic ranges

For a parsed `SequenceContainer`/`TrackRecordPair`, the existing generic API
provides:

| Range | Source | Semantics | Safe as walker range? |
|---|---|---|---|
| sequence range | `SequenceContainer.sequence_range` | half-open root sequence record | no; contains all sequence structures |
| descriptor range | `SequenceDescriptor.range` | half-open 166-byte descriptor | no; not event data |
| primary record range | `TrackRecordPair.primary.record_range` | half-open framed type-`0x02` record | no; includes framing and payload |
| primary payload range | `TrackRecordPair.primary.payload.range` | half-open declared payload | no; includes prefix and terminal bytes |
| candidate event start | `TrackRecordPair.candidate_event_start` | payload start plus established 14-byte prefix | start only |
| event-containing range | `TrackRecordPair.event_containing_range` | candidate start through primary payload end | no; explicitly includes the non-event tail |
| secondary range | `TrackRecordPair.secondary.record_range` | adjacent type-`0x29` record | no; not performance data |

The record length field establishes the primary/payload end exactly. The parser
documentation deliberately names `event_containing_range` as an upper bound,
not an exact event stream.

# Current proof-specific exact-bound derivation

The D2 Ode integration currently performs:

```text
event_start = pair.candidate_event_start
event_end   = pair.primary.payload.range.end - 7
event_range = event_start .. event_end
```

It then reads the seven bytes at `event_end..payload.end` and requires
`ff aa bb cc ff 2f 00`, preserving the middle three bytes opaquely. The bounded
walker receives this range; it does not discover the end. The D2 test also
asserts the expected manifest range, which is proof policy and must not remain
the only implementation of the rule.

# Mixed-event walker termination behavior

`walk_bounded_mixed_events` validates the caller-supplied half-open range and
loops until its cursor equals the supplied end. It recognizes event dispatch
and stateful continuations inside that range, but it has no terminal-tail
grammar and cannot distinguish an event from a non-event tail outside the
caller bound. A malformed final decode is not a safe boundary signal: it may
mean corruption, an unsupported family, or an incorrectly supplied bound.
"Walk until error" is therefore unsafe.

# Established termination evidence

Established:

* primary framing lengths and payload ends are exact;
* the candidate event start is payload `+14` across the supported corpus;
* all 132 authenticated 166-byte track primaries end in
  `ff aa bb cc ff 2f 00` at the declared payload end;
* 15 zero-count tracks contain only that seven-byte form after the common
  prefix, independently separating it from performance events;
* Bells and Ode tracks with independently bounded families terminate at the
  same boundary, including the corrected Bells Track 9 Note and Track 14
  Controller observations;
* the three middle bytes vary and remain opaque.

Provisional/unknown:

* the semantic name of the seven-byte structure;
* whether `ff 2f 00` intentionally mirrors SMF End of Track;
* meanings of the three variable bytes;
* exact internal Note/Pressure/Bend run termination;
* applicability to the unsupported 120-byte profile or other Studio Vision
  versions.

# Natural cross-track evidence

The durable termination report covers a focused Bells/Ode/Sequence-K corpus and
an all-sequence census of 132 authenticated 166-byte primaries. It includes
ordinary Controller tracks, Patch/Note tracks, Pressure/Bend tracks, multiple
Note chains, and empty candidates. The same seven-byte shape appears at every
declared payload end, while empty tracks have no performance bytes before it.
This is independent of selecting one Ode row by name, although the corpus is
still bounded to the authenticated 166-byte population.

# Controlled-save evidence

Existing Experiment 031 changes only Ode Track 3 #2's first Note timing. The
primary length and tail placement remain unchanged, while the local timing
bytes move as predicted. It validates Patch-to-first-Note navigation but is not
an append/delete-final-event boundary experiment. No existing controlled save
varies final event length, deletes the last event, or moves a terminal event
while measuring the primary tail. The broad tail rule is supported by the
132-primary structural census and empty-track evidence, not by an unperformed
single-variable append experiment.

# Candidate derivation strategies

| Strategy | Classification | Reason |
|---|---|---|
| A. existing length/offset field alone | UNSAFE | length gives primary container end, not inner event end |
| B. payload end minus validated seven-byte tail | SAFE NOW, scoped | repeated 132/132 166-profile invariant plus empty tracks; validate before subtraction |
| C. validated terminal grammar | SAFE NOW, scoped | validate invariant bytes and bounds; semantic meaning may remain opaque |
| D. walker discovers terminal marker | UNSAFE | walker has no terminal grammar and must be bounded |
| E. parse until error/exhaustion before separately identified tail | SAFE NOW, scoped | first validate tail, then require complete walker consumption before it |
| F. numeric pointer/count coincidence | UNSAFE | count-like fields disagree with reconciled event counts and lack correlation |

# Fixed-tail hypothesis

**CONFIRMED for the authenticated 166-byte profile; UNCONFIRMED for broader
Studio Vision formats.** Every member of the established 132-primary corpus
has the grammar at the exact declared payload end, and empty-track controls
show it is not performance data. Promotion beyond this profile would require
independent evidence from other profiles/versions, not more copies of the same
Ode structure.

# Latent parser fields

The reusable fields are the framed record payload length, primary payload range,
and candidate start `payload + 14`. No existing field encodes an inner event
length, pointer, stream count, or footer length. The count-like primary word is
not reliable: known reconciled event counts differ from it. The seven-byte tail
is a validated structural suffix, not a newly assigned semantic pointer.

# Exact-range API if proven

The smallest implementation seam is a parser-facing, profile-scoped helper:

```text
TrackEventBounds {
    primary_range
    payload_range
    event_range
    tail_range
}

SequenceContainer::validated_track_event_bounds(pair_ordinal)
    -> Result<TrackEventBounds, TrackEventBoundsError>
```

It must check the pair/primary framing, checked `+14` arithmetic, checked
`-7` arithmetic, exact seven-byte grammar, and `event_start <= event_end`.
Errors must distinguish unavailable profile/association, malformed structure,
inconsistent bounds, and invalid terminal grammar. It must never return the
containing range as an approximation. The helper is reusable by the bounded
walker caller and the UI0C2 evidence adapter, while remaining explicitly scoped
to `Descriptor166`.

# Evidence completeness model

The temporary UI0C2 `TrackEvidence.exact_event_range: Option<ByteRange>` and
`evidence_complete: bool` are appropriate until this helper exists. Complete
profiles require `Some` and true; incomplete evidence is rejected by UI0C1.
After the helper is implemented and the bounded walk succeeds, the adapter can
set the exact range and completeness without changing the app contract.

# Event-family/Patch follow-on

Decoded family inventory is **READY AFTER RANGE**: the existing bounded walker
already returns deterministic supported family items once supplied the exact
range, and walk failure must remain an error. Generic Patch evidence is
**PARTIAL AFTER RANGE**: the decoder establishes source range/program and some
bounded transition facts, but bank semantics and opaque tails must remain
optional until independently established. No translation policy belongs in
this seam.

# Research requirement

No new controlled experiment is required to implement the scoped 166-profile
range helper: existing cross-track census and empty-track evidence already
establish the boundary rule. A future one-variable experiment remains the
smallest useful promotion test beyond this profile: append exactly one final
Note to an unchanged baseline track, then compare primary payload length,
tail placement, and event bytes. A moving tail with unchanged seven-byte
grammar would support broader use; a fixed tail or changed framing would refute
that promotion. Do not run it as part of this task.

# Implementation gate

| Question | Result |
|---|---|
| Generic exact event range proven | YES, scoped to authenticated 166 profile |
| Reusable parser API design-ready | YES |
| Fixed seven-byte tail generalized | PARTIAL (166 profile only) |
| Walker independently detects event end | NO |
| Decoded family inventory after range | YES |
| Generic Patch evidence after range | PARTIAL |
| UI0C2 completion without new experiment | YES, for 166 profile |
| Exact-range seam implementation-ready | YES |

# Single recommended next step

Implement the scoped `Descriptor166` validated track-event-bounds helper and
its synthetic/cross-profile boundary tests, then let UI0C2 consume it without
migrating the authenticated Ode profile or changing readiness.

# UI0C2B implementation status

The scoped `TrackEventBounds` helper now validates the Descriptor166 suffix and
returns exact event/tail ranges with checked arithmetic. AppService records the
exact range when validation succeeds, while evidence remains incomplete until
later family and Patch evidence work. The helper does not apply to the
unsupported 120-byte profile.
