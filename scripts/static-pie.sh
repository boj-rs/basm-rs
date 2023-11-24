#!/bin/bash
set -e

target_name="$1"
shift
lang_name="$1"
shift
build_mode="$1"
shift

if [[ "$target_name" == "x86_64-unknown-linux-gnu" ]]; then
  stub="static-pie-stub-amd64.bin"
  if [[ "$lang_name" == "C" ]]; then
    if [[ "$*" == *"short"* ]]; then
      template="static-pie-template-amd64-short.c"
    else
      template="static-pie-template-amd64.c"
    fi
  elif [[ "$lang_name" == "Rust" ]]; then
    if [[ "$*" == *"short"* ]]; then
      template="static-pie-template-amd64-short.rs"
    else
      template="static-pie-template-amd64.rs"
    fi
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
elif [[ "$target_name" == "x86_64-pc-windows-msvc" ]]; then
  stub="static-pie-stub-amd64.bin"
  if [[ "$lang_name" == "C" ]]; then
    template="static-pie-template-amd64.c"
  elif [[ "$lang_name" == "Rust" ]]; then
    template="static-pie-template-amd64.rs"
  else
    >&2 echo "Language ${lang_name} is not supported for target ${target_name}"
    exit
  fi
else
  >&2 echo "Unknown target ${target_name}"
  exit
fi
if [[ "$build_mode" == "Debug" ]]; then
  build_mode_dir="debug"
elif [[ "$build_mode" == "Release" ]]; then
  build_mode_dir="release"
else
  >&2 echo "Unknown build mode ${build_mode}"
  exit
fi
>&2 echo "Building project for target ${target_name}, language ${lang_name}, build mode ${build_mode}"

binary_path=basm.bin
if [[ "$build_mode" == "Debug" ]]; then
  cargo +nightly build --target "$target_name" --bin basm-submit "$@"
else
  cargo +nightly build --target "$target_name" --bin basm-submit --release "$@"
fi

if [[ "$target_name" == "x86_64-pc-windows-msvc" ]]; then
  python3 scripts/static-pie-gen.py src/solution.rs "$target_name" target/"$target_name"/"$build_mode_dir"/basm-submit.exe scripts/"$stub" "$lang_name" scripts/"$template"
else
  cp target/"$target_name"/"$build_mode_dir"/basm-submit target/"$target_name"/"$build_mode_dir"/basm-submit-stripped
  objcopy --strip-all target/"$target_name"/"$build_mode_dir"/basm-submit-stripped
  objcopy --remove-section .eh_frame target/"$target_name"/"$build_mode_dir"/basm-submit-stripped
  python3 scripts/static-pie-gen.py src/solution.rs "$target_name" target/"$target_name"/"$build_mode_dir"/basm-submit-stripped scripts/"$stub" "$lang_name" scripts/"$template"
fi
