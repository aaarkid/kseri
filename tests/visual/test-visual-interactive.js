const puppeteer = require('puppeteer');

async function interactiveTest() {
  console.log('Starting interactive visual test...\n');
  console.log('This will open a browser window so you can see the rendering.');
  
  const browser = await puppeteer.launch({
    headless: false, // Show the browser
    args: ['--no-sandbox', '--disable-setuid-sandbox'],
    defaultViewport: { width: 1200, height: 800 }
  });

  try {
    const page = await browser.newPage();
    
    // Enable console logging
    page.on('console', msg => console.log('PAGE:', msg.text()));
    page.on('pageerror', error => console.log('ERROR:', error.message));
    
    console.log('Loading http://localhost:8001/ ...');
    await page.goto('http://localhost:8001/');
    
    console.log('\nWaiting 30 seconds for WASM initialization...');
    await new Promise(resolve => setTimeout(resolve, 30000));
    
    // Check canvas state
    const canvasInfo = await page.evaluate(() => {
      const canvas = document.querySelector('#bevy');
      if (!canvas) return { error: 'No canvas found' };
      
      return {
        width: canvas.width,
        height: canvas.height,
        bevyInitialized: canvas.width !== 300 || canvas.height !== 150
      };
    });
    
    console.log('\nCanvas info:', canvasInfo);
    
    // Take screenshot
    await page.screenshot({ path: 'interactive-screenshot.png', fullPage: true });
    console.log('Screenshot saved to interactive-screenshot.png');
    
    console.log('\n✅ Browser window is open. You can inspect the page.');
    console.log('Press Ctrl+C to close when done.\n');
    
    // Keep browser open
    await new Promise(() => {});
    
  } catch (error) {
    console.error('Error:', error);
  }
}

interactiveTest().catch(console.error);