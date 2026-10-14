# Bounded FF60 length-7 Controller bridge — CCA-059

The mixed-event walker now admits exactly a length-7 FF60 context whose payload
starts `57 7F 00`, followed by a bounded timing VLQ and explicit B0..BF
Controller. Checked arithmetic adds the context's leading interval once and
the successor Controller interval once. The existing Controller decoder then
establishes compact state; ordinary walking continues without scanning,
resynchronization or guessing. A later unsupported object refuses the whole
walk. Existing explicit experimental prefix recovery retains only its
separately validated prefix and leaves the entire refused suffix unresolved.

The Controller retains an optional context with exact tag, length, payload,
leading timing bytes/range and successor timing bytes/range. Its enclosing
representation includes both the context and Controller. Ordinary Controllers
retain `context: None` and unchanged values/ranges. Source-object reconciliation
counts each new context separately from its musical Controller. Existing
routing guards validate the retained payload without changing output channels;
a Controller containing a context does not extend the separate controllers-only
zero-prefix Note exception. Experimental limitations JSON retains each bridge's
complete context bytes, source ranges, both intervals and resulting position.

## Independent evidence and provenance reconciliation

`dominion with samples` is 134,204 bytes, SHA-256
`4ec77addb3b39dfb5791d853cbb6225df9da30a95431633137e02a6811a8b34b`.
Its `Opcode/MY MUSIC` duplicate is identical. The candidate passage belongs to
Reservist Revolt, sequence beginning18838, next sequence33721, descriptor7
Track6 (label19799), primary26850..28186, secondary28186..28401,
bounded musical body26869..28179. Earlier comparison with Dominion exports
failed because those represent the source's earlier Sequence B: six
note-bearing streams match Dominion Multitrack exactly. Similar project/export
filenames were not correspondence evidence.

Existing native references under `Shared/MIDI Exports from SVP` are:

| Reference | Bytes / format | SHA-256 |
|---|---|---|
| Reservist Revolt | 12,998 / 0 | `630ae0844e98180e513e76f16fc5532f0e2e43520362724fc13d502c8624521c` |
| Reservist Revolt Multitrack | 14,465 / 1 | `9c1b9b5383acbcfbc56f5b059de17702717813f5d5b6efb93ba8bae0ccabc0a6` |

Both use480PPQN. The multitrack counterpart is Track6, JV-880/2, channel9.
All61 Note onset/pitch/attack/duration tuples and all240 source Controller
tick/number/value tuples match both references, in order. One native CC0=80
at61440 accompanies Program16 and is separate from the compared source
Controller stream. Source accounting balances61 Notes+240 Controllers+
9 contexts+1 Patch=311 objects. Production ownership, exact bounds, routing
guards and context-count reconciliation also pass for this Track.

| Leading offset / FF60 tag | Previous tick | Leading interval | Successor interval | Native tick |
|---|---:|---:|---:|---:|
| 27320 / 27322 | 91919 | 1192 | 98 | 93209 |
| 27588 / 27589 | 97157 | 13 | 2 | 97172 |
| 27796 / 27798 | 100786 | 443 | 127 | 101356 |

Each leading interval is included exactly once. Exclusion and double counting
contradict native output. No counterexample was found in these matched cases;
other corpus bodies without established correspondence do not validate timing.
Experiments019/020/022 support inter-event intervals and Experiment031 separates
framed context payload from successor timing, but do not alone establish this
Controller composition. Payload semantics beyond the guarded prefix remain
opaque. Source hashes/names/reference events identify test evidence only;
the new runtime rule has no specimen dispatch or reference input.

## Sweeper result and remaining refusal

The original COMIC BOOK452,284-byte source is unchanged (SHA-256
`22a9e2589098b0d475f6e288eeebc9303bed644aa8ce09f9971a0060c9160486`).
Square Wave advances from1 Note/59 Controllers to64 Notes/763 Controllers.
Three new bridges begin44052,44281,46252. Their first Controller positions
are3856,5776,13415; the second Note starts3959. The prefix43854..46644
accounts for832 objects:64 Notes+763 Controllers+5 contexts.

The refused object begins46644: leading02, FF60 tag46645, length6,
payload`57 7F 00 7C 7F 21`, successor interval01 and explicit B0 at46655.
The suffix46644..64556 retains5269 unresolved objects. Square Wave is still
incomplete:455 of the native519 Notes remain withheld. No new length-6 rule,
other payload family or arbitrary successor is admitted. Existing length-6
zero-delta specimen bridges and the CCA-057 Pipe witness are unchanged.

Only zero-based MIDI chunk6 differs from CCA-058; all24 other chunks are
byte-identical. Prior Square channel events remain unchanged and ordered.
CCA-058's2294 pitch corrections and Pipe's151 Notes/993 Controllers/22 Bends
remain intact. New totals are3921 Notes,5861 Controllers,373 Bends,15 Programs,
1 GS Reset,26 Tempos,initial meter, and6641 unresolved source objects.

## Regression checks

Seven portable tests cover exact provenance, nonzero timing components, compact
Controllers, Note re-entry, explicit status channels, rejected lengths/prefixes/
successors, truncation, invalid data, overflow, transactional refusal and prior
event invariance. A routing regression preserves the separate zero-prefix
Note restriction. Opt-in authentic tests validate Reservist against both native
references; Sweeper checks counts, source ledgers, native prefix content,
CCA-058 invariance, CCA-057 transposition/Pipe witnesses, public export readback
and no-overwrite behavior. Existing authentic prefix/refusal tests and the
700-note SCHOOL transposition validation also pass.

Formatting, all-target/all-feature Clippy with warnings denied, full regression
suite (665 passed,0 failed,20 ignored) and diff whitespace checks pass. Seven
additional opt-in authentic checks pass: Reservist, SCHOOL, four existing
experimental spans, and the public Sweeper export. Independent SMF readback
also matches all64 Square onset/pitch/attack/duration tuples. Original projects, references,
earlier exports and unrelated uncommitted files are checked against the
pre-edit preservation snapshot. No commit, push or native experiment occurs.
