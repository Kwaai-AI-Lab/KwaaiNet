# Development shell — provides the full toolchain for hacking on KwaaiNet.
{ pkgs, packages }:

let
  asciiArt = import ./shell-functions/ascii-art.nix { };
in
pkgs.mkShell {
  name = "kwaainet-dev";

  nativeBuildInputs = packages.nativeBuildInputs ++ packages.devTools;

  inherit (packages) buildInputs;

  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";

  shellHook = ''
    ${asciiArt}
    echo "  cargo build       build kwaainet (from core/)"
    echo "  cargo test --all  run unit tests"
    echo "  nix build         build the Nix package"
    echo "  nix flake check   run Nix checks"
    echo "  nix fmt           format Nix files"
  '';
}
