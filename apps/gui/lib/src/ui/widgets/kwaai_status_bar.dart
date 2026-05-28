import 'package:flutter/material.dart';

import '../theme/kwaai_theme.dart';

/// Severity of a [KwaaiStatusBar] message. Drives the icon and color.
enum KwaaiStatusSeverity {
  /// Informational (e.g. "restart to apply"). Orange tint.
  info,

  /// Warning that requires attention but isn't blocking.
  warning,

  /// Error condition. Red tint.
  error,
}

/// A pinned status bar — same look used at the bottom of the Settings
/// shell card whether the message is an error, warning, or a routine
/// info nudge. Designed to be reused for any persistent, page-level
/// banner in the app.
///
/// Pass a [severity] for the icon/color preset, a [message], and
/// optionally an [action] (e.g. a Restart button).
class KwaaiStatusBar extends StatelessWidget {
  const KwaaiStatusBar({
    super.key,
    required this.severity,
    required this.message,
    this.action,
    this.icon,
    this.onDismiss,
    this.bottomRadius,
  });

  final KwaaiStatusSeverity severity;
  final String message;
  final Widget? action;

  /// Overrides the default icon for this severity.
  final IconData? icon;

  /// When non-null, a small × button appears on the trailing edge of the
  /// bar and invokes this callback when tapped. Use it to clear the
  /// underlying error/warning state that drove the bar.
  final VoidCallback? onDismiss;

  /// Rounds the bar's tinted background at the bottom so it follows the
  /// parent card's bottom-corner shape. The parent's clip already masks
  /// painting beyond the curve, but at small radii anti-aliasing leakage
  /// can flatten the visible corner — pre-rounding the wash gives a
  /// crisper edge.
  final BorderRadius? bottomRadius;

  @override
  Widget build(BuildContext context) {
    final ext = context.kwaai;
    final color = switch (severity) {
      KwaaiStatusSeverity.info => ext.statusTransitioning,
      KwaaiStatusSeverity.warning => ext.statusTransitioning,
      KwaaiStatusSeverity.error => ext.buttonDestructive,
    };
    final defaultIcon = switch (severity) {
      KwaaiStatusSeverity.info => Icons.info_outline,
      KwaaiStatusSeverity.warning => Icons.warning_amber_outlined,
      KwaaiStatusSeverity.error => Icons.error_outline,
    };
    // Bar paints edge-to-edge; rounded corners are handled by the
    // parent KwaaiCard's clip, which knows its own per-corner radii.
    return Container(
      width: double.infinity,
      padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 10),
      decoration: BoxDecoration(
        color: color.withValues(alpha: 0.12),
        border: Border(top: BorderSide(color: Theme.of(context).dividerColor)),
        borderRadius: bottomRadius,
      ),
      child: Row(
        children: [
          Icon(icon ?? defaultIcon, size: 16, color: color),
          const SizedBox(width: 8),
          Expanded(
            child: Text(
              message,
              style: Theme.of(context).textTheme.bodyMedium,
            ),
          ),
          if (action != null) ...[const SizedBox(width: 8), action!],
          if (onDismiss != null) ...[
            const SizedBox(width: 4),
            IconButton(
              tooltip: 'Dismiss',
              icon: const Icon(Icons.close, size: 16),
              onPressed: onDismiss,
              visualDensity: VisualDensity.compact,
              style: IconButton.styleFrom(
                padding: EdgeInsets.zero,
                minimumSize: const Size(24, 24),
                tapTargetSize: MaterialTapTargetSize.shrinkWrap,
                // Neutral gray — keeps the dismiss affordance from
                // competing with the severity color of the icon + wash.
                foregroundColor: Theme.of(context)
                    .colorScheme
                    .onSurfaceVariant
                    .withValues(alpha: 0.6),
              ),
            ),
          ],
        ],
      ),
    );
  }
}
