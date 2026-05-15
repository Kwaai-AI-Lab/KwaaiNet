import 'package:flutter/material.dart';

/// Themed radio option with a label and optional subtitle. Wrap a group in
/// a `RadioGroup<T>` to bind the value.
class KwaaiRadio<T> extends StatelessWidget {
  const KwaaiRadio({
    super.key,
    required this.value,
    required this.label,
    this.subtitle,
  });

  final T value;
  final Widget label;
  final Widget? subtitle;

  @override
  Widget build(BuildContext context) {
    return RadioListTile<T>(
      value: value,
      title: label,
      subtitle: subtitle,
      contentPadding: const EdgeInsets.symmetric(horizontal: 4),
      dense: true,
    );
  }
}
