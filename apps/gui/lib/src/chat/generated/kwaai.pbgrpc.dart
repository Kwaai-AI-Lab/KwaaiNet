// This is a generated file - do not edit.
//
// Generated from kwaai.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:async' as $async;
import 'dart:core' as $core;

import 'package:grpc/service_api.dart' as $grpc;
import 'package:protobuf/protobuf.dart' as $pb;

import 'kwaai.pb.dart' as $0;

export 'kwaai.pb.dart';

@$pb.GrpcServiceName('kwaai.v1.KwaaiNet')
class KwaaiNetClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  KwaaiNetClient(super.channel, {super.options, super.interceptors});

  /// Multiplexed bidi session. The client opens Session once per
  /// connection and sends a stream of ClientFrames; the server emits a
  /// stream of ServerFrames in response. Each frame carries a client-
  /// assigned `id` correlating requests with their responses, so many
  /// long-running operations (generate, shard_run, status) can
  /// interleave on a single channel without needing one rpc per
  /// operation type.
  ///
  /// The first frame on a new id opens a logical operation. Subsequent
  /// frames with that id either continue input to it (future multi-turn)
  /// or cancel it (a `Cancel` body). The server emits zero or more reply
  /// frames for the id, terminating with a `Done` or `Error` body —
  /// both signal the operation is finished and the id may be reused.
  ///
  /// This is the canonical surface; the legacy Chat / Ping rpcs below
  /// remain for a deprecation cycle.
  $grpc.ResponseStream<$0.ServerFrame> session(
    $async.Stream<$0.ClientFrame> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$session, request, options: options);
  }

  /// -----------------------------------------------------------------
  /// Legacy rpcs (pre-Session). Kept until all callers migrate. Don't
  /// add new functionality here — use Session.
  /// -----------------------------------------------------------------
  $grpc.ResponseStream<$0.ChatToken> chat(
    $async.Stream<$0.ChatMessage> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$chat, request, options: options);
  }

  $grpc.ResponseFuture<$0.PingReply> ping(
    $0.PingRequest request, {
    $grpc.CallOptions? options,
  }) {
    return $createUnaryCall(_$ping, request, options: options);
  }

  // method descriptors

  static final _$session = $grpc.ClientMethod<$0.ClientFrame, $0.ServerFrame>(
      '/kwaai.v1.KwaaiNet/Session',
      ($0.ClientFrame value) => value.writeToBuffer(),
      $0.ServerFrame.fromBuffer);
  static final _$chat = $grpc.ClientMethod<$0.ChatMessage, $0.ChatToken>(
      '/kwaai.v1.KwaaiNet/Chat',
      ($0.ChatMessage value) => value.writeToBuffer(),
      $0.ChatToken.fromBuffer);
  static final _$ping = $grpc.ClientMethod<$0.PingRequest, $0.PingReply>(
      '/kwaai.v1.KwaaiNet/Ping',
      ($0.PingRequest value) => value.writeToBuffer(),
      $0.PingReply.fromBuffer);
}

@$pb.GrpcServiceName('kwaai.v1.KwaaiNet')
abstract class KwaaiNetServiceBase extends $grpc.Service {
  $core.String get $name => 'kwaai.v1.KwaaiNet';

  KwaaiNetServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ClientFrame, $0.ServerFrame>(
        'Session',
        session,
        true,
        true,
        ($core.List<$core.int> value) => $0.ClientFrame.fromBuffer(value),
        ($0.ServerFrame value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.ChatMessage, $0.ChatToken>(
        'Chat',
        chat,
        true,
        true,
        ($core.List<$core.int> value) => $0.ChatMessage.fromBuffer(value),
        ($0.ChatToken value) => value.writeToBuffer()));
    $addMethod($grpc.ServiceMethod<$0.PingRequest, $0.PingReply>(
        'Ping',
        ping_Pre,
        false,
        false,
        ($core.List<$core.int> value) => $0.PingRequest.fromBuffer(value),
        ($0.PingReply value) => value.writeToBuffer()));
  }

  $async.Stream<$0.ServerFrame> session(
      $grpc.ServiceCall call, $async.Stream<$0.ClientFrame> request);

  $async.Stream<$0.ChatToken> chat(
      $grpc.ServiceCall call, $async.Stream<$0.ChatMessage> request);

  $async.Future<$0.PingReply> ping_Pre(
      $grpc.ServiceCall $call, $async.Future<$0.PingRequest> $request) async {
    return ping($call, await $request);
  }

  $async.Future<$0.PingReply> ping(
      $grpc.ServiceCall call, $0.PingRequest request);
}
