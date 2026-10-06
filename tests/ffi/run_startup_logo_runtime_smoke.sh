#!/bin/sh
set -eu
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 /path/to/built/Phoenix.app" >&2
    exit 2
fi
ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/phoenix-logo-runtime.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT
swiftc -parse-as-library -strict-concurrency=complete -warnings-as-errors \
    -module-cache-path "$TMP_DIR/module-cache" \
    "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/ProjectInspectionModels.swift" \
    "$ROOT_DIR/tests/ffi/startup_logo_runtime_smoke.swift" \
    -o "$TMP_DIR/logo-runtime-smoke"
"$TMP_DIR/logo-runtime-smoke" "$1"
