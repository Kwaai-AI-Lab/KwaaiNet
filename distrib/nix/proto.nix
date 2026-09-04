# Generate Rust protobuf code from p2pd.proto.
#
# Input: the proto file (Nix tracks its hash — rebuilds automatically on change).
# Output: $out/p2pd.pb.rs compatible with the prost runtime crate.
{
  lib,
  stdenvNoCC,
  protobuf,
  protoc-gen-prost,
}:

let
  protoSrc = ../../core/crates/kwaai-p2p-daemon/proto;
in
stdenvNoCC.mkDerivation {
  name = "p2pd-proto-rs";

  src = protoSrc;

  nativeBuildInputs = [
    protobuf
    protoc-gen-prost
  ];

  dontUnpack = true;

  buildPhase = ''
    runHook preBuild
    mkdir proto-out
    protoc \
      --prost_out=proto-out \
      --proto_path="$src" \
      "$src/p2pd.proto"
    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall
    mkdir -p $out
    find proto-out -name '*.rs' -exec cp {} $out/ \;
    runHook postInstall
  '';

  meta.description = "Pre-generated prost Rust code from p2pd.proto";
}
