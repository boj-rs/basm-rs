#!/bin/sh
cargo build --release
objcopy --strip-unneeded -j .init -j .text -j .rodata -O binary target/release/basm-rs basm.bin
python3 gen.py

