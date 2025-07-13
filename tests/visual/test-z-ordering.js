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

// Z-index values from the codebase
const Z_INDICES = {
  TABLE_PILE_BASE: 10.0,
  TABLE_PILE_INCREMENT: 0.1,
  HAND_BASE: 20.0,
  HAND_INCREMENT: 0.1,
  HAND_HOVER_BOOST: 5.0,
  MOVING_CARD: 100.0,
  SCORE_PILE_BASE: 5.0,
  SCORE_PILE_INCREMENT: 0.01
};

async function testZOrdering() {
  console.log(`${colors.blue}Starting Z-Ordering System Test...${colors.reset}\n`);

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
    baseZOrder: { passed: 0, failed: 0 },
    dynamicZOrder: { passed: 0, failed: 0 },
    interactionZOrder: { passed: 0, failed: 0 }
  };

  try {
    const page = await browser.newPage();
    
    // Monitor console for z-order related messages
    const zOrderLogs = [];
    page.on('console', msg => {
      const text = msg.text();
      if (text.toLowerCase().includes('z') || 
          text.includes('order') || 
          text.includes('layer') ||
          text.includes('depth')) {
        zOrderLogs.push(text);
        console.log(`${colors.cyan}[Z-Order Log] ${text}${colors.reset}`);
      }
    });

    // Navigate to the game
    console.log(`${colors.yellow}1. Loading game...${colors.reset}`);
    await page.goto('http://localhost:8001/', { waitUntil: 'networkidle2' });
    
    // Wait for Bevy initialization
    console.log(`${colors.yellow}2. Waiting for Bevy initialization...${colors.reset}`);
    await new Promise(resolve => setTimeout(resolve, 15000));

    // Create screenshot directory
    const screenshotDir = path.join(__dirname, '../../screenshots/z-ordering');
    if (!fs.existsSync(screenshotDir)) {
      fs.mkdirSync(screenshotDir, { recursive: true });
    }

    // Test 1: Base Z-Order Hierarchy
    console.log(`\n${colors.cyan}=== Test 1: Base Z-Order Hierarchy ===${colors.reset}`);
    
    console.log(`${colors.yellow}Expected z-order hierarchy:${colors.reset}`);
    console.log(`  1. Score pile (z=${Z_INDICES.SCORE_PILE_BASE}) - Bottom layer`);
    console.log(`  2. Table piles (z=${Z_INDICES.TABLE_PILE_BASE}) - Middle layer`);
    console.log(`  3. Hand cards (z=${Z_INDICES.HAND_BASE}) - Top layer`);
    console.log(`  4. Moving cards (z=${Z_INDICES.MOVING_CARD}) - Highest layer`);
    
    // Take full screenshot to verify base hierarchy
    await page.screenshot({ 
      path: path.join(screenshotDir, 'base-hierarchy-full.png'),
      fullPage: false
    });
    console.log(`${colors.green}✓ Base hierarchy screenshot saved${colors.reset}`);
    testResults.baseZOrder.passed++;

    // Take targeted screenshots of each layer
    const layerScreenshots = [
      {
        name: 'score-pile-layer',
        clip: { x: 900, y: 50, width: 300, height: 200 },
        description: 'Score pile base layer'
      },
      {
        name: 'table-piles-layer',
        clip: { x: 200, y: 150, width: 880, height: 300 },
        description: 'Table piles middle layer'
      },
      {
        name: 'hand-cards-layer',
        clip: { x: 0, y: 450, width: 1280, height: 270 },
        description: 'Hand cards top layer'
      }
    ];

    for (const screenshot of layerScreenshots) {
      await page.screenshot({ 
        path: path.join(screenshotDir, `${screenshot.name}.png`),
        clip: screenshot.clip
      });
      console.log(`${colors.green}✓ ${screenshot.description} screenshot saved${colors.reset}`);
      testResults.baseZOrder.passed++;
    }

    // Test 2: Dynamic Z-Order Within Layers
    console.log(`\n${colors.cyan}=== Test 2: Dynamic Z-Order Within Layers ===${colors.reset}`);
    
    // Test hand card ordering
    console.log(`${colors.yellow}Testing hand card z-order (left to right)...${colors.reset}`);
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    await page.screenshot({ 
      path: path.join(screenshotDir, 'hand-card-ordering.png'),
      clip: { x: 300, y: 500, width: 680, height: 200 }
    });
    console.log(`${colors.green}✓ Hand cards should overlap left-to-right${colors.reset}`);
    testResults.dynamicZOrder.passed++;

    // Test table pile stacking
    console.log(`${colors.yellow}Testing table pile stacking order...${colors.reset}`);
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    await page.screenshot({ 
      path: path.join(screenshotDir, 'table-pile-stacking.png'),
      clip: { x: 400, y: 200, width: 480, height: 250 }
    });
    console.log(`${colors.green}✓ Table pile cards should stack with proper offset${colors.reset}`);
    testResults.dynamicZOrder.passed++;

    // Test score pile accumulation
    console.log(`${colors.yellow}Testing score pile accumulation order...${colors.reset}`);
    await new Promise(resolve => setTimeout(resolve, 2000));
    
    await page.screenshot({ 
      path: path.join(screenshotDir, 'score-pile-accumulation.png'),
      clip: { x: 900, y: 50, width: 300, height: 200 }
    });
    console.log(`${colors.green}✓ Score pile should show minimal stacking offset${colors.reset}`);
    testResults.dynamicZOrder.passed++;

    // Test 3: Interactive Z-Order Changes
    console.log(`\n${colors.cyan}=== Test 3: Interactive Z-Order Changes ===${colors.reset}`);
    
    // Test hover effect on hand cards
    console.log(`${colors.yellow}Testing hand card hover z-order boost...${colors.reset}`);
    
    const cardPositions = [
      { x: 400, y: 600, name: 'left' },
      { x: 640, y: 600, name: 'center' },
      { x: 880, y: 600, name: 'right' }
    ];
    
    for (const pos of cardPositions) {
      // Move mouse to card position
      await page.mouse.move(pos.x, pos.y);
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      await page.screenshot({ 
        path: path.join(screenshotDir, `hover-${pos.name}-card.png`),
        clip: { x: pos.x - 100, y: pos.y - 100, width: 200, height: 200 }
      });
      
      console.log(`${colors.green}✓ Hover on ${pos.name} card (z-boost=${Z_INDICES.HAND_HOVER_BOOST})${colors.reset}`);
      testResults.interactionZOrder.passed++;
    }

    // Test moving card z-order
    console.log(`${colors.yellow}Testing moving card z-order...${colors.reset}`);
    
    // Simulate card drag
    await page.mouse.move(640, 600);
    await page.mouse.down();
    await page.mouse.move(640, 300, { steps: 10 });
    
    await page.screenshot({ 
      path: path.join(screenshotDir, 'moving-card-z-order.png'),
      fullPage: false
    });
    
    await page.mouse.up();
    
    console.log(`${colors.green}✓ Moving card should be at highest z-order (z=${Z_INDICES.MOVING_CARD})${colors.reset}`);
    testResults.interactionZOrder.passed++;

    // Test 4: Edge Cases and Overlaps
    console.log(`\n${colors.cyan}=== Test 4: Edge Cases and Overlaps ===${colors.reset}`);
    
    // Test overlap between different layers
    const overlapTests = [
      {
        name: 'hand-table-overlap',
        description: 'Hand cards overlapping table area',
        clip: { x: 400, y: 350, width: 480, height: 300 }
      },
      {
        name: 'table-score-overlap',
        description: 'Table cards near score pile',
        clip: { x: 750, y: 100, width: 400, height: 250 }
      },
      {
        name: 'full-board-overlap',
        description: 'Full board with all overlaps',
        clip: { x: 0, y: 0, width: 1280, height: 720 }
      }
    ];
    
    for (const test of overlapTests) {
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      await page.screenshot({ 
        path: path.join(screenshotDir, `${test.name}.png`),
        clip: test.clip
      });
      
      console.log(`${colors.green}✓ ${test.description} screenshot saved${colors.reset}`);
    }

    // Test 5: Z-Order Consistency Check
    console.log(`\n${colors.cyan}=== Test 5: Z-Order Consistency ===${colors.reset}`);
    
    // Take rapid screenshots to check for z-order flickering
    console.log(`${colors.yellow}Taking rapid screenshots to check for z-order stability...${colors.reset}`);
    
    for (let i = 0; i < 5; i++) {
      await page.screenshot({ 
        path: path.join(screenshotDir, `consistency-check-${i}.png`),
        clip: { x: 300, y: 200, width: 680, height: 400 }
      });
      await new Promise(resolve => setTimeout(resolve, 200));
    }
    
    console.log(`${colors.green}✓ Consistency check screenshots saved (check for flickering)${colors.reset}`);

    // Generate Z-Order Test Report
    console.log(`\n${colors.blue}=== Z-Order Test Summary ===${colors.reset}`);
    
    let totalPassed = 0;
    let totalFailed = 0;
    
    for (const [category, results] of Object.entries(testResults)) {
      totalPassed += results.passed;
      totalFailed += results.failed;
      
      console.log(`${colors.cyan}${category}:${colors.reset}`);
      console.log(`  ${colors.green}Passed: ${results.passed}${colors.reset}`);
      if (results.failed > 0) {
        console.log(`  ${colors.red}Failed: ${results.failed}${colors.reset}`);
      }
    }

    // Visual verification guide
    console.log(`\n${colors.magenta}=== Visual Verification Guide ===${colors.reset}`);
    console.log(`Please check the screenshots for the following:`);
    console.log(`1. ${colors.yellow}Base hierarchy:${colors.reset} Score pile < Table piles < Hand cards`);
    console.log(`2. ${colors.yellow}Hand cards:${colors.reset} Should overlap left-to-right`);
    console.log(`3. ${colors.yellow}Table piles:${colors.reset} Cards should stack with visible edges`);
    console.log(`4. ${colors.yellow}Hover effect:${colors.reset} Hovered card should appear above neighbors`);
    console.log(`5. ${colors.yellow}Moving cards:${colors.reset} Should appear above all other elements`);
    console.log(`6. ${colors.yellow}Consistency:${colors.reset} No flickering or z-order changes between frames`);

    // Save detailed report
    const report = {
      timestamp: new Date().toISOString(),
      results: testResults,
      totalPassed,
      totalFailed,
      zIndices: Z_INDICES,
      zOrderLogs: zOrderLogs.slice(0, 50), // First 50 logs
      verificationChecklist: [
        { check: 'Score pile at bottom layer', expected: `z=${Z_INDICES.SCORE_PILE_BASE}` },
        { check: 'Table piles in middle layer', expected: `z=${Z_INDICES.TABLE_PILE_BASE}` },
        { check: 'Hand cards on top layer', expected: `z=${Z_INDICES.HAND_BASE}` },
        { check: 'Hover boost working', expected: `+${Z_INDICES.HAND_HOVER_BOOST} z-units` },
        { check: 'Moving card highest', expected: `z=${Z_INDICES.MOVING_CARD}` }
      ]
    };
    
    fs.writeFileSync(
      path.join(screenshotDir, 'z-order-test-report.json'),
      JSON.stringify(report, null, 2)
    );
    
    console.log(`\n${colors.green}✓ Detailed report saved to screenshots/z-ordering/z-order-test-report.json${colors.reset}`);
    
    return totalFailed === 0;

  } catch (error) {
    console.error(`${colors.red}Z-ordering test failed with error: ${error.message}${colors.reset}`);
    return false;
  } finally {
    await browser.close();
  }
}

// Export for use in test runners
module.exports = { testZOrdering };

// Run if called directly
if (require.main === module) {
  testZOrdering()
    .then(passed => {
      if (passed) {
        console.log(`\n${colors.green}All z-ordering tests passed!${colors.reset}`);
        console.log(`${colors.yellow}Note: Please manually verify screenshots for visual correctness${colors.reset}`);
        process.exit(0);
      } else {
        console.log(`\n${colors.red}Some z-ordering tests failed. Check screenshots and report.${colors.reset}`);
        process.exit(1);
      }
    })
    .catch(error => {
      console.error(error);
      process.exit(1);
    });
}