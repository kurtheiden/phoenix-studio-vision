#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)
TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/phoenix-ui0g1.XXXXXX")
trap 'rm -rf "$TMP_DIR"' EXIT HUP INT TERM

ARCH=$(uname -m)
echo "UI0G1 host architecture: $ARCH"

cargo build --release --locked

if [ ! -f "$ROOT/target/release/libphoenix.a" ]; then
    echo "UI0G1 failure: release static library is missing" >&2
    exit 1
fi
if [ ! -f "$ROOT/include/phoenix.h" ]; then
    echo "UI0G1 failure: public header is missing" >&2
    exit 1
fi
if ! command -v clang >/dev/null 2>&1; then
    echo "UI0G1 failure: clang is required" >&2
    exit 1
fi

# cargo rustc --release --lib -- --print native-static-libs reports System,
# libc, and libm for libphoenix.a. Apple clang supplies libSystem itself, so
# explicitly naming libc and libm is the smallest successful link command.
clang -std=c11 -Wall -Wextra -Werror \
    -I "$ROOT/include" \
    "$SCRIPT_DIR/c_smoke.c" \
    "$ROOT/target/release/libphoenix.a" \
    -lc -lm \
    -o "$TMP_DIR/phoenix-ui0g1-c-smoke"

"$TMP_DIR/phoenix-ui0g1-c-smoke"
echo "UI0G1 external C linkage smoke passed ($ARCH)"
