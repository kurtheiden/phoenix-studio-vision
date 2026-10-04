# SCHOOL PROJECTS experimental listening evidence

## Tuba thing — first human listening pass

Source project: SCHOOL PROJECTS, 343,875 bytes, SHA-256
`bfd4fa1208e2cd884ec51ccfb1131d5c02723d7c90acafc98bd89597e1a20331`.
Decoder baseline: `2d6ef01561d79ed79d5e12c49297ea5fb7ed1922`.

Phoenix produced an experimental Standard MIDI File, Format 1, 480 PPQN,
with a conductor track, Track 1, and retained empty Track 2. Track 1 contained
75 recovered notes on MIDI channel 3. Initial tempo was approximately 110 BPM
(545,454 microseconds per quarter note), with 4/4 meter. No Program or Bank
changes were invented. The source Track 1 was saved mute-ON; that mute state
was intentionally not represented in the listening artifact.

Independent structural MIDI validation passed. All 75 note attacks and 75
generated releases matched the decoded source pitch, velocity, channel, and
timing. The experimental artifact was 787 bytes, SHA-256
`4085abaa837942e426a6c95895b414ce77345488ba2e2fcb4d73d4470f97bcc5`.

**Human listening result: PASS.** Kurt reported that it played correctly from
start to finish in Logic:

> It played fine from start to finish. It is a very simple song with just one
> music track with no expression or pitch bends or NRPN changes, so it wasn't a
> particularly heavy lift but it did work.

This is a limited musical playback observation for a simple note-only sequence.
It does not validate expression/controller automation, Pitch Bend, NRPN, ff40
fader events, complex Patch behavior, complex multitrack synchronization, every
supported event family, or production export readiness. It does not establish
equivalence to an authentic Studio Vision MIDI export. No reference MIDI was
used for this listening exercise.
