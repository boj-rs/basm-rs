#!/bin/sh
cargo +nightly build --release
objcopy --strip-unneeded -j .init -j .text -j .rodata -O binary target/x86_64-unknown-linux-gnu/release/basm-rs basm.bin
python3 gen.py

