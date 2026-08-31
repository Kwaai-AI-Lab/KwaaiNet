use crate::retriever::RetrievedChunk;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

/// Build a standalone RAG prompt string (for non-chat completions).
pub fn build_rag_prompt(
    user_query: &str,
    chunks: &[RetrievedChunk],
    max_context_chars: usize,
) -> String {
    let plan = context_plan(chunks, max_context_chars);
    let context = render_context_block(chunks, &plan);
    let n = plan.len();
    format!(
        "You are a research assistant. Use only the {n} source excerpt(s) below to answer.\n\
         Cite each fact with its source number in brackets, e.g. [1]. \
         If the answer is not in the sources, say so — do not fabricate.\n\n\
         Sources:\n{context}\n\n\
         Question: {user_query}\n\n\
         Answer:"
    )
}

/// Build a chat message list for `/v1/chat/completions`.
///
/// `doc_context` is an optional one-line preamble about the source document
/// (e.g. `"District Six - Lest We Forget" by Yousuf (Joe) Rassool (1984)`)
/// injected before the sources so the LLM always knows authorship and subject.
pub fn build_chat_messages(
    user_query: &str,
    chunks: &[RetrievedChunk],
    history: &[ChatMessage],
    max_context_chars: usize,
    doc_context: Option<&str>,
) -> Vec<ChatMessage> {
    // The plan, not the chunk list, is what the model actually sees: anything the
    // budget dropped was never numbered. Numbering by `chunks.len()` told the model it
    // could cite up to [n] while `check_citations` bounded it by `plan.len()`, so a
    // citation the prompt had authorised was reported back as a fabricated source.
    let plan = context_plan(chunks, max_context_chars);
    let context = render_context_block(chunks, &plan);

    let doc_preamble = match doc_context {
        Some(dc) if !dc.is_empty() => format!(
            "Document being queried: {dc}\n\
             Use this document context to resolve references like \"the author\", \
             \"the book\", or \"this memoir\" in the question.\n\n"
        ),
        _ => String::new(),
    };

    let n = plan.len();
    let system = ChatMessage {
        role: "system".to_string(),
        content: format!(
            "{doc_preamble}\
             You are a research assistant. The following {n} source excerpt(s) are numbered [1]–[{n}].\n\n\
             Rules you must follow:\n\
             1. Read ALL excerpts before answering. Names, dates, and facts may appear \
                in any excerpt, not just the first one.\n\
             2. Every factual claim must cite its source number in brackets, \
                e.g. \"The author is Joe Rassool [9].\" Only cite numbers that exist: [1]–[{n}]. \
                Citing a number outside this range means you are fabricating a source.\n\
             3. ABSOLUTE RULE — your general knowledge is not a valid source for this task. \
                Every name, date, place, title, or fact you state MUST appear in the numbered \
                excerpts above. Do not add information from memory or training even if you \
                believe it is correct. Do not dispute or 'correct' information in the sources — \
                if a source says 1898, write 1898.\n\
             4. If the answer is clearly present in the excerpts, give it directly — \
                do not hedge. If it is absent, say exactly: \
                \"The provided sources do not contain that information.\"\n\
             5. For factual questions (who, what, where, when), state what the sources \
                actually say first, then note any gaps.\n\
             6. If sources partially address the question, synthesise what they do say \
                and note what is missing.\n\
             7. Biographical questions about a person (\"Who was X?\", \"Tell me about X\") \
                require comprehensive answers — examine every numbered excerpt and list ALL \
                specific facts those excerpts state: dates, places, origins, ancestry, family \
                connections, occupations, achievements, and historical associations. \
                Comprehensiveness means covering every source, NOT adding facts from general \
                knowledge. A single-sentence answer is never sufficient; list each source-cited \
                fact explicitly with its bracket citation.\n\
             8. When an excerpt lists multiple items — names, organisations, places, dates — \
                enumerate ALL of them. Do not stop after 2–3 examples; include every item the \
                source provides. This applies especially to lists of notable acquaintances, \
                children, siblings, or member organisations.\n\
             9. Write all organisation and place names in full. Do not use abbreviations \
                (C.A.C., AAC, WPICU) unless you also write the full name in the same sentence, \
                e.g. \"the Coloured Advisory Council (C.A.C.)\".\n\
             10. \"From where\" and origin questions require the COMPLETE origin chain: both \
                the immediate prior location (last stop) AND the original homeland, ancestry, \
                or province where the person was born or raised. Do not answer with only \
                the penultimate stop.\n\n\
             Sources:\n{context}"
        ),
    };

    let mut messages = vec![system];
    messages.extend_from_slice(history);
    messages.push(ChatMessage {
        role: "user".to_string(),
        content: user_query.to_string(),
    });
    messages
}

/// Render a context block from an already-computed plan.
///
/// Callers that need the plan's length — to number the excerpts, or to bound a
/// citation check — must render from the same plan they measured, or the two
/// disagree about how many sources the model was given.
fn render_context_block(chunks: &[RetrievedChunk], plan: &[usize]) -> String {
    let mut out = String::new();
    for (slot, &idx) in plan.iter().enumerate() {
        out.push_str(&format!("[{}] {}\n", slot + 1, context_text(&chunks[idx])));
    }
    out
}

/// The text a chunk contributes to the context block: the wider `surrounding`
/// window when there is more of it, otherwise the chunk itself.
pub fn context_text(chunk: &RetrievedChunk) -> &str {
    if chunk.chunk_meta.surrounding.len() > chunk.chunk_meta.text.len() {
        &chunk.chunk_meta.surrounding
    } else {
        &chunk.chunk_meta.text
    }
}

/// The order in which `n` chunks are presented to the model.
///
/// Even ranks first, then odd ranks reversed, so the best evidence lands at the
/// first and last positions where LLMs attend most (mitigating lost-in-the-middle).
/// Depends only on the count, which is what makes the citation mapping recoverable.
pub fn context_order(n: usize) -> Vec<usize> {
    let evens = (0..n).filter(|i| i % 2 == 0);
    let odds = (0..n).filter(|i| i % 2 == 1).rev();
    evens.chain(odds).collect()
}

/// Indices into `chunks`, in the exact order **and count** the model will see them.
///
/// Position `p` in the returned vector is cited by the model as `[p+1]`, so this is
/// the single source of truth for citation mapping and for table numbering. A chunk
/// whose index is absent was dropped by the `max_chars` budget and never reached the
/// model at all — something the caller can usefully surface.
pub fn context_plan(chunks: &[RetrievedChunk], max_chars: usize) -> Vec<usize> {
    let mut plan = Vec::new();
    let mut used = 0usize;
    for (slot, idx) in context_order(chunks.len()).into_iter().enumerate() {
        let entry_len = format!("[{}] {}\n", slot + 1, context_text(&chunks[idx])).len();
        if used + entry_len > max_chars {
            break;
        }
        plan.push(idx);
        used += entry_len;
    }
    plan
}

#[cfg(test)]
mod context_plan_tests {
    use super::*;
    use crate::meta_store::ChunkMeta;

    fn chunk(text: &str, surrounding: &str) -> RetrievedChunk {
        RetrievedChunk {
            chunk_id: None,
            chunk_meta: ChunkMeta {
                doc_name: "d".into(),
                chunk_index: 0,
                text: text.into(),
                surrounding: surrounding.into(),
                page_num: None,
                ingested_at: String::new(),
                section_name: None,
                skip_extraction: false,
                section_note: None,
                section_type: crate::doc_schema::SectionType::Main,
            },
            score: 0.0,
            source_kb: None,
            rerank_score: None,
        }
    }

    /// Regression: the number the prompt advertises must equal the number of excerpts
    /// actually rendered.
    ///
    /// The prompt used `chunks.len()` while `check_citations` bounded citations by
    /// `context_plan(..).len()`. Whenever the budget dropped a chunk, the model was
    /// told it could cite up to [n] and then warned that a citation in that range was
    /// a fabricated source.
    #[test]
    fn prompt_numbering_matches_the_plan_when_the_budget_bites() {
        let big = "x".repeat(400);
        let chunks: Vec<RetrievedChunk> = (0..10).map(|_| chunk(&big, "")).collect();

        // A budget that admits only a few of them.
        let budget = 1000;
        let plan = context_plan(&chunks, budget);
        assert!(
            plan.len() < chunks.len(),
            "test needs the budget to actually drop chunks"
        );

        let msgs = build_chat_messages("q", &chunks, &[], budget, None);
        let system = &msgs[0].content;
        let n = plan.len();
        assert!(
            system.contains(&format!("The following {n} source excerpt(s)")),
            "prompt does not advertise the plan length {n}"
        );
        assert!(
            system.contains(&format!("Only cite numbers that exist: [1]–[{n}]")),
            "citation range does not match the plan length {n}"
        );
        // And the rendered block really does stop at n.
        let block = render_context_block(&chunks, &plan);
        assert!(block.contains(&format!("[{n}] ")));
        assert!(!block.contains(&format!("[{}] ", n + 1)));
    }

    /// The single-shot prompt builder had the same mismatch.
    #[test]
    fn rag_prompt_numbering_matches_the_plan() {
        let big = "y".repeat(400);
        let chunks: Vec<RetrievedChunk> = (0..10).map(|_| chunk(&big, "")).collect();
        let budget = 1000;
        let n = context_plan(&chunks, budget).len();
        assert!(n < chunks.len());
        let prompt = build_rag_prompt("q", &chunks, budget);
        assert!(
            prompt.contains(&format!("Use only the {n} source excerpt(s)")),
            "prompt advertises the wrong count"
        );
    }

    /// The permutation the model sees: evens ascending, then odds descending.
    #[test]
    fn order_is_evens_then_odds_reversed() {
        assert_eq!(context_order(0), Vec::<usize>::new());
        assert_eq!(context_order(1), vec![0]);
        assert_eq!(context_order(5), vec![0, 2, 4, 3, 1]);
        assert_eq!(context_order(6), vec![0, 2, 4, 5, 3, 1]);
    }

    /// Byte-for-byte reproduction of the pre-refactor `build_context_block`, which
    /// inlined the reorder and the budget cut. If this drifts, the model's context
    /// changed silently.
    #[test]
    fn block_matches_legacy_implementation() {
        fn legacy(chunks: &[RetrievedChunk], max_chars: usize) -> String {
            let (evens, odds): (Vec<_>, Vec<_>) =
                chunks.iter().enumerate().partition(|(i, _)| i % 2 == 0);
            let mut reordered: Vec<&RetrievedChunk> = evens.into_iter().map(|(_, c)| c).collect();
            reordered.extend(odds.into_iter().rev().map(|(_, c)| c));
            let mut out = String::new();
            let mut used = 0usize;
            for (i, chunk) in reordered.iter().enumerate() {
                let text = if chunk.chunk_meta.surrounding.len() > chunk.chunk_meta.text.len() {
                    &chunk.chunk_meta.surrounding
                } else {
                    &chunk.chunk_meta.text
                };
                let entry = format!("[{}] {}\n", i + 1, text);
                if used + entry.len() > max_chars {
                    break;
                }
                out.push_str(&entry);
                used += entry.len();
            }
            out
        }

        let chunks: Vec<RetrievedChunk> = (0..9)
            .map(|i| {
                // Alternate which of text/surrounding is longer, and vary length so the
                // budget cut lands mid-list.
                if i % 3 == 0 {
                    chunk(&format!("short {i}"), &"wide ".repeat(20 + i))
                } else {
                    chunk(&"body ".repeat(15 + i), "tiny")
                }
            })
            .collect();

        for max in [40usize, 120, 300, 555, 10_000] {
            assert_eq!(
                render_context_block(&chunks, &context_plan(&chunks, max)),
                legacy(&chunks, max),
                "context block drifted at max_chars={max}"
            );
        }
    }

    /// Chunks past the budget are absent from the plan — that absence is what the UI
    /// reports as "never reached the model".
    #[test]
    fn plan_drops_chunks_over_budget() {
        let chunks: Vec<RetrievedChunk> = (0..6).map(|_| chunk(&"x".repeat(50), "")).collect();
        let full = context_plan(&chunks, 10_000);
        assert_eq!(full.len(), 6);
        let cut = context_plan(&chunks, 120);
        assert!(cut.len() < 6 && !cut.is_empty(), "got {cut:?}");
        // Whatever survives is a prefix of the full presentation order.
        assert_eq!(cut[..], full[..cut.len()]);
    }
}
