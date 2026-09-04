# go-libp2p-daemon (Hivemind fork) — P2P daemon for KwaaiNet's DHT networking.
#
# KwaaiNet requires the learning-at-home fork (v0.5.0.hivemind1) for Hivemind
# DHT compatibility, not the upstream libp2p/go-libp2p-daemon in nixpkgs.
{
  lib,
  buildGoModule,
  fetchFromGitHub,
}:

buildGoModule {
  pname = "p2pd-hivemind";
  version = "0.5.0-hivemind1";

  src = fetchFromGitHub {
    owner = "learning-at-home";
    repo = "go-libp2p-daemon";
    rev = "v0.5.0.hivemind1";
    hash = "sha256-L5IyN6G6/UyRSLs124hDMjqgQ0FIXA7GyI9FcQE+aFw=";
  };

  # Run `nix build .#p2pd` — if this hash is stale Nix prints the correct one.
  vendorHash = "sha256-5j6itIU4HIdqV9N4Ak1e//1m7JfzMCiOH0QdUucHNW4=";

  subPackages = [ "p2pd" ];

  doCheck = false;

  ldflags = [
    "-s"
    "-w"
  ];

  meta = {
    description = "go-libp2p-daemon with Hivemind DHT support (learning-at-home fork)";
    homepage = "https://github.com/learning-at-home/go-libp2p-daemon";
    license = lib.licenses.mit;
    mainProgram = "p2pd";
  };
}
