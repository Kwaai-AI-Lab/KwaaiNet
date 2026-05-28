import 'package:flutter/material.dart';

/// Standard caption / muted body text.
class KwaaiLabel extends StatelessWidget {
  const KwaaiLabel(this.text, {super.key, this.muted = false});

  final String text;
  final bool muted;

  @override
  Widget build(BuildContext context) {
    final base = Theme.of(context).textTheme.bodyMedium;
    final style = muted
        ? base?.copyWith(color: Theme.of(context).colorScheme.onSurfaceVariant)
        : base;
    return Text(text, style: style);
  }
}
