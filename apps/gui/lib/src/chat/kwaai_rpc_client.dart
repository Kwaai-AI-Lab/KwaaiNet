import 'dart:async';
import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../daemon/paths.dart';

/// Thin wrapper around the daemon's gRPC `KwaaiNet` service. Owns a
/// single channel; reconnects lazily when the daemon comes up.
///
/// The generated gRPC bindings depend on the .proto landing in the
/// kwaai-rpc crate — until then this is a stub that echoes the prompt
/// one word at a time, so the UI can be developed against the same
/// `Stream<String>` contract.
class KwaaiRpcClient {
  KwaaiRpcClient();

  /// Open a bidi chat with [prompt] as the only client message.
  /// Returns a stream of token strings. When the stream completes, the
  /// daemon has finished generating.
  Stream<String> chatStream(String prompt) async* {
    // TODO(grpc): swap this stub for a real
    // `KwaaiNetClient(channel).chat(...)` once kwaai-rpc lands.
    final words = prompt.trim().split(RegExp(r'\s+'));
    yield 'Echo (stub): ';
    for (final w in words) {
      await Future<void>.delayed(const Duration(milliseconds: 60));
      yield '$w ';
    }
  }

  /// Best-effort socket path. POSIX uses a Unix socket co-located with
  /// the daemon's other run-state files; Windows / fallback will use
  /// loopback TCP. The actual port lands in config once the daemon
  /// publishes it.
  static String get unixSocketPath =>
      '${KwaainetPaths.runDir}${Platform.pathSeparator}kwaai.sock';

  void close() {
    // No-op while the client is a stub. Real impl will close the channel.
  }
}

final kwaaiRpcClientProvider = Provider<KwaaiRpcClient>((ref) {
  final client = KwaaiRpcClient();
  ref.onDispose(client.close);
  return client;
});
