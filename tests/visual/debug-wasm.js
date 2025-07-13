const puppeteer = require('puppeteer');

(async () => {
    console.log('Starting WASM debug capture...');
    
    const browser = await puppeteer.launch({
        headless: true,
        args: ['--no-sandbox', '--disable-setuid-sandbox']
    });
    
    try {
        const page = await browser.newPage();
        
        // Enable console logging
        page.on('console', msg => {
            console.log(`Browser console [${msg.type()}]:`, msg.text());
        });
        
        // Capture any page errors
        page.on('pageerror', error => {
            console.error('Page error:', error.message);
        });
        
        // Capture request failures
        page.on('requestfailed', request => {
            console.error('Request failed:', request.url(), request.failure().errorText);
        });
        
        console.log('Navigating to http://localhost:8001/test-wasm-nocache.html...');
        
        // Clear cache first
        await page.setCacheEnabled(false);
        
        // Navigate to the page with a 30 second timeout
        await page.goto('http://localhost:8001/test-wasm-nocache.html', {
            waitUntil: 'networkidle2',
            timeout: 30000
        });
        
        console.log('Page loaded, waiting 10 seconds for WASM execution...');
        
        // Wait for 10 seconds
        await new Promise(resolve => setTimeout(resolve, 10000));
        
        console.log('Capturing log content...');
        
        // Get the log content from the #log div
        const logContent = await page.evaluate(() => {
            const logDiv = document.getElementById('log');
            return logDiv ? logDiv.innerText : 'No log div found';
        });
        
        console.log('\n=== WASM Debug Log Content ===');
        console.log(logContent);
        console.log('=== End of Log Content ===\n');
        
        // Also capture any additional debugging info
        const debugInfo = await page.evaluate(() => {
            return {
                title: document.title,
                url: window.location.href,
                userAgent: navigator.userAgent,
                // Check if wasm_bindgen is loaded
                wasmBindgenLoaded: typeof window.wasm_bindgen !== 'undefined',
                // Check for any global error messages
                bodyText: document.body.innerText.substring(0, 500) // First 500 chars
            };
        });
        
        console.log('Debug Info:', JSON.stringify(debugInfo, null, 2));
        
        // Take a screenshot
        const screenshotPath = 'tests/visual/wasm-debug-screenshot.png';
        await page.screenshot({ 
            path: screenshotPath,
            fullPage: true 
        });
        console.log(`Screenshot saved to: ${screenshotPath}`);
        
    } catch (error) {
        console.error('Error during capture:', error);
    } finally {
        await browser.close();
        console.log('Browser closed.');
    }
})();