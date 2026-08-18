# kwaai-wasm crate

WASM bindings that let a KwaaiNet node run in the browser. Single-module crate exposing a
JavaScript `KwaaiNet` class over `wasm-bindgen`.

**Full project context:** `projects/kwaai-trust/` — CLAUDE.md, requirements, design, roadmap, TODO.

## Key source files

| File | Description |
|------|-------------|
| `src/lib.rs` | `wasm_bindgen` exports — `KwaaiNet` class, `initialize({ services })` |

## Build

```bash
cargo build -p kwaai-wasm
wasm-pack build core/crates/kwaai-wasm --target bundler   # requires wasm-pack
```
