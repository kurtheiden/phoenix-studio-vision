# Caller-bounded Meter/Key map decoder

This internal module decodes a nonempty event-only slice with independently supplied
bounds and object count. It does not discover records, prove envelope ownership,
qualify recovery or authorize export.

Supported structures are:

```text
7-bit big-endian VLQ delta | FF 58 04 | numerator denominator-exponent third fourth
7-bit big-endian VLQ delta | FF 59 02 | signed-accidentals mode
```

Timing uses the existing four-byte VLQ primitive. Checked cumulative `u32` ticks
start at zero; a nonzero first delta is retained. The existing timing domain is
480 PPQN, not a newly decoded clock. Source bytes, ranges, raw timing widths and
located fields remain borrowed and intact. Duplicate and simultaneous objects
retain source order; private construction protects the decoded map.

Meter payloads remain raw. Export consumers must separately apply the existing
`KnownHistoricalOnly` adaptation policy; unknown raw fields do not become
exportable merely because decoding succeeds. Key accidentals are signed -7..7
and mode is 0 or 1.

Invalid bounds, zero/impossible counts, malformed timing, unsupported tags,
altered lengths, truncation, invalid Keys, position overflow and trailing bytes
refuse the whole map. There is no scanning, repair, partial success, default
Meter, offset or timing normalization.

## Synthetic examples and checks

The public tests construct bytes directly and read no external fixtures. One
schedule uses a synthetic 5/8 Meter and a three-flat minor Key, with ticks
`0, 293, 293, 293, 586`. The two-byte delta `82 25` means 293. Another case
retains a nonzero first delta of 393 and a noncanonical two-byte zero representation
accepted by the existing primitive. These are protocol examples, not claims
about any authentic project.

Tests assert exact provenance, timing, order, values, malformed-input refusal,
complete consumption and overflow. Run `cargo test --offline --locked`.

The decoder remains unused by production recovery/export. Future integration
requires independently established envelope ownership, exact bounds/counts,
non-event accounting and all existing admission, consent and publication gates.
