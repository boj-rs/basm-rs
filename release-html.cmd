@echo off
cargo +nightly build --target wasm32-unknown-unknown --bin=basm-submit --features=submit --release || goto :error
python scripts/wasm-gen.py wasm-template.html HTML || goto :error

:; exit 0
exit /b 0

:error
exit /b %errorlevel%