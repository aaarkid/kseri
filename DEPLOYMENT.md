# Kseri Card Rendering - Deployment & Testing Guide

## Overview

This guide covers how to deploy and test the Kseri card rendering system using PM2 for process management and Puppeteer for automated testing.

## Prerequisites

- Node.js and npm installed
- PM2 installed globally: `npm install -g pm2`
- Puppeteer installed: `npm install puppeteer`
- Rust toolchain with wasm32-unknown-unknown target
- wasm-pack installed

## Deployment with PM2

### Starting the Development Server

The project uses PM2 to manage a Python HTTP server that serves the WASM application with proper CORS headers.

```bash
# Start the server
npm run serve
# or
pm2 start ecosystem.config.js

# Check status
pm2 status

# View logs
pm2 logs kseri-dev-server

# Stop the server
pm2 stop kseri-dev-server

# Restart the server
pm2 restart kseri-dev-server
```

The server runs on **port 8001** by default.

### PM2 Configuration

The `ecosystem.config.js` file configures:
- Python 3 HTTP server (`serve.py`)
- CORS headers for SharedArrayBuffer support
- Log file locations in `./logs/`
- Port 8001 via environment variable

## Building the WASM Application

```bash
# Build WASM (this may take several minutes)
npm run build
# or
./build-wasm.sh
```

The build process:
1. Compiles Rust code to WASM using wasm-pack
2. Generates JavaScript bindings in `pkg/` directory
3. Optimizes the WASM binary with wasm-opt
4. Creates `serve.py` script for development server

## Testing with Puppeteer

### Automated Visual Testing

```bash
# Run the comprehensive test
npm run test
# or
node test-render.js
```

This test:
- Loads the page in a headless browser
- Waits for WASM initialization
- Checks canvas dimensions and WebGL context
- Takes screenshots for visual verification
- Samples pixels to detect rendering
- Monitors asset loading

### Test Files

1. **test-render.js** - Comprehensive automated test
   - Checks canvas initialization
   - Verifies WebGL context
   - Monitors asset loading
   - Takes screenshots

2. **test-debug.js** - Debug test with console capture
   - Captures all console output
   - Shows initialization status
   - Helpful for troubleshooting

3. **test-minimal.js** - Interactive browser test
   - Opens visible browser window
   - Allows manual inspection
   - Keeps browser open for debugging

### Running Tests

```bash
# Full development cycle
npm run dev  # Builds, serves, and tests

# Individual test commands
node test-render.js    # Automated test
node test-debug.js     # Debug with console output
node test-minimal.js   # Interactive browser test

# Visual test with screenshot viewer
npm run test:visual
```

## Troubleshooting

### Black Canvas Issues

If you see a black canvas:

1. **Check WASM Loading**
   - Open browser DevTools
   - Look for WASM file loading in Network tab
   - Check Console for errors

2. **Verify Build**
   - Ensure `pkg/kseri_bg.wasm` exists and is ~40MB
   - Check that `pkg/kseri.js` was generated
   - Rebuild if file sizes seem wrong

3. **Asset Loading**
   - Verify card PNG files are being loaded
   - Check Network tab for 404 errors
   - Ensure assets directory is accessible

4. **CORS Headers**
   - Confirm server includes proper headers
   - Check for SharedArrayBuffer warnings

### Port Conflicts

If port 8001 is already in use:

```bash
# Find process using port
lsof -i :8001

# Kill conflicting process
kill <PID>

# Or change port in ecosystem.config.js
```

### PM2 Issues

```bash
# If PM2 process is errored
pm2 delete kseri-dev-server
pm2 start ecosystem.config.js

# Clear PM2 logs
pm2 flush

# Monitor in real-time
pm2 monit
```

## Development Workflow

1. **Make code changes**
2. **Build WASM**: `npm run build`
3. **Restart server**: `npm run restart`
4. **Test changes**: `npm run test`
5. **Check screenshots**: `open test-screenshot.png`

## Files and Directories

```
kseri-card-rendering/
├── pkg/                    # WASM build output
│   ├── kseri_bg.wasm      # WASM binary (~40MB)
│   └── kseri.js           # JavaScript bindings
├── assets/
│   └── cards/individual/   # Card PNG files
├── logs/                   # PM2 log files
├── ecosystem.config.js     # PM2 configuration
├── serve.py               # Python dev server
├── test.html              # Main test page
├── test-*.js              # Puppeteer test scripts
└── package.json           # NPM scripts
```

## NPM Scripts Reference

- `npm run build` - Build WASM package
- `npm run serve` - Start PM2 server
- `npm run stop` - Stop PM2 server
- `npm run restart` - Restart PM2 server
- `npm run logs` - View server logs
- `npm run test` - Run Puppeteer tests
- `npm run test:visual` - Test and open screenshot
- `npm run dev` - Full build, serve, and test cycle

## Expected Results

When working correctly, you should see:
- Canvas resized to 800x600 (from default 300x150)
- "Kseri Game" text at the top of the canvas
- Cards rendered in different positions:
  - Player hands (top and bottom)
  - Table pile (center)
  - Deck (left side)
- Proper WebGL context initialized
- All card textures loaded (53 PNG files)