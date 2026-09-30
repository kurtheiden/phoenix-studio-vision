#!/bin/sh
set -eu
ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/../.." && pwd)
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/phoenix-v1a.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT HUP INT TERM
TASK_TARGET_DIR=${CARGO_TARGET_DIR:-$TMP_DIR/target}
cargo build --manifest-path "$ROOT_DIR/Cargo.toml" --release --locked --target-dir "$TASK_TARGET_DIR"
swiftc -parse-as-library -swift-version 5 -strict-concurrency=complete -warnings-as-errors \
  -I "$ROOT_DIR/include" -module-cache-path "$TMP_DIR/module-cache" \
  "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/ProjectInspectionModels.swift" \
  "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/ProjectOpenPanel.swift" \
  "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/ExportDestinationPanel.swift" \
  "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/PhoenixCore.swift" \
  "$ROOT_DIR/macos/PhoenixApp/PhoenixApp/AppModel.swift" \
  "$ROOT_DIR/tests/ffi/v1a_workflow_smoke.swift" \
  "$TASK_TARGET_DIR/release/libphoenix.a" -o "$TMP_DIR/v1a_workflow_smoke"
"$TMP_DIR/v1a_workflow_smoke"
