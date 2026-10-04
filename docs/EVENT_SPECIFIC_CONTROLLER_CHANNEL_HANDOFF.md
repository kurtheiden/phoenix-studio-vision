# Event-specific Controller channel handoff

The bounded mixed-event walker already derives a MIDI Controller's channel
from its explicit `b0..bf` status and retains it for compact continuations.
The export handoff previously rejected that decoded variant, and the adapter's
Controller representation used the track channel.

`DecodedExportEvent::from_midi_controller` now carries the decoded channel,
number, value, absolute position, source ordinal, and representation range into
a distinct `MidiController` export variant. The adapter validates its channel
and data transactionally and uses that channel for the existing SMF Control
Change message. It does not require equality with the track's Note route.
The bounded routing resolver accepts these already-decoded Controllers without
changing its selected track route or unrelated context predicates.

Notes, legacy context-bearing Controllers, Patch events, and Pitch Bend retain
their existing track-channel behavior. Decoder predicates, Patch translation,
sequence parsing, readiness, and export authorization are unchanged.

Synthetic tests cover equal and differing channels, multiple explicit channels
and compact continuation, timing, source ranges and ordinals, serialized
status/data bytes, invalid channels/data, and unchanged neighboring Note,
Patch, and Pitch Bend behavior. Existing source-Controller decoder tests retain
malformed/truncated/unsupported-form refusal coverage.

A read-only SCHOOL PROJECTS recheck finds 70 Controllers in Nothing FINAL
across nine tracks: 12 explicit records and 58 compact continuations. All 70
retain channel 1, number/value, timing, and source range through the new
constructor and existing adapter/SMF message serializer, including Controllers
whose channel differs from the track's Note route.

Nothing FINAL still requires multi-instrument standalone Patch target and
translation classification, Arpegio's `57 00 00` routing-context applicability,
and complete conductor-envelope coverage. Twelve ordinary tracks now pass the
track-level experimental gate; all 15 event regions remain completely consumed
with 5,603 logical events. The sequence is not Ready or omission-free for
experimental export. No Nothing FINAL MIDI was generated.

Validation: focused portable handoff, adapter, routing, and source-Controller
decoder tests; formatting; all-target/all-feature Clippy; all-target test
compilation; safe regression execution. Unrestricted test execution is excluded
because existing tests read authentic MIDI references or write MIDI files.
Reference comparisons and MIDI-writing tests are not validation for this task.
