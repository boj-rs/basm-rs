#!/bin/sh
binary_path=basm.bin
cargo +nightly build --release "$@"
python3 scripts/remove-got.py -o no-got target/x86_64-unknown-linux-gnu/release/basm-rs
objcopy --strip-unneeded -j .init -j .text -j .rodata -O binary no-got $binary_path
if [ "$(basename $0)" = 'release-asm.sh' ]; then
    template_path=scripts/template.asm
else
    template_path=scripts/template.c
fi
python3 scripts/gen.py $binary_path $template_path
