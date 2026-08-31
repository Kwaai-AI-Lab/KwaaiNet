#![allow(clippy::disallowed_names)]
//! Benchmark documenting why OntologyIndex exists. Run explicitly:
//!   cargo test -p kwaai-rag --test bench_trig -- --ignored --nocapture
#[test]
#[ignore = "benchmark; run explicitly"]
fn bench_trigger_paths() {
    use kwaai_rag::ontology::{Ontology, OntologyIndex};
    let y = std::fs::read_to_string(
        "/Users/rezarassool/Source/KwaaiNet/tests/kwaai-knowledge/ontologies/D6.yaml",
    )
    .unwrap();
    let o = Ontology::from_yaml(&y).unwrap();
    let sentences: Vec<String> = (0..2000)
        .map(|i| format!("he lived at no. {i} chapel street and later attended trafalgar high"))
        .collect();

    let t = std::time::Instant::now();
    for s in &sentences {
        let tr = o.triggers();
        kwaai_rag::graph::best_ontology_trigger(s, &tr);
    }
    let naive = t.elapsed();

    let t = std::time::Instant::now();
    let tr = o.triggers();
    for s in &sentences {
        kwaai_rag::graph::best_ontology_trigger(s, &tr);
    }
    let hoisted = t.elapsed();

    let t = std::time::Instant::now();
    let idx = OntologyIndex::build(o.clone());
    let build = t.elapsed();
    let t = std::time::Instant::now();
    for s in &sentences {
        idx.find_trigger(s);
    }
    let indexed = t.elapsed();

    // candidate classification, the other hot path
    let cands: Vec<String> = (0..2000).map(|i| format!("Chapel Street {i}")).collect();
    let t = std::time::Instant::now();
    for c in &cands {
        o.classify_candidate(c);
    }
    let cls_naive = t.elapsed();
    let t = std::time::Instant::now();
    for c in &cands {
        idx.classify(c);
    }
    let cls_idx = t.elapsed();

    println!("  trigger  naive/sentence-rebuild : {naive:?}");
    println!("  trigger  hoisted Vec            : {hoisted:?}");
    println!(
        "  trigger  OntologyIndex          : {indexed:?}  ({:.1}x vs hoisted, {:.0}x vs naive)",
        hoisted.as_secs_f64() / indexed.as_secs_f64(),
        naive.as_secs_f64() / indexed.as_secs_f64()
    );
    println!("  index build (once per run)      : {build:?}");
    println!("  classify naive                  : {cls_naive:?}");
    println!(
        "  classify OntologyIndex          : {cls_idx:?}  ({:.1}x)",
        cls_naive.as_secs_f64() / cls_idx.as_secs_f64()
    );
}
