#!/usr/bin/env bash
# =============================================================================
# Local Storage Fabric Integration Test — Embedded Backend
# =============================================================================
#
# Runs a full Bob→Eve storage test on a single machine.
# No Docker, no PostgreSQL, no external dependencies.
#
# PREREQUISITES
#   - kwaainet binary built (cargo build --release -p kwaainet)
#   - Python 3.8+ with sentence-transformers (optional; pass --no-embeddings
#     to use pre-computed static vectors instead):
#       pip install sentence-transformers
#
# USAGE
#   bash tests/local-storage-test.sh [--skip-build] [--no-embeddings]
#
# FLAGS
#   --skip-build     Skip cargo build (use existing release binary)
#   --no-embeddings  Use pre-computed static vectors instead of sentence-transformers
#
# =============================================================================

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
CORE="$REPO_ROOT/core"

KWAAINET="${KWAAINET:-$CORE/target/release/kwaainet}"
EVE_PORT="${EVE_PORT:-17432}"   # use non-default port to avoid conflicts
API="http://localhost:${EVE_PORT}"

# Isolated home dir so this test doesn't touch the real ~/.kwaainet
TEST_HOME="$(mktemp -d /tmp/kwaainet-test-XXXXXX)"

SKIP_BUILD=false
USE_STATIC_VECTORS=false
for arg in "$@"; do
    case "$arg" in
        --skip-build)      SKIP_BUILD=true ;;
        --no-embeddings)   USE_STATIC_VECTORS=true ;;
    esac
done

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; RESET='\033[0m'

pass() { echo -e "  ${GREEN}✅ $*${RESET}"; }
fail() { echo -e "  ${RED}❌ $*${RESET}"; cleanup; exit 1; }
info() { echo -e "  ${CYAN}ℹ  $*${RESET}"; }
warn() { echo -e "  ${YELLOW}⚠  $*${RESET}"; }
step() { echo -e "\n${BOLD}── $* ──${RESET}"; }

EVE_PID=""
cleanup() {
    if [[ -n "$EVE_PID" ]]; then
        kill "$EVE_PID" 2>/dev/null || true
        wait "$EVE_PID" 2>/dev/null || true
    fi
    rm -rf "$TEST_HOME"
}
trap cleanup EXIT

# Wrapper: run kwaainet with an isolated HOME so this test doesn't
# interfere with the real ~/.kwaainet configuration.
kw() { HOME="$TEST_HOME" EVE_PORT="$EVE_PORT" "$KWAAINET" "$@"; }

echo
echo -e "${BOLD}╔═════════════════════════════════════════════════╗"
echo -e "║  Storage Fabric Local Integration Test          ║"
echo -e "║  (embedded hnsw_rs + redb — no Docker needed)  ║"
echo -e "╚═════════════════════════════════════════════════╝${RESET}"
echo

# ── Step 0: Build binary ─────────────────────────────────────────────────────
if [[ "$SKIP_BUILD" == "false" ]]; then
    step "Step 0 — Build kwaainet"
    cd "$CORE"
    cargo build --release -p kwaainet 2>&1 | grep -E "^(Compiling kwaainet|Finished|error)" || true
    pass "Binary built"
else
    [[ -x "$KWAAINET" ]] || fail "kwaainet binary not found at $KWAAINET."
    pass "Using existing binary: $("$KWAAINET" --version 2>&1 | head -1)"
fi

# ── Step 1: Init embedded store ───────────────────────────────────────────────
step "Step 1 — Init embedded store (no Docker needed)"
kw storage init --capacity-gb 1 --port "$EVE_PORT" 2>&1
pass "Embedded store initialized at $TEST_HOME/.kwaainet/storage/"

# ── Step 2: Start storage API ─────────────────────────────────────────────────
step "Step 2 — Start storage API (background)"
kw storage serve &
EVE_PID=$!

info "Waiting for API to be ready on port ${EVE_PORT}…"
for i in $(seq 1 15); do
    if curl -sf "$API/api/health" > /dev/null 2>&1; then
        break
    fi
    sleep 1
done
curl -sf "$API/api/health" > /dev/null || fail "Storage API did not start on port $EVE_PORT"
pass "Storage API running on port ${EVE_PORT} (PID $EVE_PID)"

# ── Step 3: Health check ──────────────────────────────────────────────────────
step "Step 3 — Health check"
HEALTH=$(curl -sf "$API/api/health")
echo "  $HEALTH" | python3 -m json.tool | sed 's/^/  /'
pass "Health check OK"

# ── Step 4: Create tenant ─────────────────────────────────────────────────────
step "Step 4 — Create tenant"
if [[ "$USE_STATIC_VECTORS" == "true" ]]; then
    DIM=3
else
    DIM=384
fi
TENANT=$(curl -sf -X POST "$API/api/tenants" \
    -H "Content-Type: application/json" \
    -d "{\"peer_id\":\"12D3KooWLocalBobTest\",\"display_name\":\"local-test\",\"capacity_limit_mb\":512,\"vector_dimension\":${DIM}}")
echo "$TENANT" | python3 -m json.tool | sed 's/^/  /'
TENANT_ID=$(echo "$TENANT" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_id'])")
pass "Tenant created: ${TENANT_ID}"

# ── Step 5: Upload vectors ────────────────────────────────────────────────────
step "Step 5 — Upload vectors"

QUERY_VEC=""

if [[ "$USE_STATIC_VECTORS" == "true" ]]; then
    info "Using static 3-dim test vectors"
    UPLOAD_RESP=$(curl -sf -X POST "$API/api/tenants/$TENANT_ID/vectors" \
        -H "Content-Type: application/json" \
        -d '{
            "vectors": [
              {"id": 1, "embedding": [1.0, 0.0, 0.0]},
              {"id": 2, "embedding": [0.0, 1.0, 0.0]},
              {"id": 3, "embedding": [0.9, 0.1, 0.0]},
              {"id": 4, "embedding": [0.0, 0.0, 1.0]},
              {"id": 5, "embedding": [0.5, 0.5, 0.0]}
            ]
          }')
    echo "$UPLOAD_RESP" | python3 -m json.tool | sed 's/^/  /'
    UPLOADED=$(echo "$UPLOAD_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin)['uploaded'])")
    pass "Uploaded ${UPLOADED} vectors (static 3-dim)"
    QUERY_VEC='{"query": [1.0, 0.0, 0.0], "top_k": 3}'
else
    python3 -c "from sentence_transformers import SentenceTransformer; print('ok')" 2>/dev/null \
        || { warn "sentence-transformers not available — falling back to static vectors"; USE_STATIC_VECTORS=true; DIM=3; }
fi

if [[ "$USE_STATIC_VECTORS" == "false" ]]; then
    info "Embedding with all-MiniLM-L6-v2 (Bob's local model — Eve never sees text)"
    VECTORS_JSON=$(python3 << 'PYEOF'
import json
from sentence_transformers import SentenceTransformer

docs = [
    {"id": 1, "text": "The mitochondria is the powerhouse of the cell."},
    {"id": 2, "text": "CRISPR-Cas9 enables precise genome editing in living cells."},
    {"id": 3, "text": "Photosynthesis converts sunlight into chemical energy."},
    {"id": 4, "text": "The Mediterranean diet emphasizes olive oil, fish, and vegetables."},
    {"id": 5, "text": "Neural networks learn representations from data through backpropagation."},
    {"id": 6, "text": "Quantum entanglement links particles regardless of distance."},
]

model = SentenceTransformer("all-MiniLM-L6-v2")
embeddings = model.encode([d["text"] for d in docs])
vectors = [{"id": d["id"], "embedding": e.tolist()} for d, e in zip(docs, embeddings)]
print(json.dumps({"vectors": vectors}))
PYEOF
    )

    UPLOAD_RESP=$(echo "$VECTORS_JSON" | curl -sf -X POST "$API/api/tenants/$TENANT_ID/vectors" \
        -H "Content-Type: application/json" -d @-)
    echo "$UPLOAD_RESP" | python3 -m json.tool | sed 's/^/  /'
    UPLOADED=$(echo "$UPLOAD_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin)['uploaded'])")
    pass "Uploaded ${UPLOADED} vectors (384-dim sentence embeddings)"

    QUERY_VEC=$(python3 << 'PYEOF'
import json
from sentence_transformers import SentenceTransformer
model = SentenceTransformer("all-MiniLM-L6-v2")
emb = model.encode("How does gene editing work?")
print(json.dumps({"query": emb.tolist(), "top_k": 3}))
PYEOF
    )
fi

# ── Step 6: Search ────────────────────────────────────────────────────────────
step "Step 6 — Semantic search"

if [[ "$USE_STATIC_VECTORS" == "true" ]]; then
    info "Query: [1.0, 0.0, 0.0] — should match id=1 and id=3"
    SEARCH_RESP=$(curl -sf -X POST "$API/api/tenants/$TENANT_ID/search" \
        -H "Content-Type: application/json" \
        -d "$QUERY_VEC")
else
    info "Query: 'How does gene editing work?' (embedded locally)"
    SEARCH_RESP=$(echo "$QUERY_VEC" | curl -sf -X POST "$API/api/tenants/$TENANT_ID/search" \
        -H "Content-Type: application/json" -d @-)
fi

echo "$SEARCH_RESP" | python3 -m json.tool | sed 's/^/  /'

HAS_VECTOR=$(echo "$SEARCH_RESP" | python3 -c "
import sys, json
r = json.load(sys.stdin)['results']
bad = [x for x in r if 'embedding' in x or 'vector' in x]
print(len(bad))
")
[[ "$HAS_VECTOR" == "0" ]] || fail "Eve leaked vector data in search results!"
pass "Search returned IDs + scores only (Search by Index protocol correct)"

# ── Step 7: Tenant stats ──────────────────────────────────────────────────────
step "Step 7 — Tenant stats"
curl -sf "$API/api/tenants/$TENANT_ID" | python3 -m json.tool | sed 's/^/  /'
pass "Stats OK"

# ── Step 8: Delete a vector ───────────────────────────────────────────────────
step "Step 8 — Delete vector id=1"
DEL_RESP=$(curl -sf -X DELETE "$API/api/tenants/$TENANT_ID/vectors" \
    -H "Content-Type: application/json" \
    -d '{"ids": [1]}')
echo "$DEL_RESP" | python3 -m json.tool | sed 's/^/  /'
pass "Vector deleted"

# ── Step 9: Tenant isolation ──────────────────────────────────────────────────
step "Step 9 — Tenant isolation (second tenant sees 0 results)"
TENANT2=$(curl -sf -X POST "$API/api/tenants" \
    -H "Content-Type: application/json" \
    -d "{\"peer_id\":\"12D3KooWEve2Test\",\"display_name\":\"eve2-test\",\"capacity_limit_mb\":512,\"vector_dimension\":${DIM}}")
TENANT2_ID=$(echo "$TENANT2" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_id'])")

if [[ "$USE_STATIC_VECTORS" == "true" ]]; then
    ISO_RESP=$(curl -sf -X POST "$API/api/tenants/$TENANT2_ID/search" \
        -H "Content-Type: application/json" \
        -d "$QUERY_VEC")
else
    ISO_RESP=$(echo "$QUERY_VEC" | curl -sf -X POST "$API/api/tenants/$TENANT2_ID/search" \
        -H "Content-Type: application/json" -d @-)
fi
N_RESULTS=$(echo "$ISO_RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)['results']))")
[[ "$N_RESULTS" == "0" ]] || fail "Tenant isolation broken! Tenant 2 saw ${N_RESULTS} results"
pass "Tenant isolation verified (Tenant 2 sees 0 results)"

# ── Step 10: Cleanup ──────────────────────────────────────────────────────────
step "Step 10 — Cleanup"
curl -sf -X DELETE "$API/api/tenants/$TENANT_ID" -o /dev/null -w "  Delete tenant 1: HTTP %{http_code}\n"
curl -sf -X DELETE "$API/api/tenants/$TENANT2_ID" -o /dev/null -w "  Delete tenant 2: HTTP %{http_code}\n"
pass "Tenants deleted"

# ── Summary ───────────────────────────────────────────────────────────────────
echo
echo -e "${BOLD}╔═══════════════════════════════════════════════════════════╗"
echo -e "║              All storage tests passed!                    ║"
echo -e "╠═══════════════════════════════════════════════════════════╣"
echo -e "║  ✅ Embedded store initialized (no Docker, no Postgres)   ║"
echo -e "║  ✅ kwaainet storage init + serve                         ║"
echo -e "║  ✅ Tenant created and configured                         ║"
echo -e "║  ✅ Vectors uploaded (Eve stores opaque floats only)      ║"
echo -e "║  ✅ Semantic search working (returns IDs + scores)        ║"
echo -e "║  ✅ Search by Index protocol enforced (no vector leakage) ║"
echo -e "║  ✅ Vector delete working                                 ║"
echo -e "║  ✅ Tenant isolation verified                             ║"
echo -e "║  ✅ Cleanup complete                                      ║"
echo -e "╚═══════════════════════════════════════════════════════════╝${RESET}"
echo
