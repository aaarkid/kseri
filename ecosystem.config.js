module.exports = {
  apps: [{
    name: 'kseri-dev-server',
    script: 'serve.py',
    interpreter: 'python3',
    cwd: '/home/arkid/DEV/kseri',
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