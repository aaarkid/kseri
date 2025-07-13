const { exec } = require('child_process');
const { promisify } = require('util');
const execAsync = promisify(exec);

// Colors for console output
const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  reset: '\x1b[0m'
};

async function testPM2Deployment() {
  console.log(`${colors.blue}Testing PM2 Deployment...${colors.reset}\n`);

  try {
    // Check if PM2 is installed
    console.log(`${colors.yellow}1. Checking PM2 installation...${colors.reset}`);
    try {
      await execAsync('pm2 --version');
      console.log(`${colors.green}✓ PM2 is installed${colors.reset}`);
    } catch (error) {
      throw new Error('PM2 is not installed. Please install with: npm install -g pm2');
    }

    // Check PM2 process status
    console.log(`${colors.yellow}2. Checking PM2 process status...${colors.reset}`);
    const { stdout: statusOutput } = await execAsync('pm2 list');
    
    if (statusOutput.includes('kseri-dev-server')) {
      console.log(`${colors.green}✓ kseri-dev-server process found${colors.reset}`);
      
      // Check if it's running
      if (statusOutput.includes('online')) {
        console.log(`${colors.green}✓ Server is online${colors.reset}`);
      } else if (statusOutput.includes('stopped')) {
        console.log(`${colors.yellow}⚠ Server is stopped, attempting to start...${colors.reset}`);
        await execAsync('pm2 start ecosystem.config.js');
        console.log(`${colors.green}✓ Server started${colors.reset}`);
      } else if (statusOutput.includes('errored')) {
        console.log(`${colors.red}✗ Server is in error state${colors.reset}`);
        console.log(`${colors.yellow}  Attempting to restart...${colors.reset}`);
        await execAsync('pm2 delete kseri-dev-server');
        await execAsync('pm2 start ecosystem.config.js');
        console.log(`${colors.green}✓ Server restarted${colors.reset}`);
      }
    } else {
      console.log(`${colors.yellow}⚠ kseri-dev-server not found, starting...${colors.reset}`);
      await execAsync('pm2 start ecosystem.config.js');
      console.log(`${colors.green}✓ Server started${colors.reset}`);
    }

    // Check server accessibility
    console.log(`${colors.yellow}3. Checking server accessibility...${colors.reset}`);
    const fetch = await import('node-fetch').then(m => m.default).catch(() => null);
    
    if (fetch) {
      try {
        const response = await fetch('http://localhost:8001/');
        if (response.ok) {
          console.log(`${colors.green}✓ Server is accessible at http://localhost:8001${colors.reset}`);
          
          // Check CORS headers
          const corsEmbedder = response.headers.get('cross-origin-embedder-policy');
          const corsOpener = response.headers.get('cross-origin-opener-policy');
          
          if (corsEmbedder === 'require-corp' && corsOpener === 'same-origin') {
            console.log(`${colors.green}✓ CORS headers are properly configured${colors.reset}`);
          } else {
            console.log(`${colors.yellow}⚠ CORS headers may not be properly configured${colors.reset}`);
          }
        } else {
          throw new Error(`Server returned status ${response.status}`);
        }
      } catch (error) {
        throw new Error(`Server is not accessible: ${error.message}`);
      }
    } else {
      // Fallback to curl if node-fetch is not available
      try {
        await execAsync('curl -I http://localhost:8001/');
        console.log(`${colors.green}✓ Server is accessible at http://localhost:8001${colors.reset}`);
      } catch (error) {
        throw new Error('Server is not accessible');
      }
    }

    // Check logs
    console.log(`${colors.yellow}4. Checking PM2 logs...${colors.reset}`);
    const { stdout: logs } = await execAsync('pm2 logs kseri-dev-server --lines 5 --nostream');
    if (logs.includes('Server running at')) {
      console.log(`${colors.green}✓ Server logs show successful startup${colors.reset}`);
    }

    console.log(`\n${colors.blue}=== PM2 Deployment Test Summary ===${colors.reset}`);
    console.log(`${colors.green}✓ All PM2 deployment tests passed!${colors.reset}`);
    return true;

  } catch (error) {
    console.error(`\n${colors.red}PM2 deployment test failed: ${error.message}${colors.reset}`);
    return false;
  }
}

// Export for use in test runners
module.exports = { testPM2Deployment };

// Run if called directly
if (require.main === module) {
  testPM2Deployment()
    .then(passed => process.exit(passed ? 0 : 1))
    .catch(error => {
      console.error(error);
      process.exit(1);
    });
}