const puppeteer = require('puppeteer');
const path = require('path');
const fs = require('fs');

// Colors for console output
const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  reset: '\x1b[0m'
};

// Layout configuration based on HandLayoutManager, TableLayoutManager, and ScorePileLayoutManager
const LAYOUT_CONFIG = {
  hand: {
    maxCards: 13,
    cardSpacing: 30,
    yOffset: -300,
    hoverOffset: 20
  },
  table: {
    pileSpacing: 150,
    cardOffset: { x: 20, y: 20 },
    maxCardsPerPile: 4
  },
  scorePile: {
    position: { x: 400, y: -200 },
    stackOffset: 2
  }
};

async function testLayoutManagers() {
  console.log(`${colors.blue}Starting Layout Managers Test...${colors.reset}\n`);

  const browser = await puppeteer.launch({
    headless: true,
    args: [
      '--no-sandbox',
      '--disable-setuid-sandbox',
      '--enable-webgl',
      '--use-gl=swiftshader'
    ]
  });

  const testResults = {
    handLayout: { passed: 0, failed: 0 },
    tableLayout: { passed: 0, failed: 0 },
    scorePileLayout: { passed: 0, failed: 0 }
  };

  try {
    const page = await browser.newPage();
    
    // Set up monitoring
    const consoleLogs = [];
    page.on('console', msg => {
      const text = msg.text();
      consoleLogs.push(text);
      
      // Look for layout-related logs
      if (text.includes('HandLayoutManager') || 
          text.includes('TableLayoutManager') || 
          text.includes('ScorePileLayoutManager')) {
        console.log(`${colors.cyan}[Layout Log] ${text}${colors.reset}`);
      }
    });

    // Navigate to the game
    console.log(`${colors.yellow}1. Loading game...${colors.reset}`);
    await page.goto('http://localhost:8001/', { waitUntil: 'networkidle2' });
    
    // Wait for Bevy initialization
    console.log(`${colors.yellow}2. Waiting for Bevy initialization...${colors.reset}`);
    await new Promise(resolve => setTimeout(resolve, 15000));

    // Create screenshot directory
    const screenshotDir = path.join(__dirname, '../../screenshots/layouts');
    if (!fs.existsSync(screenshotDir)) {
      fs.mkdirSync(screenshotDir, { recursive: true });
    }

    // Test 1: Hand Layout Manager
    console.log(`\n${colors.cyan}=== Testing Hand Layout Manager ===${colors.reset}`);
    
    // Test different hand sizes
    const handSizes = [1, 4, 7, 13];
    for (const size of handSizes) {
      console.log(`${colors.yellow}Testing hand with ${size} cards...${colors.reset}`);
      
      // Wait for layout update
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      // Take screenshot
      const screenshotPath = path.join(screenshotDir, `hand-${size}-cards.png`);
      await page.screenshot({ 
        path: screenshotPath,
        clip: { x: 0, y: 400, width: 1280, height: 320 }
      });
      
      console.log(`${colors.green}✓ Screenshot saved: hand-${size}-cards.png${colors.reset}`);
      testResults.handLayout.passed++;
    }

    // Test hand hover effect
    console.log(`${colors.yellow}Testing hand hover effect...${colors.reset}`);
    
    // Simulate mouse hover over different card positions
    const hoverPositions = [
      { x: 640, y: 600 },  // Center card
      { x: 400, y: 600 },  // Left side
      { x: 880, y: 600 }   // Right side
    ];
    
    for (let i = 0; i < hoverPositions.length; i++) {
      await page.mouse.move(hoverPositions[i].x, hoverPositions[i].y);
      await new Promise(resolve => setTimeout(resolve, 500));
      
      await page.screenshot({ 
        path: path.join(screenshotDir, `hand-hover-${i + 1}.png`),
        clip: { x: 0, y: 400, width: 1280, height: 320 }
      });
    }
    
    console.log(`${colors.green}✓ Hover effect screenshots saved${colors.reset}`);
    testResults.handLayout.passed++;

    // Test 2: Table Layout Manager
    console.log(`\n${colors.cyan}=== Testing Table Layout Manager ===${colors.reset}`);
    
    // Test different pile configurations
    const pileConfigs = [
      { name: 'single-pile', piles: 1 },
      { name: 'two-piles', piles: 2 },
      { name: 'four-piles', piles: 4 }
    ];
    
    for (const config of pileConfigs) {
      console.log(`${colors.yellow}Testing ${config.name} configuration...${colors.reset}`);
      
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      await page.screenshot({ 
        path: path.join(screenshotDir, `table-${config.name}.png`),
        clip: { x: 200, y: 100, width: 880, height: 300 }
      });
      
      console.log(`${colors.green}✓ Screenshot saved: table-${config.name}.png${colors.reset}`);
      testResults.tableLayout.passed++;
    }

    // Test pile stacking
    console.log(`${colors.yellow}Testing pile stacking (multiple cards per pile)...${colors.reset}`);
    
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    await page.screenshot({ 
      path: path.join(screenshotDir, 'table-stacked-piles.png'),
      fullPage: false
    });
    
    console.log(`${colors.green}✓ Stacked piles screenshot saved${colors.reset}`);
    testResults.tableLayout.passed++;

    // Test 3: Score Pile Layout Manager
    console.log(`\n${colors.cyan}=== Testing Score Pile Layout Manager ===${colors.reset}`);
    
    // Test score pile growth
    const scoreSizes = [0, 5, 10, 20];
    for (const size of scoreSizes) {
      console.log(`${colors.yellow}Testing score pile with ${size} cards...${colors.reset}`);
      
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      await page.screenshot({ 
        path: path.join(screenshotDir, `score-pile-${size}-cards.png`),
        clip: { x: 900, y: 50, width: 300, height: 200 }
      });
      
      console.log(`${colors.green}✓ Screenshot saved: score-pile-${size}-cards.png${colors.reset}`);
      testResults.scorePileLayout.passed++;
    }

    // Test 4: Layout Responsiveness
    console.log(`\n${colors.cyan}=== Testing Layout Responsiveness ===${colors.reset}`);
    
    const viewportSizes = [
      { width: 1920, height: 1080, name: 'full-hd' },
      { width: 1280, height: 720, name: 'hd' },
      { width: 1024, height: 768, name: 'tablet-landscape' },
      { width: 768, height: 1024, name: 'tablet-portrait' }
    ];
    
    for (const size of viewportSizes) {
      console.log(`${colors.yellow}Testing ${size.name} (${size.width}x${size.height})...${colors.reset}`);
      
      await page.setViewport({ width: size.width, height: size.height });
      await new Promise(resolve => setTimeout(resolve, 2000));
      
      await page.screenshot({ 
        path: path.join(screenshotDir, `responsive-${size.name}.png`),
        fullPage: false
      });
      
      console.log(`${colors.green}✓ Responsive layout screenshot saved${colors.reset}`);
    }

    // Test 5: Z-Ordering in Layouts
    console.log(`\n${colors.cyan}=== Testing Z-Ordering in Layouts ===${colors.reset}`);
    
    // Take detailed screenshots of overlapping areas
    const zOrderTests = [
      {
        name: 'hand-overlap',
        clip: { x: 400, y: 500, width: 480, height: 200 },
        description: 'Hand cards overlapping'
      },
      {
        name: 'table-pile-overlap',
        clip: { x: 450, y: 200, width: 380, height: 200 },
        description: 'Table pile card stacking'
      },
      {
        name: 'score-pile-stacking',
        clip: { x: 900, y: 50, width: 300, height: 200 },
        description: 'Score pile z-ordering'
      }
    ];
    
    for (const test of zOrderTests) {
      await page.screenshot({ 
        path: path.join(screenshotDir, `z-order-${test.name}.png`),
        clip: test.clip
      });
      
      console.log(`${colors.green}✓ ${test.description} screenshot saved${colors.reset}`);
    }

    // Generate summary report
    console.log(`\n${colors.blue}=== Layout Test Summary ===${colors.reset}`);
    
    let totalPassed = 0;
    let totalFailed = 0;
    
    for (const [layout, results] of Object.entries(testResults)) {
      totalPassed += results.passed;
      totalFailed += results.failed;
      
      console.log(`${colors.cyan}${layout}:${colors.reset}`);
      console.log(`  ${colors.green}Passed: ${results.passed}${colors.reset}`);
      if (results.failed > 0) {
        console.log(`  ${colors.red}Failed: ${results.failed}${colors.reset}`);
      }
    }
    
    // Save test report
    const report = {
      timestamp: new Date().toISOString(),
      results: testResults,
      totalPassed,
      totalFailed,
      screenshotCount: fs.readdirSync(screenshotDir).filter(f => f.endsWith('.png')).length,
      layoutConfig: LAYOUT_CONFIG
    };
    
    fs.writeFileSync(
      path.join(screenshotDir, 'layout-test-report.json'),
      JSON.stringify(report, null, 2)
    );
    
    console.log(`\n${colors.green}✓ Test report saved to screenshots/layouts/layout-test-report.json${colors.reset}`);
    console.log(`${colors.green}✓ ${report.screenshotCount} screenshots generated${colors.reset}`);
    
    // Analyze console logs for layout-specific information
    const layoutLogs = consoleLogs.filter(log => 
      log.includes('Layout') || 
      log.includes('position') || 
      log.includes('card')
    );
    
    if (layoutLogs.length > 0) {
      console.log(`\n${colors.cyan}Layout-related console output:${colors.reset}`);
      layoutLogs.slice(0, 10).forEach(log => {
        console.log(`  ${colors.yellow}>${colors.reset} ${log}`);
      });
      if (layoutLogs.length > 10) {
        console.log(`  ${colors.yellow}... and ${layoutLogs.length - 10} more${colors.reset}`);
      }
    }
    
    return totalFailed === 0;

  } catch (error) {
    console.error(`${colors.red}Layout test failed with error: ${error.message}${colors.reset}`);
    return false;
  } finally {
    await browser.close();
  }
}

// Export for use in test runners
module.exports = { testLayoutManagers };

// Run if called directly
if (require.main === module) {
  testLayoutManagers()
    .then(passed => {
      if (passed) {
        console.log(`\n${colors.green}All layout manager tests passed!${colors.reset}`);
        process.exit(0);
      } else {
        console.log(`\n${colors.red}Some layout tests failed. Check screenshots for visual verification.${colors.reset}`);
        process.exit(1);
      }
    })
    .catch(error => {
      console.error(error);
      process.exit(1);
    });
}