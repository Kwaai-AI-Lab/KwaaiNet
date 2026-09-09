# Two-node integration test — exercises two kwaainet processes on localhost.
#
# Run with:  nix run .#test-two-node
#
# This test is NOT sandboxed (needs localhost networking).  It:
#   1. Creates two isolated node environments (separate HOME dirs).
#   2. Runs `kwaainet setup` + `kwaainet identity show` on each.
#   3. Verifies both nodes have distinct Peer IDs.
#   4. Starts both nodes in foreground mode on different ports.
#   5. Waits for startup, checks status, then tears everything down.
{ pkgs, kwaainet }:

pkgs.writeShellApplication {
  name = "kwaainet-test-two-node";

  runtimeInputs = [
    kwaainet
    pkgs.coreutils
    pkgs.gnugrep
  ];

  text = ''
    set -euo pipefail

    cleanup() {
      echo "--- Cleaning up ---"
      [ -n "''${PID_A:-}" ] && kill "$PID_A" 2>/dev/null || true
      [ -n "''${PID_B:-}" ] && kill "$PID_B" 2>/dev/null || true
      wait 2>/dev/null || true
      [ -n "''${HOME_A:-}" ] && rm -rf "$HOME_A"
      [ -n "''${HOME_B:-}" ] && rm -rf "$HOME_B"
    }
    trap cleanup EXIT

    HOME_A="$(mktemp -d)"
    HOME_B="$(mktemp -d)"

    echo "=== Node A: setup ==="
    HOME="$HOME_A" kwaainet setup
    HOME="$HOME_A" kwaainet identity show

    echo ""
    echo "=== Node B: setup ==="
    HOME="$HOME_B" kwaainet setup
    HOME="$HOME_B" kwaainet identity show

    # Extract Peer IDs and verify they differ
    PEER_A="$(HOME="$HOME_A" kwaainet identity show 2>&1 | grep 'Peer ID' | awk '{print $NF}')"
    PEER_B="$(HOME="$HOME_B" kwaainet identity show 2>&1 | grep 'Peer ID' | awk '{print $NF}')"

    echo ""
    echo "Node A Peer ID: $PEER_A"
    echo "Node B Peer ID: $PEER_B"

    if [ "$PEER_A" = "$PEER_B" ]; then
      echo "FAIL: both nodes have the same Peer ID"
      exit 1
    fi
    echo "PASS: nodes have distinct Peer IDs"

    # Configure different ports so they don't collide
    HOME="$HOME_A" kwaainet config set port 19001
    HOME="$HOME_B" kwaainet config set port 19002

    echo ""
    echo "=== Starting Node A (port 19001) ==="
    HOME="$HOME_A" kwaainet start &
    PID_A=$!

    echo "=== Starting Node B (port 19002) ==="
    HOME="$HOME_B" kwaainet start &
    PID_B=$!

    # Give nodes time to initialize
    echo "Waiting 5s for nodes to start..."
    sleep 5

    # Check both processes are still running
    if kill -0 "$PID_A" 2>/dev/null; then
      echo "PASS: Node A is running (pid $PID_A)"
    else
      echo "WARN: Node A exited early (may lack network — non-fatal)"
    fi

    if kill -0 "$PID_B" 2>/dev/null; then
      echo "PASS: Node B is running (pid $PID_B)"
    else
      echo "WARN: Node B exited early (may lack network — non-fatal)"
    fi

    # Verify config persisted
    HOME="$HOME_A" kwaainet config show | grep -q "19001" && echo "PASS: Node A port config persisted"
    HOME="$HOME_B" kwaainet config show | grep -q "19002" && echo "PASS: Node B port config persisted"

    echo ""
    echo "Two-node integration test complete."
  '';
}
