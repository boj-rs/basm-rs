>&2 echo "Building project for target wasm32-unknown-unknown, language JavaScript, build mode Release"
cargo +nightly build --target wasm32-unknown-unknown --bin=basm-submit --release "$@"
python scripts/wasm-gen.py scripts/wasm-template.js