# Shared dependency definitions — single source of truth for build and dev inputs.
{ pkgs }:
let
  inherit (pkgs) lib;
in
{
  nativeBuildInputs = with pkgs; [
    pkg-config
  ];

  buildInputs =
    with pkgs;
    [
      openssl
    ]
    ++ lib.optionals stdenv.hostPlatform.isDarwin (
      with darwin.apple_sdk.frameworks;
      [
        Security
        SystemConfiguration
        CoreFoundation
      ]
    );

  devTools = with pkgs; [
    cargo
    rustc
    rustfmt
    clippy
    rust-analyzer
    go
    jp2a
    nixfmt
  ];
}
