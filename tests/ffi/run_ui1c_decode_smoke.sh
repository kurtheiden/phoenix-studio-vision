#!/bin/sh
set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/phoenix-ui1c.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT

cp "$ROOT_DIR/tests/ffi/ui1c_decode_smoke.swift" "$TMP_DIR/main.swift"
swiftc -O -module-cache-path "$TMP_DIR/module-cache" \
  "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/ProjectInspectionModels.swift" \
  "$TMP_DIR/main.swift" \
  -o "$TMP_DIR/ui1c_decode_smoke"
"$TMP_DIR/ui1c_decode_smoke"
