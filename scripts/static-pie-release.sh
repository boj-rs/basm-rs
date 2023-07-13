#!/bin/bash
set -e

target_name="$1"
shift
lang_name="$1"
shift

if [[ "$target_name" == "x86_64-unknown-linux-gnu" ]]; then
  stub="static-pie-stub-amd64.bin"
  if [[ "$lang_name" == "C" ]]; then
    template="static-pie-template-amd64.c"
  elif [[ "$lang_name" == "Rust" ]]; then
    template="static-pie-template-amd64.rs"
  else
    >&2 echo "Language ${lang_name} is not supported for target ${target_name}"
    exit
  fi
elif [[ "$target_name" == "i686-unknown-linux-gnu" ]]; then
  stub="static-pie-stub-i686.bin"
  if [[ "$lang_name" == "C" ]]; then
    template="static-pie-template-i686.c"
  else
    >&2 echo "Language ${lang_name} is not supported for target ${target_name}"
    exit
  fi
else
  >&2 echo "Unknown target ${target_name}"
  exit
fi
>&2 echo "Building project for target ${target_name}, language ${lang_name}"

binary_path=basm.bin
cargo +nightly build --target "$target_name" --release "$@"

cp target/"$target_name"/release/basm target/"$target_name"/release/basm_stripped
objcopy --strip-all target/"$target_name"/release/basm_stripped
objcopy --remove-section .eh_frame target/"$target_name"/release/basm_stripped
python3 scripts/static-pie-gen.py src/solution.rs target/"$target_name"/release/basm_stripped scripts/"$stub" "$lang_name" scripts/"$template"
