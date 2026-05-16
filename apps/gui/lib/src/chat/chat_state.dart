import 'dart:async';
import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'chat_message.dart';
import 'kwaai_rpc_client.dart';

void _log(String msg) {
  stderr.writeln('[chat] ${_elide(msg)}');
}

/// Keep log lines readable: long chat bodies get the middle elided so
/// you see the leading prompt and the tail of the response without
/// burying the console. Threshold sized for one-screen visibility.
String _elide(String s, {int maxLen = 240, int headTail = 110}) {
  final flat = s.replaceAll('\n', ' ');
  if (flat.length <= maxLen) return flat;
  final head = flat.substring(0, headTail);
  final tail = flat.substring(flat.length - headTail);
  return '$head … [${flat.length} chars] … $tail';
}

/// Append-only transcript of messages in the current chat session.
/// Tokens streamed from the daemon mutate the last (assistant) message
/// in place — `bump()` triggers UI rebuilds without rebuilding the
/// whole list.
class ChatTranscriptNotifier extends Notifier<List<ChatMessage>> {
  StreamSubscription<String>? _sub;

  @override
  List<ChatMessage> build() {
    ref.onDispose(() => _sub?.cancel());
    return [];
  }

  /// Send [prompt] and stream the response into a new assistant message.
  /// Returns when the stream completes.
  Future<void> send(String prompt) async {
    if (prompt.trim().isEmpty) return;
    if (_sub != null) return; // ignore overlapping sends
    _log('> $prompt');
    final user = ChatMessage(role: 'user', text: prompt);
    final assistant = ChatMessage(role: 'assistant', text: '', streaming: true);
    state = [...state, user, assistant];
    final client = ref.read(kwaaiRpcClientProvider);
    final completer = Completer<void>();
    _sub = client.chatStream(prompt).listen(
      (token) {
        assistant.text += token;
        _bump();
      },
      onError: (e, _) {
        _log('< [error] $e');
        assistant.error = e.toString();
        assistant.streaming = false;
        _bump();
        _sub = null;
        if (!completer.isCompleted) completer.complete();
      },
      onDone: () {
        _log('< ${assistant.text}');
        assistant.streaming = false;
        _bump();
        _sub = null;
        if (!completer.isCompleted) completer.complete();
      },
      cancelOnError: true,
    );
    return completer.future;
  }

  /// Cancel the in-flight stream (if any).
  void cancel() {
    _sub?.cancel();
    _sub = null;
    if (state.isNotEmpty && state.last.streaming) {
      state.last.streaming = false;
      _bump();
    }
  }

  /// Trigger a rebuild without changing the list reference — copying
  /// the list is cheap and only happens on each token tick.
  void _bump() => state = List.of(state);
}

final chatTranscriptProvider =
    NotifierProvider<ChatTranscriptNotifier, List<ChatMessage>>(
      ChatTranscriptNotifier.new,
    );

/// True when there's an in-flight assistant stream.
final chatStreamingProvider = Provider<bool>((ref) {
  final msgs = ref.watch(chatTranscriptProvider);
  return msgs.isNotEmpty && msgs.last.streaming;
});
