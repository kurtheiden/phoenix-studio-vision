# Bounded COMIC BOOK native-reference recovery (CCA-023)

## Evidence and scope

The owner-confirmed Archive source is 406,294 bytes, SHA-256
`40217bf534e7ac7f01ccab65d5d6f3fc873f08c134095410da2cbf5cef88ac70`:
`/Volumes/Extreme Pro 2TB/Applications/SheepShaver/Shared/MY OLD MUSIC/Archive folder/COMIC BOOK`.
The native `pick up the pace` reference is 592 bytes, SHA-256
`c268a5b58556b17a0376e585fa11f5d671d99a4c655d6c8f432170b552944557`, in the Shared directory.

CCA-019 through CCA-022 originally examined a different, 452,284-byte
COMIC BOOK source. Its channel 5 / JV-880 Orch. interpretation and secondary
suffix refusal do not describe this Archive source. The confirmed source's
existing routing proof selects Instrument row 24, device 06, channel 12,
JV-880 Vintage. No routing decoder correction is required.

## Additive policy

The compatibility policy qualifies only this exact source digest, size,
object ordinal 0 and sequence range [9095,10134). Its immutable borrowed
qualification permits the existing conductor decoder to process the exact
four ranges [9545,9588), [9588,9622), [9622,9665), [9665,9698), whose primary
header is 00 02. Counts, events, secondary copies, tails and padding still
pass all existing checks. There is no conductor-secondary suffix here.

The exact terminal [10123,10134), payload 00 00 00 01 FF FF, is retained
as a TerminalDiagnostic after exhaustive framed musical coverage passes.
No semantic meaning is assigned to that word. This source-specific native
qualification is not a general header-02 or terminal rule. No filename or
reference file is consulted in production. Generic public decoders still
refuse these forms. Changed sources and unrelated objects are unqualified.

The existing ordinary Track proof, event walker, routing, scheduling, SMF
writer, readback and service authorization remain in use without changes.
The reference validates this exact source/object; it does not resolve the
old COMIC BOOK suffix family or Prologue's unusual conductor headers.

## Fidelity and exclusions

The independent test parser compares all 128 Note On/Off messages in order,
including absolute ticks, channel, pitch and attack/release velocities.
All agree with the native reference. Initial tempo is 521739 MPQN; meter is
4/4, MIDI clocks 24 and thirty-seconds 8. Existing meter adaptation maps the
source clocks value 8 to MIDI 24. Both files are format 1, two Tracks, 480 PPQN.

As already documented in ODE_MULTITRACK_COMPARISON.md, native-only zero
SMPTE Offset and Instrument Name metadata and trailing End-of-Track padding
are expected differences under the recovery contract. The reference contains
FF54 60 00 00 00 00 and Instrument Name JV-880 Vintage. Its EOT ticks are
19200; Phoenix uses conductor EOT 0 and musical EOT 18333, the last release.
These differences are explicitly tested; files are not claimed byte-identical
or equivalent reproductions of every Studio Vision UI/playback feature.
No other event or metadata difference is silently filtered.

## Regression and authentic result

`tests/comic_book_reference120.rs` verifies source/reference hashes, complete
in-memory MIDI agreement, retained terminal bytes, unchanged generic refusals,
source mutations (routing, conductor, terminal, event), and existing service
Ready/bounded capability after all recovery gates pass. It writes no MIDI.
The authentic reference test requires the documented mounted evidence.

One additional exact-source sequence is now authorized for complete MIDI
recovery under the existing contract. No destination MIDI was exported in
CCA-023. No other candidate is qualified. Diagnostic export is not introduced:
its separate safety and omission-reporting contract is not established.

Validation: cargo fmt --check, cargo clippy --all-targets -- -D warnings,
cargo test (617 passed, 10 ignored, zero failures), and git diff --check
passed. The three focused native-reference/qualification/service tests passed.
