const { testCardRendering } = require('./test-card-rendering');
const { testPM2Deployment } = require('./test-pm2-deployment');
const { testCardRenderingSystem } = require('./test-card-rendering-system');
const { testLayoutManagers } = require('./test-layout-managers');
const { testZOrdering } = require('./test-z-ordering');

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

async function runAllTests() {
  console.log(`${colors.blue}╔════════════════════════════════════════════════════════════╗${colors.reset}`);
  console.log(`${colors.blue}║        Kseri Card Rendering System - Complete Test Suite    ║${colors.reset}`);
  console.log(`${colors.blue}╚════════════════════════════════════════════════════════════╝${colors.reset}\n`);

  const tests = [
    {
      name: 'PM2 Deployment',
      fn: testPM2Deployment,
      category: 'Infrastructure'
    },
    {
      name: 'Basic Card Rendering',
      fn: testCardRendering,
      category: 'Core'
    },
    {
      name: 'Card Rendering System',
      fn: testCardRenderingSystem,
      category: 'Task #5'
    },
    {
      name: 'Layout Managers',
      fn: testLayoutManagers,
      category: 'Task #5'
    },
    {
      name: 'Z-Ordering System',
      fn: testZOrdering,
      category: 'Task #5'
    }
  ];

  const results = [];
  const startTime = Date.now();

  for (const test of tests) {
    console.log(`\n${colors.cyan}┌─────────────────────────────────────────────────────────────┐${colors.reset}`);
    console.log(`${colors.cyan}│ Running: ${test.name.padEnd(50)} │${colors.reset}`);
    console.log(`${colors.cyan}│ Category: ${test.category.padEnd(49)} │${colors.reset}`);
    console.log(`${colors.cyan}└─────────────────────────────────────────────────────────────┘${colors.reset}\n`);

    const testStartTime = Date.now();
    let passed = false;
    let error = null;

    try {
      passed = await test.fn();
    } catch (e) {
      error = e;
      passed = false;
    }

    const duration = ((Date.now() - testStartTime) / 1000).toFixed(2);

    results.push({
      name: test.name,
      category: test.category,
      passed,
      duration,
      error
    });

    if (passed) {
      console.log(`\n${colors.green}✓ ${test.name} completed successfully (${duration}s)${colors.reset}`);
    } else {
      console.log(`\n${colors.red}✗ ${test.name} failed (${duration}s)${colors.reset}`);
      if (error) {
        console.log(`${colors.red}  Error: ${error.message}${colors.reset}`);
      }
    }
  }

  const totalDuration = ((Date.now() - startTime) / 1000).toFixed(2);

  // Print summary
  console.log(`\n${colors.blue}╔════════════════════════════════════════════════════════════╗${colors.reset}`);
  console.log(`${colors.blue}║                      TEST SUITE SUMMARY                     ║${colors.reset}`);
  console.log(`${colors.blue}╚════════════════════════════════════════════════════════════╝${colors.reset}\n`);

  // Group by category
  const categories = {};
  results.forEach(result => {
    if (!categories[result.category]) {
      categories[result.category] = [];
    }
    categories[result.category].push(result);
  });

  for (const [category, categoryResults] of Object.entries(categories)) {
    console.log(`${colors.cyan}${category}:${colors.reset}`);
    
    categoryResults.forEach(result => {
      const status = result.passed ? 
        `${colors.green}✓ PASS${colors.reset}` : 
        `${colors.red}✗ FAIL${colors.reset}`;
      
      console.log(`  ${status} ${result.name.padEnd(30)} (${result.duration}s)`);
    });
    
    console.log('');
  }

  const passedCount = results.filter(r => r.passed).length;
  const failedCount = results.filter(r => !r.passed).length;

  console.log(`${colors.magenta}Overall Results:${colors.reset}`);
  console.log(`  Total Tests: ${results.length}`);
  console.log(`  ${colors.green}Passed: ${passedCount}${colors.reset}`);
  console.log(`  ${colors.red}Failed: ${failedCount}${colors.reset}`);
  console.log(`  Total Duration: ${totalDuration}s`);

  if (failedCount === 0) {
    console.log(`\n${colors.green}╔════════════════════════════════════════════════════════════╗${colors.reset}`);
    console.log(`${colors.green}║              ALL TESTS PASSED SUCCESSFULLY! 🎉              ║${colors.reset}`);
    console.log(`${colors.green}╚════════════════════════════════════════════════════════════╝${colors.reset}`);
  } else {
    console.log(`\n${colors.red}╔════════════════════════════════════════════════════════════╗${colors.reset}`);
    console.log(`${colors.red}║                    SOME TESTS FAILED                        ║${colors.reset}`);
    console.log(`${colors.red}╚════════════════════════════════════════════════════════════╝${colors.reset}`);
  }

  console.log(`\n${colors.yellow}Note: Check the screenshots/ directory for visual verification${colors.reset}`);
  console.log(`${colors.yellow}      Test reports are available in JSON format for detailed analysis${colors.reset}\n`);

  return failedCount === 0;
}

// Run if called directly
if (require.main === module) {
  runAllTests()
    .then(allPassed => {
      process.exit(allPassed ? 0 : 1);
    })
    .catch(error => {
      console.error(`${colors.red}Fatal error running tests:${colors.reset}`, error);
      process.exit(1);
    });
}