#!/bin/sh
set -e

binary_path=basm.bin
cargo +nightly build --release "$@"
python3 scripts/static-pie-gen.py target/x86_64-unknown-linux-gnu/release/basm scripts/static-pie-template.c
