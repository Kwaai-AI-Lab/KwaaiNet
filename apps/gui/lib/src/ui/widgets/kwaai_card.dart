import 'package:flutter/material.dart';

import '../../window/window_focus.dart';
import '../theme/kwaai_theme.dart';

/// Raised, rounded surface that matches bunchabits's `AppCard.elevated`.
/// Shadow is drawn only when the application window is focused; when the
/// window is in the background, the shadow disappears and the surface takes
/// a subtle darker tint.
class KwaaiCard extends StatelessWidget {
  const KwaaiCard({
    super.key,
    required this.child,
    this.padding = const EdgeInsets.all(12),
    this.borderRadius = const BorderRadius.all(Radius.circular(8)),
  });

  final Widget child;
  final EdgeInsetsGeometry padding;
  final BorderRadius borderRadius;

  @override
  Widget build(BuildContext context) {
    final focused = WindowFocusScope.of(context);
    final ext = context.kwaai;
    final theme = Theme.of(context);

    final radius = borderRadius;
    final baseColor = ext.cardBackground;
    final unfocusedTint = ColorScheme.of(
      context,
    ).onSurface.withValues(alpha: 0.04);
    final fillColor = focused
        ? baseColor
        : Color.alphaBlend(unfocusedTint, baseColor);

    return AnimatedContainer(
      duration: const Duration(milliseconds: 150),
      curve: Curves.easeOut,
      decoration: BoxDecoration(
        color: fillColor,
        borderRadius: radius,
        border: Border.all(color: theme.dividerColor.withValues(alpha: 0.15)),
        boxShadow: focused
            ? [
                BoxShadow(
                  color: Colors.black.withValues(alpha: 0.10),
                  blurRadius: 12,
                  offset: const Offset(0, 2),
                ),
              ]
            : const [],
      ),
      padding: padding,
      child: child,
    );
  }
}
