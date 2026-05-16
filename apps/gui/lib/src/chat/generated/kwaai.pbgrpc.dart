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

/// KwaaiNet exposes the high-level node operations a client might want to
/// drive. For now it's just chat; expect node status / DHT queries / model
/// management to land here later, each as their own rpc method.
@$pb.GrpcServiceName('kwaai.v1.KwaaiNet')
class KwaaiNetClient extends $grpc.Client {
  /// The hostname for this service.
  static const $core.String defaultHost = '';

  /// OAuth scopes needed for the client.
  static const $core.List<$core.String> oauthScopes = [
    '',
  ];

  KwaaiNetClient(super.channel, {super.options, super.interceptors});

  /// Bidirectional streaming chat.
  ///
  /// The client opens a Chat stream and sends one or more ChatMessages.
  /// Today the server treats the FIRST ChatMessage as the prompt and
  /// ignores subsequent messages (single-turn). Multi-turn is a TODO —
  /// we picked bidi from the start so we don't have to break the proto
  /// when we add it (the client just keeps sending; the server learns to
  /// accumulate).
  ///
  /// The server responds with a stream of ChatTokens, one per generated
  /// text piece, ending with a single ChatToken that has done=true.
  $grpc.ResponseStream<$0.ChatToken> chat(
    $async.Stream<$0.ChatMessage> request, {
    $grpc.CallOptions? options,
  }) {
    return $createStreamingCall(_$chat, request, options: options);
  }

  // method descriptors

  static final _$chat = $grpc.ClientMethod<$0.ChatMessage, $0.ChatToken>(
      '/kwaai.v1.KwaaiNet/Chat',
      ($0.ChatMessage value) => value.writeToBuffer(),
      $0.ChatToken.fromBuffer);
}

@$pb.GrpcServiceName('kwaai.v1.KwaaiNet')
abstract class KwaaiNetServiceBase extends $grpc.Service {
  $core.String get $name => 'kwaai.v1.KwaaiNet';

  KwaaiNetServiceBase() {
    $addMethod($grpc.ServiceMethod<$0.ChatMessage, $0.ChatToken>(
        'Chat',
        chat,
        true,
        true,
        ($core.List<$core.int> value) => $0.ChatMessage.fromBuffer(value),
        ($0.ChatToken value) => value.writeToBuffer()));
  }

  $async.Stream<$0.ChatToken> chat(
      $grpc.ServiceCall call, $async.Stream<$0.ChatMessage> request);
}
