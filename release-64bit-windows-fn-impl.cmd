@echo off
cargo +nightly build --target x86_64-pc-windows-msvc --bin basm-submit --features=submit --release || goto :error
python scripts/static-pie-gen.py basm/src/solution.rs x86_64-pc-windows-msvc target/x86_64-pc-windows-msvc/release/basm-submit.exe static-pie-stub-amd64.bin C static-pie-template-amd64-fn-impl.c || goto :error

:; exit 0
exit /b 0

:error
exit /b %errorlevel%