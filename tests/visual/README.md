# Visual Tests

This directory contains Puppeteer-based visual tests for the Kseri card rendering system.

## Test Files

- `test-card-rendering.js` - Main visual rendering test that verifies:
  - Canvas initialization
  - WebGL context availability
  - Bevy initialization (canvas resize)
  - Asset loading (card PNG files)
  - Takes screenshots for visual verification

- `test-pm2-deployment.js` - PM2 deployment test that verifies:
  - PM2 installation
  - Server process status
  - Server accessibility
  - CORS headers configuration
  - Server logs

## Running Tests

```bash
# Run all visual tests
npm test

# Run individual tests
node tests/visual/test-card-rendering.js
node tests/visual/test-pm2-deployment.js
```

## Test Output

- Screenshots are saved to `screenshots/` directory
- Exit code 0 indicates success
- Exit code 1 indicates failure

## Requirements

- Node.js with Puppeteer installed
- PM2 installed globally (for deployment test)
- Development server running on port 8001