// This is a generated file - do not edit.
//
// Generated from kwaai.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports
// ignore_for_file: unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use pingRequestDescriptor instead')
const PingRequest$json = {
  '1': 'PingRequest',
};

/// Descriptor for `PingRequest`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pingRequestDescriptor =
    $convert.base64Decode('CgtQaW5nUmVxdWVzdA==');

@$core.Deprecated('Use pingReplyDescriptor instead')
const PingReply$json = {
  '1': 'PingReply',
  '2': [
    {'1': 'server_time', '3': 1, '4': 1, '5': 9, '10': 'serverTime'},
  ],
};

/// Descriptor for `PingReply`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List pingReplyDescriptor = $convert.base64Decode(
    'CglQaW5nUmVwbHkSHwoLc2VydmVyX3RpbWUYASABKAlSCnNlcnZlclRpbWU=');

@$core.Deprecated('Use chatMessageDescriptor instead')
const ChatMessage$json = {
  '1': 'ChatMessage',
  '2': [
    {'1': 'content', '3': 1, '4': 1, '5': 9, '10': 'content'},
    {'1': 'role', '3': 2, '4': 1, '5': 9, '10': 'role'},
    {
      '1': 'conversation_id',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'conversationId',
      '17': true
    },
  ],
  '8': [
    {'1': '_conversation_id'},
  ],
};

/// Descriptor for `ChatMessage`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatMessageDescriptor = $convert.base64Decode(
    'CgtDaGF0TWVzc2FnZRIYCgdjb250ZW50GAEgASgJUgdjb250ZW50EhIKBHJvbGUYAiABKAlSBH'
    'JvbGUSLAoPY29udmVyc2F0aW9uX2lkGAMgASgJSABSDmNvbnZlcnNhdGlvbklkiAEBQhIKEF9j'
    'b252ZXJzYXRpb25faWQ=');

@$core.Deprecated('Use chatTokenDescriptor instead')
const ChatToken$json = {
  '1': 'ChatToken',
  '2': [
    {'1': 'text', '3': 1, '4': 1, '5': 9, '10': 'text'},
    {'1': 'done', '3': 2, '4': 1, '5': 8, '10': 'done'},
    {
      '1': 'finish_reason',
      '3': 3,
      '4': 1,
      '5': 9,
      '9': 0,
      '10': 'finishReason',
      '17': true
    },
  ],
  '8': [
    {'1': '_finish_reason'},
  ],
};

/// Descriptor for `ChatToken`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List chatTokenDescriptor = $convert.base64Decode(
    'CglDaGF0VG9rZW4SEgoEdGV4dBgBIAEoCVIEdGV4dBISCgRkb25lGAIgASgIUgRkb25lEigKDW'
    'ZpbmlzaF9yZWFzb24YAyABKAlIAFIMZmluaXNoUmVhc29uiAEBQhAKDl9maW5pc2hfcmVhc29u');
