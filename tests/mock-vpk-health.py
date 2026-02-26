#!/usr/bin/env python3
"""
Mock VPK health server for KwaaiNet integration testing.

Listens on localhost:7432 (or $VPK_PORT) and responds to
GET /api/health with the JSON body that KwaaiNet expects.

Usage:
    python3 tests/mock-vpk-health.py
    VPK_PORT=7432 VPK_MODE=eve python3 tests/mock-vpk-health.py
"""

import http.server
import json
import os
import sys

PORT     = int(os.environ.get("VPK_PORT", "7432"))
MODE     = os.environ.get("VPK_MODE", "both")
PEER_ID  = os.environ.get("VPK_PEER_ID", "(not-set — run: kwaainet identity show)")
CAPACITY = float(os.environ.get("VPK_CAPACITY_GB", "512.0"))
VERSION  = os.environ.get("VPK_VERSION", "0.1.0-mock")

HEALTH_RESPONSE = {
    "status":               "ok",
    "version":              VERSION,
    "mode":                 MODE,
    "peer_id":              PEER_ID,
    "tenant_count":         0,
    "capacity_gb_available": CAPACITY,
}


class HealthHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        body = json.dumps(HEALTH_RESPONSE).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, fmt, *args):
        # Suppress access logs — KwaaiNet polls every 120 s
        pass


if __name__ == "__main__":
    server = http.server.HTTPServer(("", PORT), HealthHandler)
    print(f"Mock VPK health server listening on port {PORT}")
    print(f"  mode:     {MODE}")
    print(f"  peer_id:  {PEER_ID}")
    print(f"  capacity: {CAPACITY} GB")
    print(f"  version:  {VERSION}")
    print()
    print("GET /api/health  →  200 OK (JSON)")
    print("Press Ctrl-C to stop.")
    print()
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("\nStopped.")
        sys.exit(0)
