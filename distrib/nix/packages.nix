# Shared dependency definitions — single source of truth for build and dev inputs.
{ pkgs }:
let
  inherit (pkgs) lib;
in
{
  nativeBuildInputs = with pkgs; [
    pkg-config
    # kwaai-rpc's build.rs uses tonic-build against kwaai.proto; it only
    # falls back to downloading protoc over the network (which the Nix
    # sandbox blocks) when `protoc` isn't already on PATH.
    protobuf
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
    jp2a
    nixfmt
  ];
}
