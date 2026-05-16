/// A single message in the chat transcript. `assistant` messages are
/// written to incrementally as tokens stream in from the daemon — the
/// UI rebuilds when [text] grows.
class ChatMessage {
  ChatMessage({
    required this.role,
    required this.text,
    this.streaming = false,
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
}
