#!/bin/bash
# Build script for WASM

echo "Building Kseri for WASM..."

# Build the WASM package
wasm-pack build --target web --no-typescript

echo "Build complete! Run 'npm run serve' to start the development server."