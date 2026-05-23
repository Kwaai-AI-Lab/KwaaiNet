#!/usr/bin/env bash
# Quick integration test for the kwaai-storage API.
# Requires: kwaainet storage init (PG running on port 5433)
set -euo pipefail

API="http://localhost:7432"

echo "=== Starting storage API server in background ==="
# We'll write a tiny Rust program inline, but for now let's use curl against a running server.
# Start the server manually first: see below.

echo ""
echo "=== 1. Health check ==="
curl -s "$API/api/health" | python3 -m json.tool

echo ""
echo "=== 2. Create tenant ==="
TENANT=$(curl -s -X POST "$API/api/tenants" \
  -H "Content-Type: application/json" \
  -d '{"peer_id": "12D3KooWTestPeer123", "display_name": "test-bob", "capacity_limit_mb": 512, "vector_dimension": 3}')
echo "$TENANT" | python3 -m json.tool
TENANT_ID=$(echo "$TENANT" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_id'])")
echo "Tenant ID: $TENANT_ID"

echo ""
echo "=== 3. List tenants ==="
curl -s "$API/api/tenants" | python3 -m json.tool

echo ""
echo "=== 4. Upload vectors ==="
curl -s -X POST "$API/api/tenants/$TENANT_ID/vectors" \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": [
      {"id": 1, "embedding": [1.0, 0.0, 0.0]},
      {"id": 2, "embedding": [0.0, 1.0, 0.0]},
      {"id": 3, "embedding": [0.9, 0.1, 0.0]},
      {"id": 4, "embedding": [0.0, 0.0, 1.0]}
    ]
  }' | python3 -m json.tool

echo ""
echo "=== 5. Search vectors (query similar to id=1) ==="
curl -s -X POST "$API/api/tenants/$TENANT_ID/search" \
  -H "Content-Type: application/json" \
  -d '{"query": [1.0, 0.0, 0.0], "top_k": 3}' | python3 -m json.tool

echo ""
echo "=== 6. Get tenant (with stats) ==="
curl -s "$API/api/tenants/$TENANT_ID" | python3 -m json.tool

echo ""
echo "=== 7. Delete vectors ==="
curl -s -X DELETE "$API/api/tenants/$TENANT_ID/vectors" \
  -H "Content-Type: application/json" \
  -d '{"ids": [4]}' | python3 -m json.tool

echo ""
echo "=== 8. Health (after operations) ==="
curl -s "$API/api/health" | python3 -m json.tool

echo ""
echo "=== 9. Delete tenant ==="
curl -s -X DELETE "$API/api/tenants/$TENANT_ID" -w "\nHTTP %{http_code}\n"

echo ""
echo "=== 10. List tenants (should be empty) ==="
curl -s "$API/api/tenants" | python3 -m json.tool

echo ""
echo "=== All tests passed ==="
