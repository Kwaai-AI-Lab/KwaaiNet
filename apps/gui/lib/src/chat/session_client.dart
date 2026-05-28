import 'dart:async';
import 'dart:io';

import 'package:fixnum/fixnum.dart';

import 'generated/kwaai.pb.dart' as pb;
import 'generated/kwaai.pbgrpc.dart' as pbgrpc;

void _log(String msg) {
  stderr.writeln('[session] $msg');
}

/// Why a Session ended. The client treats all of these as "session
/// gone; reconnect on next operation".
enum SessionEndKind { localClose, remoteClose, transportError }

/// Multiplexed bidi session to the daemon. Owns a single open
/// `Session(...)` rpc and routes inbound `ServerFrame`s back to whichever
/// caller is awaiting them (by id). One [SessionClient] per gRPC
/// channel.
///
/// Operation entry points (`ping`, `status`, `generate`, `shardRun`,
/// `cancel`) allocate an id, send the corresponding ClientFrame, and
/// return a Future/Stream that completes/errors based on the per-id
/// ServerFrames the server emits. The stream completes on Done; it
/// errors on Error (or on session-end).
class SessionClient {
  SessionClient(this._stub);

  final pbgrpc.KwaaiNetClient _stub;

  // Outbound side of the bidi stream — a controller we write ClientFrames
  // into, which gets piped into the rpc as the request stream.
  StreamController<pb.ClientFrame>? _outbound;

  // Subscription to inbound ServerFrames; demuxes by id into per-op
  // controllers.
  StreamSubscription<pb.ServerFrame>? _inboundSub;

  // Per-operation routers. Each operation id maps to a controller its
  // caller listens on. Removed on Done/Error/session-end.
  final Map<int, StreamController<pb.ServerFrame>> _routers = {};

  // Monotonic id allocator. Starts at 1; 0 is reserved so missing-tag
  // values are obvious.
  int _nextId = 1;

  bool _closed = false;

  /// Opens the bidi stream if not already open. Idempotent.
  void ensureOpen() {
    if (_outbound != null || _closed) return;
    final out = StreamController<pb.ClientFrame>();
    _outbound = out;
    final inbound = _stub.session(out.stream);
    _log('opened Session stream');
    _inboundSub = inbound.listen(
      _handleFrame,
      onError: (Object e, StackTrace? _) {
        _log('Session inbound error: $e');
        _teardown(SessionEndKind.transportError, e.toString());
      },
      onDone: () {
        _log('Session inbound closed by server');
        _teardown(SessionEndKind.remoteClose, 'server closed Session');
      },
      cancelOnError: true,
    );
  }

  void _handleFrame(pb.ServerFrame frame) {
    final id = frame.id.toInt();
    final router = _routers[id];
    if (router == null) {
      _log('drop frame for unknown id=$id (body=${frame.whichBody()})');
      return;
    }
    router.add(frame);
    switch (frame.whichBody()) {
      case pb.ServerFrame_Body.done:
        router.close();
        _routers.remove(id);
      case pb.ServerFrame_Body.error:
        router.addError(SessionOpError(
          code: frame.error.code.value,
          message: frame.error.message,
        ));
        router.close();
        _routers.remove(id);
      default:
        break;
    }
  }

  void _teardown(SessionEndKind kind, String reason) {
    if (_closed) return;
    _closed = true;
    final routers = _routers.values.toList(growable: false);
    _routers.clear();
    for (final r in routers) {
      if (!r.isClosed) {
        r.addError(SessionEndedError(kind: kind, reason: reason));
        r.close();
      }
    }
    _outbound?.close();
    _outbound = null;
    _inboundSub?.cancel();
    _inboundSub = null;
  }

  /// Manual close — used on channel reset or app shutdown.
  Future<void> close() async =>
      _teardown(SessionEndKind.localClose, 'client close');

  // -------------------------------------------------------------------
  // Per-operation entry points
  // -------------------------------------------------------------------

  /// `ping` — cheap liveness probe; server emits Pong then Done.
  Future<pb.PingReply> ping() async {
    final frames = _open((id) => pb.ClientFrame()
      ..id = Int64(id)
      ..ping = pb.PingRequest());
    pb.PingReply? reply;
    await for (final f in frames) {
      if (f.whichBody() == pb.ServerFrame_Body.pong) {
        reply = f.pong;
      }
    }
    if (reply == null) {
      throw SessionOpError(code: 0, message: 'ping returned no Pong');
    }
    return reply;
  }

  /// `status` — daemon-side state snapshot.
  Future<pb.StatusReply> status() async {
    final frames = _open((id) => pb.ClientFrame()
      ..id = Int64(id)
      ..status = pb.StatusRequest());
    pb.StatusReply? reply;
    await for (final f in frames) {
      if (f.whichBody() == pb.ServerFrame_Body.status) {
        reply = f.status;
      }
    }
    if (reply == null) {
      throw SessionOpError(code: 0, message: 'status returned no reply');
    }
    return reply;
  }

  /// `kwaainet shard run <PROMPT>` — distributed inference. Default
  /// path used by the GUI's main chat.
  Stream<String> shardRun(String prompt, {String role = 'user'}) {
    return _tokensFromFrames(_open((id) => pb.ClientFrame()
      ..id = Int64(id)
      ..shardRun = (pb.ShardRunRequest()
        ..role = role
        ..content = prompt)));
  }

  /// `kwaainet generate <PROMPT>` — single-node local inference. Used
  /// by the Developer tab to drive the local InferenceEngine directly.
  Stream<String> generate(String prompt, {String role = 'user'}) {
    return _tokensFromFrames(_open((id) => pb.ClientFrame()
      ..id = Int64(id)
      ..generate = (pb.GenerateRequest()
        ..role = role
        ..content = prompt)));
  }

  /// Cancel an in-flight operation. The target operation's stream will
  /// error with SessionOpError(code=CANCELLED).
  Future<void> cancel(int operationId) async {
    final frames = _open((id) => pb.ClientFrame()
      ..id = Int64(id)
      ..cancel = (pb.Cancel()..targetId = Int64(operationId)));
    await frames.drain<void>();
  }

  // -------------------------------------------------------------------
  // Internals
  // -------------------------------------------------------------------

  Stream<String> _tokensFromFrames(Stream<pb.ServerFrame> frames) async* {
    await for (final f in frames) {
      if (f.whichBody() == pb.ServerFrame_Body.token) {
        final t = f.token;
        if (t.text.isNotEmpty) yield t.text;
      }
    }
  }

  /// Allocate an id, build the ClientFrame, send it, register a router,
  /// return the per-id stream.
  Stream<pb.ServerFrame> _open(pb.ClientFrame Function(int id) build) {
    ensureOpen();
    if (_closed || _outbound == null) {
      return Stream<pb.ServerFrame>.error(
        SessionEndedError(
          kind: SessionEndKind.localClose,
          reason: 'session not open',
        ),
      );
    }
    final id = _nextId++;
    final controller = StreamController<pb.ServerFrame>();
    _routers[id] = controller;
    _outbound!.add(build(id));
    return controller.stream;
  }
}

/// Thrown into a per-op stream when the server emits Error for that id.
class SessionOpError implements Exception {
  SessionOpError({required this.code, required this.message});
  final int code;
  final String message;
  @override
  String toString() => 'SessionOpError(code=$code, message=$message)';
}

/// Thrown into all in-flight per-op streams when the Session itself
/// ends (channel went away, server hung up, client closed).
class SessionEndedError implements Exception {
  SessionEndedError({required this.kind, required this.reason});
  final SessionEndKind kind;
  final String reason;
  @override
  String toString() => 'SessionEndedError(${kind.name}: $reason)';
}
