# Smoke test — verifies the binary starts, creates identity, and basic CLI
# commands work.  Runs inside the Nix sandbox (no network required).
{ pkgs, kwaainet }:

let
  smokeScript = pkgs.writeShellApplication {
    name = "kwaainet-smoke-test";
    runtimeInputs = [ kwaainet ];
    text = ''
      kwaainet --help > /dev/null
      echo "PASS: --help"

      kwaainet --version
      echo "PASS: --version"

      # setup creates config dirs and default config
      export HOME
      HOME="$(mktemp -d)"
      kwaainet setup
      test -f "$HOME/.kwaainet/config.yaml"
      echo "PASS: setup created config"

      # identity show generates the Ed25519 keypair on first call
      kwaainet identity show
      test -f "$HOME/.kwaainet/identity.key"
      echo "PASS: identity show created keypair"

      # config round-trip
      kwaainet config show > /dev/null
      echo "PASS: config show"

      echo ""
      echo "All smoke tests passed."
    '';
  };
in
# Wrap in runCommand so `nix flake check` gets a derivation with $out.
pkgs.runCommand "kwaainet-smoke-test" { } ''
  ${smokeScript}/bin/kwaainet-smoke-test
  touch "$out"
''
