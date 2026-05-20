import 'package:flutter/material.dart';

/// Section heading. Bold, [TextTheme.bodyMedium]-sized — consistent across the
/// app.
class KwaaiHeading extends StatelessWidget {
  const KwaaiHeading(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: Theme.of(
        context,
      ).textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600),
    );
  }
}
