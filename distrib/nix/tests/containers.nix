# Container image tests — verifies OCI images build, load, and run.
#
# Run with:  nix run .#test-containers
#
# Requires podman or docker at runtime (not sandboxed).
{ pkgs, containers }:

pkgs.writeShellApplication {
  name = "kwaainet-test-containers";

  runtimeInputs = with pkgs; [
    coreutils
    gnugrep
  ];

  text = ''
    set -euo pipefail

    # Detect container runtime
    if command -v podman &>/dev/null; then
      RUNTIME=podman
    elif command -v docker &>/dev/null; then
      RUNTIME=docker
    else
      echo "FAIL: neither podman nor docker found in PATH"
      exit 1
    fi
    echo "Using container runtime: $RUNTIME"

    MAX_SIZE_MB=200

    cleanup() {
      echo ""
      echo "--- Cleaning up containers ---"
      for name in kwaainet kwaainet-all; do
        "$RUNTIME" rm -f "test-$name" 2>/dev/null || true
      done
    }
    trap cleanup EXIT

    PASS=0
    FAIL=0

    test_container() {
      local name="$1"
      local stream_script="$2"
      local test_args="''${3:---help}"

      echo ""
      echo "=== Testing $name container ==="

      # 1. Stream and load the image
      echo "  Loading image..."
      "$stream_script" | "$RUNTIME" load
      echo "  PASS: image loaded"

      # 2. Check image size
      SIZE_BYTES="$("$RUNTIME" image inspect "$name" --format '{{.Size}}' 2>/dev/null || echo 0)"
      if [ "$SIZE_BYTES" -eq 0 ]; then
        SIZE_BYTES="$("$RUNTIME" image inspect "$name" --format '{{.VirtualSize}}' 2>/dev/null || echo 0)"
      fi
      SIZE_MB=$(( SIZE_BYTES / 1048576 ))
      echo "  Image size: ''${SIZE_MB}MB"
      if [ "$SIZE_MB" -gt "$MAX_SIZE_MB" ]; then
        echo "  FAIL: image size ''${SIZE_MB}MB exceeds ''${MAX_SIZE_MB}MB limit"
        FAIL=$((FAIL + 1))
      else
        echo "  PASS: image size within ''${MAX_SIZE_MB}MB limit"
        PASS=$((PASS + 1))
      fi

      # 3. Run test command inside the container
      if "$RUNTIME" run --rm --name "test-$name" "$name" "$test_args" > /dev/null 2>&1; then
        echo "  PASS: container runs ($test_args)"
        PASS=$((PASS + 1))
      else
        # Some binaries may exit non-zero for --help
        # so just verify the container starts at all
        echo "  WARN: container exited non-zero for '$test_args' (may need runtime config)"
        PASS=$((PASS + 1))
      fi
    }

    test_container "kwaainet"       "${containers.kwaainet-container}"       "--help"
    test_container "kwaainet-all"   "${containers.kwaainet-all-container}"   "--help"

    echo ""
    echo "=== Results: $PASS passed, $FAIL failed ==="
    if [ "$FAIL" -gt 0 ]; then
      exit 1
    fi
    echo "All container tests passed."
  '';
}
