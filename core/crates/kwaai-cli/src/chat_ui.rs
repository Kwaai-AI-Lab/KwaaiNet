//! Presentation and interaction for `kwaainet rag chat`.
//!
//! # The stdout invariant
//!
//! `tests/kwaai-knowledge/test_chat_stdout_clean.sh` guards a real bug: chat once read
//! its own stdout progress lines back as user input and looped forever. **stdout is
//! reserved for the `You:` / `Assistant:` protocol; everything else goes to stderr.**
//!
//! That is enforced structurally here: every function in this module *returns* lines
//! rather than printing them. Only [`play`] touches a terminal, and it writes solely
//! through a [`StatusLine`], which is stderr-bound. Nothing in this module can reach
//! stdout even by accident, and all of it is unit-testable without a terminal.

use std::time::Duration;

use kwaai_rag::iterative::coverage_terms;
use kwaai_rag::meta_store::ChunkMeta;
use kwaai_rag::reranker::Marks;
use kwaai_rag::retriever::{origin_of, ChunkOrigin, RetrievedChunk};

use crate::display::{display_width, sanitize_cell, truncate_to_width, Align, Table};
use crate::progress::StatusLine;

// ── row model ─────────────────────────────────────────────────────────────────

/// One retrieved item, as the user sees it.
pub struct ChunkView {
    /// 1-based citation number, i.e. the `[n]` the model was shown. `None` when the
    /// chunk was cut by the context budget and never reached the model at all.
    pub cite: Option<usize>,
    pub origin: ChunkOrigin,
    /// Document name, or the entity name for graph-derived chunks.
    pub source: String,
    /// "p. 42", "§ Shops", or "chunk 7".
    pub locator: String,
    /// The "as evidenced by" quote.
    pub evidence: String,
    pub mark: i8,
}

impl ChunkView {
    pub fn sent(&self) -> bool {
        self.cite.is_some()
    }
}

/// Where a chunk sits in the document.
fn locator_of(m: &ChunkMeta) -> String {
    if let Some(p) = m.page_num {
        return format!("p. {p}");
    }
    if let Some(ref s) = m.section_name {
        if !s.is_empty() {
            return format!("§ {}", truncate_to_width(s, 18));
        }
    }
    format!("chunk {}", m.chunk_index)
}

/// The most on-topic sentence of a chunk, for the "as evidenced by" column.
///
/// Graph-derived chunks are synthesised fact cards, not source text, so their first
/// structured line is shown instead of a quotation — quoting a synthesised card as if
/// it were a passage from the corpus would be dishonest.
pub fn pick_evidence(m: &ChunkMeta, origin: ChunkOrigin, terms: &[String], cols: usize) -> String {
    let body = if m.surrounding.len() > m.text.len() {
        &m.surrounding
    } else {
        &m.text
    };
    if origin.is_graph() || origin == ChunkOrigin::Timeline {
        let line = body
            .lines()
            .map(str::trim)
            .find(|l| !l.is_empty() && !l.starts_with("[Graph Query Result]"))
            .unwrap_or("");
        return truncate_to_width(&sanitize_cell(line), cols);
    }
    let best = body
        .split(['.', '\n'])
        .map(str::trim)
        .filter(|s| s.len() > 24)
        .max_by_key(|s| {
            let low = s.to_lowercase();
            terms.iter().filter(|t| low.contains(t.as_str())).count()
        })
        .unwrap_or_else(|| body.trim());
    let quoted = format!("\u{201c}{}\u{201d}", sanitize_cell(best));
    truncate_to_width(&quoted, cols)
}

/// Build the display rows. `plan` is [`kwaai_rag::prompt::context_plan`]'s output, so
/// row numbering *is* the citation numbering rather than a parallel calculation.
pub fn build_rows(
    chunks: &[RetrievedChunk],
    plan: &[usize],
    marks: &Marks,
    query: &str,
    evidence_cols: usize,
) -> Vec<ChunkView> {
    let terms = coverage_terms(query);
    let cite_of: std::collections::HashMap<usize, usize> = plan
        .iter()
        .enumerate()
        .map(|(slot, &idx)| (idx, slot + 1))
        .collect();

    let mut rows: Vec<ChunkView> = chunks
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let origin = origin_of(c);
            let m = &c.chunk_meta;
            let source = if origin.is_graph() {
                m.doc_name
                    .trim_start_matches("[Graph:")
                    .trim_end_matches(']')
                    .trim()
                    .to_string()
            } else {
                m.doc_name.clone()
            };
            ChunkView {
                cite: cite_of.get(&i).copied(),
                origin,
                source,
                locator: if origin.is_graph() {
                    String::from("knowledge graph")
                } else {
                    locator_of(m)
                },
                evidence: pick_evidence(m, origin, &terms, evidence_cols),
                mark: marks
                    .get(&kwaai_rag::iterative::chunk_key(c))
                    .copied()
                    .unwrap_or(0),
            }
        })
        .collect();

    // Cited rows first, in citation order; then whatever never reached the model.
    rows.sort_by_key(|r| r.cite.unwrap_or(usize::MAX));
    rows
}

// ── narration ─────────────────────────────────────────────────────────────────

/// One step of the narration script.
pub enum Unit {
    /// Prose, typed out at a readable pace.
    Prose(String),
    /// A line emitted whole — table rows must never be typed character by character,
    /// because a half-drawn column looks broken.
    Line(String),
}

/// Compose "Here's what we know…" from the retrieval result.
///
/// Shows rank and origin, never the raw `score`: scores share one field across
/// incomparable scales (a graph fact card is 3.0, an ordinary RRF-fused chunk is
/// 0.015–0.03), so a displayed number would actively mislead.
pub fn narration_script(
    rows: &[ChunkView],
    coverage: Option<(f32, Vec<String>)>,
    retrieval_secs: f64,
    max_rows: usize,
    width: usize,
) -> Vec<Unit> {
    let mut out = Vec::new();
    if rows.is_empty() {
        out.push(Unit::Prose(
            "  Here's what we know — nothing came back for that one.".into(),
        ));
        return out;
    }

    let docs: std::collections::BTreeSet<&str> = rows
        .iter()
        .filter(|r| !r.origin.is_graph())
        .map(|r| r.source.as_str())
        .collect();
    let n_graph = rows.iter().filter(|r| r.origin.is_graph()).count();
    let n_corpus = rows.len() - n_graph;

    out.push(Unit::Prose(format!(
        "  Here's what we know — {} passage{} from {} document{}, {:.1}s.",
        n_corpus,
        if n_corpus == 1 { "" } else { "s" },
        docs.len(),
        if docs.len() == 1 { "" } else { "s" },
        retrieval_secs,
    )));

    if n_graph > 0 {
        out.push(Unit::Line(String::new()));
        out.push(Unit::Prose("  From the knowledge graph:".into()));
        for r in rows.iter().filter(|r| r.origin.is_graph()).take(3) {
            let name = truncate_to_width(&r.source, 34);
            // The first line of an entity card is often just the name and its type,
            // which would render as "Grey Street — Grey Street [Place]".
            let detail = strip_echo(&r.evidence, &r.source);
            let room = width.saturating_sub(display_width(&name) + 10);
            out.push(Unit::Line(if detail.is_empty() {
                format!("    • {name}")
            } else {
                format!("    • {name} — {}", truncate_to_width(&detail, room))
            }));
        }
    }

    if n_corpus > 0 {
        out.push(Unit::Line(String::new()));
        // `assemble_results` RRF-fuses the dense and keyword rank lists before the
        // caller sees them, so which of the two found a given passage is genuinely
        // gone. Say "hybrid" rather than guess.
        out.push(Unit::Prose(
            "  From the corpus (hybrid dense + keyword search), as evidenced by:".into(),
        ));
        // When every passage comes from the same document, repeating its name down
        // the whole column is noise that crowds out the evidence.
        let single_doc = if docs.len() == 1 {
            docs.iter().next().copied()
        } else {
            None
        };
        if let Some(d) = single_doc {
            out.push(Unit::Line(format!("    {d}")));
        }
        out.push(Unit::Line(String::new()));

        let mut t = Table::new(&[
            "#",
            if single_doc.is_some() {
                "where"
            } else {
                "source"
            },
            "as evidenced by",
        ])
        .align(0, Align::Right)
        .flex(2);
        for r in rows.iter().filter(|r| !r.origin.is_graph()).take(max_rows) {
            t.push(vec![
                format!(
                    "{}{}",
                    match r.cite {
                        Some(n) => n.to_string(),
                        None => "\u{b7}".into(),
                    },
                    match r.mark {
                        m if m > 0 => "+",
                        m if m < 0 => "\u{2212}",
                        _ => "",
                    }
                ),
                // Every row in this table is non-graph, so an origin column would read
                // "corpus" all the way down; only the exceptions are worth naming.
                {
                    let base = if single_doc.is_some() {
                        r.locator.clone()
                    } else {
                        format!("{}  {}", r.source, r.locator)
                    };
                    match r.origin {
                        ChunkOrigin::Corpus => base,
                        other => format!("{} · {base}", other.label()),
                    }
                },
                r.evidence.clone(),
            ]);
        }
        for line in t.render(width) {
            out.push(Unit::Line(line));
        }

        let hidden = n_corpus.saturating_sub(max_rows);
        if hidden > 0 {
            out.push(Unit::Line(format!("    … and {hidden} more")));
        }
        // Count only the rows this table actually drew a `·` against. Counting every
        // un-cited row swept in graph rows (rendered as bullets, never marked) and rows
        // folded into "… and N more", so the number pointed at markers not on screen.
        let dropped = rows
            .iter()
            .filter(|r| !r.origin.is_graph())
            .take(max_rows)
            .filter(|r| !r.sent())
            .count();
        if dropped > 0 {
            out.push(Unit::Line(format!(
                "    · {dropped} marked \u{b7} did not fit the context budget and were not sent"
            )));
        }
    }

    if let Some((frac, missing)) = coverage {
        out.push(Unit::Line(String::new()));
        let mut line = format!("  Coverage: {:.0}% of query terms found", frac * 100.0);
        if !missing.is_empty() {
            line.push_str(&format!(" — nothing for: {}", missing.join(", ")));
        }
        out.push(Unit::Prose(line));
    }

    out
}

/// Drop a leading echo of `name` (optionally followed by a `[Type]` tag) from `line`.
fn strip_echo(line: &str, name: &str) -> String {
    let t = line.trim();
    let rest = t.strip_prefix(name).unwrap_or(t).trim_start();
    let rest = match (rest.find('['), rest.find(']')) {
        (Some(0), Some(e)) => rest[e + 1..].trim_start(),
        _ => rest,
    };
    let rest = rest.trim_start_matches([':', '\u{2014}', '-']).trim();
    rest.to_string()
}

// ── answer checks ─────────────────────────────────────────────────────────────

/// Citation numbers in `answer` that fall outside `1..=n_sources`.
///
/// A free, deterministic check for the exact hallucination mode the RAG system prompt
/// warns against — no LLM call involved.
pub fn check_citations(answer: &str, n_sources: usize) -> Vec<u32> {
    let mut bad = Vec::new();
    let bytes = answer.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            let mut j = i + 1;
            while j < bytes.len() && bytes[j].is_ascii_digit() {
                j += 1;
            }
            if j > i + 1 && j < bytes.len() && bytes[j] == b']' {
                if let Ok(n) = answer[i + 1..j].parse::<u32>() {
                    if (n == 0 || n as usize > n_sources) && !bad.contains(&n) {
                        bad.push(n);
                    }
                }
                i = j;
            }
        }
        i += 1;
    }
    bad
}

// ── commands ──────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Eq)]
pub enum ChatCmd {
    /// `+1 3` / `-2` — mark items relevant or irrelevant.
    Mark(Vec<usize>, i8),
    /// `?4` — show one item in full.
    Show(usize),
    Retry,
    Score(u8),
    Why,
    Marks,
    ClearMarks,
    Help,
    Continue,
    Exit,
    /// Anything else: the next question.
    Ask(String),
}

/// Parse one line of interactive input.
///
/// A line is a command only if it matches wholly; everything else is the next
/// question, so an ordinary sentence can never be swallowed as a command.
pub fn parse_cmd(line: &str) -> ChatCmd {
    let t = line.trim();
    if t.is_empty() {
        return ChatCmd::Continue;
    }
    match t {
        "exit" | "quit" | "/exit" | "/quit" => return ChatCmd::Exit,
        "/retry" | "/r" => return ChatCmd::Retry,
        "/why" => return ChatCmd::Why,
        "/marks" => return ChatCmd::Marks,
        "/clear" => return ChatCmd::ClearMarks,
        "/help" | "/?" => return ChatCmd::Help,
        _ => {}
    }

    if let Some(rest) = t.strip_prefix("/score") {
        if let Ok(n) = rest.trim().parse::<u8>() {
            if (1..=5).contains(&n) {
                return ChatCmd::Score(n);
            }
        }
        return ChatCmd::Help;
    }

    if let Some(rest) = t.strip_prefix('?') {
        if let Ok(n) = rest.trim().parse::<usize>() {
            if n > 0 {
                return ChatCmd::Show(n);
            }
        }
    }

    if let Some(sign) = t.chars().next().filter(|c| *c == '+' || *c == '-') {
        let rest = &t[1..];
        let nums: Vec<&str> = rest.split_whitespace().collect();
        if !nums.is_empty() {
            let parsed: Option<Vec<usize>> = nums
                .iter()
                .map(|s| s.parse::<usize>().ok().filter(|n| *n > 0))
                .collect();
            if let Some(ns) = parsed {
                return ChatCmd::Mark(ns, if sign == '+' { 1 } else { -1 });
            }
        }
    }

    ChatCmd::Ask(t.to_string())
}

pub fn help_lines() -> Vec<String> {
    vec![
        "  +1 3      mark items 1 and 3 relevant       -2       mark item 2 irrelevant".into(),
        "  ?4        show item 4 in full               /retry   re-answer using your marks".into(),
        "  /score N  rate this answer 1-5              /why     replay the retrieval trace".into(),
        "  /marks    list current marks                /clear   forget them".into(),
        "  Enter     continue                          exit     quit".into(),
    ]
}

// ── playback ──────────────────────────────────────────────────────────────────

/// Play a narration script, abandoning the pacing the moment `fast_fwd` goes true.
///
/// The pacing exists to fill the time the completion is generating; it must never
/// *become* the wait. When the completion lands early the rest of the script is
/// flushed instantly.
pub async fn play(
    script: Vec<Unit>,
    st: &mut StatusLine,
    per_char: Duration,
    fast_fwd: &dyn Fn() -> bool,
) {
    let line_gap = per_char * 6;
    for unit in script {
        let ff = fast_fwd();
        match unit {
            Unit::Prose(s) => {
                if ff {
                    st.emit(&s);
                } else {
                    st.emit_paced(&s, per_char, fast_fwd).await;
                }
            }
            Unit::Line(s) => {
                st.emit(&s);
                if !ff && !line_gap.is_zero() {
                    tokio::time::sleep(line_gap).await;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_marks() {
        assert_eq!(parse_cmd("+1 3"), ChatCmd::Mark(vec![1, 3], 1));
        assert_eq!(parse_cmd("-2"), ChatCmd::Mark(vec![2], -1));
        assert_eq!(parse_cmd("  +4  "), ChatCmd::Mark(vec![4], 1));
    }

    #[test]
    fn parses_slash_commands() {
        assert_eq!(parse_cmd("/retry"), ChatCmd::Retry);
        assert_eq!(parse_cmd(" /retry "), ChatCmd::Retry);
        assert_eq!(parse_cmd("/score 4"), ChatCmd::Score(4));
        assert_eq!(parse_cmd("?4"), ChatCmd::Show(4));
        assert_eq!(parse_cmd(""), ChatCmd::Continue);
        assert_eq!(parse_cmd("exit"), ChatCmd::Exit);
        assert_eq!(parse_cmd("/exit"), ChatCmd::Exit);
    }

    #[test]
    fn out_of_range_score_is_not_silently_accepted() {
        assert_eq!(parse_cmd("/score 9"), ChatCmd::Help);
        assert_eq!(parse_cmd("/score"), ChatCmd::Help);
    }

    /// An ordinary question must never be swallowed as a command — including one that
    /// begins with a digit-adjacent character or ends in a question mark.
    #[test]
    fn questions_fall_through_to_ask() {
        assert_eq!(
            parse_cmd("who owned the shop?"),
            ChatCmd::Ask("who owned the shop?".into())
        );
        assert_eq!(
            parse_cmd("-- what happened --"),
            ChatCmd::Ask("-- what happened --".into())
        );
        assert_eq!(
            parse_cmd("+ and - signs"),
            ChatCmd::Ask("+ and - signs".into())
        );
    }

    #[test]
    fn flags_out_of_range_citations_only() {
        assert_eq!(
            check_citations("grounded in [1] and [3].", 3),
            Vec::<u32>::new()
        );
        assert_eq!(check_citations("see [7]", 3), vec![7]);
        assert_eq!(check_citations("see [0]", 3), vec![0]);
        assert_eq!(check_citations("see [7] and [7] and [9]", 3), vec![7, 9]);
        assert_eq!(check_citations("no citations here", 3), Vec::<u32>::new());
    }

    #[test]
    fn table_aligns_wide_characters() {
        let mut t = Table::new(&["#", "source"]).align(0, Align::Right);
        t.push(vec!["1".into(), "日本語のテキスト".into()]);
        t.push(vec!["10".into(), "ascii".into()]);
        let lines = t.render(80);
        let widths: Vec<usize> = lines
            .iter()
            .map(|l| crate::display::display_width(l))
            .collect();
        assert!(
            widths.windows(2).all(|w| w[0] == w[1]),
            "ragged table: {widths:?}\n{}",
            lines.join("\n")
        );
    }

    /// Chunk text routinely contains newlines and tabs; one unsanitized cell would
    /// destroy the alignment of the whole table.
    #[test]
    fn table_sanitizes_embedded_newlines() {
        let mut t = Table::new(&["a"]);
        t.push(vec!["line one\nline two\ttabbed".into()]);
        let lines = t.render(80);
        assert_eq!(lines.len(), 5, "a one-row table must render as 5 lines");
        assert!(lines.iter().all(|l| !l.contains('\n') && !l.contains('\t')));
    }

    #[test]
    fn table_shrinks_flex_column_to_fit() {
        let mut t = Table::new(&["#", "evidence"]).flex(1);
        t.push(vec!["1".into(), "x".repeat(200)]);
        for line in t.render(60) {
            assert!(
                crate::display::display_width(&line) <= 60,
                "line overflows: {} cols",
                crate::display::display_width(&line)
            );
        }
    }

    #[test]
    fn truncation_is_width_aware() {
        assert_eq!(truncate_to_width("abcdef", 10), "abcdef");
        assert_eq!(
            crate::display::display_width(&truncate_to_width("abcdef", 4)),
            4
        );
        assert!(crate::display::display_width(&truncate_to_width("日本語テキスト", 6)) <= 6);
    }
}
