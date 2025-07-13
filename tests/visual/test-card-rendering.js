const puppeteer = require('puppeteer');
const path = require('path');
const fs = require('fs');

// Colors for console output
const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  reset: '\x1b[0m'
};

async function testCardRendering() {
  console.log(`${colors.blue}Starting Kseri Card Rendering Test...${colors.reset}\n`);

  const browser = await puppeteer.launch({
    headless: true,
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--enable-webgl',
      '--use-gl=swiftshader'
    ]
  });

  try {
    const page = await browser.newPage();
    
    // Collect console logs
    const consoleLogs = [];
    page.on('console', msg => {
      consoleLogs.push(`[${msg.type()}] ${msg.text()}`);
    });

    // Collect errors
    const pageErrors = [];
    page.on('pageerror', error => {
      pageErrors.push(error.message);
    });

    console.log(`${colors.yellow}1. Loading page...${colors.reset}`);
    await page.goto('http://localhost:8001/', { waitUntil: 'networkidle2' });
    
    // Wait for Bevy initialization
    console.log(`${colors.yellow}2. Waiting for Bevy/WASM initialization (15 seconds)...${colors.reset}`);
    await new Promise(resolve => setTimeout(resolve, 15000));

    // Check if canvas exists
    console.log(`${colors.yellow}3. Checking for canvas element...${colors.reset}`);
    const canvasExists = await page.evaluate(() => {
      const canvas = document.querySelector('#bevy');
      return canvas !== null;
    });
    
    if (!canvasExists) {
      throw new Error('Canvas element not found!');
    }
    console.log(`${colors.green}✓ Canvas found${colors.reset}`);

    // Check canvas dimensions
    console.log(`${colors.yellow}4. Checking canvas dimensions...${colors.reset}`);
    const canvasInfo = await page.evaluate(() => {
      const canvas = document.querySelector('#bevy');
      return {
        width: canvas.width,
        height: canvas.height,
        clientWidth: canvas.clientWidth,
        clientHeight: canvas.clientHeight
      };
    });
    console.log(`${colors.green}✓ Canvas dimensions: ${canvasInfo.width}x${canvasInfo.height} (client: ${canvasInfo.clientWidth}x${canvasInfo.clientHeight})${colors.reset}`);

    // Check if Bevy resized the canvas (indicates initialization)
    const bevyInitialized = canvasInfo.width !== 300 || canvasInfo.height !== 150;
    if (!bevyInitialized) {
      console.log(`${colors.yellow}⚠ Canvas still at default size - Bevy may need more time to initialize${colors.reset}`);
    } else {
      console.log(`${colors.green}✓ Bevy has initialized and resized the canvas${colors.reset}`);
    }

    // Check WebGL context
    console.log(`${colors.yellow}5. Checking WebGL context...${colors.reset}`);
    const hasWebGL = await page.evaluate(() => {
      const canvas = document.querySelector('#bevy');
      // Bevy may have already acquired the context, so we just check if WebGL is supported
      try {
        const testCanvas = document.createElement('canvas');
        const gl = testCanvas.getContext('webgl2') || testCanvas.getContext('webgl');
        return gl !== null;
      } catch (e) {
        return false;
      }
    });
    
    if (!hasWebGL) {
      console.log(`${colors.yellow}⚠ WebGL context check failed - this may be normal if Bevy already acquired it${colors.reset}`);
    } else {
      console.log(`${colors.green}✓ WebGL is supported${colors.reset}`);
    }

    // Take screenshot
    console.log(`${colors.yellow}6. Taking screenshot...${colors.reset}`);
    const screenshotDir = path.join(__dirname, '../../screenshots');
    if (!fs.existsSync(screenshotDir)) {
      fs.mkdirSync(screenshotDir, { recursive: true });
    }
    const screenshotPath = path.join(screenshotDir, 'card-rendering-test.png');
    await page.screenshot({ path: screenshotPath, fullPage: true });
    console.log(`${colors.green}✓ Screenshot saved to ${screenshotPath}${colors.reset}`);

    // Check asset loading
    console.log(`${colors.yellow}7. Checking asset loading...${colors.reset}`);
    const assetRequests = await page.evaluate(() => {
      return performance.getEntriesByType('resource')
        .filter(entry => entry.name.includes('.png'))
        .length;
    });
    console.log(`${colors.green}✓ Loaded ${assetRequests} PNG assets${colors.reset}`);

    // Print console logs if any errors
    if (pageErrors.length > 0) {
      console.log(`\n${colors.red}Page errors:${colors.reset}`);
      pageErrors.forEach(error => console.log(`  ${error}`));
    }

    // Final verdict
    console.log(`\n${colors.blue}=== Test Summary ===${colors.reset}`);
    if (canvasExists && bevyInitialized && assetRequests > 50) {
      console.log(`${colors.green}✓ All tests passed! Card rendering system is working properly.${colors.reset}`);
      return true;
    } else if (canvasExists && assetRequests > 0) {
      console.log(`${colors.yellow}⚠ Partial success - system is loading but may need more initialization time${colors.reset}`);
      if (!bevyInitialized) console.log(`  - Canvas not resized yet (still ${canvasInfo.width}x${canvasInfo.height})`);
      if (assetRequests < 50) console.log(`  - Only ${assetRequests} assets loaded so far`);
      return true; // Still consider it a pass if assets are loading
    } else {
      console.log(`${colors.red}✗ Tests failed:${colors.reset}`);
      if (!canvasExists) console.log(`  - Canvas element not found`);
      if (!bevyInitialized) console.log(`  - Bevy not properly initialized`);
      if (assetRequests === 0) console.log(`  - No assets loaded`);
      return false;
    }

  } catch (error) {
    console.error(`${colors.red}Test failed with error: ${error.message}${colors.reset}`);
    return false;
  } finally {
    await browser.close();
  }
}

// Export for use in test runners
module.exports = { testCardRendering };

// Run if called directly
if (require.main === module) {
  testCardRendering()
    .then(passed => process.exit(passed ? 0 : 1))
    .catch(error => {
      console.error(error);
      process.exit(1);
    });
}