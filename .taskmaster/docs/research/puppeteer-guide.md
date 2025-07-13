# Puppeteer Testing Guide for Greek Political Compass

## Overview
This guide explains how Puppeteer was used to test and verify the Greek Political Compass application on mobile devices, specifically emulating an iPhone 12 Pro.

## Setup

### Dependencies
```bash
npm install puppeteer
```

### Basic Test Script Structure
```javascript
const puppeteer = require('puppeteer');
const path = require('path');

(async () => {
    const browser = await puppeteer.launch({
        headless: false,  // Show browser window for visual verification
        args: ['--window-size=400,900']  // Set browser window size
    });

    const page = await browser.newPage();
    
    // Emulate iPhone 12 Pro
    await page.emulate({
        viewport: {
            width: 390,
            height: 844,
            deviceScaleFactor: 3,
            isMobile: true,
            hasTouch: true
        },
        userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1'
    });

    // Load the local HTML file
    const filePath = `file://${path.resolve(__dirname, 'index.html')}`;
    await page.goto(filePath);
    
    // Keep browser open for manual inspection
    await new Promise(() => {});
})();
```

## Key Testing Patterns Used

### 1. Mobile Emulation
```javascript
await page.emulate({
    viewport: {
        width: 390,
        height: 844,
        deviceScaleFactor: 3,
        isMobile: true,
        hasTouch: true
    }
});
```
This accurately emulates the iPhone 12 Pro viewport and touch capabilities.

### 2. Quick Timeout for Rapid Testing
```javascript
await page.waitForTimeout(5000);  // 5 second timeout as requested
```
Used shorter timeouts for faster feedback during development.

### 3. Interactive Testing
```javascript
await page.click('#showHeatmap');  // Toggle controls
await page.click('.party[data-party="ΝΔ"]');  // Select parties
```
Simulated user interactions to verify toggle functionality.

### 4. Visual Verification Script
```javascript
const puppeteer = require('puppeteer');
const path = require('path');

(async () => {
    const browser = await puppeteer.launch({
        headless: false,
        args: ['--window-size=400,900']
    });

    const page = await browser.newPage();
    
    await page.emulate({
        viewport: {
            width: 390,
            height: 844,
            deviceScaleFactor: 3,
            isMobile: true,
            hasTouch: true
        },
        userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.0 Mobile/15E148 Safari/604.1'
    });

    const filePath = `file://${path.resolve(__dirname, 'index.html')}`;
    await page.goto(filePath);
    
    // Wait for initial render
    await page.waitForTimeout(2000);
    
    // Test different visualizations
    console.log('Testing party size modes...');
    await page.click('input[value="votes"]');
    await page.waitForTimeout(1000);
    
    await page.click('input[value="efficiency"]');
    await page.waitForTimeout(1000);
    
    await page.click('input[value="default"]');
    await page.waitForTimeout(1000);
    
    // Test toggles
    console.log('Testing visualization toggles...');
    await page.click('#showPolicies');
    await page.waitForTimeout(1000);
    
    // Keep open for manual inspection
    console.log('Browser ready for manual inspection...');
    await new Promise(() => {});
})();
```

## Testing Workflow

### 1. Local File Serving
Instead of running a separate HTTP server, Puppeteer can directly load local files:
```javascript
const filePath = `file://${path.resolve(__dirname, 'index.html')}`;
await page.goto(filePath);
```

### 2. Responsive Design Verification
Test different mobile viewports:
```javascript
// iPhone 12 Pro
await page.setViewport({ width: 390, height: 844 });

// iPhone SE
await page.setViewport({ width: 375, height: 667 });

// iPad
await page.setViewport({ width: 768, height: 1024 });
```

### 3. Control Panel Testing
Verify mobile-specific fixes:
```javascript
// Check if panels are visible at bottom on mobile
const analysisPanel = await page.$('#infoPanel');
const boundingBox = await analysisPanel.boundingBox();
console.log('Analysis panel position:', boundingBox);
```

### 4. Screenshot Capture
```javascript
await page.screenshot({ 
    path: 'mobile-view.png',
    fullPage: true 
});
```

## Minimal Test Script
Here's the minimal script used most frequently:
```javascript
const puppeteer = require('puppeteer');
const path = require('path');

(async () => {
    const browser = await puppeteer.launch({
        headless: false,
        args: ['--window-size=400,900']
    });

    const page = await browser.newPage();
    
    await page.emulate({
        viewport: {
            width: 390,
            height: 844,
            deviceScaleFactor: 3,
            isMobile: true,
            hasTouch: true
        }
    });

    await page.goto(`file://${path.resolve(__dirname, 'index.html')}`);
    
    await page.waitForTimeout(5000);
    console.log('Ready for inspection');
    
    await new Promise(() => {});
})();
```

## Benefits of This Approach

1. **No HTTP Server Required**: Direct file loading eliminates server setup
2. **Visual Verification**: Non-headless mode allows immediate visual feedback
3. **Accurate Mobile Emulation**: iPhone 12 Pro specs ensure accurate testing
4. **Quick Iteration**: 5-second timeout provides rapid feedback
5. **Manual Inspection**: Keeping browser open allows thorough testing

## Running Tests
```bash
node test-mobile.js
```

The browser stays open for manual interaction and verification of:
- Touch interactions
- Control panel responsiveness
- Party selection
- Visualization toggles
- Label positioning
- Overall mobile UX

This approach provided immediate visual feedback during development, making it easy to verify fixes and improvements in real-time.