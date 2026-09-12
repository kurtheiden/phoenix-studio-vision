#!/bin/sh
set -eu

if [ "$#" -ne 1 ]; then
    exit 64
fi

script_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
python3 "$script_dir/Tests/test_helper.py" "$1"
