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
  elif [[ "$lang_name" == "CFnImpl" ]]; then
    template="static-pie-template-amd64-fn-impl.c"
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
elif [[ "$target_name" == "x86_64-pc-windows-msvc" ]] || [[ "$target_name" == "x86_64-pc-windows-gnu" ]]; then
  stub="static-pie-stub-amd64.bin"
  if [[ "$lang_name" == "C" ]]; then
    template="static-pie-template-amd64.c"
  elif [[ "$lang_name" == "CFnImpl" ]]; then
    template="static-pie-template-amd64-fn-impl.c"
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

if [[ "$target_name" == "x86_64-unknown-linux-gnu" && "$*" == *"short"* ]]; then
  target_name_cargo=".cargo/x86_64-unknown-linux-gnu-short.json"
  target_name="x86_64-unknown-linux-gnu-short"
  extra_config='-Zbuild-std=core,compiler_builtins,alloc -Zbuild-std-features=compiler-builtins-mem'
else
  target_name_cargo="$target_name"
  extra_config=""
fi

if [[ "$lang_name" == "CFnImpl" ]]; then
  lang_name="C"
fi

>&2 echo "Building project for target ${target_name}, language ${lang_name}, build mode ${build_mode}"

if [[ "$build_mode" == "Debug" ]]; then
  cargo +nightly build $extra_config --target "$target_name_cargo" --bin basm-submit --features=submit "$@"
else
  cargo +nightly build $extra_config --target "$target_name_cargo" --bin basm-submit --features=submit --release "$@"
fi

if [[ "$target_name" == "x86_64-pc-windows-msvc" ]] || [[ "$target_name" == "x86_64-pc-windows-gnu" ]]; then
  python3 scripts/static-pie-gen.py basm/ "$target_name" target/"$target_name"/"$build_mode_dir"/basm-submit.exe "$stub" "$lang_name" "$template"
else
  cp target/"$target_name"/"$build_mode_dir"/basm-submit target/"$target_name"/"$build_mode_dir"/basm-submit-stripped
  objcopy --strip-all target/"$target_name"/"$build_mode_dir"/basm-submit-stripped
  objcopy --remove-section .eh_frame --remove-section .gcc_except_table --remove-section .gnu.hash target/"$target_name"/"$build_mode_dir"/basm-submit-stripped
  python3 scripts/static-pie-gen.py basm/ "$target_name" target/"$target_name"/"$build_mode_dir"/basm-submit-stripped "$stub" "$lang_name" "$template"
fi
