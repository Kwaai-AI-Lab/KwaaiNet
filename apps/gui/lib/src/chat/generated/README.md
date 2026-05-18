# Generated gRPC bindings

These files are generated from
[`core/crates/kwaai-rpc/proto/kwaai.proto`](../../../../../../core/crates/kwaai-rpc/proto/kwaai.proto)
— do not edit by hand. They mirror the Rust bindings produced by tonic for
the daemon side, so any changes there must be re-run through `protoc`
to keep the GUI in sync.

## Regenerate

Run from `apps/gui/lib/src/chat/generated/` (paths below are all relative
to the repo root, so this works on any developer's machine):

```bash
dart pub global activate protoc_plugin  # one-time
export PATH="$HOME/.pub-cache/bin:$PATH"

REPO_ROOT=$(git rev-parse --show-toplevel)
PROTOC=$(find "$REPO_ROOT/core/target" -name 'protoc' -path '*kwaai-p2p-daemon*' -print -quit)
PROTO_DIR="$REPO_ROOT/core/crates/kwaai-rpc/proto"

cd "$(git rev-parse --show-toplevel)/apps/gui/lib/src/chat/generated"
"$PROTOC" --dart_out=grpc:. -I"$PROTO_DIR" "$PROTO_DIR/kwaai.proto"
```

The `protoc` lookup reuses the binary the `kwaai-p2p-daemon` build script
already downloads — no system install needed. If you haven't built the
Rust workspace yet, run `cargo build -p kwaai-p2p-daemon` first so the
binary is present under `core/target`.
