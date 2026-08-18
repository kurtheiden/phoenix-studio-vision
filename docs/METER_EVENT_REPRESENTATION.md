# Objective

Record the byte-exact evidence for Studio Vision's initial Meter representation
and separate established binary fields from derived musical meaning, historical
SMF export behavior, and unresolved general Meter-map semantics.

# Provenance

The authentic source project is the untouched Experiment 007
`newest STUFF baseline`, size 211,468 bytes, SHA-256
`e5a70056a4f8d6331b0c536a1c9841be1ec2f7f2c379c7123b3e1890767e5132`.
Its active sequence inventory and repeated sequence-level Meter/Tempo areas are
independently established.

Natural export correlation uses provenance-controlled full multitrack exports
of `Bells for her` and `Sequence K`. Controlled evidence uses Experiment 030,
which changed only the initial `Bells for her` Meter from 4/4 to 7/8 before its
matching export was created. The detailed evidence is in the external reports:

- `2026-08-18 Studio Vision Meter Correlation.md`;
- `2026-08-18 Experiment 030 Meter 7-8 Correlation.md`; and
- `2026-08-18 Sequence K 6-8 Meter Correlation.md`.

# Primary representation

All 18 authenticated initial project examples have this eight-byte form:

```text
00 ff 58 04 nn dd xx yy
```

Observed framing and supported interpretation are:

- `00`: zero initial-position byte; general position semantics are unknown;
- `ff 58`: Meter / Time Signature tag;
- `04`: four-byte payload length;
- `nn`: direct numerator byte;
- `dd`: direct denominator-exponent byte;
- `xx`: Studio Vision internal third Meter payload byte;
- `yy`: fourth Meter payload byte.

The complete exact primary boundary begins at the zero byte and ends after
`yy`. The following bytes belong to neighboring structure, not this bound.

# Natural 4/4 evidence

The untouched `Bells for her` primary is at
`0x0000eb80..0x0000eb88`:

```text
00 ff 58 04 04 02 08 08
```

Its provenance-controlled SMF event at tick zero is:

```text
00 ff 58 04 04 02 18 08
```

Thus `nn`, `dd`, and `yy` are byte-equal, while project `xx = 08` is exported
as SMF `cc = 18`.

# Natural 6/8 evidence

The untouched `Sequence K` primary is at
`0x000258df..0x000258e7`:

```text
00 ff 58 04 06 03 06 08
```

Its provenance-controlled SMF contains exactly one Time Signature event, at
tick zero:

```text
00 ff 58 04 06 03 0c 08
```

This independently correlates natural 6/8 and repeats the conversion
`xx = 06` to SMF `cc = 0c`.

# Controlled 7/8 evidence

Experiment 030 preserves the `Bells for her` primary boundary at
`0x0000eb80..0x0000eb88` and changes its bytes to:

```text
00 ff 58 04 07 03 06 08
```

The matching SMF tick-zero event is:

```text
00 ff 58 04 07 03 0c 08
```

The preregistered predictions for unchanged framing, `04 -> 07` numerator,
`02 -> 03` denominator exponent, `08 -> 06` third payload, and unchanged
fourth payload all matched.

# Natural 10/8 evidence

The untouched project contains a structurally supported `mission impossibl`
primary at `0x0001c864..0x0001c86c`:

```text
00 ff 58 04 0a 03 06 08
```

This is project-only evidence for 10/8. No provenance-controlled matching SMF
export independently establishes its historical `cc` value.

# Numerator

`nn` is the stored numerator byte. Observed values are `04`, `06`, `07`, and
`0a`, representing numerators 4, 6, 7, and 10. Natural variation and the
controlled `04 -> 07` edit establish the field directly. Binary evidence does
not justify rejecting other byte values in a framing-valid record.

# Denominator exponent

`dd` is the stored denominator exponent used by SMF Time Signature metadata.
The musical denominator is derived as `2^dd`. Observed `02` means 4 and `03`
means 8. The controlled edit establishes `02 -> 03` directly.

The source byte remains authoritative. A convenience derivation must return no
value when `2^dd` cannot fit its chosen integer type; that is not a structural
decode failure.

# Third payload byte

`xx` is directly observed and must be preserved with provenance. Its general
semantic name or unit is not established. It is not the SMF clocks-per-
metronome value stored directly, because both correlated values change during
export.

Firmly correlated historical mappings are:

```text
SVP xx 08 -> SMF cc 18
SVP xx 06 -> SMF cc 0c
```

The second mapping repeats for natural 6/8 and controlled 7/8. This supports an
evidence-bounded lookup over the correlated population, not a universal formula
or semantic interpretation. The decoder must expose `xx` as
`third_payload`, not as MIDI clocks or metronome grouping.

# Fourth payload byte

`yy` is `08` in every authenticated project example and is byte-equal to SMF
`bb = 08` in every firmly correlated export. This supports direct preservation
and export for the observed population. It does not establish that other
values are structurally invalid, so a bounded decoder should preserve rather
than reject them.

# Initial position field

Every authenticated primary begins with `00`, and every correlated export
places the event at sequence tick zero. Evidence does not distinguish an
absolute position, delta, or Meter-map-local location. Durable APIs should call
the byte `initial_position_byte`, require zero for this bounded initial form,
and assign no meaning to nonzero values.

# SMF export relationship

Standard SMF Time Signature payload is `nn dd cc bb`. Studio Vision's primary
payload is `nn dd xx yy`. Across correlated examples, `nn` and `dd` export
directly, `yy` exports directly as `bb`, and `xx` is converted to `cc`.

Binary decoding and SMF generation are separate concerns. A future exporter
may use exact evidence-backed mappings where known and an explicitly documented
standards-valid fallback otherwise. Musical Meter recovery depends on `nn` and
`dd`; unresolved historical `cc` policy does not erase the signature.

# Historical cc mapping

The complete firmly correlated population is reproduced by the two mappings
`08 -> 18` and `06 -> 0c`. Exact Studio Vision historical `cc` reconstruction
is therefore supported for those observed values but remains **PARTIAL** as a
general policy for arbitrary Meter forms or unknown internal values. This
lookup is not part of the binary decoder contract.

# Secondary value-bearing form

Each primary has a nearby correlated secondary form:

```text
58 nn dd xx yy
```

Examples include natural Bells at `0x0000ebbd..0x0000ebc2`, natural
`Sequence K` at `0x0002591c..0x00025921`, and Experiment 030 Bells at
`0x0000ebbd..0x0000ebc2`. The payload tracks the primary, but the broader
containing-record boundary, ownership, and purpose are unresolved. It is not
part of the bounded primary decoder.

# Sequence-level structural context

The primary occurs once per authenticated sequence in a named sequence-level
area, before a paired secondary Meter value and the initial Tempo structure.
Meter is not part of Note chains, Controller chains, Channel Pressure runs,
Pitch Bend runs, or generic mixed performance-event walking. A future
sequence/container parser must locate the structure and supply its exact bound.

# Evidence supported

- Exact eight-byte initial primary boundary and framing: **YES**.
- Numerator and denominator exponent: **YES**.
- Third payload preservation: **YES**.
- Third payload general semantics: **PARTIAL**.
- Fourth payload preservation and observed direct export: **YES**.
- Zero initial-form support: **YES**; general position semantics: **PARTIAL**.
- Bounded initial Meter decoder implementation readiness: **YES**.
- Standards-valid SMF Meter export: **YES**.
- General Meter-map parsing: **NO**.

# Unknowns

Unknowns include nonzero or variable-width position encoding, mid-sequence
Meter events, Meter-map discovery and walking, a universal semantic meaning or
historical `xx -> cc` rule, behavior of unobserved `yy` values, complete
secondary-copy framing, and full Meter-map reconstruction.

# Decoder implications

Decode only an exact caller-supplied eight-byte half-open range matching
`00 ff 58 04 nn dd xx yy`. Preserve every byte and absolute offset, expose the
four payload bytes directly, and optionally derive `2^dd` safely. Do not scan,
discover, parse secondary copies, infer position, impose musical validity, or
generate SMF policy inside the decoder.

# Experiment decision

**NO FURTHER METER EXPERIMENT NEEDED.** The bounded representation and musical
signature fields are established. Universal historical metronome-click policy
is not required for current Phoenix recovery and does not justify another
parameter-by-parameter experiment.

# Single recommended next step

The exact caller-bounded decoder is implemented. Next, establish
sequence/container discovery and integration capable of supplying exact Meter
and Tempo bounds without scanning.
