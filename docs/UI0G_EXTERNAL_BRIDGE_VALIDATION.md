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
