@echo off
cargo +nightly build --target x86_64-pc-windows-msvc --bin basm-submit --release || goto :error
python scripts/static-pie-gen.py src/solution.rs x86_64-pc-windows-msvc target/x86_64-pc-windows-msvc/release/basm-submit.exe scripts/static-pie-stub-amd64.bin Rust scripts/static-pie-template-amd64.rs || goto :error

:; exit 0
exit /b 0

:error
exit /b %errorlevel%