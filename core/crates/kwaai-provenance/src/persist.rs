//! JSONL persistence for lineage log entries and gate outcomes.
//!
//! One record per line, each tagged with its kind and schema version:
//!
//! ```text
//! {"record":"lineage","schema_version":1,"entry":{"entry":"proposed","event":{...}}}
//! {"record":"gate","schema_version":1,"outcome":{"mode":"observe",...}}
//! ```
//!
//! Reading is strict by default: a malformed line, an unknown schema version, a
//! tampered event ID, an invalid state transition or a cycle stops the read and
//! reports the line number. Lineage is rebuilt by replaying entries through
//! [`LineageLog`]'s own `propose` / `commit` / `fail`, so a file can never load
//! into a state the in-memory API would have refused.
//!
//! The only tolerated defect is a torn final line (a write interrupted before its
//! newline), and only when the caller asks for [`TailPolicy::DropTornTail`].
//!
//! Scope: single writer, no file locking, no hash chain. Integrity rests on event
//! IDs and replay validation; a snapshot digest gives a reproducible fingerprint.

use crate::gate::GateOutcome;
use crate::lineage::{LineageError, LineageLog, LogEntry};
use crate::schema::SCHEMA_VERSION;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// One persisted line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "record", rename_all = "snake_case")]
pub enum Record {
    /// A lineage log entry.
    Lineage {
        /// Schema version the record was written with.
        schema_version: u32,
        /// The entry.
        entry: LogEntry,
    },
    /// A gate evaluation.
    Gate {
        /// Schema version the record was written with.
        schema_version: u32,
        /// The outcome.
        outcome: GateOutcome,
    },
}

impl Record {
    /// Wrap a lineage entry at the current schema version.
    pub fn lineage(entry: LogEntry) -> Self {
        Record::Lineage {
            schema_version: SCHEMA_VERSION,
            entry,
        }
    }

    /// Wrap a gate outcome at the current schema version.
    pub fn gate(outcome: GateOutcome) -> Self {
        Record::Gate {
            schema_version: SCHEMA_VERSION,
            outcome,
        }
    }

    fn schema_version(&self) -> u32 {
        match self {
            Record::Lineage { schema_version, .. } | Record::Gate { schema_version, .. } => {
                *schema_version
            }
        }
    }
}

/// Errors from writing or reading persisted records. Line numbers are 1-based.
#[derive(Debug)]
pub enum PersistError {
    /// Underlying I/O failure.
    Io(io::Error),
    /// Serializing a record failed.
    Serialize(serde_json::Error),
    /// A line is not a valid record.
    Parse {
        /// Line number.
        line: usize,
        /// Parser message.
        message: String,
    },
    /// A record was written with a schema version this build does not read.
    UnsupportedVersion {
        /// Line number.
        line: usize,
        /// Version found.
        found: u32,
    },
    /// Appending was refused because the file does not end with a newline, so
    /// a new record would be glued onto a torn line. See [`truncate_torn_tail`].
    UnterminatedFile,
    /// A lineage entry failed validation or replay.
    Lineage {
        /// Line number.
        line: usize,
        /// Why it was refused.
        error: LineageError,
    },
}

impl fmt::Display for PersistError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersistError::Io(e) => write!(f, "I/O error: {e}"),
            PersistError::Serialize(e) => write!(f, "serialize error: {e}"),
            PersistError::Parse { line, message } => write!(f, "line {line}: {message}"),
            PersistError::UnsupportedVersion { line, found } => write!(
                f,
                "line {line}: schema_version {found} is not supported (this build reads {SCHEMA_VERSION})"
            ),
            PersistError::UnterminatedFile => write!(
                f,
                "file does not end with a newline; repair it with truncate_torn_tail before appending"
            ),
            PersistError::Lineage { line, error } => write!(f, "line {line}: {error}"),
        }
    }
}

impl std::error::Error for PersistError {}

impl From<io::Error> for PersistError {
    fn from(e: io::Error) -> Self {
        PersistError::Io(e)
    }
}

/// Writes records as JSONL. Each record is written with a single `write_all`
/// of the full line including its newline, which keeps lines whole on ordinary
/// local filesystems when there is one writer.
#[derive(Debug)]
pub struct JsonlWriter<W: Write> {
    inner: W,
    lineage_cursor: usize,
}

impl<W: Write> JsonlWriter<W> {
    /// Wrap a writer. The lineage cursor starts at 0.
    pub fn new(inner: W) -> Self {
        JsonlWriter {
            inner,
            lineage_cursor: 0,
        }
    }

    /// Wrap a writer whose destination already holds the first `cursor` entries
    /// of the log, e.g. a file reopened for append after [`ReadOutput::replay`].
    pub fn with_lineage_cursor(inner: W, cursor: usize) -> Self {
        JsonlWriter {
            inner,
            lineage_cursor: cursor,
        }
    }

    /// Write one record.
    pub fn write_record(&mut self, record: &Record) -> Result<(), PersistError> {
        let mut line = serde_json::to_vec(record).map_err(PersistError::Serialize)?;
        line.push(b'\n');
        self.inner.write_all(&line)?;
        Ok(())
    }

    /// Write one gate outcome.
    pub fn write_gate(&mut self, outcome: &GateOutcome) -> Result<(), PersistError> {
        self.write_record(&Record::gate(outcome.clone()))
    }

    /// Write every lineage entry added to `log` since the last sync, so callers
    /// can persist after each `propose` / `commit` / `fail` without duplicates.
    /// Returns how many entries were written.
    pub fn sync_lineage(&mut self, log: &LineageLog) -> Result<usize, PersistError> {
        let entries = log.entries();
        let start = self.lineage_cursor.min(entries.len());
        for entry in &entries[start..] {
            self.write_record(&Record::lineage(entry.clone()))?;
            self.lineage_cursor += 1;
        }
        Ok(entries.len() - start)
    }

    /// Number of lineage entries this writer has accounted for.
    pub fn lineage_cursor(&self) -> usize {
        self.lineage_cursor
    }

    /// Flush buffered output.
    pub fn flush(&mut self) -> Result<(), PersistError> {
        self.inner.flush()?;
        Ok(())
    }

    /// Recover the inner writer.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl JsonlWriter<File> {
    /// Open `path` for appending, creating the file (not its parent directories)
    /// if it does not exist. Refuses a non-empty file whose last byte is not a
    /// newline ([`PersistError::UnterminatedFile`]).
    pub fn append_to(path: &Path, lineage_cursor: usize) -> Result<Self, PersistError> {
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(path)?;
        if !ends_with_newline_or_empty(&mut file)? {
            return Err(PersistError::UnterminatedFile);
        }
        Ok(Self::with_lineage_cursor(file, lineage_cursor))
    }

    /// Flush and ask the OS to persist the file to storage.
    pub fn sync_all(&mut self) -> Result<(), PersistError> {
        self.inner.flush()?;
        self.inner.sync_all()?;
        Ok(())
    }
}

/// What to do with a final line that has no trailing newline and does not parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TailPolicy {
    /// Treat it as an error (default).
    Strict,
    /// Drop it and report it in [`ReadOutput::dropped_torn_tail`].
    DropTornTail,
}

/// Result of reading a JSONL stream.
#[derive(Debug, Clone, PartialEq)]
pub struct ReadOutput {
    /// Every record, in file order, with its 1-based line number.
    pub records: Vec<(usize, Record)>,
    /// True if a torn final line was dropped under [`TailPolicy::DropTornTail`].
    pub dropped_torn_tail: bool,
}

impl ReadOutput {
    /// Gate outcomes in file order.
    pub fn gate_outcomes(&self) -> Vec<&GateOutcome> {
        self.records
            .iter()
            .filter_map(|(_, r)| match r {
                Record::Gate { outcome, .. } => Some(outcome),
                Record::Lineage { .. } => None,
            })
            .collect()
    }

    /// Rebuild a [`LineageLog`] by replaying every lineage entry through the log's
    /// own API. Any entry the API would refuse is an error naming its line.
    pub fn replay(&self) -> Result<LineageLog, PersistError> {
        let mut log = LineageLog::new();
        for (line, record) in &self.records {
            let Record::Lineage { entry, .. } = record else {
                continue;
            };
            let at = |error| PersistError::Lineage { line: *line, error };
            match entry {
                LogEntry::Proposed { event } => {
                    log.propose(event.clone()).map_err(at)?;
                }
                LogEntry::Committed { event_id } => log.commit(event_id).map_err(at)?,
                LogEntry::Failed { event_id, reason } => log.fail(event_id, reason).map_err(at)?,
            }
        }
        Ok(log)
    }
}

/// Read records from any buffered reader. Blank lines are skipped.
pub fn read_records<R: BufRead>(
    mut reader: R,
    tail: TailPolicy,
) -> Result<ReadOutput, PersistError> {
    let mut records = Vec::new();
    let mut dropped_torn_tail = false;
    let mut buf = String::new();
    let mut line_no = 0usize;
    loop {
        buf.clear();
        if reader.read_line(&mut buf)? == 0 {
            break;
        }
        line_no += 1;
        let terminated = buf.ends_with('\n');
        let text = buf.trim_end_matches(['\n', '\r']);
        if text.trim().is_empty() {
            continue;
        }
        match serde_json::from_str::<Record>(text) {
            Ok(record) => {
                if record.schema_version() != SCHEMA_VERSION {
                    return Err(PersistError::UnsupportedVersion {
                        line: line_no,
                        found: record.schema_version(),
                    });
                }
                if let Record::Lineage {
                    entry: LogEntry::Proposed { event },
                    ..
                } = &record
                {
                    event.validate().map_err(|error| PersistError::Lineage {
                        line: line_no,
                        error,
                    })?;
                }
                records.push((line_no, record));
            }
            Err(_) if !terminated && tail == TailPolicy::DropTornTail => {
                dropped_torn_tail = true;
            }
            Err(e) => {
                return Err(PersistError::Parse {
                    line: line_no,
                    message: e.to_string(),
                });
            }
        }
    }
    Ok(ReadOutput {
        records,
        dropped_torn_tail,
    })
}

fn ends_with_newline_or_empty(file: &mut File) -> io::Result<bool> {
    let len = file.metadata()?.len();
    if len == 0 {
        return Ok(true);
    }
    file.seek(SeekFrom::Start(len - 1))?;
    let mut last = [0u8; 1];
    file.read_exact(&mut last)?;
    Ok(last[0] == b'\n')
}

/// Remove everything after the last newline in `path` (a torn final line left by
/// an interrupted write). Returns the number of bytes removed; 0 if the file
/// already ends with a newline or is empty. This discards data, so call it only
/// when [`read_file`] with [`TailPolicy::DropTornTail`] reports a dropped tail.
pub fn truncate_torn_tail(path: &Path) -> Result<u64, PersistError> {
    let bytes = fs::read(path)?;
    let keep = bytes.iter().rposition(|&b| b == b'\n').map_or(0, |i| i + 1);
    let removed = (bytes.len() - keep) as u64;
    if removed > 0 {
        let file = OpenOptions::new().write(true).open(path)?;
        file.set_len(keep as u64)?;
        file.sync_all()?;
    }
    Ok(removed)
}

/// Read records from a file.
pub fn read_file(path: &Path, tail: TailPolicy) -> Result<ReadOutput, PersistError> {
    read_records(BufReader::new(File::open(path)?), tail)
}

/// SHA-256 fingerprint of a lineage log's entries in order, as lowercase hex.
/// Two logs with the same entries in the same order have the same digest.
/// Record it with experiment results to pin exactly which history they came from.
pub fn lineage_digest(log: &LineageLog) -> Result<String, PersistError> {
    let mut h = Sha256::new();
    h.update(b"kwaai-provenance:lineage-digest:v1\n");
    for entry in log.entries() {
        let line =
            serde_json::to_vec(&Record::lineage(entry.clone())).map_err(PersistError::Serialize)?;
        h.update((line.len() as u64).to_be_bytes());
        h.update(&line);
    }
    Ok(hex::encode(h.finalize()))
}

/// Write the whole lineage log to `path` as a self-contained JSONL snapshot and
/// return its digest. The file is written to a temporary sibling, synced, then
/// renamed into place, so `path` is either the old file or the complete new one.
pub fn write_snapshot(log: &LineageLog, path: &Path) -> Result<String, PersistError> {
    let file_name = path.file_name().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "snapshot path has no file name",
        )
    })?;
    let mut tmp_name = file_name.to_os_string();
    tmp_name.push(".tmp");
    let tmp = path.with_file_name(tmp_name);
    {
        let mut w = JsonlWriter::new(File::create(&tmp)?);
        w.sync_lineage(log)?;
        w.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    lineage_digest(log)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::{Gate, GateMode, UnresolvedVerifier};
    use crate::lineage::{LineageEvent, LineageEventKind::*};
    use crate::schema::{Claim, ClaimKind};
    use std::io::Cursor;

    fn ev(
        kind: crate::lineage::LineageEventKind,
        i: &[&str],
        o: &[&str],
        seq: u64,
    ) -> LineageEvent {
        LineageEvent::new(
            kind,
            i.iter().copied(),
            o.iter().copied(),
            "test",
            Some(1),
            seq,
        )
        .unwrap()
    }

    fn sample_log() -> LineageLog {
        let mut log = LineageLog::new();
        let a = log.propose(ev(Extract, &["orig:1"], &["c1"], 0)).unwrap();
        log.commit(&a).unwrap();
        let b = log.propose(ev(ClaimRewrite, &["c1"], &["c2"], 1)).unwrap();
        log.commit(&b).unwrap();
        let c = log.propose(ev(Derive, &["orig:2"], &["sum"], 2)).unwrap();
        log.fail(&c, "graph write failed:\nlock held").unwrap();
        log
    }

    fn to_bytes(log: &LineageLog) -> Vec<u8> {
        let mut w = JsonlWriter::new(Vec::new());
        w.sync_lineage(log).unwrap();
        w.into_inner()
    }

    fn read_strict(bytes: &[u8]) -> Result<ReadOutput, PersistError> {
        read_records(Cursor::new(bytes), TailPolicy::Strict)
    }

    #[test]
    fn lineage_round_trips_and_replays_to_same_history() {
        let log = sample_log();
        let bytes = to_bytes(&log);
        assert_eq!(
            bytes.iter().filter(|&&b| b == b'\n').count(),
            log.entries().len()
        );
        let back = read_strict(&bytes).unwrap().replay().unwrap();
        assert_eq!(back.entries(), log.entries());
        assert_eq!(back.ancestors("c2"), log.ancestors("c2"));
        assert!(back.traces_to("c2", |a| a.starts_with("orig:")));
        assert!(!back.traces_to("sum", |a| a.starts_with("orig:")));
        assert_eq!(
            lineage_digest(&back).unwrap(),
            lineage_digest(&log).unwrap()
        );
    }

    #[test]
    fn records_carry_kind_and_schema_version() {
        let line = String::from_utf8(to_bytes(&sample_log())).unwrap();
        let first = line.lines().next().unwrap();
        assert!(first.starts_with("{\"record\":\"lineage\",\"schema_version\":1,"));
        assert!(first.contains("\"entry\":\"proposed\""));
    }

    #[test]
    fn gate_outcomes_round_trip_alongside_lineage() {
        let claim = Claim::new(ClaimKind::Relation, "alice", "works_at", "kwaai");
        let out = Gate::new(GateMode::Observe)
            .check(&claim, &[], &UnresolvedVerifier)
            .unwrap();
        let log = sample_log();
        let mut w = JsonlWriter::new(Vec::new());
        w.sync_lineage(&log).unwrap();
        w.write_gate(&out).unwrap();
        let read = read_strict(&w.into_inner()).unwrap();
        assert_eq!(read.gate_outcomes(), vec![&out]);
        assert_eq!(read.replay().unwrap().entries(), log.entries());
    }

    #[test]
    fn sync_lineage_writes_only_new_entries() {
        let mut log = LineageLog::new();
        let mut w = JsonlWriter::new(Vec::new());
        let id = log.propose(ev(Extract, &["orig:1"], &["c1"], 0)).unwrap();
        assert_eq!(w.sync_lineage(&log).unwrap(), 1);
        assert_eq!(w.sync_lineage(&log).unwrap(), 0);
        log.commit(&id).unwrap();
        assert_eq!(w.sync_lineage(&log).unwrap(), 1);
        assert_eq!(w.lineage_cursor(), 2);
        let back = read_strict(&w.into_inner()).unwrap().replay().unwrap();
        assert_eq!(back.entries(), log.entries());
    }

    #[test]
    fn tampered_event_is_rejected_with_line_number() {
        let text = String::from_utf8(to_bytes(&sample_log())).unwrap();
        let tampered = text.replacen("\"orig:1\"", "\"orig:X\"", 1);
        match read_strict(tampered.as_bytes()) {
            Err(PersistError::Lineage {
                line: 1,
                error: LineageError::IdMismatch,
            }) => {}
            other => panic!("expected IdMismatch on line 1, got {other:?}"),
        }
    }

    #[test]
    fn invalid_transition_on_disk_is_rejected_at_replay() {
        let log = sample_log();
        let committed = Record::lineage(log.entries()[1].clone());
        let mut w = JsonlWriter::new(Vec::new());
        w.write_record(&committed).unwrap();
        let read = read_strict(&w.into_inner()).unwrap();
        assert!(matches!(
            read.replay(),
            Err(PersistError::Lineage {
                line: 1,
                error: LineageError::UnknownEvent(_)
            })
        ));
    }

    #[test]
    fn cycle_on_disk_is_rejected_at_replay() {
        let mut log = LineageLog::new();
        for (i, (a, b)) in [("a", "b"), ("b", "c")].into_iter().enumerate() {
            let id = log.propose(ev(Derive, &[a], &[b], i as u64)).unwrap();
            log.commit(&id).unwrap();
        }
        let back = ev(Derive, &["c"], &["a"], 2);
        let mut w = JsonlWriter::new(Vec::new());
        w.sync_lineage(&log).unwrap();
        w.write_record(&Record::lineage(LogEntry::Proposed {
            event: back.clone(),
        }))
        .unwrap();
        w.write_record(&Record::lineage(LogEntry::Committed { event_id: back.id }))
            .unwrap();
        let read = read_strict(&w.into_inner()).unwrap();
        assert!(matches!(
            read.replay(),
            Err(PersistError::Lineage {
                line: 6,
                error: LineageError::CycleDetected { .. }
            })
        ));
    }

    #[test]
    fn unsupported_schema_version_is_rejected() {
        let text = String::from_utf8(to_bytes(&sample_log())).unwrap();
        let bumped = text.replacen("\"schema_version\":1", "\"schema_version\":2", 1);
        assert!(matches!(
            read_strict(bumped.as_bytes()),
            Err(PersistError::UnsupportedVersion { line: 1, found: 2 })
        ));
    }

    #[test]
    fn malformed_middle_line_is_an_error_even_when_lenient() {
        let mut text = String::from_utf8(to_bytes(&sample_log())).unwrap();
        text.insert_str(0, "{not json}\n");
        for policy in [TailPolicy::Strict, TailPolicy::DropTornTail] {
            assert!(matches!(
                read_records(Cursor::new(text.as_bytes()), policy),
                Err(PersistError::Parse { line: 1, .. })
            ));
        }
    }

    #[test]
    fn torn_tail_is_strict_error_or_dropped_on_request() {
        let mut bytes = to_bytes(&sample_log());
        bytes.extend_from_slice(b"{\"record\":\"lineage\",\"schema_ver");
        assert!(matches!(
            read_strict(&bytes),
            Err(PersistError::Parse { line: 7, .. })
        ));
        let lenient = read_records(Cursor::new(&bytes), TailPolicy::DropTornTail).unwrap();
        assert!(lenient.dropped_torn_tail);
        assert_eq!(lenient.records.len(), 6);
        assert_eq!(lenient.replay().unwrap().entries(), sample_log().entries());
    }

    #[test]
    fn blank_lines_and_crlf_are_tolerated() {
        let text = String::from_utf8(to_bytes(&sample_log())).unwrap();
        let crlf = format!("\n{}\n\n", text.replace('\n', "\r\n"));
        let read = read_strict(crlf.as_bytes()).unwrap();
        assert_eq!(read.replay().unwrap().entries(), sample_log().entries());
        assert!(!read.dropped_torn_tail);
    }

    #[test]
    fn newline_in_a_reason_stays_on_one_line() {
        let bytes = to_bytes(&sample_log());
        let text = String::from_utf8(bytes).unwrap();
        assert_eq!(text.lines().count(), 6);
        assert!(text.contains("graph write failed:\\nlock held"));
    }

    #[test]
    fn digest_is_deterministic_and_order_sensitive() {
        let a = sample_log();
        assert_eq!(
            lineage_digest(&a).unwrap(),
            lineage_digest(&sample_log()).unwrap()
        );
        assert_eq!(lineage_digest(&a).unwrap().len(), 64);
        let mut b = LineageLog::new();
        let x = b.propose(ev(Extract, &["orig:1"], &["c1"], 0)).unwrap();
        assert_ne!(lineage_digest(&a).unwrap(), lineage_digest(&b).unwrap());
        b.commit(&x).unwrap();
        assert_ne!(lineage_digest(&a).unwrap(), lineage_digest(&b).unwrap());
        assert_ne!(
            lineage_digest(&LineageLog::new()).unwrap(),
            lineage_digest(&b).unwrap()
        );
    }

    #[test]
    fn file_append_reopen_and_snapshot() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lineage.jsonl");
        let mut log = LineageLog::new();
        let id = log.propose(ev(Extract, &["orig:1"], &["c1"], 0)).unwrap();
        {
            let mut w = JsonlWriter::append_to(&path, 0).unwrap();
            w.sync_lineage(&log).unwrap();
            w.sync_all().unwrap();
        }
        // Reopen: replay what is on disk, continue appending from there.
        let mut log = read_file(&path, TailPolicy::Strict)
            .unwrap()
            .replay()
            .unwrap();
        log.commit(&id).unwrap();
        {
            let mut w = JsonlWriter::append_to(&path, 1).unwrap();
            assert_eq!(w.sync_lineage(&log).unwrap(), 1);
            w.sync_all().unwrap();
        }
        let reread = read_file(&path, TailPolicy::Strict)
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!(reread.entries(), log.entries());

        let snap = dir.path().join("snapshot.jsonl");
        let digest = write_snapshot(&log, &snap).unwrap();
        assert_eq!(digest, lineage_digest(&log).unwrap());
        assert!(!dir.path().join("snapshot.jsonl.tmp").exists());
        let from_snap = read_file(&snap, TailPolicy::Strict)
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!(lineage_digest(&from_snap).unwrap(), digest);
    }

    #[test]
    fn append_refuses_torn_file_until_repaired() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lineage.jsonl");
        let log = sample_log();
        let mut bytes = to_bytes(&log);
        let torn: &[u8] = b"{\"record\":\"lin";
        bytes.extend_from_slice(torn);
        fs::write(&path, &bytes).unwrap();

        assert!(matches!(
            JsonlWriter::append_to(&path, 0),
            Err(PersistError::UnterminatedFile)
        ));
        let read = read_file(&path, TailPolicy::DropTornTail).unwrap();
        assert!(read.dropped_torn_tail);

        assert_eq!(truncate_torn_tail(&path).unwrap(), torn.len() as u64);
        assert_eq!(truncate_torn_tail(&path).unwrap(), 0);
        let mut w = JsonlWriter::append_to(&path, log.entries().len()).unwrap();
        assert_eq!(w.sync_lineage(&log).unwrap(), 0);
        let back = read_file(&path, TailPolicy::Strict)
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!(back.entries(), log.entries());
    }

    #[test]
    fn missing_file_is_an_io_error() {
        let dir = tempfile::tempdir().unwrap();
        assert!(matches!(
            read_file(&dir.path().join("nope.jsonl"), TailPolicy::Strict),
            Err(PersistError::Io(_))
        ));
    }
}
