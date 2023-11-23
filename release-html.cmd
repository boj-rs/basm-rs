@echo off
cargo +nightly build --target wasm32-unknown-unknown --bin=basm-submit --release || goto :error
python scripts/wasm-gen.py scripts/wasm-template.html || goto :error

:; exit 0
exit /b 0

:error
exit /b %errorlevel%