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
    format!(
        "Use the following context to answer the question.\n\n\
         Context:\n{context}\n\n\
         Question: {user_query}\n\n\
         Answer:"
    )
}

/// Build a chat message list for `/v1/chat/completions`.
pub fn build_chat_messages(
    user_query: &str,
    chunks: &[RetrievedChunk],
    history: &[ChatMessage],
    max_context_chars: usize,
) -> Vec<ChatMessage> {
    let context = build_context_block(chunks, max_context_chars);

    let system = ChatMessage {
        role: "system".to_string(),
        content: format!(
            "You are a helpful assistant. Use the provided context to answer the user's question. \
             If the answer is not in the context, say so honestly.\n\n\
             Context:\n{context}"
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
    let mut out = String::new();
    let mut used = 0usize;
    for (i, chunk) in chunks.iter().enumerate() {
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
