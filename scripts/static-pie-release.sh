#!/bin/bash
set -e

target_name="$1"
shift

if [[ "$target_name" == "x86_64-unknown-linux-gnu" ]]; then
  template="static-pie-template-amd64.c"
elif [[ "$target_name" == "i686-unknown-linux-gnu" ]]; then
  template="static-pie-template-i686.c"
else
  >&2 echo "Unknown target ${target_name}"
  exit
fi
>&2 echo "Building project for target ${target_name}"

binary_path=basm.bin
cargo +nightly build --target "$target_name" --release "$@"

cp target/"$target_name"/release/basm target/"$target_name"/release/basm_stripped
objcopy --strip-all target/"$target_name"/release/basm_stripped
objcopy --remove-section .eh_frame target/"$target_name"/release/basm_stripped
python3 scripts/static-pie-gen.py src/solution.rs target/"$target_name"/release/basm_stripped target/"$target_name"/release/basm_stripped.bin scripts/"$template"
