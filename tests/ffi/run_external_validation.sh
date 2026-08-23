#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)

echo "Phoenix external bridge validation"
echo "Repository: $ROOT"
echo "Host architecture: $(uname -m)"

if ! command -v clang >/dev/null 2>&1; then
    echo "UI0G failure: clang is required for the C smoke" >&2
    exit 1
fi
if ! command -v swiftc >/dev/null 2>&1; then
    echo "UI0G failure: swiftc is required for the Swift smoke" >&2
    exit 1
fi

echo "Swift toolchain:"
swiftc --version | head -1

"$SCRIPT_DIR/run_c_smoke.sh"
"$SCRIPT_DIR/run_swift_smoke.sh"

echo "Phoenix external bridge validation passed"
