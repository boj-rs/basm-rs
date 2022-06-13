#!/bin/sh

# To override rustflags in .cargo/config.toml
export RUSTFLAGS=

cargo test -- --test-threads 1
