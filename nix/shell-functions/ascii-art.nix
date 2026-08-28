# nix/shell-functions/ascii-art.nix
#
# ASCII art logo display for KwaaiNet development shell.
#
# Uses jp2a to convert the Kwaai logo to colored ASCII art.
# Falls back to a simple text banner if jp2a is not available.
#
# Usage in devshell.nix:
#   asciiArt = import ./shell-functions/ascii-art.nix { };
#

{ }:

''
  if command -v jp2a >/dev/null 2>&1 && [ -f "./docs/assets/kwaai-logo.png" ]; then
    echo "$(jp2a --colors ./docs/assets/kwaai-logo.png)"
    echo ""
  else
    echo "=== KwaaiNet Development Shell ==="
  fi
''
