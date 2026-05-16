import 'dart:async';
import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:grpc/grpc.dart';

import '../daemon/paths.dart';
import 'generated/kwaai.pb.dart' as pb;
import 'generated/kwaai.pbgrpc.dart' as pbgrpc;

void _log(String msg) {
  stderr.writeln('[rpc] $msg');
}

/// Default TCP port the daemon binds — mirrors
/// `kwaai_cli::grpc_server::DEFAULT_GRPC_TCP_PORT` on the Rust side. If we
/// expose this in config.yaml later, wire it through here too.
const int kDefaultGrpcPort = 8093;

/// Thin wrapper around the daemon's gRPC `KwaaiNet` service. Lazily
/// opens a channel on first call; reconnects on the next call after
/// any channel-level failure.
///
/// Transport selection: prefer the daemon's Unix socket when it exists
/// (no port collisions, FS-level ACL), fall back to loopback TCP.
class KwaaiRpcClient {
  KwaaiRpcClient();

  ClientChannel? _channel;
  pbgrpc.KwaaiNetClient? _stub;

  pbgrpc.KwaaiNetClient _client() {
    final existing = _stub;
    if (existing != null) return existing;

    final channel = _openChannel();
    _channel = channel;
    final stub = pbgrpc.KwaaiNetClient(channel);
    _stub = stub;
    return stub;
  }

  ClientChannel _openChannel() {
    if (Platform.isMacOS || Platform.isLinux) {
      final sockPath = unixSocketPath;
      if (File(sockPath).existsSync()) {
        _log('connecting via Unix socket: $sockPath');
        return ClientChannel(
          InternetAddress(sockPath, type: InternetAddressType.unix),
          port: 0,
          options: const ChannelOptions(
            credentials: ChannelCredentials.insecure(),
          ),
        );
      }
      _log('Unix socket not found, falling back to TCP');
    }
    _log('connecting via TCP: 127.0.0.1:$kDefaultGrpcPort');
    return ClientChannel(
      '127.0.0.1',
      port: kDefaultGrpcPort,
      options: const ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ),
    );
  }

  /// Open a bidi Chat with [prompt] as the only client message. Yields
  /// every non-empty token text from the daemon as it arrives. Completes
  /// when the daemon sends `done=true` (or the stream errors).
  Stream<String> chatStream(String prompt) async* {
    final stub = _client();
    final request = pb.ChatMessage()
      ..content = prompt
      ..role = 'user';
    // The server currently treats the FIRST inbound message as the
    // prompt and ignores the rest (see kwaai-cli/src/grpc_server.rs).
    // Wrapping a single message in a one-shot stream matches that
    // contract while staying compatible with the bidi rpc shape — we
    // can add multi-turn later without changing the proto.
    final outbound = Stream<pb.ChatMessage>.value(request);
    try {
      await for (final token in stub.chat(outbound)) {
        if (token.text.isNotEmpty) yield token.text;
        if (token.done) break;
      }
    } catch (e) {
      // Treat a channel error as a signal the daemon went away; drop
      // the cached stub so the next call reconnects from scratch.
      await _resetChannel();
      rethrow;
    }
  }

  Future<void> _resetChannel() async {
    final ch = _channel;
    _channel = null;
    _stub = null;
    if (ch != null) {
      try {
        await ch.shutdown();
      } catch (_) {}
    }
  }

  /// Best-effort Unix socket path matching the daemon's bind location
  /// (`kwaai_cli::grpc_server::unix_socket_path` on the Rust side).
  static String get unixSocketPath =>
      '${KwaainetPaths.runDir}${Platform.pathSeparator}kwaai.sock';

  Future<void> close() => _resetChannel();
}

final kwaaiRpcClientProvider = Provider<KwaaiRpcClient>((ref) {
  final client = KwaaiRpcClient();
  ref.onDispose(client.close);
  return client;
});
