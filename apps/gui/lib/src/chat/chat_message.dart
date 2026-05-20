/// A single message in the chat transcript. `assistant` messages are
/// written to incrementally as tokens stream in from the daemon — the
/// UI rebuilds when [text] grows.
class ChatMessage {
  ChatMessage({
    required this.role,
    required this.text,
    this.streaming = false,
    this.error,
  });

  /// 'user' or 'assistant'. Free-form string so the future server-side
  /// can introduce 'system'/'tool' without a client change.
  final String role;

  /// Mutable so the UI can append streamed tokens in place without
  /// rebuilding the whole list. Pair with bumping a notifier when
  /// growth happens.
  String text;

  /// True while tokens are still arriving. Cleared once the stream
  /// completes (success, error, or cancel).
  bool streaming;

  /// Structured error if the stream failed. The UI renders a friendly
  /// headline from [ChatError.code] with [ChatError.message] available
  /// as expandable details.
  ChatError? error;
}

/// Structured failure pulled off a SessionOpError so the UI can render
/// a friendly message from the code without grepping the underlying
/// daemon string. `code` is `Error_Code.value` from the proto enum.
class ChatError {
  ChatError({required this.code, required this.message});
  final int code;
  final String message;
}
