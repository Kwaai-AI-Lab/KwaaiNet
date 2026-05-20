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

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

import 'kwaai.pbenum.dart';

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

export 'kwaai.pbenum.dart';

enum ClientFrame_Body { ping, generate, shardRun, status, cancel, notSet }

/// Frame sent from client → server on the Session stream. The `body`
/// oneof selects which operation type this frame drives. Operation
/// names match the CLI subcommand they correspond to, dot-to-camelcase:
///
///   ping              ←  (no CLI equivalent; gRPC-only liveness probe)
///   generate          ←  `kwaainet generate <PROMPT>`
///   shardRun          ←  `kwaainet shard run <PROMPT>`
///   status            ←  `kwaainet status`
///   cancel            ←  (no CLI equivalent; aborts an in-flight op)
///
/// New operations are added as siblings here; preserving the flat shape
/// keeps the dispatch trivial on both sides.
class ClientFrame extends $pb.GeneratedMessage {
  factory ClientFrame({
    $fixnum.Int64? id,
    PingRequest? ping,
    GenerateRequest? generate,
    ShardRunRequest? shardRun,
    StatusRequest? status,
    Cancel? cancel,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (ping != null) result.ping = ping;
    if (generate != null) result.generate = generate;
    if (shardRun != null) result.shardRun = shardRun;
    if (status != null) result.status = status;
    if (cancel != null) result.cancel = cancel;
    return result;
  }

  ClientFrame._();

  factory ClientFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ClientFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ClientFrame_Body> _ClientFrame_BodyByTag = {
    10: ClientFrame_Body.ping,
    11: ClientFrame_Body.generate,
    12: ClientFrame_Body.shardRun,
    13: ClientFrame_Body.status,
    14: ClientFrame_Body.cancel,
    0: ClientFrame_Body.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ClientFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..oo(0, [10, 11, 12, 13, 14])
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<PingRequest>(10, _omitFieldNames ? '' : 'ping',
        subBuilder: PingRequest.create)
    ..aOM<GenerateRequest>(11, _omitFieldNames ? '' : 'generate',
        subBuilder: GenerateRequest.create)
    ..aOM<ShardRunRequest>(12, _omitFieldNames ? '' : 'shardRun',
        subBuilder: ShardRunRequest.create)
    ..aOM<StatusRequest>(13, _omitFieldNames ? '' : 'status',
        subBuilder: StatusRequest.create)
    ..aOM<Cancel>(14, _omitFieldNames ? '' : 'cancel',
        subBuilder: Cancel.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ClientFrame copyWith(void Function(ClientFrame) updates) =>
      super.copyWith((message) => updates(message as ClientFrame))
          as ClientFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ClientFrame create() => ClientFrame._();
  @$core.override
  ClientFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ClientFrame getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ClientFrame>(create);
  static ClientFrame? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  ClientFrame_Body whichBody() => _ClientFrame_BodyByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  void clearBody() => $_clearField($_whichOneof(0));

  /// Operation correlation id. Pick any non-zero value unused by an
  /// active operation on this Session. Reuse after Done/Error.
  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(10)
  PingRequest get ping => $_getN(1);
  @$pb.TagNumber(10)
  set ping(PingRequest value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasPing() => $_has(1);
  @$pb.TagNumber(10)
  void clearPing() => $_clearField(10);
  @$pb.TagNumber(10)
  PingRequest ensurePing() => $_ensure(1);

  /// Single-node inference using the local InferenceEngine. The
  /// node must have the model loaded locally — most useful for
  /// dev / fallback when the shard mesh isn't ready.
  @$pb.TagNumber(11)
  GenerateRequest get generate => $_getN(2);
  @$pb.TagNumber(11)
  set generate(GenerateRequest value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasGenerate() => $_has(2);
  @$pb.TagNumber(11)
  void clearGenerate() => $_clearField(11);
  @$pb.TagNumber(11)
  GenerateRequest ensureGenerate() => $_ensure(2);

  /// Distributed inference across the shard mesh. Routes blocks
  /// to peer nodes via the existing block_rpc layer. This is the
  /// default chat path for the GUI.
  @$pb.TagNumber(12)
  ShardRunRequest get shardRun => $_getN(3);
  @$pb.TagNumber(12)
  set shardRun(ShardRunRequest value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasShardRun() => $_has(3);
  @$pb.TagNumber(12)
  void clearShardRun() => $_clearField(12);
  @$pb.TagNumber(12)
  ShardRunRequest ensureShardRun() => $_ensure(3);

  /// Snapshot of node status (uptime, peers, model state). One
  /// ServerFrame.status reply, then Done.
  @$pb.TagNumber(13)
  StatusRequest get status => $_getN(4);
  @$pb.TagNumber(13)
  set status(StatusRequest value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasStatus() => $_has(4);
  @$pb.TagNumber(13)
  void clearStatus() => $_clearField(13);
  @$pb.TagNumber(13)
  StatusRequest ensureStatus() => $_ensure(4);

  /// Cancel an in-flight operation by id.
  @$pb.TagNumber(14)
  Cancel get cancel => $_getN(5);
  @$pb.TagNumber(14)
  set cancel(Cancel value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasCancel() => $_has(5);
  @$pb.TagNumber(14)
  void clearCancel() => $_clearField(14);
  @$pb.TagNumber(14)
  Cancel ensureCancel() => $_ensure(5);
}

enum ServerFrame_Body { pong, token, done, error, status, notSet }

/// Frame sent from server → client on the Session stream. The `id`
/// matches the originating ClientFrame.id, so the client can dispatch
/// frames to whichever caller is awaiting them.
class ServerFrame extends $pb.GeneratedMessage {
  factory ServerFrame({
    $fixnum.Int64? id,
    PingReply? pong,
    ChatToken? token,
    Done? done,
    Error? error,
    StatusReply? status,
  }) {
    final result = create();
    if (id != null) result.id = id;
    if (pong != null) result.pong = pong;
    if (token != null) result.token = token;
    if (done != null) result.done = done;
    if (error != null) result.error = error;
    if (status != null) result.status = status;
    return result;
  }

  ServerFrame._();

  factory ServerFrame.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ServerFrame.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, ServerFrame_Body> _ServerFrame_BodyByTag = {
    10: ServerFrame_Body.pong,
    11: ServerFrame_Body.token,
    12: ServerFrame_Body.done,
    13: ServerFrame_Body.error,
    14: ServerFrame_Body.status,
    0: ServerFrame_Body.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ServerFrame',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..oo(0, [10, 11, 12, 13, 14])
    ..a<$fixnum.Int64>(1, _omitFieldNames ? '' : 'id', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..aOM<PingReply>(10, _omitFieldNames ? '' : 'pong',
        subBuilder: PingReply.create)
    ..aOM<ChatToken>(11, _omitFieldNames ? '' : 'token',
        subBuilder: ChatToken.create)
    ..aOM<Done>(12, _omitFieldNames ? '' : 'done', subBuilder: Done.create)
    ..aOM<Error>(13, _omitFieldNames ? '' : 'error', subBuilder: Error.create)
    ..aOM<StatusReply>(14, _omitFieldNames ? '' : 'status',
        subBuilder: StatusReply.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerFrame clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ServerFrame copyWith(void Function(ServerFrame) updates) =>
      super.copyWith((message) => updates(message as ServerFrame))
          as ServerFrame;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ServerFrame create() => ServerFrame._();
  @$core.override
  ServerFrame createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ServerFrame getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ServerFrame>(create);
  static ServerFrame? _defaultInstance;

  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  ServerFrame_Body whichBody() => _ServerFrame_BodyByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(10)
  @$pb.TagNumber(11)
  @$pb.TagNumber(12)
  @$pb.TagNumber(13)
  @$pb.TagNumber(14)
  void clearBody() => $_clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $fixnum.Int64 get id => $_getI64(0);
  @$pb.TagNumber(1)
  set id($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasId() => $_has(0);
  @$pb.TagNumber(1)
  void clearId() => $_clearField(1);

  @$pb.TagNumber(10)
  PingReply get pong => $_getN(1);
  @$pb.TagNumber(10)
  set pong(PingReply value) => $_setField(10, value);
  @$pb.TagNumber(10)
  $core.bool hasPong() => $_has(1);
  @$pb.TagNumber(10)
  void clearPong() => $_clearField(10);
  @$pb.TagNumber(10)
  PingReply ensurePong() => $_ensure(1);

  /// Mid-stream token from generate / shard_run. Multiple tokens
  /// arrive for one id before the operation ends.
  @$pb.TagNumber(11)
  ChatToken get token => $_getN(2);
  @$pb.TagNumber(11)
  set token(ChatToken value) => $_setField(11, value);
  @$pb.TagNumber(11)
  $core.bool hasToken() => $_has(2);
  @$pb.TagNumber(11)
  void clearToken() => $_clearField(11);
  @$pb.TagNumber(11)
  ChatToken ensureToken() => $_ensure(2);

  /// Operation completed cleanly. Always the last frame for a
  /// given id on success. After Done the id is free to reuse.
  @$pb.TagNumber(12)
  Done get done => $_getN(3);
  @$pb.TagNumber(12)
  set done(Done value) => $_setField(12, value);
  @$pb.TagNumber(12)
  $core.bool hasDone() => $_has(3);
  @$pb.TagNumber(12)
  void clearDone() => $_clearField(12);
  @$pb.TagNumber(12)
  Done ensureDone() => $_ensure(3);

  /// Operation failed. Mutually exclusive with Done; same
  /// semantics (operation is over, id is free).
  @$pb.TagNumber(13)
  Error get error => $_getN(4);
  @$pb.TagNumber(13)
  set error(Error value) => $_setField(13, value);
  @$pb.TagNumber(13)
  $core.bool hasError() => $_has(4);
  @$pb.TagNumber(13)
  void clearError() => $_clearField(13);
  @$pb.TagNumber(13)
  Error ensureError() => $_ensure(4);

  /// Reply to a StatusRequest.
  @$pb.TagNumber(14)
  StatusReply get status => $_getN(5);
  @$pb.TagNumber(14)
  set status(StatusReply value) => $_setField(14, value);
  @$pb.TagNumber(14)
  $core.bool hasStatus() => $_has(5);
  @$pb.TagNumber(14)
  void clearStatus() => $_clearField(14);
  @$pb.TagNumber(14)
  StatusReply ensureStatus() => $_ensure(5);
}

/// Sent from client → server to abort an in-flight operation by id.
class Cancel extends $pb.GeneratedMessage {
  factory Cancel({
    $fixnum.Int64? targetId,
  }) {
    final result = create();
    if (targetId != null) result.targetId = targetId;
    return result;
  }

  Cancel._();

  factory Cancel.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Cancel.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Cancel',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..a<$fixnum.Int64>(
        1, _omitFieldNames ? '' : 'targetId', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Cancel clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Cancel copyWith(void Function(Cancel) updates) =>
      super.copyWith((message) => updates(message as Cancel)) as Cancel;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Cancel create() => Cancel._();
  @$core.override
  Cancel createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Cancel getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Cancel>(create);
  static Cancel? _defaultInstance;

  /// The operation id (not the Cancel frame's own id) to abort. The
  /// server emits an Error{code=CANCELLED} for `target_id` and the
  /// operation winds down.
  @$pb.TagNumber(1)
  $fixnum.Int64 get targetId => $_getI64(0);
  @$pb.TagNumber(1)
  set targetId($fixnum.Int64 value) => $_setInt64(0, value);
  @$pb.TagNumber(1)
  $core.bool hasTargetId() => $_has(0);
  @$pb.TagNumber(1)
  void clearTargetId() => $_clearField(1);
}

/// Generic terminator for a successful operation. Reserved tag range
/// 1-15 left for future per-op summary data.
class Done extends $pb.GeneratedMessage {
  factory Done() => create();

  Done._();

  factory Done.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Done.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Done',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Done clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Done copyWith(void Function(Done) updates) =>
      super.copyWith((message) => updates(message as Done)) as Done;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Done create() => Done._();
  @$core.override
  Done createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Done getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Done>(create);
  static Done? _defaultInstance;
}

/// Generic operation-level failure. Distinct from grpc transport errors
/// (which surface as the rpc's own Status) — these are application-level
/// failures inside an otherwise-healthy Session.
class Error extends $pb.GeneratedMessage {
  factory Error({
    Error_Code? code,
    $core.String? message,
  }) {
    final result = create();
    if (code != null) result.code = code;
    if (message != null) result.message = message;
    return result;
  }

  Error._();

  factory Error.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory Error.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'Error',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aE<Error_Code>(1, _omitFieldNames ? '' : 'code',
        enumValues: Error_Code.values)
    ..aOS(2, _omitFieldNames ? '' : 'message')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Error clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  Error copyWith(void Function(Error) updates) =>
      super.copyWith((message) => updates(message as Error)) as Error;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static Error create() => Error._();
  @$core.override
  Error createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static Error getDefault() =>
      _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<Error>(create);
  static Error? _defaultInstance;

  @$pb.TagNumber(1)
  Error_Code get code => $_getN(0);
  @$pb.TagNumber(1)
  set code(Error_Code value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasCode() => $_has(0);
  @$pb.TagNumber(1)
  void clearCode() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get message => $_getSZ(1);
  @$pb.TagNumber(2)
  set message($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasMessage() => $_has(1);
  @$pb.TagNumber(2)
  void clearMessage() => $_clearField(2);
}

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

  /// Wall-clock time on the daemon, RFC 3339.
  @$pb.TagNumber(1)
  $core.String get serverTime => $_getSZ(0);
  @$pb.TagNumber(1)
  set serverTime($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerTime() => $_clearField(1);
}

/// `kwaainet generate <PROMPT>` — single-node local inference.
class GenerateRequest extends $pb.GeneratedMessage {
  factory GenerateRequest({
    $core.String? role,
    $core.String? content,
    $core.String? conversationId,
  }) {
    final result = create();
    if (role != null) result.role = role;
    if (content != null) result.content = content;
    if (conversationId != null) result.conversationId = conversationId;
    return result;
  }

  GenerateRequest._();

  factory GenerateRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory GenerateRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'GenerateRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'role')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..aOS(3, _omitFieldNames ? '' : 'conversationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GenerateRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  GenerateRequest copyWith(void Function(GenerateRequest) updates) =>
      super.copyWith((message) => updates(message as GenerateRequest))
          as GenerateRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static GenerateRequest create() => GenerateRequest._();
  @$core.override
  GenerateRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static GenerateRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<GenerateRequest>(create);
  static GenerateRequest? _defaultInstance;

  /// Conventional roles: "user", "assistant", "system". Free-form.
  @$pb.TagNumber(1)
  $core.String get role => $_getSZ(0);
  @$pb.TagNumber(1)
  set role($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearRole() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);

  /// Optional client-supplied conversation id for future multi-turn.
  /// Ignored today.
  @$pb.TagNumber(3)
  $core.String get conversationId => $_getSZ(2);
  @$pb.TagNumber(3)
  set conversationId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasConversationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearConversationId() => $_clearField(3);
}

/// `kwaainet shard run <PROMPT>` — distributed inference across the mesh.
class ShardRunRequest extends $pb.GeneratedMessage {
  factory ShardRunRequest({
    $core.String? role,
    $core.String? content,
    $core.String? model,
    $core.String? conversationId,
  }) {
    final result = create();
    if (role != null) result.role = role;
    if (content != null) result.content = content;
    if (model != null) result.model = model;
    if (conversationId != null) result.conversationId = conversationId;
    return result;
  }

  ShardRunRequest._();

  factory ShardRunRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory ShardRunRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'ShardRunRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'role')
    ..aOS(2, _omitFieldNames ? '' : 'content')
    ..aOS(3, _omitFieldNames ? '' : 'model')
    ..aOS(4, _omitFieldNames ? '' : 'conversationId')
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ShardRunRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  ShardRunRequest copyWith(void Function(ShardRunRequest) updates) =>
      super.copyWith((message) => updates(message as ShardRunRequest))
          as ShardRunRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static ShardRunRequest create() => ShardRunRequest._();
  @$core.override
  ShardRunRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static ShardRunRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<ShardRunRequest>(create);
  static ShardRunRequest? _defaultInstance;

  @$pb.TagNumber(1)
  $core.String get role => $_getSZ(0);
  @$pb.TagNumber(1)
  set role($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasRole() => $_has(0);
  @$pb.TagNumber(1)
  void clearRole() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get content => $_getSZ(1);
  @$pb.TagNumber(2)
  set content($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasContent() => $_has(1);
  @$pb.TagNumber(2)
  void clearContent() => $_clearField(2);

  /// Optional model override. Defaults to the daemon's configured
  /// model when empty.
  @$pb.TagNumber(3)
  $core.String get model => $_getSZ(2);
  @$pb.TagNumber(3)
  set model($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasModel() => $_has(2);
  @$pb.TagNumber(3)
  void clearModel() => $_clearField(3);

  @$pb.TagNumber(4)
  $core.String get conversationId => $_getSZ(3);
  @$pb.TagNumber(4)
  set conversationId($core.String value) => $_setString(3, value);
  @$pb.TagNumber(4)
  $core.bool hasConversationId() => $_has(3);
  @$pb.TagNumber(4)
  void clearConversationId() => $_clearField(4);
}

/// `kwaainet status` — daemon-side state snapshot.
class StatusRequest extends $pb.GeneratedMessage {
  factory StatusRequest() => create();

  StatusRequest._();

  factory StatusRequest.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory StatusRequest.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StatusRequest',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StatusRequest clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StatusRequest copyWith(void Function(StatusRequest) updates) =>
      super.copyWith((message) => updates(message as StatusRequest))
          as StatusRequest;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StatusRequest create() => StatusRequest._();
  @$core.override
  StatusRequest createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static StatusRequest getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StatusRequest>(create);
  static StatusRequest? _defaultInstance;
}

class StatusReply extends $pb.GeneratedMessage {
  factory StatusReply({
    $core.String? serverTime,
    $core.String? model,
    $core.bool? shardReady,
    $core.int? peerCount,
    $fixnum.Int64? uptimeSecs,
  }) {
    final result = create();
    if (serverTime != null) result.serverTime = serverTime;
    if (model != null) result.model = model;
    if (shardReady != null) result.shardReady = shardReady;
    if (peerCount != null) result.peerCount = peerCount;
    if (uptimeSecs != null) result.uptimeSecs = uptimeSecs;
    return result;
  }

  StatusReply._();

  factory StatusReply.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory StatusReply.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'StatusReply',
      package: const $pb.PackageName(_omitMessageNames ? '' : 'kwaai.v1'),
      createEmptyInstance: create)
    ..aOS(1, _omitFieldNames ? '' : 'serverTime')
    ..aOS(2, _omitFieldNames ? '' : 'model')
    ..aOB(3, _omitFieldNames ? '' : 'shardReady')
    ..aI(4, _omitFieldNames ? '' : 'peerCount', fieldType: $pb.PbFieldType.OU3)
    ..a<$fixnum.Int64>(
        5, _omitFieldNames ? '' : 'uptimeSecs', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StatusReply clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  StatusReply copyWith(void Function(StatusReply) updates) =>
      super.copyWith((message) => updates(message as StatusReply))
          as StatusReply;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static StatusReply create() => StatusReply._();
  @$core.override
  StatusReply createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static StatusReply getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<StatusReply>(create);
  static StatusReply? _defaultInstance;

  /// Daemon wall-clock time, RFC 3339 — same format as PingReply.
  @$pb.TagNumber(1)
  $core.String get serverTime => $_getSZ(0);
  @$pb.TagNumber(1)
  set serverTime($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasServerTime() => $_has(0);
  @$pb.TagNumber(1)
  void clearServerTime() => $_clearField(1);

  /// The model this node is configured to serve.
  @$pb.TagNumber(2)
  $core.String get model => $_getSZ(1);
  @$pb.TagNumber(2)
  set model($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasModel() => $_has(1);
  @$pb.TagNumber(2)
  void clearModel() => $_clearField(2);

  /// True when the local shard server (the one this node contributes
  /// to the mesh) is up and serving its block range.
  @$pb.TagNumber(3)
  $core.bool get shardReady => $_getBF(2);
  @$pb.TagNumber(3)
  set shardReady($core.bool value) => $_setBool(2, value);
  @$pb.TagNumber(3)
  $core.bool hasShardReady() => $_has(2);
  @$pb.TagNumber(3)
  void clearShardReady() => $_clearField(3);

  /// Number of peers currently in the routing table (best-effort
  /// snapshot from kwaai-p2p-daemon).
  @$pb.TagNumber(4)
  $core.int get peerCount => $_getIZ(3);
  @$pb.TagNumber(4)
  set peerCount($core.int value) => $_setUnsignedInt32(3, value);
  @$pb.TagNumber(4)
  $core.bool hasPeerCount() => $_has(3);
  @$pb.TagNumber(4)
  void clearPeerCount() => $_clearField(4);

  /// Daemon process uptime in seconds.
  @$pb.TagNumber(5)
  $fixnum.Int64 get uptimeSecs => $_getI64(4);
  @$pb.TagNumber(5)
  set uptimeSecs($fixnum.Int64 value) => $_setInt64(4, value);
  @$pb.TagNumber(5)
  $core.bool hasUptimeSecs() => $_has(4);
  @$pb.TagNumber(5)
  void clearUptimeSecs() => $_clearField(5);
}

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

  @$pb.TagNumber(1)
  $core.String get content => $_getSZ(0);
  @$pb.TagNumber(1)
  set content($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasContent() => $_has(0);
  @$pb.TagNumber(1)
  void clearContent() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.String get role => $_getSZ(1);
  @$pb.TagNumber(2)
  set role($core.String value) => $_setString(1, value);
  @$pb.TagNumber(2)
  $core.bool hasRole() => $_has(1);
  @$pb.TagNumber(2)
  void clearRole() => $_clearField(2);

  @$pb.TagNumber(3)
  $core.String get conversationId => $_getSZ(2);
  @$pb.TagNumber(3)
  set conversationId($core.String value) => $_setString(2, value);
  @$pb.TagNumber(3)
  $core.bool hasConversationId() => $_has(2);
  @$pb.TagNumber(3)
  void clearConversationId() => $_clearField(3);
}

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

  @$pb.TagNumber(1)
  $core.String get text => $_getSZ(0);
  @$pb.TagNumber(1)
  set text($core.String value) => $_setString(0, value);
  @$pb.TagNumber(1)
  $core.bool hasText() => $_has(0);
  @$pb.TagNumber(1)
  void clearText() => $_clearField(1);

  @$pb.TagNumber(2)
  $core.bool get done => $_getBF(1);
  @$pb.TagNumber(2)
  set done($core.bool value) => $_setBool(1, value);
  @$pb.TagNumber(2)
  $core.bool hasDone() => $_has(1);
  @$pb.TagNumber(2)
  void clearDone() => $_clearField(2);

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
