# PM2 Setup Guide for Kseri Card Rendering

## Overview

PM2 is used to manage the local development server for the Kseri card rendering project. It provides process management, automatic restarts, and logging capabilities for the Python HTTP server that serves the WASM build.

## Installation

PM2 is already installed as part of the project dependencies. If needed, install globally:

```bash
npm install -g pm2
```

## Configuration

The PM2 configuration is defined in `ecosystem.config.js`:

```javascript
module.exports = {
  apps: [{
    name: 'kseri-branch-server',
    script: 'serve.py',
    interpreter: 'python3',
    cwd: '/home/arkid/DEV/kseri-card-rendering',
    env: {
      PORT: 8001
    },
    watch: false,
    ignore_watch: ['node_modules', 'target', '.git'],
    error_file: './logs/pm2-error.log',
    out_file: './logs/pm2-out.log',
    log_file: './logs/pm2-combined.log',
    time: true
  }]
}
```

### Configuration Details

- **name**: `kseri-branch-server` - The process name in PM2
- **script**: `serve.py` - Python script that serves the WASM files
- **interpreter**: `python3` - Specifies Python 3 as the interpreter
- **cwd**: Working directory (project root)
- **env.PORT**: `8001` - Server port
- **Log files**: Stored in `./logs/` directory with timestamps

## Server Script (serve.py)

The Python server includes critical CORS headers for WASM:

```python
self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
```

These headers are required for SharedArrayBuffer support in WASM applications.

## PM2 Commands

### Quick Commands (via package.json scripts)

```bash
npm run serve     # Start the server
npm run stop      # Stop the server
npm run restart   # Restart the server
npm run logs      # View server logs
```

### Direct PM2 Commands

```bash
# Start server
pm2 start ecosystem.config.js

# View status
pm2 status

# View logs
pm2 logs kseri-branch-server

# Stop server
pm2 stop kseri-branch-server

# Restart server
pm2 restart kseri-branch-server

# Delete process (removes from PM2)
pm2 delete kseri-branch-server

# Kill process (stops without removing)
pm2 kill
```

### Managing Port Conflicts

If port 8001 is already in use:

```bash
# Find process using port 8001
lsof -i :8001

# Kill process by PID
kill -9 <PID>

# Or use fuser
fuser -k 8001/tcp
```

## Log Management

Logs are stored in the `./logs/` directory:

- `pm2-out.log` - Standard output
- `pm2-error.log` - Error output
- `pm2-combined.log` - Combined logs

View logs in real-time:
```bash
pm2 logs kseri-branch-server --lines 50
```

## Development Workflow

1. **Build WASM first**:
   ```bash
   npm run build
   ```

2. **Start PM2 server**:
   ```bash
   npm run serve
   ```

3. **Verify server is running**:
   ```bash
   pm2 status
   ```

4. **Access the application**:
   - Development: http://localhost:8001
   - Test page: http://localhost:8001/test.html
   - Final test: http://localhost:8001/final-test.html

5. **Run tests**:
   ```bash
   npm run test
   ```

## Troubleshooting

### Server Won't Start

1. Check if port is in use:
   ```bash
   lsof -i :8001
   ```

2. Check PM2 status:
   ```bash
   pm2 status
   ```

3. View error logs:
   ```bash
   pm2 logs kseri-branch-server --err
   ```

### WASM Loading Issues

If WASM fails to load:

1. Verify CORS headers in browser DevTools Network tab
2. Check that `serve.py` has proper headers
3. Ensure WASM files exist in `./web/` directory

### Process Management Issues

If PM2 process is stuck:

```bash
# Force stop all PM2 processes
pm2 kill

# Clear PM2 process list
pm2 flush

# Restart PM2 daemon
pm2 resurrect
```

## Server Name History

The PM2 server was renamed from `kseri-dev-server` to `kseri-branch-server` to better reflect branch-based development workflows. When updating:

1. Stop old server: `pm2 stop kseri-dev-server`
2. Delete old process: `pm2 delete kseri-dev-server`
3. Update `ecosystem.config.js` with new name
4. Start new server: `pm2 start ecosystem.config.js`

## Integration with Tests

The PM2 server is required for running Puppeteer tests:

```bash
# Full test workflow
npm run build && npm run serve && npm run test

# Or use the dev script
npm run dev
```

Tests expect the server to be running on http://localhost:8001.