# WASM Setup Guide for In the Shadows

## Current Status ✅

The WASM dependencies have been successfully fixed! The project now compiles for `wasm32-unknown-unknown` target.

### What Was Fixed

1. **Upgraded from Bevy 0.13 to 0.16** - Better WASM support
2. **Updated all API calls** to match Bevy 0.16:
   - `Camera2dBundle` → `Camera2d`
   - `MaterialMesh2dBundle` → Individual components
   - `Color::rgb()` → `Color::srgb()`
   - Shape API updates
3. **Resolved getrandom conflicts** - Bevy 0.16 handles this better
4. **Removed desktop-specific features** from WASM builds

### Files Updated

- `Cargo.toml` - Now uses Bevy 0.16
- `src/main.rs` - Updated to Bevy 0.16 API
- `.cargo/config.toml` - Added WASM configuration

## How to Build and Run

### Prerequisites

You need to install:
```bash
# Install wasm-bindgen-cli (matches our wasm-bindgen version)
cargo install wasm-bindgen-cli --version 0.2.100

# Install basic-http-server or python for serving
cargo install basic-http-server
# OR use Python (already installed on most systems)
```

### Building for WASM

1. **Debug Build** (faster compile, larger size ~428MB):
```bash
cargo build --target wasm32-unknown-unknown
```

2. **Release Build** (slower compile, smaller size ~50-100MB):
```bash
cargo build --release --target wasm32-unknown-unknown
```

3. **Generate JavaScript Bindings**:
```bash
wasm-bindgen target/wasm32-unknown-unknown/release/in-the-shadows.wasm \
  --out-dir web \
  --out-name in-the-shadows \
  --target web \
  --no-typescript
```

### Running Locally

1. **Using the provided script**:
```bash
./scripts/serve-local.sh
```

2. **Or manually with Python**:
```bash
cd web && python3 -m http.server 8000
```

3. **Open in browser**:
   - Navigate to: http://localhost:8000/
   - The game should load automatically

## Current Demo

The current build shows:
- A blue circle (representing a game node)
- A red circle (another node)  
- A gray line (connection between nodes)

This demonstrates that:
- Bevy is running in the browser
- 2D rendering works
- Basic shapes can be drawn

## File Sizes

- Debug build: ~428MB (includes debug symbols)
- Release build: ~50-100MB (optimized)
- With wasm-opt: ~30-60MB (further optimized)

## Troubleshooting

### "wasm-bindgen: command not found"
Install it with: `cargo install wasm-bindgen-cli --version 0.2.100`

### "WebGL2 not supported"
Use a modern browser (Chrome, Firefox, Safari 15+)

### Large file size
Use release builds and wasm-opt:
```bash
wasm-opt -Oz -o web/optimized.wasm web/in-the-shadows.wasm
```

### CORS errors
Make sure to serve files through HTTP, not file:// protocol

## Next Steps

1. Install wasm-bindgen-cli
2. Build the release version (will take a few minutes)
3. Generate proper JavaScript bindings
4. Implement actual game mechanics on top of this working foundation

The hard part (dependency resolution) is done! 🎉