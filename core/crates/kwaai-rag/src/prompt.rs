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
    let context = build_context_block(chunks, max_context_chars);
    let n = chunks.len();
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
    let context = build_context_block(chunks, max_context_chars);

    let doc_preamble = match doc_context {
        Some(dc) if !dc.is_empty() => format!(
            "Document being queried: {dc}\n\
             Use this document context to resolve references like \"the author\", \
             \"the book\", or \"this memoir\" in the question.\n\n"
        ),
        _ => String::new(),
    };

    let n = chunks.len();
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
                require comprehensive answers — include ALL specific facts found across ALL sources: \
                dates, places, ethnic or geographic origins, ancestry, family connections, \
                occupations, achievements, and historical associations. Examine every numbered \
                excerpt for relevant details. A single-sentence answer is never sufficient for \
                a biographical question; list each fact explicitly.\n\
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

fn build_context_block(chunks: &[RetrievedChunk], max_chars: usize) -> String {
    // Reorder to mitigate lost-in-the-middle: put even ranks at start, odd ranks
    // reversed at end. Best evidence lands at positions 1 and last where LLMs attend most.
    let reordered = reorder_for_context(chunks);
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

fn reorder_for_context(chunks: &[RetrievedChunk]) -> Vec<&RetrievedChunk> {
    let (evens, odds): (Vec<_>, Vec<_>) = chunks.iter().enumerate().partition(|(i, _)| i % 2 == 0);
    let mut result: Vec<&RetrievedChunk> = evens.into_iter().map(|(_, c)| c).collect();
    result.extend(odds.into_iter().rev().map(|(_, c)| c));
    result
}
