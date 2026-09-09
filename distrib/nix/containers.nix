# OCI container images for KwaaiNet binaries.
#
# Each image uses streamLayeredImage — the derivation output is an
# executable script that streams a Docker-compatible tarball to stdout.
#
# Usage:
#   nix build .#kwaainet-container && ./result | docker load
#   nix build .#kwaainet-container && ./result | podman load
#
# Inputs:
#   pkgs             — nixpkgs package set
#   kwaainet         — kwaainet binary derivation
{
  pkgs,
  kwaainet,
}:

let
  # Shared base contents for all containers.
  baseContents = [
    pkgs.cacert # TLS CA certificates
    pkgs.tzdata # timezone data for log timestamps
  ];

  # Helper — builds a streamLayeredImage for a single binary.
  mkContainer =
    {
      name,
      binary,
      port ? null,
      extraContents ? [ ],
      extraConfig ? { },
    }:
    pkgs.dockerTools.streamLayeredImage ({
      inherit name;
      tag = binary.version or "latest";

      contents = baseContents ++ [ binary ] ++ extraContents;

      config = {
        Entrypoint = [ "${binary}/bin/${name}" ];
        Env = [
          "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
          "TZDIR=${pkgs.tzdata}/share/zoneinfo"
        ];
      }
      // (
        if port != null then
          {
            ExposedPorts = {
              "${toString port}/tcp" = { };
            };
          }
        else
          { }
      )
      // extraConfig;
    });

in
{
  kwaainet-container = mkContainer {
    name = "kwaainet";
    binary = kwaainet;
  };

  # All-in-one container with every KwaaiNet binary.
  kwaainet-all-container = pkgs.dockerTools.streamLayeredImage {
    name = "kwaainet-all";
    tag = kwaainet.version or "latest";

    contents = baseContents ++ [
      kwaainet
    ];

    config = {
      Entrypoint = [ "${kwaainet}/bin/kwaainet" ];
      Env = [
        "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
        "TZDIR=${pkgs.tzdata}/share/zoneinfo"
      ];
    };
  };
}
