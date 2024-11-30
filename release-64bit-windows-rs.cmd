@echo off
setlocal
IF [%CARGO_TARGET_DIR%] == [] (
    SET CARGO_TARGET_DIR=target
)
cargo +nightly build --target x86_64-pc-windows-msvc --bin basm-submit --features=submit --release || goto :error
python scripts/static-pie-gen.py basm/src/solution.rs x86_64-pc-windows-msvc "%CARGO_TARGET_DIR%"/x86_64-pc-windows-msvc/release/basm-submit.exe static-pie-stub-amd64.bin Rust static-pie-template-amd64.rs || goto :error

:; exit 0
endlocal
exit /b 0

:error
endlocal
exit /b %errorlevel%