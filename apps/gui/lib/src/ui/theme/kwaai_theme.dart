import 'package:flutter/material.dart';

/// Semantic colors for the kwaainet-gui app. Use [Theme.of(context).kwaai] to
/// read these; see [KwaaiBuildContext] below.
class KwaaiThemeExtension extends ThemeExtension<KwaaiThemeExtension> {
  const KwaaiThemeExtension({
    required this.scaffoldBackground,
    required this.appBarBackground,
    required this.appBarForeground,
    required this.navRailBackground,
    required this.divider,
    required this.cardBackground,
    required this.elevatedSurface,
    required this.menuBackground,
    required this.inputBackground,
    required this.accentPrimary,
    required this.statusRunning,
    required this.statusStopped,
    required this.statusTransitioning,
    required this.buttonDestructive,
    required this.error,
  });

  final Color scaffoldBackground;
  final Color appBarBackground;
  final Color appBarForeground;
  final Color navRailBackground;
  final Color divider;
  final Color cardBackground;

  /// Surface sitting one step "above" [cardBackground] — used for sectional
  /// cards inside a content area (e.g. each feature section in Settings). In
  /// dark mode this is slightly lighter than the card; in light mode it's
  /// slightly darker so it still reads as a distinct surface.
  final Color elevatedSurface;

  /// Surface for popover-style menus (dropdown popups, context menus). Reads
  /// brighter than [elevatedSurface] — closer to the way macOS NSMenu lifts
  /// off the background.
  final Color menuBackground;

  final Color inputBackground;
  final Color accentPrimary;
  final Color statusRunning;
  final Color statusStopped;

  /// Status indicator while the daemon is starting or stopping (orange).
  final Color statusTransitioning;

  /// Fill for destructive actions, e.g. the Stop service button (red).
  final Color buttonDestructive;

  final Color error;

  @override
  KwaaiThemeExtension copyWith({
    Color? scaffoldBackground,
    Color? appBarBackground,
    Color? appBarForeground,
    Color? navRailBackground,
    Color? divider,
    Color? cardBackground,
    Color? elevatedSurface,
    Color? menuBackground,
    Color? inputBackground,
    Color? accentPrimary,
    Color? statusRunning,
    Color? statusStopped,
    Color? statusTransitioning,
    Color? buttonDestructive,
    Color? error,
  }) {
    return KwaaiThemeExtension(
      scaffoldBackground: scaffoldBackground ?? this.scaffoldBackground,
      appBarBackground: appBarBackground ?? this.appBarBackground,
      appBarForeground: appBarForeground ?? this.appBarForeground,
      navRailBackground: navRailBackground ?? this.navRailBackground,
      divider: divider ?? this.divider,
      cardBackground: cardBackground ?? this.cardBackground,
      elevatedSurface: elevatedSurface ?? this.elevatedSurface,
      menuBackground: menuBackground ?? this.menuBackground,
      inputBackground: inputBackground ?? this.inputBackground,
      accentPrimary: accentPrimary ?? this.accentPrimary,
      statusRunning: statusRunning ?? this.statusRunning,
      statusStopped: statusStopped ?? this.statusStopped,
      statusTransitioning: statusTransitioning ?? this.statusTransitioning,
      buttonDestructive: buttonDestructive ?? this.buttonDestructive,
      error: error ?? this.error,
    );
  }

  @override
  KwaaiThemeExtension lerp(covariant KwaaiThemeExtension? other, double t) {
    if (other == null) return this;
    return KwaaiThemeExtension(
      scaffoldBackground: Color.lerp(
        scaffoldBackground,
        other.scaffoldBackground,
        t,
      )!,
      appBarBackground: Color.lerp(
        appBarBackground,
        other.appBarBackground,
        t,
      )!,
      appBarForeground: Color.lerp(
        appBarForeground,
        other.appBarForeground,
        t,
      )!,
      navRailBackground: Color.lerp(
        navRailBackground,
        other.navRailBackground,
        t,
      )!,
      divider: Color.lerp(divider, other.divider, t)!,
      cardBackground: Color.lerp(cardBackground, other.cardBackground, t)!,
      elevatedSurface: Color.lerp(
        elevatedSurface,
        other.elevatedSurface,
        t,
      )!,
      menuBackground: Color.lerp(menuBackground, other.menuBackground, t)!,
      inputBackground: Color.lerp(inputBackground, other.inputBackground, t)!,
      accentPrimary: Color.lerp(accentPrimary, other.accentPrimary, t)!,
      statusRunning: Color.lerp(statusRunning, other.statusRunning, t)!,
      statusStopped: Color.lerp(statusStopped, other.statusStopped, t)!,
      statusTransitioning: Color.lerp(
        statusTransitioning,
        other.statusTransitioning,
        t,
      )!,
      buttonDestructive: Color.lerp(
        buttonDestructive,
        other.buttonDestructive,
        t,
      )!,
      error: Color.lerp(error, other.error, t)!,
    );
  }
}

extension KwaaiBuildContext on BuildContext {
  KwaaiThemeExtension get kwaai =>
      Theme.of(this).extension<KwaaiThemeExtension>()!;
}
