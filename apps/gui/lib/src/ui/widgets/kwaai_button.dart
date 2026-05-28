import 'package:flutter/material.dart';

import '../../window/window_focus.dart';
import '../theme/kwaai_theme.dart';

/// Visual variant for [KwaaiButton].
///
/// * [primary] — accent fill, white text. Desaturates to gray when the
///   window is unfocused, matching native macOS button behaviour.
/// * [destructive] — red fill, white text. Used for actions that remove
///   or stop things (e.g. Stop service).
/// * [secondary] — light-gray fill, onSurface text. The neutral "do
///   something secondary" pairing for a primary button (e.g. Cancel).
enum KwaaiButtonVariant { primary, destructive, secondary }

/// Themed action button used across the app. Same look as the existing
/// Start / Stop service buttons. Pass an [icon] for the leading glyph;
/// omit it for a label-only button.
class KwaaiButton extends StatelessWidget {
  const KwaaiButton({
    super.key,
    required this.label,
    required this.onPressed,
    this.variant = KwaaiButtonVariant.primary,
    this.icon,
  });

  final String label;
  final VoidCallback? onPressed;
  final KwaaiButtonVariant variant;
  final IconData? icon;

  @override
  Widget build(BuildContext context) {
    final ext = context.kwaai;
    final scheme = Theme.of(context).colorScheme;

    Color bg;
    Color fg;
    switch (variant) {
      case KwaaiButtonVariant.primary:
        // Desaturates to gray when the app window is unfocused.
        final focused = WindowFocusScope.of(context);
        bg = focused ? ext.accentPrimary : const Color(0xFFD4D4D4);
        fg = focused ? Colors.white : scheme.onSurface;
      case KwaaiButtonVariant.destructive:
        bg = ext.buttonDestructive;
        fg = Colors.white;
      case KwaaiButtonVariant.secondary:
        bg = const Color(0xFFEFEFEF);
        fg = scheme.onSurface;
    }

    final style = FilledButton.styleFrom(
      backgroundColor: bg,
      foregroundColor: fg,
    );

    if (icon != null) {
      return FilledButton.icon(
        onPressed: onPressed,
        style: style,
        icon: Icon(icon),
        label: Text(label),
      );
    }
    return FilledButton(
      onPressed: onPressed,
      style: style,
      child: Text(label),
    );
  }
}
