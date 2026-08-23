#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/phoenix-ui0g2.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT HUP INT TERM

ARCH=$(uname -m)
echo "UI0G2 host architecture: $ARCH"
swiftc --version | head -1

cargo build --release --locked

for required in \
    "$ROOT/target/release/libphoenix.a" \
    "$ROOT/include/phoenix.h" \
    "$ROOT/include/module.modulemap" \
    "$SCRIPT_DIR/swift_smoke.swift"
do
    if [ ! -f "$required" ]; then
        echo "UI0G2 failure: missing $required" >&2
        exit 1
    fi
done
if ! command -v swiftc >/dev/null 2>&1; then
    echo "UI0G2 failure: swiftc is required" >&2
    exit 1
fi

swiftc -I "$ROOT/include" \
    -module-cache-path "$TMP_DIR/module-cache" \
    "$SCRIPT_DIR/swift_smoke.swift" \
    "$ROOT/target/release/libphoenix.a" \
    -o "$TMP_DIR/phoenix-ui0g2-swift-smoke"

"$TMP_DIR/phoenix-ui0g2-swift-smoke"
echo "UI0G2 Swift interoperability smoke passed ($ARCH)"
