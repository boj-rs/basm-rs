@echo off
setlocal
IF [%CARGO_TARGET_DIR%] == [] (
    SET CARGO_TARGET_DIR=target
)
cargo +nightly build --target x86_64-pc-windows-msvc || goto :error
python scripts/static-pie-gen.py basm/src/solution.rs x86_64-pc-windows-msvc "%CARGO_TARGET_DIR%"/x86_64-pc-windows-msvc/debug/basm.exe scripts/static-pie-stub-amd64.bin C static-pie-template-amd64.c || goto :error

:; exit 0
endlocal
exit /b 0

:error
endlocal
exit /b %errorlevel%