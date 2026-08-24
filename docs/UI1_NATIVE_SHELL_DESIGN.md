# UI1A native shell design

UI1A is the smallest native application slice after the completed UI0 Core
contract and UI0G external bridge validation. It creates one macOS SwiftUI
application target and proves a real Phoenix Core handshake without beginning
file access or product workflows.

## Project and target

Create the first native target under:

```text
macos/PhoenixApp/PhoenixApp.xcodeproj
macos/PhoenixApp/PhoenixApp/
```

Use `Phoenix` as the product name and `PhoenixApp` as the target name. A
provisional development bundle identifier may be used if Xcode requires one;
commercial identity, signing, and distribution identifiers remain unsettled.

UI1A uses a provisional macOS 13.0 deployment target. This is a development
choice for the basic SwiftUI/concurrency APIs used here, not a declared final
Phoenix support policy; lowering or raising it requires later build evidence.

## Development linkage

The target consumes the existing bridge directly:

- header/module: `include/phoenix.h` and `include/module.modulemap`;
- library: `target/release/libphoenix.a`;
- header/module search path: repository `include`;
- library search path: `target/release`;
- linker name: `phoenix`.

An Xcode Run Script build phase should execute `cargo build --release --locked`
from the repository root before the app target links. The project must derive
paths from its own location, not from a developer-specific absolute path. The
current macOS toolchain supplies `libSystem`; use only additional native flags
shown to be necessary by the target build. Final packaging and universal
library construction are later decisions.

## Shell model and bridge ownership

`AppModel` is the one application-owned `@MainActor` presentation model. It
publishes exactly three states: `starting`, `ready(contractVersion)`, and
`failed(message)`. It owns one private `PhoenixCore` bridge object for the
shell lifetime; SwiftUI view lifetime does not own the service handle.

`PhoenixCore` is a minimal private actor that:

- creates one Phoenix service handle;
- encodes the exact `get_api_info` request as UTF-8 JSON;
- calls synchronous `phoenix_call` off the main actor;
- copies response bytes into Swift-owned `Data` before
  `phoenix_free_buffer`;
- decodes only the API-info envelope and contract version needed by UI1A;
- maps ABI, transport, and Core errors into a bounded Swift error;
- destroys the handle exactly once when its owner is released.

It is not a general Swift SDK and does not mirror the full Rust DTO surface.

`AppModel` starts a task that awaits the actor's handshake, then mutates UI
state only on `MainActor`. No synchronous Core call runs on the main actor.

## Visible states

The first window is a simple centered SwiftUI view:

- `starting`: Phoenix identity plus a modest `ProgressView`; no project
  controls;
- `ready`: “Phoenix Core connected” and application contract version `1`;
- `failed`: concise heading and bounded diagnostic text, without fake retry or
  recovery behavior.

No Open button, file picker, drag target, diagnostics panel, export control, or
technical parser detail belongs in UI1A.

## Validation

UI1A must build the Xcode target from the command line with `xcodebuild`, link
the release static library, import the Phoenix module, and launch far enough
to observe the ready handshake state. Existing
`tests/ffi/run_external_validation.sh` and the Rust suite remain required
regressions. GUI automation is unnecessary for this first checkpoint; a
successful process launch plus recorded ready-state evidence is sufficient.

## Later UI1 slices

- UI1B: Open/file-access boundary for one selected file;
- UI1C: inspection result, project summary, and sequence presentation;
- UI1D: readiness, limitations, and diagnostics presentation;
- UI1E: one-sequence export workflow and result presentation.

Drag/drop, security-scoped bookmark persistence, batch export, overwrite,
genuine cancellation, audio recovery, DAW/remapping, final branding, Intel or
universal validation, XCFrameworks, signing, notarization, and distribution
remain deferred.

## Implementation status

UI1A is implemented in `macos/PhoenixApp`. The `PhoenixApp` target uses the
provisional macOS 13.0 deployment target, invokes `cargo build --release
--locked` from its Xcode build phase, and links the existing release static
library through the public module map. A real arm64 macOS launch reached
`UI1A_READY contract_version=1`; the service is owned by the private
`PhoenixCore` actor and destroyed by its owner. UI1E is the next slice.

## UI1B status

UI1B adds one-file `NSOpenPanel` selection without extension filtering and sends
the selected `URL.path` through the existing `inspect_project` contract at
summary diagnostics level. The app retains the opaque session ID and shows a
compact Core-derived project summary or bounded nested `AppError` message.
The development target remains unsandboxed; security-scoped access and
bookmarks are deferred.

## UI1C status

UI1C is implemented: a successful inspection retains the Core project summary
and typed sequence data, presents explicit project counts, and shows an
informational sequence list or truthful zero-sequence empty state. Sequence
readiness, detailed diagnostics, and export remain deferred to UI1D/UI1E.

## UI1D implementation status

UI1D is implemented as the readiness, limitations, warnings, and project-level
diagnostics presentation phase. It maps Core readiness to “Ready”, “Partially
supported”, “Not currently exportable”, or “Readiness unknown”; presents Core-
provided readiness details and warning messages; and uses opaque sequence
selection only for that context. Details load lazily through project/session-
scoped `get_diagnostics` at summary level and are cached for the current
inspection. Successful replacement resets selection and diagnostics; Details
failure stays local to the disclosure.

## UI1E implementation status

UI1E implements one selected-sequence MIDI export. The action is enabled only
when Core projects both `ready` and a non-null `export_capability`; Core remains
the final authority and revalidates during export. A directory-only
`NSOpenPanel` supplies `destination_folder`, while the sequence display name is
passed unchanged as `filename_stem`. Swift always requests
`generate_unique_name`, sends `operation_id: null`, and leaves extension,
filename validation, collision suffixes, and final path construction to Core.

The existing service handle performs one synchronous `export_sequence` call
off the main actor. The inspected project remains visible while exporting and
after bounded export failure. Success shows Core's output path, track counts,
untranslated-metadata count when nonzero, ordered export warning messages, and
a Reveal in Finder action. Destination-panel cancellation is a no-op;
in-progress Core cancellation remains unsupported. Authentic UI export
validation completed with the 18-sequence `newest STUFF baseline`: `Ode to
Clarke` exported twice with unique names, Finder reveal worked, cancellation
preserved state, and a non-exportable sibling remained disabled. Batch export,
audio recovery and the broader project-resurrection workflow remain deferred.
