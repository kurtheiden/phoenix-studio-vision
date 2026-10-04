# Bounded zero-prefix single-context Note routing

The same-source single-Instrument routing resolver previously refused an
otherwise complete track because its context payload starts `57 00 00`, while
the shared ff60 guard accepts only `57 7f 00`.

The new exception applies only to a production-decoded single ContextMediatedNote
following one or more source MIDI Controllers, before any other event family.
It requires exactly eight declared/actual context bytes, prefix `57 00 00`,
equal opaque payload bytes 3 and 4, and an explicit `90` Note status. Timing
and Note decoding remain the existing walker's responsibility. Opaque bytes
and source ranges are retained without assigning them additional MIDI meaning.

`validate_bounded_routing_contexts` now shares these applicability checks with
bounded observation callers. Such callers must already establish same-source
ownership, complete event bounds and Instrument routing. The helper verifies
complete walk consumption and does not resolve a channel, authorize exports,
or replace a source-layout association contract. The Descriptor166 resolver
continues using its existing F/Z association and routing-row/device checks.

The shared context guard is unchanged. Initial Patch contexts, Patch transitions,
double contexts, subsequent zero-prefix contexts, wrong lengths/prefixes or
unequal repeated bytes remain unsupported. Ordinary `57 7f 00` forms retain
their existing acceptance. No fallback route or name-derived channel is added.

Evidence came from the original SCHOOL source and already-written independent
native observation reports. All 243 Arpegio note attacks match in pitch, velocity
and timing, and all 243 releases match pitch/end time on channel 6, including the
context-mediated Note. That supports the bounded Note applicability rule; it
is not a universal ff60 interpretation or byte-for-byte native equivalence.
The current Controller contract preserves original status-derived channels,
independently of Note routing and native resave behavior.

Synthetic tests cover F/Z associations with distinct source routes, timing,
context/Note provenance, compact Note continuation, old contexts and neighboring
event families, malformed representations, unsupported compositions and routing
refusals. The original Nothing FINAL probe now passes 15/15 ordinary tracks,
18/18 Patch events and 70/70 source-channel Controllers. Event counts remain
5,603. Complete conductor-envelope coverage remains a separate blocker; it was
not investigated or changed. No readiness, authorization, mute or UI change,
and no Nothing FINAL MIDI file was generated.
