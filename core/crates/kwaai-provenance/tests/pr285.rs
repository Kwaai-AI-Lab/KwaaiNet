use kwaai_provenance::gate::{Gate, GateMode, UnresolvedVerifier};
use kwaai_provenance::lineage::{
    LineageError, LineageEvent, LineageEventKind as K, LineageLog, LogEntry, MutationState,
};
use kwaai_provenance::persist::{self, JsonlWriter, PersistError, Record, TailPolicy};
use kwaai_provenance::{Claim, ClaimKind};
use std::io::Cursor;

fn event(k: K, inputs: &[&str], outputs: &[&str], seq: u64) -> LineageEvent {
    LineageEvent::new(
        k,
        inputs.iter().copied(),
        outputs.iter().copied(),
        "pr285",
        Some(1),
        seq,
    )
    .unwrap()
}
fn commit(log: &mut LineageLog, k: K, inputs: &[&str], outputs: &[&str], seq: u64) -> String {
    let id = log.propose(event(k, inputs, outputs, seq)).unwrap();
    log.commit(&id).unwrap();
    id
}
fn bytes(records: &[Record]) -> Vec<u8> {
    let mut w = JsonlWriter::new(Vec::new());
    for r in records {
        w.write_record(r).unwrap();
    }
    w.into_inner()
}
#[test]
fn pr285_utf8_tail_and_numbered_middle_errors() {
    let prefix = bytes(&[Record::lineage(LogEntry::Proposed {
        event: event(K::Extract, &["source"], &["A"], 0),
    })]);
    for ch in ["é", "€", "🦀"] {
        for cut in 1..ch.len() {
            let mut damaged = prefix.clone();
            damaged.extend_from_slice(b"{\"record\":\"");
            damaged.extend_from_slice(&ch.as_bytes()[..cut]);
            assert!(
                matches!(
                    persist::read_records(Cursor::new(&damaged), TailPolicy::Strict),
                    Err(PersistError::Parse { line: 2, .. })
                ),
                "strict {ch}/{cut}"
            );
            let recovered =
                persist::read_records(Cursor::new(&damaged), TailPolicy::DropTornTail).unwrap();
            assert!(recovered.dropped_torn_tail);
            assert_eq!(recovered.records.len(), 1);
            assert_eq!(recovered.replay().unwrap().entries().len(), 1);
            damaged.push(b'\n');
            damaged.extend_from_slice(&prefix);
            for policy in [TailPolicy::Strict, TailPolicy::DropTornTail] {
                assert!(matches!(
                    persist::read_records(Cursor::new(&damaged), policy),
                    Err(PersistError::Parse { line: 2, .. })
                ));
                assert!(matches!(
                    persist::read_records(Cursor::new(&damaged[prefix.len()..]), policy),
                    Err(PersistError::Parse { line: 1, .. })
                ));
            }
        }
    }
}
#[test]
fn pr285_repair_preserves_valid_unterminated_records() {
    let ev = event(K::Extract, &["source"], &["A"], 0);
    let proposed = Record::lineage(LogEntry::Proposed { event: ev.clone() });
    let c = Claim::new(ClaimKind::Relation, "s", "p", "o");
    let gate = Record::gate(
        Gate::new(GateMode::Observe)
            .check(&c, &[], &UnresolvedVerifier)
            .unwrap(),
    );
    let cases = [
        vec![proposed.clone()],
        vec![
            proposed.clone(),
            Record::lineage(LogEntry::Committed {
                event_id: ev.id.clone(),
            }),
        ],
        vec![
            proposed,
            Record::lineage(LogEntry::Failed {
                event_id: ev.id,
                reason: "échec".into(),
            }),
        ],
        vec![gate.clone()],
    ];
    for records in cases {
        for crlf in [false, true] {
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("lineage.jsonl");
            let data = bytes(&records);
            let mut data = if crlf {
                data.split(|b| *b == b'\n')
                    .collect::<Vec<_>>()
                    .join(&b"\r\n"[..])
            } else {
                data
            };
            data.pop(); // missing LF; optional CR remains
            std::fs::write(&path, &data).unwrap();
            let before = persist::read_file(&path, TailPolicy::Strict).unwrap();
            let digest = persist::lineage_digest(&before.replay().unwrap()).unwrap();
            assert_eq!(persist::truncate_torn_tail(&path).unwrap(), 0);
            assert!(std::fs::read(&path).unwrap().starts_with(&data));
            let mut w =
                JsonlWriter::append_to(&path, before.replay().unwrap().entries().len()).unwrap();
            w.write_record(&gate).unwrap();
            w.sync_all().unwrap();
            let after = persist::read_file(&path, TailPolicy::Strict).unwrap();
            assert_eq!(&after.records[..records.len()], &before.records);
            assert_eq!(after.records.len(), records.len() + 1);
            assert_eq!(
                persist::lineage_digest(&after.replay().unwrap()).unwrap(),
                digest
            );
        }
    }
}
#[test]
fn pr285_repair_refuses_semantic_or_middle_corruption_without_writes() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lineage.jsonl");
    let valid = bytes(&[Record::lineage(LogEntry::Proposed {
        event: event(K::Extract, &["s"], &["A"], 0),
    })]);
    let invalid_transition = bytes(&[Record::lineage(LogEntry::Committed {
        event_id: "unknown".into(),
    })]);
    let unknown_version = String::from_utf8(valid.clone())
        .unwrap()
        .replace("\"schema_version\":1", "\"schema_version\":99")
        .into_bytes();
    for mut data in [
        invalid_transition,
        unknown_version,
        b"invalid\n{torn".to_vec(),
    ] {
        if data.last() == Some(&b'\n') {
            data.pop();
        }
        std::fs::write(&path, &data).unwrap();
        assert!(persist::truncate_torn_tail(&path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), data);
    }
    for tail in [b"{\"record\":\"".as_slice(), &[0xc3]] {
        let mut data = valid.clone();
        data.extend_from_slice(tail);
        std::fs::write(&path, &data).unwrap();
        assert_eq!(
            persist::truncate_torn_tail(&path).unwrap(),
            tail.len() as u64
        );
        assert_eq!(std::fs::read(&path).unwrap(), valid);
    }
}
#[test]
fn pr285_traces_intermediate_original_and_ignores_uncommitted_paths() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Derive, &["synthetic:0"], &["orig:1"], 0);
    commit(&mut log, K::Extract, &["orig:1"], &["A"], 1);
    commit(&mut log, K::Derive, &["synthetic:2"], &["A"], 2);
    let failed = log
        .propose(event(K::Extract, &["orig:2"], &["B"], 3))
        .unwrap();
    log.fail(&failed, "not applied").unwrap();
    log.propose(event(K::Extract, &["orig:3"], &["B"], 4))
        .unwrap();
    let mut w = JsonlWriter::new(Vec::new());
    w.sync_lineage(&log).unwrap();
    let replay = persist::read_records(Cursor::new(w.into_inner()), TailPolicy::Strict)
        .unwrap()
        .replay()
        .unwrap();
    for l in [&log, &replay] {
        assert!(l.traces_to("A", |a| a.starts_with("orig:")));
        assert!(l.traces_to("orig:1", |a| a.starts_with("orig:")));
        assert!(!l.traces_to("B", |a| a.starts_with("orig:")));
        assert_eq!(
            l.traceability_rate(["A", "B"], |a| a.starts_with("orig:")),
            Some(0.5)
        );
        assert!(!l.roots("A").contains("orig:1"));
    }
}
#[test]
fn pr285_recurrent_claim_and_survivor_are_explicit_dag_limitations() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Extract, &["orig:1"], &["A"], 0);
    commit(&mut log, K::ClaimRewrite, &["A"], &["B"], 1);
    let id = log
        .propose(event(K::ClaimRewrite, &["B"], &["A"], 2))
        .unwrap();
    assert!(matches!(
        log.commit(&id),
        Err(LineageError::CycleDetected { .. })
    ));
    assert_eq!(log.state(&id), Some(MutationState::Proposed));
    assert!(matches!(
        LineageEvent::new(K::EntityMerge, ["e1", "e2"], ["e1"], "test", None, 3),
        Err(LineageError::SelfDerivation(_))
    ));
}
#[test]
fn pr285_pruned_is_historical_and_reintroduction_retains_ancestry() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Extract, &["orig:1"], &["A"], 0);
    commit(&mut log, K::Prune, &["A"], &[], 1);
    commit(&mut log, K::Extract, &["orig:2"], &["A"], 2);
    commit(&mut log, K::Derive, &["A"], &["B"], 3);
    let mut w = JsonlWriter::new(Vec::new());
    w.sync_lineage(&log).unwrap();
    let replay = persist::read_records(Cursor::new(w.into_inner()), TailPolicy::Strict)
        .unwrap()
        .replay()
        .unwrap();
    for l in [&log, &replay] {
        assert!(l.is_pruned("A")); // ever-pruned API, not graph liveness
        assert!(l.ancestors("A").contains("orig:1"));
        assert!(l.ancestors("A").contains("orig:2"));
        assert!(l.traces_to("B", |a| a == "orig:2"));
    }
}

#[test]
fn pr285_repair_empty_and_whitespace_files_is_idempotent() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lineage.jsonl");
    for data in ["", "\r\n", "\u{2003}\r\n", "  \t", "\u{2003}"] {
        std::fs::write(&path, data).unwrap();
        assert_eq!(persist::truncate_torn_tail(&path).unwrap(), 0);
        let repaired = std::fs::read(&path).unwrap();
        assert!(repaired.starts_with(data.as_bytes()));
        assert_eq!(persist::truncate_torn_tail(&path).unwrap(), 0);
        assert_eq!(std::fs::read(&path).unwrap(), repaired);
        let read = persist::read_file(&path, TailPolicy::Strict).unwrap();
        assert!(read.records.is_empty());
        JsonlWriter::append_to(&path, 0).unwrap();
    }
}

#[test]
fn pr285_review_complete_invalid_json_record_is_not_a_torn_write() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Extract, &["orig:1"], &["A"], 0);
    let mut w = JsonlWriter::new(Vec::new());
    w.sync_lineage(&log).unwrap();
    let prefix = w.into_inner();
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lineage.jsonl");
    for tail in [
        r#"{"record":"lineage","schema_version":1,"entry":{"entry":"committed","event_id":12}}"#,
        r#"{"record":"lineage","schema_version":1,"entry":{"entry":"committed","event_i":"A"}}"#,
        r#"{"record":"future","schema_version":1}"#,
        r#"{"record":"lineage","entry":{"entry":"committed","event_id":"A"}}"#,
        r#"{"record":"lineage","schema_version":1,"entry":null}"#,
    ] {
        serde_json::from_str::<serde_json::Value>(tail).unwrap();
        let mut data = prefix.clone();
        data.extend_from_slice(tail.as_bytes());
        for policy in [TailPolicy::Strict, TailPolicy::DropTornTail] {
            assert!(
                matches!(
                    persist::read_records(Cursor::new(&data), policy),
                    Err(PersistError::Parse { line: 3, .. })
                ),
                "complete invalid record was discarded: {tail}"
            );
        }
        std::fs::write(&path, &data).unwrap();
        assert!(persist::truncate_torn_tail(&path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), data);
    }
}

#[test]
fn pr285_review_numbered_errors_preserve_committed_prefix_and_io_errors() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Extract, &["orig:1"], &["A"], 0);
    let mut w = JsonlWriter::new(Vec::new());
    w.sync_lineage(&log).unwrap();
    let prefix = w.into_inner();
    // This is actual serialized UTF-8 payload, cut at every byte before its LF.
    let full = bytes(&[Record::lineage(LogEntry::Failed {
        event_id: "unused".into(),
        reason: "é € 🦀".into(),
    })]);
    for cut in 1..full.len() - 1 {
        let mut data = prefix.clone();
        data.extend_from_slice(&full[..cut]);
        let read = persist::read_records(Cursor::new(&data), TailPolicy::DropTornTail).unwrap();
        assert!(read.dropped_torn_tail);
        let replay = read.replay().unwrap();
        assert_eq!(
            replay.state(
                &log.entries()
                    .iter()
                    .find_map(|e| if let LogEntry::Committed { event_id } = e {
                        Some(event_id.clone())
                    } else {
                        None
                    })
                    .unwrap()
            ),
            Some(MutationState::Committed)
        );
        assert_eq!(replay.entries(), log.entries());
    }
    struct Broken;
    impl std::io::Read for Broken {
        fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("device error"))
        }
    }
    for policy in [TailPolicy::Strict, TailPolicy::DropTornTail] {
        assert!(matches!(
            persist::read_records(std::io::BufReader::new(Broken), policy),
            Err(PersistError::Io(_))
        ));
    }
}

#[test]
fn pr285_review_branching_shared_missing_and_large_ancestry_is_deterministic() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Extract, &["orig:1", "orig:1"], &["A", "A"], 0);
    commit(&mut log, K::Derive, &["A"], &["left", "right"], 1);
    commit(
        &mut log,
        K::Derive,
        &["left", "right", "missing:unknown"],
        &["join"],
        2,
    );
    let original = |a: &str| a == "orig:1";
    assert!(log.traces_to("A", original));
    assert!(log.traces_to("join", original));
    assert!(!log.traces_to("missing:unknown", original));
    assert!(log.ancestors("join").contains("missing:unknown"));
    let mut last = "join".to_string();
    for seq in 3..515 {
        let next = format!("deep:{seq}");
        commit(&mut log, K::Derive, &[&last], &[&next], seq);
        last = next;
    }
    let expected = log.ancestors(&last);
    for _ in 0..3 {
        assert_eq!(log.ancestors(&last), expected);
        assert!(log.traces_to(&last, original));
    }
    let mut back = event(K::Derive, &[&last], &["A"], 600);
    let id = log.propose(back.clone()).unwrap();
    assert!(matches!(
        log.commit(&id),
        Err(LineageError::CycleDetected { .. })
    ));
    let mut w = JsonlWriter::new(Vec::new());
    w.sync_lineage(&log).unwrap();
    w.write_record(&Record::lineage(LogEntry::Committed { event_id: id }))
        .unwrap();
    let read =
        persist::read_records(Cursor::new(w.into_inner()), TailPolicy::DropTornTail).unwrap();
    assert!(matches!(
        read.replay(),
        Err(PersistError::Lineage {
            error: LineageError::CycleDetected { .. },
            ..
        })
    ));
    back.inputs.push(String::new());
    assert!(matches!(
        log.propose(back),
        Err(LineageError::EmptyArtifactId)
    ));
}

#[test]
fn pr285_review_syntax_corruption_is_not_torn_tail() {
    let valid = bytes(&[Record::lineage(LogEntry::Proposed {
        event: event(K::Extract, &["source"], &["A"], 0),
    })]);
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("lineage.jsonl");
    for tail in [
        b"{torn".as_slice(),
        b"{\"record\":truX",
        b"{} trailing",
        b"{\"record\":\"bad\xff",
    ] {
        let mut data = valid.clone();
        data.extend_from_slice(tail);
        assert!(
            matches!(
                persist::read_records(Cursor::new(&data), TailPolicy::DropTornTail),
                Err(PersistError::Parse { line: 2, .. })
            ),
            "non-truncation corruption was dropped: {tail:?}"
        );
        std::fs::write(&path, &data).unwrap();
        assert!(persist::truncate_torn_tail(&path).is_err());
        assert_eq!(std::fs::read(&path).unwrap(), data);
    }
}

#[test]
fn pr285_review_repeated_pruning_retains_history_and_reassociation() {
    let mut log = LineageLog::new();
    commit(&mut log, K::Extract, &["orig:1"], &["A"], 0);
    for seq in 1..=3 {
        commit(&mut log, K::Prune, &["A"], &[], seq * 2 - 1);
        commit(
            &mut log,
            K::Extract,
            &[&format!("orig:{}", seq + 1)],
            &["A"],
            seq * 2,
        );
        assert!(log.is_pruned("A"));
    }
    assert_eq!(
        log.ancestors("A"),
        ["orig:1", "orig:2", "orig:3", "orig:4"]
            .into_iter()
            .map(str::to_string)
            .collect()
    );
    let mut w = JsonlWriter::new(Vec::new());
    w.sync_lineage(&log).unwrap();
    let replay = persist::read_records(Cursor::new(w.into_inner()), TailPolicy::Strict)
        .unwrap()
        .replay()
        .unwrap();
    assert_eq!(replay.entries(), log.entries());
    assert_eq!(replay.ancestors("A"), log.ancestors("A"));
}
