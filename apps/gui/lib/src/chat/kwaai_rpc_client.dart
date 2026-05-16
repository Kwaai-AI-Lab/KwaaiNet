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

/// High-level connection state the GUI gates UI on. The grpc-dart
/// package's own ConnectionState has more states (idle vs ready vs
/// transientFailure); collapsing them down keeps callers from caring
/// about the difference between "haven't tried yet" and "trying now".
enum RpcConnection {
  /// We're attempting to open or re-open the channel.
  connecting,

  /// The channel reached `ready` and a probe call has succeeded.
  connected,

  /// No usable channel right now (daemon down, socket gone, refused).
  disconnected,
}

/// Thin wrapper around the daemon's gRPC `KwaaiNet` service. Maintains
/// a single channel + a state stream so the rest of the app can gate
/// UI on real connectivity (not just whether the daemon's PID is
/// alive — those can diverge during startup, restart, or when the
/// daemon's been killed but the listener socket is stale).
///
/// Transport selection: prefer the daemon's Unix socket when it exists
/// (no port collisions, FS-level ACL), fall back to loopback TCP.
class KwaaiRpcClient {
  KwaaiRpcClient() {
    // Start the keep-alive probe loop straight away so the connection
    // state stream has a value to publish before anyone subscribes.
    _startProbeLoop();
  }

  ClientChannel? _channel;
  pbgrpc.KwaaiNetClient? _stub;

  /// Human-readable transport descriptor for the currently-open channel,
  /// remembered so close/error logs can name what we just lost.
  String? _connectionPath;

  Timer? _probeTimer;
  RpcConnection _lastState = RpcConnection.connecting;
  bool _probingEnabled = true;

  final _stateController = StreamController<RpcConnection>.broadcast();

  /// Broadcasts the *current* high-level connection state. Late
  /// subscribers immediately receive the most recent value.
  Stream<RpcConnection> get connectionState async* {
    yield _lastState;
    yield* _stateController.stream;
  }

  // ---------------------------------------------------------------------
  // Probe loop — re-checks connectivity every few seconds even when no
  // chat is in flight, so the UI knows to re-enable as soon as the
  // daemon comes back up (and to disable when it goes away).
  // ---------------------------------------------------------------------

  static const _probeInterval = Duration(seconds: 3);

  void _startProbeLoop() {
    // Fire the first probe immediately so initial UI doesn't spend
    // _probeInterval seconds in "connecting".
    scheduleMicrotask(_probe);
    _probeTimer =
        Timer.periodic(_probeInterval, (_) => _probe());
  }

  /// Toggle the periodic Ping probe. Used by the GUI to suppress probes
  /// while the daemon is known-stopped (no point spamming connect-refused
  /// every 3 s when we already know there's no listener). Re-enable as
  /// soon as the daemon starts so the channel comes up promptly.
  void setProbingEnabled(bool enabled) {
    if (enabled == _probingEnabled) return;
    _probingEnabled = enabled;
    if (enabled) {
      // Fire one probe immediately so the UI doesn't wait for the next
      // periodic tick to learn the daemon is reachable.
      scheduleMicrotask(_probe);
    } else {
      // Drop any cached channel so we don't sit on a closed socket; the
      // next enabled probe will open a fresh one.
      _publish(RpcConnection.disconnected);
      // ignore: discarded_futures
      _resetChannel(silent: true);
    }
  }

  Future<void> _probe() async {
    if (!_probingEnabled) return;
    // If we already have a stub and the underlying connection is
    // happy, skip — the state-changed subscription will tell us
    // about disconnects.
    if (_stub != null && _lastState == RpcConnection.connected) return;
    try {
      final stub = _client(); // (re)opens channel if needed
      await stub.ping(pb.PingRequest()).timeout(const Duration(seconds: 2));
      _publish(RpcConnection.connected);
    } catch (_) {
      _publish(RpcConnection.disconnected);
      // Drop the dead channel so the next probe tries a fresh
      // _openChannel — important when the daemon was restarted and
      // the unix socket inode changed.
      await _resetChannel(silent: true);
    }
  }

  void _publish(RpcConnection s) {
    if (s == _lastState) return;
    _lastState = s;
    _stateController.add(s);
    _log('state → ${s.name}');
  }

  // ---------------------------------------------------------------------
  // Channel management
  // ---------------------------------------------------------------------

  pbgrpc.KwaaiNetClient _client() {
    final existing = _stub;
    if (existing != null) return existing;

    final channel = _openChannel();
    _channel = channel;
    // Intentionally NOT subscribing to channel.onConnectionStateChanged
    // here: the channel transitions through connecting/transientFailure
    // on routine HTTP/2 housekeeping (idle timeouts, keep-alives) and
    // overriding our probe's "connected" verdict with those flickers
    // produces a connecting/disconnected loop in the UI even when
    // Pings are succeeding. The periodic Ping probe is the only
    // source of truth.
    final stub = pbgrpc.KwaaiNetClient(channel);
    _stub = stub;
    return stub;
  }

  ClientChannel _openChannel() {
    if (Platform.isMacOS || Platform.isLinux) {
      final sockPath = unixSocketPath;
      if (File(sockPath).existsSync()) {
        _connectionPath = 'unix://$sockPath';
        _log('opening Unix socket: $sockPath');
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
    _connectionPath = 'tcp://127.0.0.1:$kDefaultGrpcPort';
    _log('opening TCP: 127.0.0.1:$kDefaultGrpcPort');
    return ClientChannel(
      '127.0.0.1',
      port: kDefaultGrpcPort,
      options: const ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ),
    );
  }

  // ---------------------------------------------------------------------
  // Chat
  // ---------------------------------------------------------------------

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

  Future<void> _resetChannel({bool silent = false}) async {
    final ch = _channel;
    final path = _connectionPath;
    _channel = null;
    _stub = null;
    _connectionPath = null;
    if (ch != null) {
      if (!silent) _log('connection closed (${path ?? "unknown"})');
      try {
        await ch.shutdown();
      } catch (_) {}
    }
  }

  /// Best-effort Unix socket path matching the daemon's bind location
  /// (`kwaai_cli::grpc_server::unix_socket_path` on the Rust side).
  static String get unixSocketPath =>
      '${KwaainetPaths.runDir}${Platform.pathSeparator}kwaai.sock';

  Future<void> close() async {
    _probeTimer?.cancel();
    _probeTimer = null;
    await _resetChannel(silent: true);
    await _stateController.close();
  }
}

final kwaaiRpcClientProvider = Provider<KwaaiRpcClient>((ref) {
  final client = KwaaiRpcClient();
  ref.onDispose(client.close);
  return client;
});

/// Live connection state — drives the main page's enable/disable.
final kwaaiRpcConnectionProvider = StreamProvider<RpcConnection>((ref) {
  return ref.watch(kwaaiRpcClientProvider).connectionState;
});
