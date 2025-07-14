# Visual Tests Guide

This directory contains visual tests for the Kseri card rendering system. These tests use Puppeteer to verify that the WASM build loads correctly and renders as expected.

## Test Files Overview

### test-final.js + final-test.html
**Purpose**: The primary working test that provides real-time status monitoring
- **Key Features**:
  - Opens browser window and keeps it open for manual inspection
  - Shows real-time WASM initialization status
  - Monitors canvas size changes (indicates Bevy initialization)
  - Tracks asset loading via PerformanceObserver
  - Takes screenshots for documentation
- **Use Case**: Run this when you need to debug rendering issues or verify the full initialization process
- **Command**: `node tests/visual/test-final.js`

### test-card-rendering.js
**Purpose**: Automated visual test for CI/CD pipelines
- **Key Features**:
  - Runs in headless mode
  - Captures screenshots at different stages
  - Verifies canvas rendering
  - Checks for WASM initialization messages
- **Use Case**: Automated testing to ensure rendering works after changes
- **Command**: `npm run test`

### test-pm2-deployment.js
**Purpose**: Verifies PM2 server is running correctly
- **Key Features**:
  - Checks if PM2 process is active
  - Verifies server responds on port 8001
  - Validates CORS headers are set correctly
- **Use Case**: Ensure deployment infrastructure is working
- **Command**: `npm run test:pm2`

### test-simple.js
**Purpose**: Minimal test for quick verification
- **Key Features**:
  - Basic page load check
  - Console message capture
  - Quick screenshot
- **Use Case**: Fast sanity check during development

### test-visual-interactive.js
**Purpose**: Interactive visual debugging with extended wait times
- **Key Features**:
  - Longer timeouts for slow initialization
  - Interactive browser window
  - Detailed console logging
- **Use Case**: Deep debugging of rendering issues

### view-screenshot.html
**Purpose**: Simple HTML page to view captured screenshots
- **Key Features**:
  - Displays test screenshots in browser
  - Useful for comparing visual output over time
- **Use Case**: Review test results without external image viewer

## Running Tests

### Prerequisites
1. Build WASM first: `npm run build`
2. Start PM2 server: `npm run serve`

### Test Commands
```bash
# Run main visual test (recommended)
node tests/visual/test-final.js

# Run all tests
npm run test:all

# Run specific test
node tests/visual/test-simple.js
```

## Debugging Tips

1. **Black Canvas Issues**:
   - Run `test-final.js` - it shows real-time status
   - Check if WASM initialized (status should show "WASM: Initialized successfully!")
   - Verify canvas size changed from default 300x150
   - Look for asset loading in the status display

2. **WASM Loading Failures**:
   - Check browser console in final-test.html
   - Verify CORS headers with test-pm2-deployment.js
   - Ensure ./web/ directory has WASM files

3. **Asset Loading Issues**:
   - final-test.html tracks PNG loading in real-time
   - Check network tab in browser DevTools
   - Verify assets exist in web/assets/

## Test Output

Tests generate various outputs:
- `final-status.png` - Screenshot from test-final.js
- `screenshot.png` - Screenshot from test-card-rendering.js
- Console logs with status indicators:
  - 🎮 Game messages
  - ✅ Success
  - ⚠️ Warnings
  - ❌ Failures

## Why test-final.js Works Best

The test-final.js + final-test.html combination has proven most reliable because:
1. It provides real-time feedback during initialization
2. Keeps the browser open for manual inspection
3. Tracks multiple metrics (WASM, canvas, assets)
4. Uses proper timeouts for WASM initialization
5. Shows clear success/failure indicators

This test was specifically restored after other tests failed to provide adequate debugging information.