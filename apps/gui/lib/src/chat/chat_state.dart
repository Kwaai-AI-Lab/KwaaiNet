import 'dart:async';
import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'chat_message.dart';
import 'kwaai_rpc_client.dart';
import 'session_client.dart';

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

/// Which gRPC method drives a given transcript. Each path keeps its
/// own message history + in-flight subscription, so the main chat
/// (shard_run) and the Developer tab (generate) don't share state.
enum ChatPath {
  /// `kwaainet shard run` — distributed inference across the mesh.
  shardRun,

  /// `kwaainet generate` — single-node local inference.
  generateLocal,
}

/// Append-only transcript of messages for one [ChatPath]. Tokens
/// streamed from the daemon mutate the last (assistant) message in
/// place — `_bump()` triggers UI rebuilds without copying the list
/// per token.
class ChatTranscriptNotifier extends FamilyNotifier<List<ChatMessage>, ChatPath> {
  StreamSubscription<String>? _sub;
  ChatPath get _path => arg;

  @override
  List<ChatMessage> build(ChatPath arg) {
    ref.onDispose(() => _sub?.cancel());
    return [];
  }

  /// Send [prompt] and stream the response into a new assistant message.
  /// Returns when the stream completes.
  Future<void> send(String prompt) async {
    if (prompt.trim().isEmpty) return;
    if (_sub != null) return; // ignore overlapping sends
    _log('[${_path.name}] > $prompt');
    final user = ChatMessage(role: 'user', text: prompt);
    final assistant = ChatMessage(role: 'assistant', text: '', streaming: true);
    state = [...state, user, assistant];
    final client = ref.read(kwaaiRpcClientProvider);
    final stream = switch (_path) {
      ChatPath.shardRun => client.chatStream(prompt),
      ChatPath.generateLocal => client.generateLocal(prompt),
    };
    final completer = Completer<void>();
    _sub = stream.listen(
      (token) {
        assistant.text += token;
        _bump();
      },
      onError: (e, _) {
        _log('[${_path.name}] < [error] $e');
        // Preserve the structured (code, message) when it's a
        // SessionOpError; fall back to a 0/UNKNOWN with the toString
        // for any other thrown type (transport hiccups, asserts, etc).
        assistant.error = e is SessionOpError
            ? ChatError(code: e.code, message: e.message)
            : ChatError(code: 0, message: e.toString());
        assistant.streaming = false;
        _bump();
        _sub = null;
        if (!completer.isCompleted) completer.complete();
      },
      onDone: () {
        _log('[${_path.name}] < ${assistant.text}');
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

final chatTranscriptProvider = NotifierProvider.family<
    ChatTranscriptNotifier, List<ChatMessage>, ChatPath>(
  ChatTranscriptNotifier.new,
);

/// True when there's an in-flight assistant stream on the given path.
final chatStreamingProvider = Provider.family<bool, ChatPath>((ref, path) {
  final msgs = ref.watch(chatTranscriptProvider(path));
  return msgs.isNotEmpty && msgs.last.streaming;
});
