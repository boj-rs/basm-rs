@echo off
cargo +nightly build --target x86_64-pc-windows-msvc --profile submit --config .cargo/config_submit.toml || goto :error
python scripts/static-pie-gen.py src/solution.rs x86_64-pc-windows-msvc target/x86_64-pc-windows-msvc/submit/basm.exe scripts/static-pie-stub-amd64.bin C scripts/static-pie-template-amd64.c || goto :error

:; exit 0
exit /b 0

:error
exit /b %errorlevel%