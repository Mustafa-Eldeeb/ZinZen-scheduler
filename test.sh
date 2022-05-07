#!/bin/bash

set -euo pipefail

TARGET=wasm32-unknown-unknown
BINARY=target/$TARGET/release/output.wasm

# Build
cargo build --target $TARGET --release

# Process final binary
wasm-strip $BINARY
mkdir -p test
wasm-opt -o test/output.wasm -O3 $BINARY
ls -lh test/output.wasm

# Finally execute wasm inside node.js
node test/test.mjs