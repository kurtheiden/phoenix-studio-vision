# UI0G external bridge validation

UI0G validates that Phoenix can be consumed outside the Rust crate. It begins
with a C program compiled against the version-controlled `include/phoenix.h`
and linked to the release `target/release/libphoenix.a`.

## UI0G1 — external C linkage and lifecycle smoke

Run:

```text
tests/ffi/run_c_smoke.sh
```

The runner builds the release static library, reports the current macOS host
architecture, compiles `tests/ffi/c_smoke.c` as C11 with the system `clang`,
links the archive with the native libraries reported by Rust (`-lSystem -lc
-lm` on the current host; Apple clang supplies `libSystem` itself), and runs
the executable from a temporary directory.

The smoke proves ABI version discovery, service creation, `get_api_info` JSON
success, Rust-owned response release, successful destruction, stale-handle
`phoenix_call` returning status 0 plus `invalid_handle` transport JSON, and
stale-handle destruction returning `PHOENIX_STATUS_INVALID_HANDLE`. It checks
selected stable JSON markers rather than freezing object-key order or complete
response bytes.

This validates the current host architecture only. It does not prove universal
packaging, Swift interoperability, GUI integration, or native application
behavior.

## Remaining UI0G work

UI0G2 adds the smallest command-line Swift importer/linkage smoke using the
repository-owned `include/module.modulemap`. UI0G3 will freeze the durable
cross-language fixtures and document CI/tool availability. Xcode,
SwiftUI/AppKit, XCFrameworks, universal packaging, signing, and the desktop
application remain deferred to UI1 or later.

## UI0G2 — Swift command-line interoperability smoke

Run:

```text
tests/ffi/run_swift_smoke.sh
```

The Swift program imports `Phoenix`, calls the same ABI lifecycle as UI0G1,
copies response bytes into Swift-owned `Data` before calling
`phoenix_free_buffer`, decodes the copy with Foundation, and verifies both
success and stale-handle transport JSON. The executable is temporary and the
smoke validates the current host architecture only.

## UI0G3 — aggregate validation and fixture policy

Run the complete bridge-facing validation with:

```text
tests/ffi/run_external_validation.sh
```

The aggregate runner requires both `clang` and `swiftc`, reports the host and
Swift toolchain, and runs UI0G1 and UI0G2 without silently skipping either one.
Each smoke contains the canonical `get_api_info` request and semantic checks
for the stable envelope, application contract version, response ownership,
and stale-handle behavior. This deliberately avoids a second hand-maintained
JSON schema or raw-byte fixtures: key ordering, whitespace, display prose,
technical text, handles, session IDs, and temporary paths are not contracts.

The durable cross-language values frozen by these checks are ABI version,
status meanings, operation spelling, envelope discriminators, contract
version, and invalid-handle transport identity. The existing Rust transport
tests remain the detailed source for enum strings, explicit-null fields,
AppError identity, contract-version errors, and export/diagnostics semantics.

No hosted CI workflow is added in UI0G3 because the repository has no existing
CI framework and the Swift/macOS gate is host-specific. The aggregate command
is the reproducible local handoff gate for UI1; a later CI decision can add a
macOS runner without changing the ABI or fixtures.

## UI0G status and limits

UI0G1, UI0G2, and UI0G3 are complete for the current arm64 macOS host. This
does not claim Intel validation, universal binaries, Linux/Windows support,
deployment-target coverage, Xcode application linkage, XCFramework support,
signing, notarization, or native UI behavior.
