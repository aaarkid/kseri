const puppeteer = require('puppeteer');

async function testSimple() {
  console.log('Opening browser to show card rendering demo...');
  
  const browser = await puppeteer.launch({
    headless: false,
    args: ['--no-sandbox', '--disable-setuid-sandbox']
  });

  try {
    const page = await browser.newPage();
    
    console.log('Navigating to http://localhost:8001/final-test.html');
    await page.goto('http://localhost:8001/final-test.html');
    
    console.log('Browser is open. The demo should be visible.');
    console.log('Press Ctrl+C to close.');
    
    // Keep browser open
    await new Promise(() => {});
    
  } catch (error) {
    console.error('Error:', error);
  }
}

testSimple().catch(console.error);