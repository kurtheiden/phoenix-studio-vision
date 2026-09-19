# Opcode Studio Vision documentation foundation checkpoint

Date: 2026-09-19. Repository baseline: `b9ab9b18030922545f431c3b50804f6724db2f5c`. This checkpoint records research conclusions; it does not change Phoenix production behavior.

## Primary PDF coverage

The six primary Opcode PDFs have **1,128 of 1,128 pages reviewed as extracted text**. The page ranges in the outside-repository coverage ledger cover each PDF page exactly once.

| PDF | Pages reviewed |
|---|---:|
| MIDI Reference Manual | 499/499 |
| Audio Reference Manual | 467/467 |
| What's New in 4.5 | 127/127 |
| QuickStart - Vis 4.5 | 2/2 |
| Vision opPLUGS Manual | 32/32 |
| Authorizing Studio Vision | 1/1 |

The durable research records are in `/Users/kurtheiden/Documents/Phoenix Research/Opcode-Studio-Vision-Documentation-Foundation-2026-09-18/`:

| Research file | SHA-256 |
|---|---|
| `CORPUS_INVENTORY.md` | `fe61c7f052facb41f5949d024957daf5686c21b905646790fec8fab1861015c7` |
| `COVERAGE_LEDGER.md` | `30207c0014479fbf6f30d1e9618fb5dc26e6fce4ba5b658998da8ddc51d8a5af` |
| `STUDIO_VISION_FUNCTIONAL_REFERENCE.md` | `de443c0326ca5df19b9e9a000eef296396522d49f21b30e365f278553ddd90b6` |

## Recovery and export implications

- Opcode documents sequence events as references to sequences or segments. Capture renders their playback and loops into events; a native reference and its rendered MIDI export have different functions. Some score export paths can omit uncaptured sequence events. [MIDI Reference Manual, PDF pp. 112–120, 308, 418–419, 430–431]
- Tracks route through MIDI Instruments to OMS devices and channels. Instruments may layer or transform output, so a track label or displayed channel alone does not establish all routing. [MIDI Reference Manual, PDF pp. 309–316]
- Vision documents audio events as pointers into external audio files. Files can be shared across documents, relinked, consolidated, compacted, or replaced by DSP output. A project file alone need not contain its waveform samples. [Audio Reference Manual, PDF pp. 21–23, 258, 297–313, 437–443]
- The complete MIDI manual text contains no explicit RPN/NRPN discussion. This does not establish absence of their controller messages or software support. [MIDI Reference Manual, PDF pp. 1–499]

These are documented application behaviors. The manuals do not establish native serialized byte layout; Phoenix byte observations and interpretations remain separate evidence.

## Remaining limits and decision

Figure-by-figure visual verification is incomplete. The bundled 730-resource and alternate 553-resource PICT Help collections remain unread. Opcode's Last Minute Notes say Studio Vision Help was not updated for version 4.5. Thus the **primary PDF text review is complete**, while the **entire available documentation corpus is not fully reviewed**.

No immediate Phoenix 0.1 production implementation change is established solely by this review. Return to Phoenix 0.1 release work rather than expand this documentation task now.
