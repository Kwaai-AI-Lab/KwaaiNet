import 'package:flutter/material.dart';

class BrandedTitle extends StatelessWidget {
  const BrandedTitle({super.key, this.subtitle});

  final String? subtitle;

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Image.asset('assets/kwaaiai.png', height: 18),
        const SizedBox(width: 6),
        Text('Kwaai AI', style: Theme.of(context).textTheme.titleSmall),
        if (subtitle != null) ...[
          const SizedBox(width: 8),
          Text(
            '— $subtitle',
            style: Theme.of(context).textTheme.bodySmall?.copyWith(
              color: Theme.of(context).colorScheme.onSurfaceVariant,
            ),
          ),
        ],
      ],
    );
  }
}
