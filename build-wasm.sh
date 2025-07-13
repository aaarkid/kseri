#!/bin/bash
set -e

echo "Building WASM target..."

# Build the WASM binary
cargo build --target wasm32-unknown-unknown --release

# Create web directory if it doesn't exist
mkdir -p web

# Run wasm-bindgen to generate JS bindings
wasm-bindgen --out-dir web \
    --target web \
    --no-typescript \
    target/wasm32-unknown-unknown/release/kseri.wasm

echo "WASM build complete! Files are in the 'web' directory."
echo "To serve locally, run: python3 serve.py"