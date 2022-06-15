#!/bin/sh

# To override rustflags in .cargo/config.toml
export RUSTFLAGS=

cargo test --lib -- --test-threads 1 "$@"
