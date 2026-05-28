# kwaai-knowledge — TODO

## Bug fixes
- [ ] Verify index seed entities are correctly embedded (not just graph-inserted) after D6 rebuild
- [ ] Check Organization entity recall — consistently 0.00–0.67; investigate if D6 has any orgs in eval set

## Features
- [ ] Index seed embedding — embed index-seeded names into vector index (currently only graph)
- [ ] Graph scoring v2 — entity confidence weighting by extraction frequency
- [ ] Multi-document knowledge bases — cross-document entity deduplication
- [ ] Obsidian vault ingestion (`obsidian.rs` → CLI integration)
- [ ] `kwaainet rag graph dedup` — merge near-duplicate entity nodes

## Tests
- [ ] Complete D6 rebuild once metro machines are accessible via p2p relay
- [ ] Run `kwaainet rag eval` after rebuild and record accuracy in `d6_accuracy_progress.md`
- [ ] Graph seed round-trip test: seed YAML → graph → retrieval includes seeded relations
- [ ] Doc schema auto-detect test: D6 PDF header → auto-detected ISBN/publisher/author

## Docs
- [ ] Design docs: overview.md and data-flows.md (see `design/`)
- [ ] Update `d6_accuracy_progress.md` after next full rebuild + eval
