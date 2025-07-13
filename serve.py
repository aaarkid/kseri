#!/usr/bin/env python3
"""
Simple HTTP server for serving WASM files with proper MIME types
"""

import http.server
import socketserver
import os
import sys

PORT = 8000

class WASMHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        # Add CORS headers
        self.send_header('Cross-Origin-Embedder-Policy', 'require-corp')
        self.send_header('Cross-Origin-Opener-Policy', 'same-origin')
        super().end_headers()
    
    def guess_type(self, path):
        mimetype = super().guess_type(path)
        if path.endswith('.wasm'):
            return 'application/wasm'
        return mimetype

def main():
    # Change to web directory
    web_dir = os.path.join(os.path.dirname(__file__), 'web')
    if os.path.exists(web_dir):
        os.chdir(web_dir)
    else:
        print(f"Error: Web directory '{web_dir}' not found")
        print("Please run './build-wasm.sh' first to build the WASM files")
        sys.exit(1)
    
    # Check if WASM files exist
    if not os.path.exists('kseri.wasm'):
        print("Error: kseri.wasm not found in web directory")
        print("Please run './build-wasm.sh' first to build the WASM files")
        sys.exit(1)
    
    handler = WASMHTTPRequestHandler
    with socketserver.TCPServer(("", PORT), handler) as httpd:
        print(f"Serving Kseri at http://localhost:{PORT}")
        print("Press Ctrl+C to stop the server")
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\nServer stopped")

if __name__ == "__main__":
    main()