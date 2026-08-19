# Objective

Design the smallest useful macOS Phoenix desktop shell around capabilities
already proven in Rust. The app is a thin client: Phoenix Core owns all file
identification, Studio Vision interpretation, readiness decisions, conversion,
validation, and reports. The UI owns user interaction and presentation only.

# Current capability boundary

Phoenix can identify Studio Vision Finder metadata without trusting filename
extensions, structurally parse the established project/sequence profile, walk
supported mixed events, adapt decoded events, serialize Format 1 SMF, and
assemble tracks transactionally. The provenance-locked `Ode to Clarke` profile
has passed automated and Logic Pro validation.

Phoenix cannot yet derive MIDI channels generally, classify arbitrary projects
as exportable, cover every profile/event family, recover/relink audio, remap OMS
devices, create Logic projects, or provide a general conversion workflow. The
desktop design must expose these limits as readiness results, never UI guesses.

# Technology recommendation

Choose **A: a native SwiftUI frontend, using focused AppKit interop where
SwiftUI does not provide enough macOS control, calling a Rust Phoenix Core
library through a small stable C-compatible boundary**.

| Direction | Strengths | Costs / risks | Decision |
|---|---|---|---|
| SwiftUI/AppKit + Rust Core | Native drag/drop, panels, sandbox access, Finder integration, accessibility, signing/notarization, macOS look | Requires a deliberately small Swift/Rust FFI boundary and Xcode build | **Recommended** |
| Rust-native UI | One primary language; possible portability | Less native macOS behavior/polish; accessibility and platform integration vary; extra UI ecosystem choices | Not first choice |
| Tauri + webview | Rust backend, cross-platform frontend, mature web UI skills | Adds web stack/webview and bridge surface; native feel/accessibility require extra discipline; two dependency ecosystems | Defer unless cross-platform becomes the leading constraint |

SwiftUI directly supports file import/export dialogs and drag destinations;
AppKit supplies `NSOpenPanel`, `NSSavePanel`, and Finder integration where
needed. Apple documents security-scoped access for user-selected sandbox files,
so the app layer must acquire and release that access around a Core operation.
The normal macOS signing/notarization path stays in Xcode rather than being
recreated by a third-party shell.

The bridge should expose versioned request/response operations using owned,
FFI-safe data—not Rust borrows, parser structs, SwiftUI types, or callbacks into
reverse-engineering code. A generated or handwritten C header can initially
carry opaque handles plus serialized DTO payloads; the exact binding mechanism
belongs to UI0 evaluation.

Official platform references used for this direction:

- [Apple: adopting drag and drop with SwiftUI](https://developer.apple.com/documentation/swiftui/adopting-drag-and-drop-using-swiftui)
- [Apple: accessing files from the macOS App Sandbox](https://developer.apple.com/documentation/security/accessing-files-from-the-macos-app-sandbox)
- [Apple: SwiftUI file importer and security-scoped URLs](https://developer.apple.com/documentation/swiftui/view/fileimporter%28ispresented%3Aallowedcontenttypes%3Aallowsmultipleselection%3Aoncompletion%3Aoncancellation%3A%29)

# Core/App boundary

Phoenix Core owns:

- read-only input opening after the app grants a user-selected URL;
- identification and structural inspection;
- sequence enumeration and stable opaque sequence identities;
- readiness assessment and the reasons/evidence behind it;
- selection of an explicitly allowed conversion policy/profile;
- decoded-event conversion, transactional validation, and MIDI bytes;
- collision-safe export operation or a byte/result contract governed by one
  application service (the final ownership choice is UI0 work);
- structured reports, warnings, unsupported content, diagnostic context, and
  stable error categories.

The desktop app owns:

- one-file drag/drop and Open interaction;
- security-scoped URL lifetime and standard open/save/folder panels;
- displaying summaries, readiness, warnings, progress, and errors;
- collecting sequence and destination choices;
- invoking one high-level Core operation at a time;
- revealing a successful output in Finder.

The app never reads SVP bytes for meaning, derives bounds/channels, classifies
Patch data, serializes MIDI, validates export completeness, or contains an Ode
hash/range/channel constant. It must render Core results, not independently
recalculate them.

UI0 formalizes this boundary as an owned, versioned application-service
contract: session-scoped opaque project/sequence IDs, readiness/reason codes,
diagnostics, compatibility-profile labels, export reports, and categorized
errors. The frontend receives no borrowed parser data.

# Missing app-facing API

## Already available

- `parse_project_166` and established decoded structures;
- bounded Tempo, Meter, Patch, Controller, Pressure, Bend, Note, and mixed-event
  APIs;
- pure decoded-event adaptation and explicit policy types;
- transactional `assemble_multitrack_sequence` returning SMF bytes and counts;
- low-level inspection, hashing, and Finder-metadata identification behavior.

These are library/research primitives, not a UI contract. Identification is
currently crate-private, parser results borrow input bytes, and the authenticated
Ode integration remains test-owned proof policy.

## Small wrapper needed

Add a future application-service layer with owned, versioned DTOs for operations
conceptually equivalent to:

- inspect one selected path and return file/project/sequence summaries;
- assess every discovered sequence and return readiness plus reasons;
- export one selected sequence under an explicit supported-profile policy to a
  caller-approved destination;
- optionally export all READY sequences transactionally per sequence later;
- return a complete application export report and categorized error.

It should own file reads/hashing, map borrowed parser values into owned text and
identities, enforce profile policy, and keep diagnostics separate from primary
user wording. A stable sequence identifier must be opaque to Swift and remain
valid only for the inspected project/session or include a Core-issued project
identity token.

## Not yet generally possible

- declaring arbitrary sequences READY;
- deriving arbitrary track channels/routing;
- translating unknown Patch forms or unsupported families;
- audio-reference recovery/relinking;
- OMS/device mapping or DAW project generation.

# User journey

1. Launch into a single calm drop/open window.
2. Drop one file or choose Open.
3. The app gives the user-selected URL to Core and shows indeterminate
   “Inspecting…” activity.
4. Core returns an owned project assessment.
5. Show filename, identification, sequence list, readiness, and warnings.
6. Select one sequence. Export is enabled only for READY sequences backed by an
   explicit supported compatibility profile.
7. Choose an output folder using the standard macOS panel.
8. Click **Export MIDI**; disable conflicting actions and run Core work off the
   main actor.
9. Core either returns one complete success report or an error—never partial
   success presented as completion.
10. Show output path, track/event counts, warnings, and **Reveal in Finder**.

# Project summary

The normal summary shows filename, core identification wording/confidence,
size, project recognition/profile status, sequence count, sequence names,
readiness per sequence, and warning count. It does not show hashes, offsets,
descriptor/pair ordinals, raw ranges, or proof-policy internals. Those belong
only in Diagnostics.

# Sequence readiness

- **READY:** Core has a complete explicit policy and can export transactionally
  without known silent musical loss. In v0 this may mean a validated research
  profile, clearly labeled as such.
- **PARTIALLY SUPPORTED:** Core recognizes and can inventory the sequence, but
  one or more recognized structures/events/routing facts cannot be exported.
  Export is disabled.
- **UNSUPPORTED:** Core positively identifies a required profile, structure,
  or event family that this Phoenix version does not support. Export is
  disabled.
- **UNKNOWN:** Core cannot safely classify structure or completeness. Export is
  disabled.

Only Core assigns these states. An authenticated Ode proof must not become a
name-based readiness rule; the entire Core compatibility profile must validate.

# Limitations messaging

Primary wording is short and actionable:

- “This Studio Vision project was recognized, but Phoenix cannot yet determine
  the MIDI routing for one or more tracks.”
- “This sequence contains event types this version of Phoenix does not yet
  support.”
- “Phoenix could inspect this file but cannot safely determine whether this
  sequence can be exported.”

Each message offers **Show Technical Details**. The primary view never dumps
offsets or Rust debug errors and never suggests renaming/extensions as a fix.

# Diagnostics

An optional disclosure shows Phoenix/Core version, SHA-256, identification and
profile evidence, structural sequence status, readiness reason codes,
unsupported families, preserved parser/export context, warnings, export report,
and a copyable diagnostic bundle as text. Secrets and unrelated filesystem
paths should be minimized. Diagnostics are supporting evidence, not a second
parser in Swift.

# Drag and drop

Prototype v0 accepts **ONE FILE ONLY**. Open and drop use the same operation.
Multiple dropped items or a folder produce a concise prompt to choose one file;
they do not create a queue. Extensionless and misleading-extension files are
accepted for Core identification. An unsupported file remains inspectable only
to the extent Core can return a safe identification/diagnostic result.

# Output workflow

Choose **A: ask for an output folder for each export action** using the standard
macOS panel. Suggest `<Sequence Name>.mid`; do not default to writing beside the
source and do not require Preferences.

If the destination exists, never overwrite silently. v0 presents Cancel or a
Core/app-generated nonconflicting filename such as `Name 2.mid`; explicit
Replace may be added only with a standard confirmation and atomic-write policy.
Core must receive/reserve the final destination under one collision contract so
the UI cannot report a path different from the one actually written.

# Progress and cancellation

All inspection/export work runs off the main actor behind a single operation
state. v0 shows stage text plus indeterminate progress: “Reading file,”
“Inspecting project,” “Validating sequence,” “Exporting MIDI.” It does not
invent percentages.

The operation API should accept a cancellation token/checkpoints even if v0
offers cancellation only before the final atomic write. Hashing large inputs,
parsing, and future queues can then become incrementally cancellable without
changing UI state models. Closing/replacing a project cancels or awaits the
current task safely.

# Error presentation

Core returns a stable category, concise suggested user message, technical
detail, and provenance/context. The UI maps categories as follows:

| Category | Main presentation |
|---|---|
| File unreadable | “Phoenix couldn’t read this file.” |
| Not recognized | “Phoenix couldn’t identify this as a supported Studio Vision project.” |
| Unsupported profile | “This Studio Vision project uses a profile this version cannot safely process.” |
| Sequence unsupported | “This sequence can be inspected but not exported.” |
| Missing routing/channel | “Phoenix cannot yet determine MIDI routing for one or more tracks.” |
| Unsupported event family | “This sequence contains MIDI event types Phoenix cannot yet export.” |
| Export validation failure | “Phoenix stopped because the exported result could not be validated.” |
| Output I/O failure | “Phoenix couldn’t save the MIDI file at the selected location.” |
| Unexpected internal error | “Phoenix encountered an unexpected error and did not export a file.” |

Technical details preserve the underlying typed error chain and structural
context. No category converts partial bytes into a successful UI result.

# Authenticated-profile policy

Choose a combination of **B/C** operationally: **inspect any structurally
recognized project, but permit export only through explicitly labeled,
provenance-validated compatibility profiles held in a Core policy registry**.

For v0, the only real export may be the authenticated Ode profile if—and only
if—the future application service can validate its complete project/sequence/
track policy exactly. The UI displays “Validated research profile” and never
selects it by filename or sequence name. Profile constants remain in Core
policy code, not generic parsers or Swift. A profile mismatch yields
PARTIALLY SUPPORTED/UNKNOWN and no Export button.

This permits a real end-to-end prototype without pretending general channel
derivation exists. Arbitrary projects are useful inspection targets but never
receive an enabled Export action.

# Window structure

Use one window with four states:

1. **Welcome/Drop:** app purpose, drop target, Open button.
2. **Project:** compact summary above a sequence list; selection shows readiness
   reason and counts known from inspection.
3. **Exporting:** same context, disabled mutations, stage/progress and Cancel
   where safe.
4. **Result/Error:** success report with Reveal in Finder, or actionable error.

Diagnostics is a disclosure/sidebar sheet within this window, not a separate
research application. A toolbar Open action replaces the current project after
safe cancellation/confirmation.

# Visual principles

Use standard macOS materials, spacing, typography, tables/lists, symbols, and
selection behavior. The tone is calm, preservation-focused, musical, and
technically credible. Phoenix branding can use restrained warm accents and
language about recovery; avoid flames, distressed nostalgia, fake hardware,
and celebratory success before validation.

# Accessibility

v0 requires full keyboard traversal and default-button behavior; meaningful
VoiceOver labels, values, and readiness explanations; visible focus; status
icons plus text rather than color alone; adequate contrast; scalable system
text where practical; and announcements when inspection/export state changes.
Drop functionality always has an equivalent Open button.

# Settings

No Preferences window in v0. The standard folder panel can remember its own
last location. Collision handling is fixed and safe. Diagnostics disclosure is
per-window state. Add settings only after repeated workflows establish a need.

# App-facing export report

Core should return a platform-neutral owned result containing:

- source identity: display name, size, SHA-256/profile identifier as diagnostic
  fields;
- sequences attempted and exported, with stable opaque identifiers;
- exact output paths actually committed;
- per-sequence musical/total track counts;
- Note, generated Note Off, ordinary Controller, bank MSB/LSB, Program Change,
  Channel Pressure, Pitch Bend, Tempo, and Meter counts;
- warnings and unsupported/non-emitting content;
- compatibility/readiness policy identifier and version;
- future optional audio-reference records, absent until established.

The result contains no SwiftUI types and no raw parser references. Success means
all requested output for that operation committed transactionally according to
the declared policy.

# Prototype v0 scope

## In

- macOS only;
- one input file at a time;
- drag/drop and Open;
- Core identification/inspection and sequence list;
- explicit readiness states/reasons;
- export only an exact Core-validated research compatibility profile;
- choose output folder, safe collision handling, transactional result;
- counts/warnings report and Reveal in Finder;
- optional diagnostics panel;
- background operation state, basic cancellation seam, accessibility baseline.

## Out

- batch/queue conversion;
- general arbitrary-project export or channel inference;
- audio recovery/relinking and playback;
- OMS/device remapping or synth recommendations;
- Logic/Fender Studio project generation;
- MIDI editing, waveform views, cloud/accounts;
- broad Preferences;
- Windows/Linux shipping in v0.

# Explicit exclusions

No Studio Vision byte interpretation, proof offsets/channels, Patch inference,
SMF writing, or readiness decision exists in Swift. No successful result is
shown for partial output. The design adds no framework, dependency, app target,
bundle, CLI behavior, or production code.

# Implementation phases

1. **UI0 — Core application contract:** design/test owned DTOs, readiness/error
   categories, profile registry boundary, path/file-I/O ownership, cancellation,
   and C-compatible ABI; keep it callable without a UI.
2. **UI1 — native shell:** create the SwiftUI/AppKit Xcode app, single-window
   state machine, Open/drop, sandbox entitlements, and Core version handshake.
3. **UI2 — inspection:** connect one-file inspection, project summary, sequence
   list, readiness, limitations, and fixture-backed contract tests.
4. **UI3 — export:** add folder selection and real export only for a completely
   validated compatibility profile, with collision/atomic semantics.
5. **UI4 — results:** counts/warnings, errors, Reveal in Finder, and diagnostic
   details/copying.
6. **UI5 — hardening:** cancellation behavior, VoiceOver/keyboard/contrast QA,
   signing, sandbox, notarization, packaging, and manual DAW acceptance.

UI0's contract design is recorded separately in
`docs/APP_SERVICE_CONTRACT_DESIGN.md`; its first implementation slice is owned
DTOs before transport or Swift code.

# Strategic implementation gate

- **A. Begin UI implementation now: YES.** Begin with UI0, then the shell and
  inspection. The proven Core justifies a real thin-client prototype.
- **B. Include real MIDI export: PARTIAL.** Yes only for an exact Core-validated,
  explicitly labeled research compatibility profile; not arbitrary projects.
- **C. Arbitrary projects expose Export: NO.** They remain inspectable with
  readiness/limitations.
- **D. Unsupported projects remain inspectable: YES.** Inspection has product
  value and generates useful diagnostics.
- **E. UI knows authenticated Ode offsets/channels: NO.** Those stay in an
  isolated Core compatibility policy.
- **F. Wait for general channel derivation before UI architecture: NO.** The
  shell, inspection, readiness, report, and bridge architecture remain valid;
  general routing later changes Core readiness, not UI logic.

# Unknowns

- Exact C ABI/binding generator and static-library/XCFramework packaging.
- Minimum supported macOS/Xcode/Swift version and distribution channel.
- Whether Core or the app service owns final atomic output writing across the
  sandbox boundary.
- Durable security-scoped bookmarks, unnecessary for one-shot v0 but relevant
  to remembered folders/batch work.
- Stable general readiness rules after channel derivation expands.
- Commercial licensing, update delivery, telemetry/privacy, and support policy.

# Single recommended next step

Design UI0's application-facing Rust service contract and versioned owned DTOs,
including readiness, error/report schemas, compatibility-profile isolation,
cancellation, and sandbox-safe file-I/O ownership—without creating UI code yet.
