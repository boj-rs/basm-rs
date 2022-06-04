#!/bin/sh
cargo +nightly build --release "$@"
python3 remove-got.py -o no-got target/x86_64-unknown-linux-gnu/release/basm-rs
objcopy --strip-unneeded -j .init -j .text -j .rodata -O binary no-got basm.bin
python3 gen-asm.py

