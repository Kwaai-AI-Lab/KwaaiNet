#!/usr/bin/env bash
# =============================================================================
# Storage Fabric — End-to-End Smoke Test
# =============================================================================
#
# Tests the full lifecycle of the embedded hnsw_rs+redb storage API:
#   init → serve → health → tenant CRUD → vector upload → ANN search →
#   vector delete → tenant delete → destroy
#
# USAGE
#   bash tests/storage-fabric-test.sh
#
# REQUIRES
#   kwaainet binary (defaults to ./core/target/release/kwaainet)
#   curl, python3
# =============================================================================

set -euo pipefail

KWAAINET="${KWAAINET:-./core/target/release/kwaainet}"
EVE_PORT="${EVE_PORT:-17432}"   # non-default port to avoid colliding with a live node
API="http://localhost:${EVE_PORT}/api"
SERVE_PID=""

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

pass() { echo -e "  ${GREEN}✅ $*${RESET}"; PASS=$((PASS+1)); }
fail() { echo -e "  ${RED}❌ $*${RESET}"; FAIL=$((FAIL+1)); }
info() { echo -e "  ${CYAN}ℹ  $*${RESET}"; }
step() { echo -e "\n${BOLD}── $* ──${RESET}"; }

PASS=0; FAIL=0

cleanup() {
    if [ -n "$SERVE_PID" ] && kill -0 "$SERVE_PID" 2>/dev/null; then
        kill "$SERVE_PID" 2>/dev/null || true
    fi
    # Also kill any other process that grabbed the port (e.g. from a prior run)
    lsof -ti "tcp:${EVE_PORT}" 2>/dev/null | xargs kill -9 2>/dev/null || true
    "$KWAAINET" storage destroy --yes 2>/dev/null || true
}
trap cleanup EXIT

# ---------------------------------------------------------------------------
# 0. Pre-flight
# ---------------------------------------------------------------------------
step "Pre-flight"

if [ ! -x "$KWAAINET" ]; then
    echo -e "${RED}binary not found: $KWAAINET${RESET}"
    echo "Build with: cd core && cargo build --release -p kwaainet"
    exit 1
fi
pass "binary: $KWAAINET"

command -v curl  >/dev/null || { echo "curl required"; exit 1; }
command -v python3 >/dev/null || { echo "python3 required"; exit 1; }
pass "curl + python3 available"

# Evict anything already holding the test port so we start clean.
if lsof -ti "tcp:${EVE_PORT}" >/dev/null 2>&1; then
    warn "port ${EVE_PORT} already in use — killing stale process"
    lsof -ti "tcp:${EVE_PORT}" | xargs kill -9 2>/dev/null || true
    sleep 0.5
fi

# ---------------------------------------------------------------------------
# 1. Init
# ---------------------------------------------------------------------------
step "Storage init"

"$KWAAINET" storage destroy --yes 2>/dev/null || true
"$KWAAINET" storage init --capacity-gb 1 --port "$EVE_PORT" 2>&1 | tail -5
pass "storage init succeeded"

# ---------------------------------------------------------------------------
# 2. Serve
# ---------------------------------------------------------------------------
step "Start API server"

"$KWAAINET" storage serve &
SERVE_PID=$!
info "serve PID=$SERVE_PID"

# Poll until health endpoint responds (up to 10 s)
for i in $(seq 1 20); do
    if curl -sf "$API/health" >/dev/null 2>&1; then
        pass "health endpoint reachable (${i}×0.5 s)"
        break
    fi
    sleep 0.5
    if [ "$i" -eq 20 ]; then
        fail "health endpoint never responded"
        exit 1
    fi
done

HEALTH=$(curl -sf "$API/health")
info "health: $HEALTH"
echo "$HEALTH" | python3 -c "import sys,json; d=json.load(sys.stdin); exit(0 if d['status']=='ok' else 1)" \
    && pass "health.status == ok" || fail "health.status != ok"

# ---------------------------------------------------------------------------
# 3. Create tenant
# ---------------------------------------------------------------------------
step "Tenant CRUD"

TENANT=$(curl -sf -X POST "$API/tenants" \
    -H "Content-Type: application/json" \
    -d '{"peer_id":"test-bob-peer","capacity_limit_mb":256,"display_name":"Test Bob","vector_dimension":8}')
info "create tenant: $TENANT"

TENANT_ID=$(echo "$TENANT" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_id'])")
[ -n "$TENANT_ID" ] && pass "tenant created: $TENANT_ID" || fail "no tenant_id in response"

# List tenants
LIST=$(curl -sf "$API/tenants")
COUNT=$(echo "$LIST" | python3 -c "import sys,json; print(len(json.load(sys.stdin)))")
[ "$COUNT" -eq 1 ] && pass "list tenants: 1 tenant" || fail "list tenants: expected 1, got $COUNT"

# Get single tenant
GET=$(curl -sf "$API/tenants/$TENANT_ID")
echo "$GET" | python3 -c "import sys,json; d=json.load(sys.stdin); exit(0 if d['status']=='Active' else 1)" \
    && pass "get tenant: status=Active" || fail "get tenant returned wrong status"

# ---------------------------------------------------------------------------
# 4. Upload vectors
# ---------------------------------------------------------------------------
step "Vector upload"

# Generate 5 vectors of dimension 8 using python.
# Vector 3 (id=3) is a distinct unit vector pointing in the [1,0,...] direction.
VECTORS=$(python3 - <<'EOF'
import json, random, math
random.seed(42)

def rand_unit(dim):
    v = [random.gauss(0, 1) for _ in range(dim)]
    norm = math.sqrt(sum(x*x for x in v))
    return [x/norm for x in v]

dim = 8
vecs = []
for i in range(1, 6):
    if i == 3:
        v = [1.0] + [0.0]*(dim-1)   # canonical query target
    else:
        v = rand_unit(dim)
    vecs.append({"id": i, "embedding": v})

print(json.dumps({"vectors": vecs}))
EOF
)

UPLOAD=$(curl -sf -X POST "$API/tenants/$TENANT_ID/vectors" \
    -H "Content-Type: application/json" \
    -d "$VECTORS")
info "upload: $UPLOAD"
UPLOADED=$(echo "$UPLOAD" | python3 -c "import sys,json; print(json.load(sys.stdin)['uploaded'])")
[ "$UPLOADED" -eq 5 ] && pass "uploaded 5 vectors" || fail "expected 5 uploaded, got $UPLOADED"

# Stats after upload
STATS=$(curl -sf "$API/tenants/$TENANT_ID")
VC=$(echo "$STATS" | python3 -c "import sys,json; print(json.load(sys.stdin)['vector_count'])")
[ "$VC" -eq 5 ] && pass "tenant stats: vector_count=5" || fail "expected vector_count=5, got $VC"

# ---------------------------------------------------------------------------
# 5. ANN search
# ---------------------------------------------------------------------------
step "ANN search"

# Query vector identical to id=3 — should be top result.
SEARCH_BODY='{"query":[1.0,0.0,0.0,0.0,0.0,0.0,0.0,0.0],"top_k":3}'
RESULTS=$(curl -sf -X POST "$API/tenants/$TENANT_ID/search" \
    -H "Content-Type: application/json" \
    -d "$SEARCH_BODY")
info "search results: $RESULTS"

TOP_ID=$(echo "$RESULTS" | python3 -c "import sys,json; r=json.load(sys.stdin)['results']; print(r[0]['id']) if r else print(-1)")
[ "$TOP_ID" -eq 3 ] && pass "top result is id=3 (exact match)" || fail "expected id=3 at top, got $TOP_ID"

RES_COUNT=$(echo "$RESULTS" | python3 -c "import sys,json; print(len(json.load(sys.stdin)['results']))")
[ "$RES_COUNT" -eq 3 ] && pass "returned 3 results (top_k=3)" || fail "expected 3 results, got $RES_COUNT"

# ---------------------------------------------------------------------------
# 6. Delete vectors
# ---------------------------------------------------------------------------
step "Vector delete"

DEL=$(curl -sf -X DELETE "$API/tenants/$TENANT_ID/vectors" \
    -H "Content-Type: application/json" \
    -d '{"ids":[1,2]}')
info "delete vectors: $DEL"
DELETED=$(echo "$DEL" | python3 -c "import sys,json; print(json.load(sys.stdin)['deleted'])")
[ "$DELETED" -eq 2 ] && pass "deleted 2 vectors" || fail "expected 2 deleted, got $DELETED"

STATS2=$(curl -sf "$API/tenants/$TENANT_ID")
VC2=$(echo "$STATS2" | python3 -c "import sys,json; print(json.load(sys.stdin)['vector_count'])")
[ "$VC2" -eq 3 ] && pass "vector_count after delete: 3" || fail "expected 3, got $VC2"

# ---------------------------------------------------------------------------
# 7. Delete tenant
# ---------------------------------------------------------------------------
step "Tenant delete"

HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" -X DELETE "$API/tenants/$TENANT_ID")
[ "$HTTP_CODE" -eq 204 ] && pass "delete tenant: 204 No Content" || fail "delete tenant: expected 204, got $HTTP_CODE"

HTTP_404=$(curl -s -o /dev/null -w "%{http_code}" "$API/tenants/$TENANT_ID")
[ "$HTTP_404" -eq 404 ] && pass "deleted tenant returns 404" || fail "expected 404, got $HTTP_404"

# ---------------------------------------------------------------------------
# 8. Final health check
# ---------------------------------------------------------------------------
step "Final health"

HEALTH2=$(curl -sf "$API/health")
TC=$(echo "$HEALTH2" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_count'])")
[ "$TC" -eq 0 ] && pass "health: tenant_count=0 after cleanup" || fail "expected 0 tenants, got $TC"

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo -e "${BOLD}═══════════════════════════════════════${RESET}"
echo -e "${BOLD}  Storage Fabric Smoke Test — Results   ${RESET}"
echo -e "${BOLD}═══════════════════════════════════════${RESET}"
echo -e "  ${GREEN}Passed: $PASS${RESET}"
if [ "$FAIL" -gt 0 ]; then
    echo -e "  ${RED}Failed: $FAIL${RESET}"
    echo ""
    exit 1
else
    echo -e "  ${GREEN}All checks passed ✅${RESET}"
fi
echo ""
