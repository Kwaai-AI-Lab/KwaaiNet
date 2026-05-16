# Generated gRPC bindings

These files are generated from
[`KwaaiNet/core/crates/kwaai-rpc/proto/kwaai.proto`](../../../../../KwaaiNet/core/crates/kwaai-rpc/proto/kwaai.proto)
— do not edit by hand. They mirror the Rust bindings produced by tonic for
the daemon side, so any changes there must be re-run through `protoc`
to keep the GUI in sync.

## Regenerate

```bash
dart pub global activate protoc_plugin  # one-time
export PATH="$HOME/.pub-cache/bin:$PATH"

PROTOC=$(find /Volumes/Projects/kwaaiai/KwaaiNet-GUI/core/target -name 'protoc' -path '*kwaai-p2p-daemon*' -print -quit)
PROTO_DIR=/Volumes/Projects/kwaaiai/KwaaiNet/core/crates/kwaai-rpc/proto

cd "$(dirname "$0")"
"$PROTOC" --dart_out=grpc:. -I"$PROTO_DIR" "$PROTO_DIR/kwaai.proto"
```

The `protoc` lookup reuses the binary the `kwaai-p2p-daemon` build script
already downloads — no system install needed.
