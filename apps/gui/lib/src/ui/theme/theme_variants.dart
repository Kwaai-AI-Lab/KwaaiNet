import 'package:flutter/material.dart';

import 'kwaai_theme.dart';

enum ThemeVariantKey { native, kwaai, ocean, forest, sunset }

extension ThemeVariantKeyLabel on ThemeVariantKey {
  String get displayName => switch (this) {
    ThemeVariantKey.native => 'Native',
    ThemeVariantKey.kwaai => 'Kwaai',
    ThemeVariantKey.ocean => 'Ocean',
    ThemeVariantKey.forest => 'Forest',
    ThemeVariantKey.sunset => 'Sunset',
  };

  String get description => switch (this) {
    ThemeVariantKey.native => 'Clean white macOS look — system look and feel',
    ThemeVariantKey.kwaai => 'Soft lilac — the Kwaai palette',
    ThemeVariantKey.ocean => 'Cool blues',
    ThemeVariantKey.forest => 'Greens',
    ThemeVariantKey.sunset => 'Warm yellows and oranges',
  };

  /// Swatch color used by the theme picker dots. Native shows its scaffold
  /// background ("the look"); Kwaai shows its defining lilac tint; the other
  /// colored variants show their accent primary (the recognisable hue).
  Color swatch(Brightness brightness) {
    final ext = kwaaiVariants[this]![brightness]!;
    return switch (this) {
      ThemeVariantKey.native => ext.scaffoldBackground,
      ThemeVariantKey.kwaai => _kwaaiLilac,
      _ => ext.accentPrimary,
    };
  }
}

/// The defining Kwaai brand tint (#DED0E3) — used as the picker swatch and as
/// the base for the Kwaai variant's surfaces.
final Color _kwaaiLilac = _hex('#DED0E3');

Color _hex(String hex) {
  final h = hex.replaceFirst('#', '');
  return Color(int.parse('FF$h', radix: 16));
}

final Map<ThemeVariantKey, Map<Brightness, KwaaiThemeExtension>> kwaaiVariants =
    {
      // ─── Native: clean macOS look, white-first ───────────────────
      ThemeVariantKey.native: {
        Brightness.light: KwaaiThemeExtension(
          scaffoldBackground: _hex('#ffffff'),
          appBarBackground: _hex('#f5f5f7'),
          appBarForeground: _hex('#1d1d1f'),
          navRailBackground: _hex('#f5f5f7'),
          divider: _hex('#d2d2d7'),
          cardBackground: _hex('#ffffff'),
          elevatedSurface: _hex('#f5f5f7'),
          menuBackground: _hex('#fafafa'),
          inputBackground: _hex('#ffffff'),
          accentPrimary: _hex('#007aff'),
          statusRunning: _hex('#34c759'),
          statusStopped: _hex('#8e8e93'),
          statusTransitioning: _hex('#ff9500'),
          buttonDestructive: _hex('#ff3b30'),
          error: _hex('#ff3b30'),
          // Apple HIG-aligned semantic palette.
          semanticInfo: _hex('#007aff'),
          semanticWarning: _hex('#ff9500'),
          semanticSuccess: _hex('#34c759'),
          semanticError: _hex('#ff3b30'),
        ),
        Brightness.dark: KwaaiThemeExtension(
          scaffoldBackground: _hex('#1e1e1e'),
          appBarBackground: _hex('#2c2c2e'),
          appBarForeground: _hex('#f5f5f7'),
          navRailBackground: _hex('#2c2c2e'),
          divider: _hex('#3a3a3c'),
          cardBackground: _hex('#2c2c2e'),
          elevatedSurface: _hex('#3a3a3c'),
          menuBackground: _hex('#4a4a4c'),
          inputBackground: _hex('#3a3a3c'),
          accentPrimary: _hex('#0a84ff'),
          statusRunning: _hex('#30d158'),
          statusStopped: _hex('#98989d'),
          statusTransitioning: _hex('#ff9f0a'),
          buttonDestructive: _hex('#ff453a'),
          error: _hex('#ff453a'),
          semanticInfo: _hex('#0a84ff'),
          semanticWarning: _hex('#ff9f0a'),
          semanticSuccess: _hex('#30d158'),
          semanticError: _hex('#ff453a'),
        ),
      },
      // ─── Kwaai: soft lilac brand palette (#DED0E3) ───────────────
      ThemeVariantKey.kwaai: {
        Brightness.light: KwaaiThemeExtension(
          scaffoldBackground: _hex('#faf7fb'),
          appBarBackground: _hex('#ded0e3'),
          appBarForeground: _hex('#2e2333'),
          navRailBackground: _hex('#e8ddec'),
          divider: _hex('#cdb9d5'),
          cardBackground: _hex('#F5F3F6'),
          elevatedSurface: _hex('#ece7ee'),
          menuBackground: _hex('#faf7fb'),
          inputBackground: _hex('#ffffff'),
          accentPrimary: _hex('#9b7ebd'),
          statusRunning: _hex('#34c759'),
          statusStopped: _hex('#9b8fa3'),
          statusTransitioning: _hex('#e8930c'),
          buttonDestructive: _hex('#d6336c'),
          error: _hex('#d6336c'),
          // Semantic palette stays informational across themes — info
          // is always blue, not the lilac accent.
          semanticInfo: _hex('#2563eb'),
          semanticWarning: _hex('#e8930c'),
          semanticSuccess: _hex('#16a34a'),
          semanticError: _hex('#d6336c'),
        ),
        Brightness.dark: KwaaiThemeExtension(
          scaffoldBackground: _hex('#1e1a21'),
          appBarBackground: _hex('#2c2433'),
          appBarForeground: _hex('#ded0e3'),
          navRailBackground: _hex('#2c2433'),
          divider: _hex('#473b50'),
          cardBackground: _hex('#2c2433'),
          elevatedSurface: _hex('#382e40'),
          menuBackground: _hex('#473b50'),
          inputBackground: _hex('#383040'),
          accentPrimary: _hex('#c4a7e0'),
          statusRunning: _hex('#30d158'),
          statusStopped: _hex('#9b8fa3'),
          statusTransitioning: _hex('#f0a830'),
          buttonDestructive: _hex('#f06595'),
          error: _hex('#f06595'),
          semanticInfo: _hex('#60a5fa'),
          semanticWarning: _hex('#f0a830'),
          semanticSuccess: _hex('#4ade80'),
          semanticError: _hex('#f06595'),
        ),
      },
      // ─── Ocean ───────────────────────────────────────────────────
      ThemeVariantKey.ocean: {
        Brightness.light: KwaaiThemeExtension(
          scaffoldBackground: _hex('#f0f7ff'),
          appBarBackground: _hex('#d8ebf8'),
          appBarForeground: _hex('#0f2535'),
          navRailBackground: _hex('#e0f2fe'),
          divider: _hex('#bae6fd'),
          cardBackground: _hex('#e8f5fc'),
          elevatedSurface: _hex('#dceef8'),
          menuBackground: _hex('#f4faff'),
          inputBackground: _hex('#ffffff'),
          accentPrimary: _hex('#0284c7'),
          statusRunning: _hex('#0d9488'),
          statusStopped: _hex('#5b8ca8'),
          statusTransitioning: _hex('#d97706'),
          buttonDestructive: _hex('#dc2626'),
          error: _hex('#dc2626'),
          semanticInfo: _hex('#0284c7'),
          semanticWarning: _hex('#d97706'),
          semanticSuccess: _hex('#0d9488'),
          semanticError: _hex('#dc2626'),
        ),
        Brightness.dark: KwaaiThemeExtension(
          scaffoldBackground: _hex('#0d1b2a'),
          appBarBackground: _hex('#071520'),
          appBarForeground: _hex('#e0f2fe'),
          navRailBackground: _hex('#0c4a6e'),
          divider: _hex('#1e3a5f'),
          cardBackground: _hex('#0f2535'),
          elevatedSurface: _hex('#173247'),
          menuBackground: _hex('#1f4360'),
          inputBackground: _hex('#0a1a28'),
          accentPrimary: _hex('#38bdf8'),
          statusRunning: _hex('#5eead4'),
          statusStopped: _hex('#5a9bc8'),
          statusTransitioning: _hex('#fbbf24'),
          buttonDestructive: _hex('#f87171'),
          error: _hex('#f87171'),
          semanticInfo: _hex('#38bdf8'),
          semanticWarning: _hex('#fbbf24'),
          semanticSuccess: _hex('#5eead4'),
          semanticError: _hex('#f87171'),
        ),
      },
      // ─── Forest ──────────────────────────────────────────────────
      ThemeVariantKey.forest: {
        Brightness.light: KwaaiThemeExtension(
          scaffoldBackground: _hex('#f0fdf4'),
          appBarBackground: _hex('#d8f0e0'),
          appBarForeground: _hex('#052e16'),
          navRailBackground: _hex('#dcfce7'),
          divider: _hex('#bbf7d0'),
          cardBackground: _hex('#e8f8ed'),
          elevatedSurface: _hex('#dcf1e3'),
          menuBackground: _hex('#f3fcf6'),
          inputBackground: _hex('#ffffff'),
          accentPrimary: _hex('#16a34a'),
          statusRunning: _hex('#16a34a'),
          statusStopped: _hex('#5a8a6a'),
          statusTransitioning: _hex('#d97706'),
          buttonDestructive: _hex('#dc2626'),
          error: _hex('#dc2626'),
          semanticInfo: _hex('#2563eb'),
          semanticWarning: _hex('#d97706'),
          semanticSuccess: _hex('#16a34a'),
          semanticError: _hex('#dc2626'),
        ),
        Brightness.dark: KwaaiThemeExtension(
          scaffoldBackground: _hex('#052e16'),
          appBarBackground: _hex('#001810'),
          appBarForeground: _hex('#dcfce7'),
          navRailBackground: _hex('#14532d'),
          divider: _hex('#1d4d2b'),
          cardBackground: _hex('#0a3a1c'),
          elevatedSurface: _hex('#124a26'),
          menuBackground: _hex('#1a5e33'),
          inputBackground: _hex('#062414'),
          accentPrimary: _hex('#4ade80'),
          statusRunning: _hex('#86efac'),
          statusStopped: _hex('#5aaa7a'),
          statusTransitioning: _hex('#fbbf24'),
          buttonDestructive: _hex('#f87171'),
          error: _hex('#f87171'),
          semanticInfo: _hex('#60a5fa'),
          semanticWarning: _hex('#fbbf24'),
          semanticSuccess: _hex('#86efac'),
          semanticError: _hex('#f87171'),
        ),
      },
      // ─── Sunset ──────────────────────────────────────────────────
      ThemeVariantKey.sunset: {
        Brightness.light: KwaaiThemeExtension(
          scaffoldBackground: _hex('#fffbeb'),
          appBarBackground: _hex('#fef3c7'),
          appBarForeground: _hex('#7c2d12'),
          navRailBackground: _hex('#fed7aa'),
          divider: _hex('#fcd34d'),
          cardBackground: _hex('#fef9e7'),
          elevatedSurface: _hex('#fbf3d4'),
          menuBackground: _hex('#fffcf0'),
          inputBackground: _hex('#ffffff'),
          accentPrimary: _hex('#ea580c'),
          statusRunning: _hex('#16a34a'),
          statusStopped: _hex('#a8a29e'),
          statusTransitioning: _hex('#ca8a04'),
          buttonDestructive: _hex('#b91c1c'),
          error: _hex('#b91c1c'),
          semanticInfo: _hex('#2563eb'),
          semanticWarning: _hex('#ca8a04'),
          semanticSuccess: _hex('#16a34a'),
          semanticError: _hex('#b91c1c'),
        ),
        Brightness.dark: KwaaiThemeExtension(
          scaffoldBackground: _hex('#1c0a02'),
          appBarBackground: _hex('#2a1408'),
          appBarForeground: _hex('#fed7aa'),
          navRailBackground: _hex('#451a03'),
          divider: _hex('#7c2d12'),
          cardBackground: _hex('#2a1408'),
          elevatedSurface: _hex('#3a1d0c'),
          menuBackground: _hex('#4a2812'),
          inputBackground: _hex('#1a0a02'),
          accentPrimary: _hex('#fb923c'),
          statusRunning: _hex('#4ade80'),
          statusStopped: _hex('#a8a29e'),
          statusTransitioning: _hex('#fbbf24'),
          buttonDestructive: _hex('#f87171'),
          error: _hex('#f87171'),
          semanticInfo: _hex('#60a5fa'),
          semanticWarning: _hex('#fbbf24'),
          semanticSuccess: _hex('#86efac'),
          semanticError: _hex('#f87171'),
        ),
      },
    };

KwaaiThemeExtension resolveExtension(
  ThemeVariantKey variant,
  Brightness brightness,
) {
  return kwaaiVariants[variant]![brightness]!;
}

/// Rescales a Material [TextTheme] to macOS UI metrics: ~13pt for standard
/// body/label text, ~11pt for captions, with a tighter ~1.25 line height
/// (Material's default ~1.43 is the main reason text reads "non-native" and
/// rows feel tall). The macOS engine already renders SF Pro as the family —
/// only the size and line height need adjusting. Title styles are left at
/// Material's larger sizes so page/section titles stay prominent.
TextTheme _macOSTextTheme(TextTheme base) {
  const lineHeight = 1.25;
  return base.copyWith(
    bodyLarge: base.bodyLarge?.copyWith(fontSize: 13, height: lineHeight),
    bodyMedium: base.bodyMedium?.copyWith(fontSize: 13, height: lineHeight),
    bodySmall: base.bodySmall?.copyWith(fontSize: 11, height: lineHeight),
    labelLarge: base.labelLarge?.copyWith(fontSize: 13, height: lineHeight),
    labelMedium: base.labelMedium?.copyWith(fontSize: 12, height: lineHeight),
    labelSmall: base.labelSmall?.copyWith(fontSize: 11, height: lineHeight),
    titleSmall: base.titleSmall?.copyWith(fontSize: 13, height: lineHeight),
  );
}

/// Build a complete ThemeData from a variant + brightness.
ThemeData buildKwaaiTheme(ThemeVariantKey variant, Brightness brightness) {
  final ext = resolveExtension(variant, brightness);
  final base = ThemeData(
    useMaterial3: true,
    brightness: brightness,
    colorSchemeSeed: ext.accentPrimary,
  );
  final textTheme = _macOSTextTheme(base.textTheme);
  final bodyMediumStyle = textTheme.bodyMedium;
  return base.copyWith(
    extensions: [ext],
    textTheme: textTheme,
    scaffoldBackgroundColor: ext.scaffoldBackground,
    dividerColor: ext.divider,
    appBarTheme: AppBarTheme(
      backgroundColor: ext.appBarBackground,
      foregroundColor: ext.appBarForeground,
    ),
    navigationRailTheme: base.navigationRailTheme.copyWith(
      backgroundColor: ext.navRailBackground,
      selectedLabelTextStyle: bodyMediumStyle,
      unselectedLabelTextStyle: bodyMediumStyle,
      indicatorColor: Colors.transparent,
      useIndicator: false,
    ),
    listTileTheme: base.listTileTheme.copyWith(
      titleTextStyle: bodyMediumStyle,
      subtitleTextStyle: bodyMediumStyle?.copyWith(
        color: base.colorScheme.onSurfaceVariant,
      ),
    ),
    cardTheme: base.cardTheme.copyWith(color: ext.cardBackground),
    // Selected radios fill with the themed accent; unselected use the
    // muted outline colour.
    radioTheme: base.radioTheme.copyWith(
      fillColor: WidgetStateProperty.resolveWith(
        (states) => states.contains(WidgetState.selected)
            ? ext.accentPrimary
            : base.colorScheme.onSurfaceVariant,
      ),
    ),
    // Text selection: caret + handles use the themed accent; the selection
    // highlight band is a lighter, translucent tint of it.
    textSelectionTheme: TextSelectionThemeData(
      cursorColor: ext.accentPrimary,
      selectionHandleColor: ext.accentPrimary,
      selectionColor: ext.accentPrimary.withValues(alpha: 0.3),
    ),
  );
}
