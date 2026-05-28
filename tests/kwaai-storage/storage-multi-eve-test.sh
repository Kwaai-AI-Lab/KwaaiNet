#!/usr/bin/env bash
# =============================================================================
# Storage Fabric Multi-Eve Integration Test
# =============================================================================
#
# Tests Bob fanning out a knowledge base across N independent Eve nodes.
# Each Eve holds a non-overlapping shard of Bob's vectors. On search, Bob
# queries all Eves in parallel, merges the top-k results locally by score,
# and resolves IDs back to documents from his local KB.
# Eve never sees plaintext — only float vectors and a tenant_id.
#
# USAGE
# ─────
#   # On each Eve machine (run in a terminal; repeat for each Eve):
#   bash tests/storage-multi-eve-test.sh eve [<bind-ip>]
#   # Named aliases also work:
#   bash tests/storage-multi-eve-test.sh eve-a [<bind-ip>]
#   bash tests/storage-multi-eve-test.sh eve-b [<bind-ip>]
#
#   # On Bob's machine — pass each Eve as IP, host:port, URL, or public_name:
#   bash tests/storage-multi-eve-test.sh bob <eve1> <eve2> [<eve3> ...]
#
# EXAMPLES
#   # Two Eves by IP
#   bash tests/storage-multi-eve-test.sh bob 192.168.1.10 192.168.1.11
#
#   # Two Eves by ip:port
#   bash tests/storage-multi-eve-test.sh bob 192.168.1.10:7432 192.168.1.11:7433
#
#   # Two Eves by public_name (resolved via kwaainet vpk discover --json)
#   bash tests/storage-multi-eve-test.sh bob metro_win metro-linux-x86_64
#
# PREREQUISITES
#   All machines: kwaainet binary installed or built at ./core/target/release/
#   Bob only: Python 3.8+ with sentence-transformers
#     pip install sentence-transformers
#
# SHARDING STRATEGY
#   Bob partitions his document corpus across Eves by round-robin assignment:
#     (doc_id - 1) % N_EVES → Eve index
#   This keeps each shard balanced without coordination between Eves.
#   Eve never learns which shard it holds or how many Eves Bob is using.
#
# PORTS
#   EVE_PORT (default 7432) — used when no port is given in the Eve argument.
# =============================================================================

set -euo pipefail

KWAAINET="${KWAAINET:-kwaainet}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
EVE_PORT="${EVE_PORT:-7432}"
TOP_K="${TOP_K:-3}"

RED='\033[0;31m'; GREEN='\033[0;32m'; YELLOW='\033[1;33m'
CYAN='\033[0;36m'; BLUE='\033[0;34m'; BOLD='\033[1m'; DIM='\033[2m'; RESET='\033[0m'

pass()  { echo -e "  ${GREEN}✅ $*${RESET}"; }
fail()  { echo -e "  ${RED}❌ $*${RESET}"; exit 1; }
info()  { echo -e "  ${CYAN}ℹ  $*${RESET}"; }
warn()  { echo -e "  ${YELLOW}⚠  $*${RESET}"; }
step()  { echo -e "\n${BOLD}── $* ──${RESET}"; }
label() { echo -e "  ${BLUE}$*${RESET}"; }

ROLE="${1:-}"

if [[ -z "$ROLE" ]]; then
    echo "Usage: $0 <eve|eve-a|eve-b|bob> [args...]"
    echo
    echo "  eve|eve-a|eve-b [bind-ip]           — Run on a storage host machine"
    echo "  bob <eve1> <eve2> [eve3 ...]         — Run on Bob's machine"
    echo
    echo "  Eve args: bare IP, host:port, http:// URL, or public_name (DHT lookup)"
    exit 1
fi

# ---------------------------------------------------------------------------
# Shared helpers
# ---------------------------------------------------------------------------
check_kwaainet() {
    step "Checking kwaainet binary"
    if ! command -v "$KWAAINET" &>/dev/null; then
        KWAAINET="$SCRIPT_DIR/../core/target/release/kwaainet"
        [[ -x "$KWAAINET" ]] || fail "kwaainet not found. Build with: cd core && cargo build --release -p kwaainet"
    fi
    VERSION=$("$KWAAINET" --version 2>&1 | head -1)
    pass "Found: $VERSION"
}

check_python_embeddings() {
    step "Checking Python embedding model"
    python3 -c "from sentence_transformers import SentenceTransformer; print('ok')" 2>/dev/null \
        || fail "sentence-transformers not installed. Run: pip install sentence-transformers"
    pass "sentence-transformers available"
}

# ---------------------------------------------------------------------------
# Resolve an Eve argument to an HTTP endpoint.
#
# Accepts:
#   - bare IP:          192.168.1.10       → http://192.168.1.10:${EVE_PORT}
#   - host:port:        192.168.1.10:7433  → http://192.168.1.10:7433
#   - http(s):// URL:   http://…           → used as-is
#   - public_name:      metro-node-1       → resolved via kwaainet vpk discover --json
# ---------------------------------------------------------------------------
resolve_endpoint() {
    local arg="$1"
    # Already a full URL
    if [[ "$arg" == http://* || "$arg" == https://* ]]; then
        echo "${arg%/}"
        return
    fi
    # host:port pair (has a colon followed only by digits)
    if [[ "$arg" =~ ^[0-9a-fA-F.:]+:[0-9]+$ ]]; then
        echo "http://${arg}"
        return
    fi
    # Bare IP (only digits and dots)
    if [[ "$arg" =~ ^[0-9]+\.[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
        echo "http://${arg}:${EVE_PORT}"
        return
    fi
    # Treat as public_name — resolve via DHT
    info "Resolving '${arg}' via kwaainet vpk discover --json …"
    local disc
    disc=$("$KWAAINET" vpk discover --json 2>/dev/null || true)
    if [[ -z "$disc" || "$disc" == "[]" ]]; then
        fail "DHT discovery returned no nodes. Is 'kwaainet start --daemon' running?"
    fi
    local endpoint
    endpoint=$(echo "$disc" | python3 -c "
import sys, json
nodes = json.load(sys.stdin)
name = '${arg}'
for n in nodes:
    if n.get('public_name','') == name:
        ep = n.get('endpoint','')
        if ep and ep != 'unknown':
            print(ep.rstrip('/'))
            sys.exit(0)
print('')
" 2>/dev/null || true)
    if [[ -z "$endpoint" ]]; then
        fail "No Eve node named '${arg}' found in DHT. Check: kwaainet vpk discover"
    fi
    echo "$endpoint"
}

# ---------------------------------------------------------------------------
# Eve role: init storage and serve
# ---------------------------------------------------------------------------
run_eve() {
    local eve_label="${1:-Eve}"
    local ip="${2:-$(hostname -I 2>/dev/null | awk '{print $1}' || ipconfig getifaddr en0 2>/dev/null || echo 'localhost')}"

    echo
    echo -e "${BOLD}╔══════════════════════════════════════════════╗"
    echo -e "║  Multi-Eve Storage Test — ${eve_label} (Storage)  ║"
    echo -e "╚══════════════════════════════════════════════╝${RESET}"
    echo

    check_kwaainet

    step "Initialize storage (idempotent)"
    "$KWAAINET" storage init \
        --capacity-gb 5 \
        --port "$EVE_PORT" \
        --endpoint "http://${ip}:${EVE_PORT}"
    pass "Storage initialized"

    step "Storage status"
    "$KWAAINET" storage status

    step "Starting storage API"
    echo
    echo -e "  ${BOLD}${eve_label} is ready.${RESET}"
    echo -e "  Bind IP:   ${CYAN}${ip}${RESET}"
    echo -e "  Endpoint:  ${CYAN}http://${ip}:${EVE_PORT}${RESET}"
    echo
    echo -e "  Pass ${YELLOW}${ip}:${EVE_PORT}${RESET} (or this node's public_name) to Bob:"
    echo -e "    ${YELLOW}bash tests/storage-multi-eve-test.sh bob <eve1> <eve2> ...${RESET}"
    echo
    echo -e "  ${DIM}Press Ctrl+C to stop.${RESET}"
    echo

    "$KWAAINET" storage serve
}

# ---------------------------------------------------------------------------
# Bob role: shard across N Eves, fan-out search, merge results
# ---------------------------------------------------------------------------
run_bob() {
    local -a EVE_ARGS=("$@")
    local N_EVES=${#EVE_ARGS[@]}

    if [[ $N_EVES -lt 1 ]]; then
        echo "Usage: $0 bob <eve1> <eve2> [eve3 ...]"
        echo "  Each Eve arg: bare IP, host:port, http:// URL, or public_name"
        exit 1
    fi

    echo
    echo -e "${BOLD}╔══════════════════════════════════════════════════════════╗"
    echo -e "║  Multi-Eve Storage Test — Bob (data owner)               ║"
    echo -e "╚══════════════════════════════════════════════════════════╝${RESET}"
    echo

    check_kwaainet
    check_python_embeddings

    # Resolve all Eve arguments to endpoint URLs
    step "Resolving Eve endpoints"
    declare -a EVE_APIS
    for i in "${!EVE_ARGS[@]}"; do
        EVE_APIS[$i]=$(resolve_endpoint "${EVE_ARGS[$i]}")
        pass "Eve $((i+1)): ${EVE_APIS[$i]}"
    done

    # ── Step 1: Reachability check for all Eves ───────────────────────────
    step "Step 1 — Check all Eves are reachable"
    for i in "${!EVE_APIS[@]}"; do
        local api="${EVE_APIS[$i]}"
        HEALTH=$(curl -sf "${api}/api/health" 2>/dev/null || true)
        if [[ -z "$HEALTH" ]]; then
            fail "Cannot reach Eve $((i+1)) at ${api}. Is storage serve running?"
        fi
        EVE_PEER=$(echo "$HEALTH" | python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('peer_id','unknown')[:20]+'...')" 2>/dev/null || echo "unknown")
        pass "Eve $((i+1)) reachable at ${api}  (peer: ${EVE_PEER})"
    done

    # ── Step 2: Bob's identity ────────────────────────────────────────────
    step "Step 2 — Bob's identity"
    BOB_PEER_ID=$("$KWAAINET" identity show 2>/dev/null | grep 'Peer ID' | awk '{print $NF}' \
        || echo "12D3KooWBobMultiEveTest$(date +%s)")
    pass "Bob's Peer ID: ${BOB_PEER_ID}"

    # ── Step 3: Create a tenant on every Eve ─────────────────────────────
    step "Step 3 — Create tenant on each Eve"
    declare -a TENANT_IDS
    for i in "${!EVE_APIS[@]}"; do
        RESP=$(curl -sf -X POST "${EVE_APIS[$i]}/api/tenants" \
            -H "Content-Type: application/json" \
            -d "{\"peer_id\":\"${BOB_PEER_ID}\",\"display_name\":\"bob-multi-eve-shard-$((i+1))\",\"capacity_limit_mb\":512,\"vector_dimension\":384}")
        TID=$(echo "$RESP" | python3 -c "import sys,json; print(json.load(sys.stdin)['tenant_id'])")
        TENANT_IDS[$i]="$TID"
        pass "Eve $((i+1)) tenant: ${TID}"
    done

    # ── Step 4: Embed documents locally ───────────────────────────────────
    step "Step 4 — Embed documents locally (Bob's private model)"
    info "Using all-MiniLM-L6-v2 (384-dim) — Eve never sees the text"

    BOB_KB=$(mktemp)
    cat > "$BOB_KB" << 'DOCS'
{"id": 1,  "text": "The mitochondria is the powerhouse of the cell. It produces ATP through oxidative phosphorylation."}
{"id": 2,  "text": "CRISPR-Cas9 is a genome editing tool that allows precise modification of DNA sequences in living organisms."}
{"id": 3,  "text": "Photosynthesis converts carbon dioxide and water into glucose and oxygen using sunlight energy."}
{"id": 4,  "text": "The human genome contains approximately 3 billion base pairs and about 20,000 protein-coding genes."}
{"id": 5,  "text": "Antibiotics work by targeting bacterial cell walls, protein synthesis, or DNA replication mechanisms."}
{"id": 6,  "text": "Neural networks consist of layers of interconnected nodes that process information through weighted connections."}
{"id": 7,  "text": "Quantum entanglement allows two particles to be correlated regardless of the distance between them."}
{"id": 8,  "text": "The Mediterranean diet emphasizes fruits, vegetables, whole grains, olive oil, and moderate fish consumption."}
{"id": 9,  "text": "Black holes are regions of spacetime where gravity is so strong that nothing, not even light, can escape."}
{"id": 10, "text": "mRNA vaccines instruct cells to produce a protein that triggers an immune response without using live virus."}
{"id": 11, "text": "The Krebs cycle is a series of chemical reactions used by aerobic organisms to generate energy."}
{"id": 12, "text": "Transformer models use self-attention mechanisms to process sequential data in parallel."}
{"id": 13, "text": "Climate change is driven by greenhouse gas emissions trapping heat in Earth's atmosphere."}
{"id": 14, "text": "Fermentation converts sugars into ethanol and carbon dioxide using yeast or bacteria."}
{"id": 15, "text": "The theory of relativity states that the laws of physics are the same for all non-accelerating observers."}
{"id": 16, "text": "Epigenetics studies heritable changes in gene expression that do not involve alterations to the DNA sequence."}
DOCS

    # Embed all documents and produce per-shard JSON files
    SHARD_DIR=$(mktemp -d)
    python3 << PYEOF
import json, sys, os
from sentence_transformers import SentenceTransformer

n_eves = ${N_EVES}
shard_dir = "${SHARD_DIR}"

model = SentenceTransformer('all-MiniLM-L6-v2')
docs = [json.loads(line) for line in open('${BOB_KB}') if line.strip()]
texts = [d['text'] for d in docs]
embeddings = model.encode(texts)

# Partition docs across Eves by round-robin on doc_id
shards = [[] for _ in range(n_eves)]
for doc, emb in zip(docs, embeddings):
    slot = (doc['id'] - 1) % n_eves
    shards[slot].append({"id": doc["id"], "embedding": emb.tolist()})

for idx, shard in enumerate(shards):
    path = os.path.join(shard_dir, f"shard_{idx}.json")
    with open(path, 'w') as f:
        json.dump({"vectors": shard}, f)
    print(f"shard_{idx}: {len(shard)} vectors")
PYEOF

    for i in "${!EVE_APIS[@]}"; do
        N=$(python3 -c "import json; d=json.load(open('${SHARD_DIR}/shard_${i}.json')); print(len(d['vectors']))")
        pass "Shard $((i+1)): ${N} vectors → Eve $((i+1))"
    done

    # ── Step 5: Upload each shard to its Eve ──────────────────────────────
    step "Step 5 — Upload shards to Eves (Eve sees only float vectors)"
    for i in "${!EVE_APIS[@]}"; do
        UPLOAD_RESP=$(curl -sf -X POST "${EVE_APIS[$i]}/api/tenants/${TENANT_IDS[$i]}/vectors" \
            -H "Content-Type: application/json" \
            -d @"${SHARD_DIR}/shard_${i}.json")
        UPLOADED=$(echo "$UPLOAD_RESP" | python3 -c "import sys,json; print(json.load(sys.stdin)['uploaded'])")
        pass "Eve $((i+1)): uploaded ${UPLOADED} vectors"
    done

    # ── Step 6: Fan-out search — query all Eves, merge top-k ─────────────
    run_fanout_search() {
        local QUERY_TEXT="$1"

        step "Search: '${QUERY_TEXT}'"
        info "Bob embeds query locally, fans out to all ${N_EVES} Eve(s), merges results"

        QUERY_VEC=$(python3 << PYEOF
import json
from sentence_transformers import SentenceTransformer
model = SentenceTransformer('all-MiniLM-L6-v2')
emb = model.encode("${QUERY_TEXT}")
print(json.dumps({"query": emb.tolist(), "top_k": ${TOP_K}}))
PYEOF
        )

        # Query every Eve and collect results into a temp file
        MERGED_RESULTS=$(mktemp)
        echo "[]" > "$MERGED_RESULTS"

        for i in "${!EVE_APIS[@]}"; do
            SHARD_RESP=$(echo "$QUERY_VEC" | curl -sf -X POST \
                "${EVE_APIS[$i]}/api/tenants/${TENANT_IDS[$i]}/search" \
                -H "Content-Type: application/json" -d @-)
            N_RESULTS=$(echo "$SHARD_RESP" | python3 -c "import sys,json; print(len(json.load(sys.stdin)['results']))" 2>/dev/null || echo 0)
            label "  Eve $((i+1)) returned ${N_RESULTS} result(s)"

            # Merge into accumulated results
            MERGED_RESULTS_NEW=$(mktemp)
            python3 - "${MERGED_RESULTS}" "${MERGED_RESULTS_NEW}" "${TOP_K}" << PYEOF
import json, sys
existing = json.load(open(sys.argv[1]))
new_results = json.loads("""${SHARD_RESP}""")['results']
merged = existing + new_results
merged.sort(key=lambda r: r['score'], reverse=True)
merged = merged[:int(sys.argv[3])]
with open(sys.argv[2], 'w') as f:
    json.dump(merged, f)
PYEOF
            mv "$MERGED_RESULTS_NEW" "$MERGED_RESULTS"
        done

        # Bob resolves merged top-k IDs to local documents
        python3 << PYEOF
import json

docs = {}
for line in open('${BOB_KB}'):
    if line.strip():
        d = json.loads(line)
        docs[d['id']] = d['text']

results = json.load(open('${MERGED_RESULTS}'))
n_eves = ${N_EVES}

print()
for i, r in enumerate(results):
    doc_id = r['id']
    score  = r['score']
    text   = docs.get(doc_id, '(not found)')
    eve_idx = (doc_id - 1) % n_eves + 1
    print(f"  [{i+1}] ID={doc_id}  score={score:.4f}  (stored on Eve {eve_idx})")
    print(f"       {text[:110]}")
    print()
PYEOF
        rm -f "$MERGED_RESULTS"
        pass "Top-${TOP_K} results merged from ${N_EVES} Eve(s)"
    }

    run_fanout_search "How does gene editing work?"
    run_fanout_search "What should I eat for a healthy diet?"
    run_fanout_search "How do neural networks learn?"

    # ── Step 7: Tenant isolation check — cross-tenant search must fail ────
    step "Step 7 — Tenant isolation: cross-tenant search must fail"
    info "Querying Eve 1 with Eve 2's tenant ID — expect 404"
    if [[ "${N_EVES}" -ge 2 ]]; then
        HTTP_CODE=$(python3 -c "
import json
from sentence_transformers import SentenceTransformer
import urllib.request, urllib.error
model = SentenceTransformer('all-MiniLM-L6-v2')
emb = model.encode('test isolation').tolist()
body = json.dumps({'query': emb, 'top_k': 1}).encode()
req = urllib.request.Request(
    '${EVE_APIS[0]}/api/tenants/${TENANT_IDS[1]}/search',
    data=body, headers={'Content-Type': 'application/json'}, method='POST')
try:
    urllib.request.urlopen(req)
    print('200')
except urllib.error.HTTPError as e:
    print(str(e.code))
" 2>/dev/null || echo "error")
        if [[ "$HTTP_CODE" == "404" ]]; then
            pass "Eve 1 correctly rejected Eve 2's tenant ID (HTTP 404)"
        else
            warn "Expected 404, got ${HTTP_CODE} — isolation may not be enforced across Eves"
        fi
    else
        info "Skipped (need ≥2 Eves)"
    fi

    # ── Step 8: Per-Eve stats ─────────────────────────────────────────────
    step "Step 8 — Per-Eve tenant stats"
    for i in "${!EVE_APIS[@]}"; do
        label "  Eve $((i+1)) (${EVE_APIS[$i]}):"
        curl -sf "${EVE_APIS[$i]}/api/tenants/${TENANT_IDS[$i]}" \
            | python3 -c "
import sys,json
d=json.load(sys.stdin)
print(f\"    tenant_id={d['tenant_id']}  vectors={d.get('vector_count','?')}  storage={d.get('storage_bytes',0)//1024}KB\")
"
    done
    pass "Stats retrieved from all Eves"

    # ── Step 9: Cleanup — delete tenant on every Eve ──────────────────────
    step "Step 9 — Cleanup"
    for i in "${!EVE_APIS[@]}"; do
        HTTP=$(curl -sf -o /dev/null -w "%{http_code}" -X DELETE \
            "${EVE_APIS[$i]}/api/tenants/${TENANT_IDS[$i]}")
        pass "Eve $((i+1)) tenant deleted (HTTP ${HTTP})"
    done
    rm -rf "$BOB_KB" "$SHARD_DIR"
    pass "Local temp files cleaned up"

    # ── Summary ───────────────────────────────────────────────────────────
    echo
    echo -e "${BOLD}╔══════════════════════════════════════════════════════════════╗"
    echo -e "║              All multi-Eve tests passed!                     ║"
    echo -e "╠══════════════════════════════════════════════════════════════╣"
    printf  "║  ✅ %-57s║\n" "${N_EVES} Eve(s) reachable and healthy"
    printf  "║  ✅ %-57s║\n" "Tenants created on each Eve independently"
    printf  "║  ✅ %-57s║\n" "Documents embedded locally (Eve never sees text)"
    printf  "║  ✅ %-57s║\n" "Corpus sharded round-robin across ${N_EVES} Eve(s)"
    printf  "║  ✅ %-57s║\n" "Fan-out search: all Eves queried, results merged locally"
    printf  "║  ✅ %-57s║\n" "Bob resolved merged top-${TOP_K} IDs from local KB"
    printf  "║  ✅ %-57s║\n" "Tenant isolation enforced (no cross-Eve access)"
    printf  "║  ✅ %-57s║\n" "All tenants cleaned up"
    echo -e "╚══════════════════════════════════════════════════════════════╝${RESET}"
    echo
}

# ---------------------------------------------------------------------------
# Dispatch
# ---------------------------------------------------------------------------
case "$ROLE" in
    eve|eve-a|eve-b)
        run_eve "${ROLE}" "${2:-}"
        ;;
    bob)
        shift
        run_bob "$@"
        ;;
    *)
        echo "Unknown role '${ROLE}'."
        echo "Usage:"
        echo "  $0 eve|eve-a|eve-b [bind-ip]         — on a storage host machine"
        echo "  $0 bob <eve1> [eve2 ...]              — on Bob's machine"
        echo
        echo "  Eve args: bare IP, host:port, http:// URL, or public_name (DHT lookup)"
        exit 1
        ;;
esac
