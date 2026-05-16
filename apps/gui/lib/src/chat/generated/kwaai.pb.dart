// This is a generated file - do not edit.
//
// Generated from kwaai.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

/// Liveness probe input. No fields today; reserved range 1-15 left for
/// future probe metadata (client version, capability flags) without
/// breaking older callers.
class PingRequest extends $pb.GeneratedMessage {
  factory PingRequest() => create();

  PingRequest._();

  factory PingRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PingRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PingRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PingRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PingRequest copyWith(void Function(PingRequest) updates) =>
      super.copyWith((message) => updates(message as PingRequest))
          as PingRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PingRequest create() => PingRequest._();
  @$core.override
  PingRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PingRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<PingRequest>(create);
  static PingRequest? _defaultInstance;
}

/// Liveness probe response. server_time confirms a non-empty round-trip
/// payload and lets the client surface clock skew if it ever cares.
class PingReply extends $pb.GeneratedMessage {
  factory PingReply({
    $core.String? serverTime,
  }) {
    final result = create();
    if (serverTime != null) result.serverTime = serverTime;
    return result;
  }

  PingReply._();

  factory PingReply.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory PingReply.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'PingReply',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'serverTime')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PingReply clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  PingReply copyWith(void Function(PingReply) updates) =>
      super.copyWith((message) => updates(message as PingReply)) as PingReply;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static PingReply create() => PingReply._();
  @$core.override
  PingReply createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static PingReply getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<PingReply>(create);
  static PingReply? _defaultInstance;

  /// Wall-clock time on the daemon when the ping was handled, formatted
  /// as RFC 3339.
  @$pb.TagNumber(1)
  $core.String get serverTime => $_getSZ(0);
  @$pb.TagNumber(1)
  set serverTime($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerTime() => $_clearField(1);
}

/// A single message in a chat conversation. Modelled loosely after the
/// OpenAI chat API so existing client code maps over cleanly.
class ChatMessage extends $pb.GeneratedMessage {
  factory ChatMessage({
    $core.String? content,
    $core.String? role,
    $core.String? conversationId,
  }) {
    final result = create();
    if (content != null) result.content = content;
    if (role != null) result.role = role;
    if (conversationId != null) result.conversationId = conversationId;
    return result;
  }

  ChatMessage._();

  factory ChatMessage.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatMessage.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatMessage',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'content')
    ..aOS(2, _omitFieldNames ? '' : 'role')
    ..aOS(3, _omitFieldNames ? '' : 'conversationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatMessage clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatMessage copyWith(void Function(ChatMessage) updates) =>
      super.copyWith((message) => updates(message as ChatMessage))
          as ChatMessage;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatMessage create() => ChatMessage._();
  @$core.override
  ChatMessage createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatMessage getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ChatMessage>(create);
  static ChatMessage? _defaultInstance;

  /// The text payload of the message.
  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  /// Conventional roles: "user", "assistant", "system". Free-form so we
  /// can add roles later without a proto change.
  @$pb.TagNumber(2)
  $core.String get role => $_getSZ(1);
  @$pb.TagNumber(2)
  set role($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => $_clearField(2);

  /// Optional client-supplied conversation ID. Lets the daemon associate
  /// multiple Chat streams with the same logical conversation when we
  /// add multi-turn support. Ignored today.
  @$pb.TagNumber(3)
  $core.String get conversationId => $_getSZ(2);
  @$pb.TagNumber(3)
  set conversationId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasConversationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearConversationId() => $_clearField(3);
}

/// One streamed output chunk. The server SHOULD send each generated token
/// (or token-piece) as its own ChatToken; the client is responsible for
/// concatenating `text`.
class ChatToken extends $pb.GeneratedMessage {
  factory ChatToken({
    $core.String? text,
    $core.bool? done,
    $core.String? finishReason,
  }) {
    final result = create();
    if (text != null) result.text = text;
    if (done != null) result.done = done;
    if (finishReason != null) result.finishReason = finishReason;
    return result;
  }

  ChatToken._();

  factory ChatToken.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ChatToken.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ChatToken',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'text')
    ..aOB(2, _omitFieldNames ? '' : 'done')
    ..aOS(3, _omitFieldNames ? '' : 'finishReason')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatToken clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ChatToken copyWith(void Function(ChatToken) updates) =>
      super.copyWith((message) => updates(message as ChatToken)) as ChatToken;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ChatToken create() => ChatToken._();
  @$core.override
  ChatToken createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ChatToken getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<ChatToken>(create);
  static ChatToken? _defaultInstance;

  /// The next text piece to append. Empty when done=true.
  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);

  /// True on the final message in the stream. After done=true the server
  /// closes the response stream.
  @$pb.TagNumber(2)
  $core.bool get done => $_getBF(1);
  @$pb.TagNumber(2)
  set done($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDone() => $_has(1);
  @$pb.TagNumber(2)
  void clearDone() => $_clearField(2);

  /// Optional reason the stream ended. Conventional values: "stop"
  /// (EOS / natural completion), "length" (hit max_tokens), "error",
  /// "cancelled". Only set on the final (done=true) message.
  @$pb.TagNumber(3)
  $core.String get finishReason => $_getSZ(2);
  @$pb.TagNumber(3)
  set finishReason($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasFinishReason() => $_has(2);
  @$pb.TagNumber(3)
  void clearFinishReason() => $_clearField(3);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
