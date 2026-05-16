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

class Error_Code extends $pb.ProtobufEnum {
  static const Error_Code UNKNOWN =
      Error_Code._(0, _omitEnumNames ? '' : 'UNKNOWN');
  static const Error_Code INVALID_ARGUMENT =
      Error_Code._(1, _omitEnumNames ? '' : 'INVALID_ARGUMENT');
  static const Error_Code NOT_FOUND =
      Error_Code._(2, _omitEnumNames ? '' : 'NOT_FOUND');
  static const Error_Code UNAVAILABLE =
      Error_Code._(3, _omitEnumNames ? '' : 'UNAVAILABLE');
  static const Error_Code CANCELLED =
      Error_Code._(4, _omitEnumNames ? '' : 'CANCELLED');
  static const Error_Code INTERNAL =
      Error_Code._(5, _omitEnumNames ? '' : 'INTERNAL');
  static const Error_Code UNIMPLEMENTED =
      Error_Code._(6, _omitEnumNames ? '' : 'UNIMPLEMENTED');

  static const $core.List<Error_Code> values = <Error_Code>[
    UNKNOWN,
    INVALID_ARGUMENT,
    NOT_FOUND,
    UNAVAILABLE,
    CANCELLED,
    INTERNAL,
    UNIMPLEMENTED,
  ];

  static final $core.List<Error_Code?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 6);
  static Error_Code? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Error_Code._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
