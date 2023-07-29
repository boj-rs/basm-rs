@echo off
cargo +nightly build --target x86_64-pc-windows-msvc --release
python scripts/static-pie-gen.py src/solution.rs x86_64-pc-windows-msvc target/x86_64-pc-windows-msvc/release/basm.exe scripts/static-pie-stub-amd64.bin C scripts/static-pie-template-amd64.c