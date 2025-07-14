const puppeteer = require('puppeteer');

async function finalTest() {
  console.log('Running final card rendering test...\n');
  
  const browser = await puppeteer.launch({
    headless: false,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  try {
    const page = await browser.newPage();
    
    // Capture console messages
    page.on('console', msg => {
      if (msg.text().includes('Kseri')) {
        console.log(`🎮 ${msg.text()}`);
      }
    });
    
    console.log('Loading final test page...');
    await page.goto('http://localhost:8001/final-test.html');
    
    // Wait for initialization
    console.log('Waiting for WASM initialization...');
    await new Promise(resolve => setTimeout(resolve, 20000));
    
    // Get all status information
    const status = await page.evaluate(() => {
      return {
        init: document.getElementById('init-status').textContent,
        wasm: document.getElementById('wasm-status').textContent,
        canvas: document.getElementById('canvas-status').textContent,
        assets: document.getElementById('asset-status').textContent,
        canvasSize: {
          width: document.getElementById('bevy').width,
          height: document.getElementById('bevy').height
        }
      };
    });
    
    console.log('\n📊 Status Report:');
    console.log('================');
    console.log(`Init: ${status.init}`);
    console.log(`WASM: ${status.wasm}`);
    console.log(`Canvas: ${status.canvas}`);
    console.log(`Assets: ${status.assets}`);
    console.log(`Canvas Size: ${status.canvasSize.width}x${status.canvasSize.height}`);
    
    // Take screenshot
    await page.screenshot({ path: 'final-status.png', fullPage: true });
    console.log('\n📸 Screenshot saved to final-status.png');
    
    // Check for success
    if (status.canvasSize.width !== 300 || status.canvasSize.height !== 150) {
      console.log('\n✅ SUCCESS: Bevy initialized and resized the canvas!');
    } else if (status.wasm.includes('successfully')) {
      console.log('\n⚠️  WASM loaded but Bevy may not have initialized properly');
      console.log('   Check browser console for Bevy-specific errors');
    } else {
      console.log('\n❌ FAILURE: WASM did not load properly');
    }
    
    console.log('\nKeeping browser open for inspection. Press Ctrl+C to exit.');
    await new Promise(() => {}); // Keep browser open
    
  } catch (error) {
    console.error('Test error:', error);
  }
}

finalTest().catch(console.error);