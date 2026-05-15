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
