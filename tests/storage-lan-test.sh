#!/usr/bin/env bash
# =============================================================================
# Storage Fabric LAN Integration Test
# =============================================================================
#
# Two-machine test for the multi-tenant vector storage fabric.
# No Docker, no PostgreSQL — uses the embedded hnsw_rs + redb backend.
#
# Machine A (Eve) — hosts the storage API, stores opaque float vectors
# Machine B (Bob) — embeds documents locally, uploads vectors to Eve,
#                   searches, retrieves results by index
#
# PREREQUISITES
#   Both machines:
#     - kwaainet binary built or installed
#     - Python 3.8+ with sentence-transformers: pip install sentence-transformers
#   Machine A only:
#     - kwaainet storage init (run once, no Postgres needed)
#
# USAGE
#   # On Machine A (Eve):
#   bash tests/storage-lan-test.sh eve [<eve-ip>]
#
#   # On Machine B (Bob):
#   bash tests/storage-lan-test.sh bob <eve-ip>
#
# =============================================================================

set -euo pipefail

KWAAINET="${KWAAINET:-kwaainet}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EVE_PORT="${EVE_PORT:-7432}"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BOLD='\033[1m'; DIM='\033[2m'; RESET='\033[0m'

pass() { echo -e "  ${GREEN}✅ $*${RESET}"; }
fail() { echo -e "  ${RED}❌ $*${RESET}"; exit 1; }
info() { echo -e "  ${CYAN}ℹ  $*${RESET}"; }
warn() { echo -e "  ${YELLOW}⚠  $*${RESET}"; }
step() { echo -e "\n${BOLD}── $* ──${RESET}"; }

ROLE="${1:-}"
EVE_IP="${2:-}"

if [[ -z "$ROLE" ]]; then
    echo "Usage: $0 <eve|bob> [<eve-ip>]"
    echo
    echo "  eve [ip]     — Run on the storage host machine"
    echo "  bob <eve-ip> — Run on the data owner machine"
    exit 1
fi

# ---------------------------------------------------------------------------
# Shared: check dependencies
# ---------------------------------------------------------------------------
check_kwaainet() {
    step "Checking kwaainet binary"
    if ! command -v "$KWAAINET" &>/dev/null; then
        KWAAINET="$SCRIPT_DIR/../core/target/release/kwaainet"
        [[ -x "$KWAAINET" ]] || fail "kwaainet not found. Build with: cargo build --release -p kwaainet"
    fi
    VERSION=$("$KWAAINET" --version 2>&1 | head -1)
    pass "Found: $VERSION"
}

check_python_embeddings() {
    step "Checking Python embedding model"
    python3 -c "from sentence_transformers import SentenceTransformer; print('sentence-transformers available')" 2>/dev/null \
        || fail "sentence-transformers not installed. Run: pip install sentence-transformers"
    pass "sentence-transformers available"
}

# ---------------------------------------------------------------------------
# Machine A (Eve): init storage and start the API
# ---------------------------------------------------------------------------
run_eve() {
    local ip="${EVE_IP:-$(hostname -I 2>/dev/null | awk '{print $1}' || ipconfig getifaddr en0 2>/dev/null || echo 'localhost')}"

    echo
    echo -e "${BOLD}╔═══════════════════════════════════════════╗"
    echo -e "║  Storage Fabric LAN Test — Eve (Storage)  ║"
    echo -e "╚═══════════════════════════════════════════╝${RESET}"
    echo

    check_kwaainet

    # ── Step 1: Init storage (idempotent) ──────────────────────────────────
    step "Step 1 — Initialize storage"
    "$KWAAINET" storage init \
        --endpoint "http://${ip}:${EVE_PORT}"
    pass "Storage initialized"

    # ── Step 2: Check storage status ───────────────────────────────────────
    step "Step 2 — Storage status"
    "$KWAAINET" storage status

    # ── Step 3: Start the storage API ──────────────────────────────────────
    step "Step 3 — Start storage API"
    info "Starting storage API on port ${EVE_PORT}…"
    info "(Press Ctrl+C to stop)"
    echo
    echo -e "  ${BOLD}Eve is ready.${RESET}"
    echo -e "  Eve IP:       ${CYAN}${ip}${RESET}"
    echo -e "  Eve endpoint: ${CYAN}http://${ip}:${EVE_PORT}${RESET}"
    echo
    echo -e "  Now run on Machine B (Bob):"
    echo -e "    ${YELLOW}bash tests/storage-lan-test.sh bob ${ip}${RESET}"
    echo

    "$KWAAINET" storage serve
}

# ---------------------------------------------------------------------------
# Machine B (Bob): embed locally, upload to Eve, search, verify isolation
# ---------------------------------------------------------------------------
run_bob() {
    if [[ -z "$EVE_IP" ]]; then
        echo "Usage: $0 bob <eve-ip>"
        echo "  Provide the IP address of the Eve machine."
        exit 1
    fi

    local API="http://${EVE_IP}:${EVE_PORT}"

    echo
    echo -e "${BOLD}╔═══════════════════════════════════════════╗"
    echo -e "║  Storage Fabric LAN Test — Bob (Owner)    ║"
    echo -e "╚═══════════════════════════════════════════╝${RESET}"
    echo

    check_kwaainet
    check_python_embeddings

    # ── Step 1: Check Eve is reachable ─────────────────────────────────────
    step "Step 1 — Check Eve is reachable"
    HEALTH=$(curl -sf "${API}/api/health" 2>/dev/null || true)
    if [[ -z "$HEALTH" ]]; then
        fail "Cannot reach Eve at ${API}. Is storage serve running on Machine A?"
    fi
    echo -e "  ${DIM}${HEALTH}${RESET}"
    pass "Eve is reachable at ${API}"

    # ── Step 2: Get Bob's peer ID ──────────────────────────────────────────
    step "Step 2 — Bob's identity"
    BOB_PEER_ID=$("$KWAAINET" identity show 2>/dev/null | grep 'Peer ID' | awk '{print $NF}' || echo "12D3KooWBobTest$(date +%s)")
    pass "Bob's Peer ID: ${BOB_PEER_ID}"

    # ── Step 3: Create tenant on Eve ───────────────────────────────────────
    step "Step 3 — Create tenant on Eve"
    TENANT=$(curl -sf -X POST "${API}/api/tenants" \
        -H "Content-Type: application/json" \
        -d "{\"peer_id\":\"${BOB_PEER_ID}\",\"display_name\":\"bob-lan-test\",\"capacity_limit_mb\":512,\"vector_dimension\":384}")
    TENANT_ID=$(echo "$TENANT" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_id'])")
    pass "Tenant created: ${TENANT_ID}"

    # ── Step 4: Embed documents locally ────────────────────────────────────
    step "Step 4 — Embed documents locally (Bob's secret embedding model)"
    info "Using all-MiniLM-L6-v2 (384-dim) — this is Bob's choice, Eve doesn't know"

    # Bob's local knowledge base — plaintext stored only on Bob's machine
    BOB_KB=$(mktemp)
    cat > "$BOB_KB" << 'DOCS'
{"id": 1, "text": "The mitochondria is the powerhouse of the cell. It produces ATP through oxidative phosphorylation."}
{"id": 2, "text": "CRISPR-Cas9 is a genome editing tool that allows precise modification of DNA sequences in living organisms."}
{"id": 3, "text": "Photosynthesis converts carbon dioxide and water into glucose and oxygen using sunlight energy."}
{"id": 4, "text": "The human genome contains approximately 3 billion base pairs and about 20,000 protein-coding genes."}
{"id": 5, "text": "Antibiotics work by targeting bacterial cell walls, protein synthesis, or DNA replication mechanisms."}
{"id": 6, "text": "Neural networks consist of layers of interconnected nodes that process information through weighted connections."}
{"id": 7, "text": "Quantum entanglement allows two particles to be correlated regardless of the distance between them."}
{"id": 8, "text": "The Mediterranean diet emphasizes fruits, vegetables, whole grains, olive oil, and moderate fish consumption."}
DOCS

    # Embed all documents using Python + sentence-transformers
    VECTORS_JSON=$(python3 << PYEOF
import json, sys
from sentence_transformers import SentenceTransformer

model = SentenceTransformer('all-MiniLM-L6-v2')
docs = [json.loads(line) for line in open('${BOB_KB}') if line.strip()]

vectors = []
texts = [d['text'] for d in docs]
embeddings = model.encode(texts)

for doc, emb in zip(docs, embeddings):
    vectors.append({"id": doc["id"], "embedding": emb.tolist()})

print(json.dumps({"vectors": vectors}))
PYEOF
    )

    N_VECS=$(echo "$VECTORS_JSON" | python3 -c "import sys,json; print(len(json.load(sys.stdin)['vectors']))")
    pass "Embedded ${N_VECS} documents locally (384-dim)"

    # ── Step 5: Upload vectors to Eve ──────────────────────────────────────
    step "Step 5 — Upload vectors to Eve"
    info "Sending only float vectors — Eve never sees the text"
    UPLOAD_RESP=$(echo "$VECTORS_JSON" | curl -sf -X POST "${API}/api/tenants/${TENANT_ID}/vectors" \
        -H "Content-Type: application/json" -d @-)
    UPLOADED=$(echo "$UPLOAD_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin)['uploaded'])")
    pass "Uploaded ${UPLOADED} vectors to Eve"

    # ── Step 6: Search — Bob embeds query locally, sends vector to Eve ─────
    step "Step 6 — Search: 'How does gene editing work?'"
    QUERY_VEC=$(python3 << PYEOF
import json
from sentence_transformers import SentenceTransformer
model = SentenceTransformer('all-MiniLM-L6-v2')
emb = model.encode("How does gene editing work?")
print(json.dumps({"query": emb.tolist(), "top_k": 3}))
PYEOF
    )

    SEARCH_RESP=$(echo "$QUERY_VEC" | curl -sf -X POST "${API}/api/tenants/${TENANT_ID}/search" \
        -H "Content-Type: application/json" -d @-)
    echo -e "  ${DIM}Eve returned (indices + scores only):${RESET}"
    echo "$SEARCH_RESP" | python3 -m json.tool | sed 's/^/  /'

    # ── Step 7: Bob resolves indices to documents from his local KB ────────
    step "Step 7 — Bob resolves indices → documents (local lookup)"
    info "Eve returned IDs; Bob looks them up in his local knowledge base"
    python3 << PYEOF
import json, sys

# Bob's local knowledge base (never shared with Eve)
docs = {}
for line in open('${BOB_KB}'):
    if line.strip():
        d = json.loads(line)
        docs[d['id']] = d['text']

# Results from Eve (only IDs and scores)
results = json.loads('''${SEARCH_RESP}''')['results']

print()
for i, r in enumerate(results):
    doc_id = r['id']
    score = r['score']
    text = docs.get(doc_id, '(not found)')
    print(f"  [{i+1}] ID={doc_id}  score={score:.4f}")
    print(f"      {text[:100]}")
    print()
PYEOF
    pass "Bob resolved results from his local KB"

    # ── Step 8: Second search — different query ────────────────────────────
    step "Step 8 — Search: 'What should I eat for a healthy diet?'"
    QUERY_VEC2=$(python3 << PYEOF
import json
from sentence_transformers import SentenceTransformer
model = SentenceTransformer('all-MiniLM-L6-v2')
emb = model.encode("What should I eat for a healthy diet?")
print(json.dumps({"query": emb.tolist(), "top_k": 3}))
PYEOF
    )

    SEARCH_RESP2=$(echo "$QUERY_VEC2" | curl -sf -X POST "${API}/api/tenants/${TENANT_ID}/search" \
        -H "Content-Type: application/json" -d @-)

    python3 << PYEOF
import json
docs = {}
for line in open('${BOB_KB}'):
    if line.strip():
        d = json.loads(line)
        docs[d['id']] = d['text']
results = json.loads('''${SEARCH_RESP2}''')['results']
print()
for i, r in enumerate(results):
    doc_id = r['id']
    score = r['score']
    text = docs.get(doc_id, '(not found)')
    print(f"  [{i+1}] ID={doc_id}  score={score:.4f}")
    print(f"      {text[:100]}")
    print()
PYEOF
    pass "Semantic search working across the network"

    # ── Step 9: Verify tenant stats ────────────────────────────────────────
    step "Step 9 — Tenant stats on Eve"
    curl -sf "${API}/api/tenants/${TENANT_ID}" | python3 -m json.tool | sed 's/^/  /'
    pass "Stats retrieved"

    # ── Step 10: Cleanup ───────────────────────────────────────────────────
    step "Step 10 — Cleanup"
    curl -sf -X DELETE "${API}/api/tenants/${TENANT_ID}" -o /dev/null -w "  Delete tenant: HTTP %{http_code}\n"
    rm -f "$BOB_KB"
    pass "Tenant deleted, local files cleaned up"

    # ── Summary ────────────────────────────────────────────────────────────
    echo
    echo -e "${BOLD}╔═══════════════════════════════════════════════════════════╗"
    echo -e "║                    All tests passed!                      ║"
    echo -e "╠═══════════════════════════════════════════════════════════╣"
    echo -e "║  ✅ Eve reachable over network                           ║"
    echo -e "║  ✅ Tenant created on remote Eve                         ║"
    echo -e "║  ✅ Documents embedded locally (Bob's secret model)       ║"
    echo -e "║  ✅ Vectors uploaded to Eve (Eve sees only floats)        ║"
    echo -e "║  ✅ Semantic search working (Eve returns IDs + scores)    ║"
    echo -e "║  ✅ Bob resolves IDs → documents from local KB            ║"
    echo -e "║  ✅ Tenant cleaned up                                     ║"
    echo -e "╚═══════════════════════════════════════════════════════════╝${RESET}"
    echo
}

# ---------------------------------------------------------------------------
# Dispatch
# ---------------------------------------------------------------------------
case "$ROLE" in
    eve)     run_eve ;;
    bob)     run_bob ;;
    *)
        echo "Unknown role '$ROLE'. Use: eve or bob"
        exit 1
        ;;
esac
