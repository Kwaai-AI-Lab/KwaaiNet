import 'package:flutter/material.dart';

import 'kwaai_card.dart';

/// Native macOS title-bar zone height. Shell cards extend *under* this; their
/// internal top padding pushes content below it so the traffic lights have a
/// clear region.
const double kMacOSTitlebarHeight = 28;

/// Standard gutter between shell cards and the window edge.
const double kShellGutter = 6;

/// Corner radius of the native macOS window itself (measured on-screen —
/// current macOS uses a notably larger radius than the ~10pt of the Big Sur
/// era). The source of truth for [kShellOuterRadius].
const double kWindowRadius = 28;

/// Radius for a shell-card corner that touches the window edge. Derived so the
/// card corner is *concentric* with the native window corner: with a uniform
/// [kShellGutter] inset, inner radius = outer radius − gap.
const double kShellOuterRadius = kWindowRadius - kShellGutter;

/// Radius for a shell-card corner that faces *into* the layout (next to
/// another card or the interior) rather than the window edge. Kept sharp.
const double kShellInnerRadius = 4;

/// Legacy alias — uniform radius. Prefer [shellRadius] for per-corner control.
const double kShellRadius = kShellInnerRadius;

/// Inner padding between a [ShellCard] edge and content laid inside it.
const double kShellInset = 4;

/// Builds a [BorderRadius] for a shell card, using the larger
/// [kShellOuterRadius] only on the corners that touch the window edge.
BorderRadius shellRadius({
  bool topLeft = false,
  bool topRight = false,
  bool bottomLeft = false,
  bool bottomRight = false,
}) {
  Radius r(bool outer) =>
      Radius.circular(outer ? kShellOuterRadius : kShellInnerRadius);
  return BorderRadius.only(
    topLeft: r(topLeft),
    topRight: r(topRight),
    bottomLeft: r(bottomLeft),
    bottomRight: r(bottomRight),
  );
}

/// Concentric corner radius for an element inset by [inset] from a parent
/// that itself has radius [parentRadius].
double concentricRadius(double parentRadius, double inset) {
  final r = parentRadius - inset;
  return r < 2 ? 2 : r;
}

/// A floating, raised panel used as a top-level region of a page. Wraps
/// [KwaaiCard] with the app's standard corner radius and edge gutter.
///
/// The card surface extends right to the top of the window (under the native
/// traffic lights). It does NOT inset its child — pages place a top bar tall
/// enough to span the titlebar zone (see [kMacOSTitlebarHeight]) so brand /
/// action buttons sit level with the traffic lights.
class ShellCard extends StatelessWidget {
  const ShellCard({
    super.key,
    required this.child,
    this.gutter = const EdgeInsets.all(kShellGutter),
    this.contentPadding = EdgeInsets.zero,
    this.borderRadius,
  });

  final Widget child;
  final EdgeInsets gutter;
  final EdgeInsets contentPadding;

  /// Per-corner radius. Defaults to uniform [kShellInnerRadius]; pass a value
  /// from [shellRadius] to round only the corners that touch the window edge.
  final BorderRadius? borderRadius;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: gutter,
      child: KwaaiCard(
        borderRadius: borderRadius ?? BorderRadius.circular(kShellInnerRadius),
        padding: contentPadding,
        child: child,
      ),
    );
  }
}

/// Page-level scaffold: paints the themed background and lays out one or more
/// [ShellCard]s passed as [child]. Both MainPage and SettingsPage use this so
/// window chrome (background, titlebar clearance) is identical across pages.
class AppShell extends StatelessWidget {
  const AppShell({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Scaffold(body: child);
  }
}
