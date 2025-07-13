#!/bin/bash
# Build script for WASM

echo "Building Kseri for WASM..."

# Build the WASM package
wasm-pack build --target web --no-typescript

# Create a simple HTTP server script if needed
cat > serve.py << EOF
#!/usr/bin/env python3
import http.server
import socketserver
import os

PORT = 8001

class MyHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()

os.chdir(os.path.dirname(os.path.abspath(__file__)))

with socketserver.TCPServer(("", PORT), MyHTTPRequestHandler) as httpd:
    print(f"Server running at http://localhost:{PORT}/")
    print("Open test.html to see the card rendering demo")
    httpd.serve_forever()
EOF

chmod +x serve.py

echo "Build complete! Run ./serve.py to start the development server."