#!/bin/sh
set -e

binary_path=basm.bin
cargo +nightly build --release "$@"
cp target/x86_64-unknown-linux-gnu/release/basm target/x86_64-unknown-linux-gnu/release/basm_stripped
objcopy --strip-all target/x86_64-unknown-linux-gnu/release/basm_stripped
python3 scripts/static-pie-gen.py target/x86_64-unknown-linux-gnu/release/basm_stripped scripts/static-pie-template.c
