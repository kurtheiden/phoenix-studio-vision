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

## Unnamed complete sequence — second human listening pass

Phoenix produced an experimental Format 1 MIDI, 480 PPQN, with a conductor
track, Track 1, and retained empty Track 2. Track 1 contained 113 recovered
notes on MIDI channel 2, with 7/8 meter and approximately 82 BPM (731,707
microseconds per quarter note). No unsupported Program or Bank information
was invented.

Independent structural readback passed. All 113 note attacks and generated
releases matched decoded source timing, pitch, velocity, and channel. The
artifact was 1,098 bytes, SHA-256
`a4421055e4f6d6915e7b04e196b87590f3689735a768a0d25ca9b62feca00fc2`.

**Human listening result: PASS.** Kurt's exact observation was:

> That seems to play just fine too.

This remains a note-oriented listening observation with one non-empty musical
track. It does not validate complex multitrack synchronization, Controller
automation, Patch behavior, Pitch Bend, NRPN, or ff40 fader behavior. It does
not establish production readiness or equivalence to an authentic Studio
Vision MIDI export.

## Nothing FINAL — full multitrack human listening validation

**Human listening result: PASS.** Kurt imported the Phoenix experimental MIDI
into Logic, assigned replacement/new instruments and reported:

> Yeah that all sounds pretty good! I was able to assign some new instruments
> to it. Comes up all pianos by default in Logic.

This successfully validates the recovered musical performance by owner listening.
Logic initially presented the imported material using piano sounds by default;
the owner assigned new sounds. It does not demonstrate recreation of the original
Studio Vision instrument sounds, sonic identity with historical hardware,
bit-for-bit native-export equivalence or production export readiness.

The experimental artifact `/tmp/phoenix-school-experimental-Nothing-FINAL.mid`
is Format 1, 480 PPQN, with conductor plus all 15 ordinary source tracks.
It contains 5,109 recovered Notes, 70 source-channel Controllers, 18 Program
Changes and 406 Pitch Bends. Independent structural readback matched every
serialized channel message's tick, status and data to the bounded source handoff.
The initial tempo, meter and key annotation were included. Saved-muted tracks
were included; native timecode metadata and trailing padding were not synthesized.
Artifact: 44,633 bytes, SHA-256
`4366b0b3f83eadc4e36166ae48258bc0d0282f29713c783da489f1881eae4ca3`.
Implementation checkpoint: `02a46154e2805854bc728fe3a8693218b4eec782`.

The listening result reinforces the future Phoenix Recovery Report requirement:
expose original Studio Vision Instrument assignments and Patch information,
separately from translated MIDI Program/Bank events, to help users assign modern
instruments in Logic. Imported piano sounds are not evidence of original source
instrument identity. No Recovery Report or export UI is implemented here.

Saved mute is historical project state, not permission to discard data. Future
export should offer **Include All** (recommended/default) or **Exclude Muted**,
and identify saved-muted tracks in the Recovery Report regardless of the choice.
This is a product requirement, not an implemented prompt or authorization change.
