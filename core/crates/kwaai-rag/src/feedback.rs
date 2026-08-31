//! Human relevance judgements and answer scores from interactive chat.
//!
//! Persists to `<rag_dir>/feedback.db` (SQLite WAL), alongside `query_cache.db`.
//! Chunks are keyed by `(doc_name, chunk_index)` — see [`crate::iterative::chunk_key`] —
//! because retrieval discards the storage row id, so that pair is the only identity
//! that survives retrieval, reranking and reordering.
//!
//! Every write is best-effort: a feedback failure must never break a chat turn.

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::path::Path;

pub struct FeedbackStore {
    conn: Connection,
}

// rusqlite::Connection is !Send. Like QueryCache, FeedbackStore is only ever touched
// from the single task that owns the chat loop; it must not be moved into a spawned
// task.
unsafe impl Send for FeedbackStore {}

/// One recorded relevance mark.
#[derive(Debug, Clone)]
pub struct ChunkMark {
    pub doc_name: String,
    pub chunk_index: u32,
    /// `+1` relevant, `-1` irrelevant.
    pub mark: i8,
}

impl FeedbackStore {
    pub fn open(data_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(data_dir)?;
        let path = data_dir.join("feedback.db");
        let conn = Connection::open(&path)
            .with_context(|| format!("opening feedback store at {}", path.display()))?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL;
             PRAGMA synchronous=NORMAL;",
        )?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS chunk_marks (
                kb          TEXT    NOT NULL,
                query       TEXT    NOT NULL,
                doc_name    TEXT    NOT NULL,
                chunk_index INTEGER NOT NULL,
                mark        INTEGER NOT NULL,
                ts          INTEGER NOT NULL,
                PRIMARY KEY (kb, query, doc_name, chunk_index)
             );
             CREATE TABLE IF NOT EXISTS answer_scores (
                kb           TEXT    NOT NULL,
                query        TEXT    NOT NULL,
                answer_hash  TEXT    NOT NULL,
                judge_score  REAL,
                user_score   REAL,
                grounded     INTEGER,
                rationale    TEXT,
                ts           INTEGER NOT NULL,
                PRIMARY KEY (kb, query, answer_hash)
             );",
        )?;
        Ok(Self { conn })
    }

    fn now() -> i64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// Record relevance marks for one query. Re-marking an item replaces the mark.
    pub fn record_marks(&self, kb: &str, query: &str, marks: &[ChunkMark]) -> Result<()> {
        let ts = Self::now();
        for m in marks {
            self.conn.execute(
                "INSERT INTO chunk_marks (kb, query, doc_name, chunk_index, mark, ts)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)
                 ON CONFLICT(kb, query, doc_name, chunk_index)
                 DO UPDATE SET mark = excluded.mark, ts = excluded.ts",
                params![kb, query, m.doc_name, m.chunk_index, m.mark as i64, ts],
            )?;
        }
        Ok(())
    }

    /// Record a judge and/or human score for one answer.
    ///
    /// Called once when the judge lands and again if the user scores it; the update
    /// preserves whichever value is not being supplied.
    #[allow(clippy::too_many_arguments)]
    pub fn record_score(
        &self,
        kb: &str,
        query: &str,
        answer: &str,
        judge_score: Option<f32>,
        user_score: Option<f32>,
        grounded: Option<bool>,
        rationale: Option<&str>,
    ) -> Result<()> {
        let hash = answer_hash(answer);
        self.conn.execute(
            "INSERT INTO answer_scores
                (kb, query, answer_hash, judge_score, user_score, grounded, rationale, ts)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
             ON CONFLICT(kb, query, answer_hash) DO UPDATE SET
                judge_score = COALESCE(excluded.judge_score, answer_scores.judge_score),
                user_score  = COALESCE(excluded.user_score,  answer_scores.user_score),
                grounded    = COALESCE(excluded.grounded,    answer_scores.grounded),
                rationale   = COALESCE(excluded.rationale,   answer_scores.rationale),
                ts          = excluded.ts",
            params![
                kb,
                query,
                hash,
                judge_score,
                user_score,
                grounded.map(|g| g as i64),
                rationale,
                Self::now()
            ],
        )?;
        Ok(())
    }

    pub fn mark_count(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM chunk_marks", [], |r| r.get(0))?)
    }

    pub fn score_count(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("SELECT COUNT(*) FROM answer_scores", [], |r| r.get(0))?)
    }
}

fn answer_hash(answer: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(answer.as_bytes());
    format!("{:x}", h.finalize())[..16].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn store() -> (FeedbackStore, tempfile::TempDir) {
        let d = tempfile::tempdir().unwrap();
        let s = FeedbackStore::open(d.path()).unwrap();
        (s, d)
    }

    #[test]
    fn marks_upsert_rather_than_duplicate() {
        let (s, _d) = store();
        let m = |mark| {
            vec![ChunkMark {
                doc_name: "memoir.pdf".into(),
                chunk_index: 7,
                mark,
            }]
        };
        s.record_marks("D6", "who?", &m(1)).unwrap();
        s.record_marks("D6", "who?", &m(-1)).unwrap();
        assert_eq!(
            s.mark_count().unwrap(),
            1,
            "re-marking must replace, not append"
        );
    }

    /// The judge writes first and the user scores later; neither may erase the other.
    #[test]
    fn judge_and_user_scores_merge() {
        let (s, _d) = store();
        s.record_score(
            "D6",
            "q",
            "the answer",
            Some(3.8),
            None,
            Some(true),
            Some("ok"),
        )
        .unwrap();
        s.record_score("D6", "q", "the answer", None, Some(5.0), None, None)
            .unwrap();
        assert_eq!(s.score_count().unwrap(), 1);
        let (j, u, g): (f32, f32, i64) = s
            .conn
            .query_row(
                "SELECT judge_score, user_score, grounded FROM answer_scores",
                [],
                |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
            )
            .unwrap();
        assert!((j - 3.8).abs() < 1e-6);
        assert!((u - 5.0).abs() < 1e-6);
        assert_eq!(g, 1);
    }

    #[test]
    fn distinct_answers_are_separate_rows() {
        let (s, _d) = store();
        s.record_score("D6", "q", "first", Some(2.0), None, None, None)
            .unwrap();
        s.record_score("D6", "q", "second", Some(4.0), None, None, None)
            .unwrap();
        assert_eq!(s.score_count().unwrap(), 2);
    }
}
