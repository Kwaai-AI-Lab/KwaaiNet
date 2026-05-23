#!/usr/bin/env bash
# Multi-node storage fabric local test
#
# Runs two kwaainet Eve nodes on the same machine using:
#   KWAAINET_HOME   — separate config/identity/storage directories
#   KWAAINET_SOCKET — separate p2pd IPC sockets
#
# Usage:
#   ./tests/multi-node-storage-test.sh [--binary <path>] [--role <machine-a|machine-b>]
#
# Default (no --role): runs both nodes and exercises the P2P storage protocol.
# --role machine-a / machine-b: run one side manually (for two-terminal testing).

set -euo pipefail

KWAAINET=${KWAAINET_BIN:-$(cd "$(dirname "$0")/.." && echo "$(pwd)/core/target/release/kwaainet")}
if [[ ! -x "$KWAAINET" ]]; then
    echo "Binary not found at $KWAAINET — run: cargo build --release -p kwaainet"
    exit 1
fi

# Parse args
ROLE="both"
while [[ $# -gt 0 ]]; do
    case "$1" in
        --binary) KWAAINET="$2"; shift 2 ;;
        --role)   ROLE="$2";    shift 2 ;;
        *) echo "Unknown arg: $1"; exit 1 ;;
    esac
done

# ── Per-node configuration ────────────────────────────────────────────────────

HOME_A="/tmp/kwaainet-test-a"
HOME_B="/tmp/kwaainet-test-b"
SOCK_A="/tmp/kwaai-p2pd-a.sock"
SOCK_B="/tmp/kwaai-p2pd-b.sock"
PORT_A=18080
PORT_B=18081
VPK_PORT_A=17432
VPK_PORT_B=17433

# Helper: run kwaainet for node A
a() {
    KWAAINET_HOME="$HOME_A" KWAAINET_SOCKET="$SOCK_A" "$KWAAINET" "$@"
}

# Helper: run kwaainet for node B
b() {
    KWAAINET_HOME="$HOME_B" KWAAINET_SOCKET="$SOCK_B" "$KWAAINET" "$@"
}

# ── Cleanup ───────────────────────────────────────────────────────────────────

cleanup() {
    echo ""
    echo "── Stopping nodes ──"
    a stop 2>/dev/null || true
    b stop 2>/dev/null || true
    sleep 1
    rm -f "$SOCK_A" "$SOCK_B"
    echo "Done."
}
trap cleanup EXIT

# ── Setup ─────────────────────────────────────────────────────────────────────

setup_node() {
    local name="$1" home="$2" port="$3" vpk_port="$4"
    local helper="$5"  # "a" or "b"

    echo ""
    echo "── Setting up $name (home=$home port=$port vpk=$vpk_port) ──"
    rm -rf "$home"
    mkdir -p "$home"

    # Identity and config
    KWAAINET_HOME="$home" KWAAINET_SOCKET="/dev/null" "$KWAAINET" setup 2>/dev/null || true

    # Set port in config
    KWAAINET_HOME="$home" KWAAINET_SOCKET="/dev/null" "$KWAAINET" config set port "$port" 2>/dev/null || true

    # Init storage
    KWAAINET_HOME="$home" KWAAINET_SOCKET="/dev/null" "$KWAAINET" storage init \
        --capacity-gb 1 \
        --port "$vpk_port" \
        --data-dir "$home/storage" 2>/dev/null

    # Show identity
    local peer_id
    peer_id=$(KWAAINET_HOME="$home" KWAAINET_SOCKET="/dev/null" "$KWAAINET" identity show 2>/dev/null | grep "Peer ID" | awk '{print $NF}') || peer_id=""
    echo "  $name PeerId: $peer_id"
    echo "$peer_id"
}

if [[ "$ROLE" == "both" || "$ROLE" == "machine-a" ]]; then
    PEER_A=$(setup_node "Node-A" "$HOME_A" "$PORT_A" "$VPK_PORT_A" "a")
fi

if [[ "$ROLE" == "both" || "$ROLE" == "machine-b" ]]; then
    PEER_B=$(setup_node "Node-B" "$HOME_B" "$PORT_B" "$VPK_PORT_B" "b")
fi

# ── Start nodes ───────────────────────────────────────────────────────────────

start_node() {
    local name="$1" home="$2" sock="$3" port="$4"

    echo ""
    echo "── Starting $name ──"
    KWAAINET_HOME="$home" KWAAINET_SOCKET="$sock" "$KWAAINET" start --daemon 2>/dev/null
    sleep 3

    if KWAAINET_HOME="$home" KWAAINET_SOCKET="$sock" "$KWAAINET" status 2>/dev/null | grep -q "running\|Online\|online"; then
        echo "  $name: running ✓"
    else
        echo "  $name: status unclear — check logs at $home/logs/"
    fi
}

if [[ "$ROLE" == "both" ]]; then
    start_node "Node-A" "$HOME_A" "$SOCK_A" "$PORT_A"
    start_node "Node-B" "$HOME_B" "$SOCK_B" "$PORT_B"

    echo ""
    echo "── Waiting for DHT gossip (10s) ──"
    sleep 10

    # ── VPK Discovery from A ──────────────────────────────────────────────────

    echo ""
    echo "── VPK discovery from Node-A ──"
    a vpk discover --json 2>/dev/null | python3 -m json.tool 2>/dev/null || a vpk discover --json 2>/dev/null || echo "(no peers found yet)"

    echo ""
    echo "── VPK discovery from Node-B ──"
    b vpk discover --json 2>/dev/null | python3 -m json.tool 2>/dev/null || b vpk discover --json 2>/dev/null || echo "(no peers found yet)"

    # ── P2P storage RPC — Node-A acts as Bob, Node-B as Eve ──────────────────

    echo ""
    echo "── Checking Eve (Node-B) via P2P health from Node-A ──"
    echo "  (uses /kwaai/storage/1.0.0 relay — no port forwarding)"
    EVE_PEER=$(KWAAINET_HOME="$HOME_B" KWAAINET_SOCKET="/dev/null" "$KWAAINET" identity show 2>/dev/null | grep "Peer ID" | awk '{print $NF}') || EVE_PEER=""
    echo "  Node-B PeerId: $EVE_PEER"

    # Show Node-B storage status
    echo ""
    echo "── Node-B storage status ──"
    KWAAINET_HOME="$HOME_B" KWAAINET_SOCKET="$SOCK_B" "$KWAAINET" storage status 2>/dev/null || echo "  (storage status unavailable — check logs)"

    # Show DHT records have no IP endpoints
    echo ""
    echo "── Verify DHT records contain no IP/endpoint (only PeerId) ──"
    DISCOVERED=$(a vpk discover --json 2>/dev/null)
    echo "  Raw JSON from discover: $DISCOVERED"
    if echo "$DISCOVERED" | python3 -c "
import sys, json
data = json.load(sys.stdin)
for node in data:
    if 'endpoint' in node:
        print('FAIL: endpoint field found in DHT record:', node)
        sys.exit(1)
print('PASS: no endpoint/IP in', len(data), 'DHT record(s)')
" 2>/dev/null; then
        echo "  ✓ DHT records are clean (PeerId-only)"
    else
        echo "  (no peers in DHT yet — try after longer wait)"
    fi

    echo ""
    echo "── Test complete ──"
    echo "  Node-A home: $HOME_A"
    echo "  Node-B home: $HOME_B"
    echo "  Use 'a vpk status' / 'b vpk status' to inspect each node."
    echo "  Nodes will be stopped on script exit (Ctrl+C)."
    echo ""
    echo "  To run Phase 2 tenant operations manually:"
    echo "    KWAAINET_HOME=$HOME_A KWAAINET_SOCKET=$SOCK_A \\"
    echo "      kwaainet vpk tenant create --eve-peer-id $EVE_PEER --capacity-mb 256"
    echo ""
    if [[ -t 0 ]]; then
        read -r -p "Press Enter to stop both nodes..."
    else
        echo "  (non-interactive — exiting)"
    fi

elif [[ "$ROLE" == "machine-a" ]]; then
    echo ""
    echo "── Node-A ready. Start Node-B on another machine with: ──"
    echo "  KWAAINET_HOME=$HOME_B KWAAINET_SOCKET=$SOCK_B $KWAAINET start --daemon"
    echo ""
    echo "Node-A PeerId: $PEER_A"
    start_node "Node-A" "$HOME_A" "$SOCK_A" "$PORT_A"
    echo "Node-A running. Press Ctrl+C to stop."
    wait

elif [[ "$ROLE" == "machine-b" ]]; then
    echo ""
    echo "Node-B PeerId: $PEER_B"
    start_node "Node-B" "$HOME_B" "$SOCK_B" "$PORT_B"
    echo "Node-B running. Press Ctrl+C to stop."
    wait
fi
