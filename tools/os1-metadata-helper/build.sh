#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    exit 64
fi

output=$1
case "$output" in
    /*) ;;
    *) exit 64 ;;
esac

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
source_file="$script_dir/Sources/main.swift"
source_hash=$(shasum -a 256 "$source_file" | awk '{print $1}')

printf '%s\n' 'enum BuildIdentity {' "    static let sourceSHA256 = \"$source_hash\"" '}' > "$script_dir/Sources/BuildIdentity.generated.swift"
trap 'rm -f "$script_dir/Sources/BuildIdentity.generated.swift"' EXIT HUP INT TERM

swiftc -O -whole-module-optimization \
    -module-cache-path "${TMPDIR:-/tmp}/phoenix-os1-swift-module-cache" \
    "$source_file" \
    "$script_dir/Sources/BuildIdentity.generated.swift" \
    -o "$output"
chmod 700 "$output"
