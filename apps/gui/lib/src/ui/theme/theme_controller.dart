import 'dart:convert';

import 'package:flutter/material.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'theme_variants.dart';

enum AppThemeMode { auto, light, dark }

class ThemeState {
  const ThemeState({
    this.mode = AppThemeMode.auto,
    this.lightVariant = ThemeVariantKey.native,
    this.darkVariant = ThemeVariantKey.native,
  });

  final AppThemeMode mode;
  final ThemeVariantKey lightVariant;
  final ThemeVariantKey darkVariant;

  ThemeState copyWith({
    AppThemeMode? mode,
    ThemeVariantKey? lightVariant,
    ThemeVariantKey? darkVariant,
  }) {
    return ThemeState(
      mode: mode ?? this.mode,
      lightVariant: lightVariant ?? this.lightVariant,
      darkVariant: darkVariant ?? this.darkVariant,
    );
  }

  Brightness effectiveBrightness(Brightness systemBrightness) => switch (mode) {
    AppThemeMode.auto => systemBrightness,
    AppThemeMode.light => Brightness.light,
    AppThemeMode.dark => Brightness.dark,
  };

  ThemeVariantKey variantFor(Brightness b) =>
      b == Brightness.dark ? darkVariant : lightVariant;

  Map<String, dynamic> toJson() => {
    'mode': mode.name,
    'lightVariant': lightVariant.name,
    'darkVariant': darkVariant.name,
  };

  static ThemeState fromJson(Map<String, dynamic> j) {
    return ThemeState(
      mode: AppThemeMode.values.firstWhere(
        (m) => m.name == j['mode'],
        orElse: () => AppThemeMode.auto,
      ),
      lightVariant: ThemeVariantKey.values.firstWhere(
        (v) => v.name == j['lightVariant'],
        orElse: () => ThemeVariantKey.native,
      ),
      darkVariant: ThemeVariantKey.values.firstWhere(
        (v) => v.name == j['darkVariant'],
        orElse: () => ThemeVariantKey.native,
      ),
    );
  }
}

class ThemeController extends ChangeNotifier {
  ThemeController._(this._prefs, this._state);

  static const _key = 'theme';

  final SharedPreferences _prefs;
  ThemeState _state;

  ThemeState get state => _state;

  static Future<ThemeController> load() async {
    final prefs = await SharedPreferences.getInstance();
    final raw = prefs.getString(_key);
    var initial = const ThemeState();
    if (raw != null && raw.isNotEmpty) {
      try {
        initial = ThemeState.fromJson(json.decode(raw) as Map<String, dynamic>);
      } catch (_) {}
    }
    return ThemeController._(prefs, initial);
  }

  Future<void> _save() async {
    await _prefs.setString(_key, json.encode(_state.toJson()));
  }

  void setMode(AppThemeMode mode) {
    if (mode == _state.mode) return;
    _state = _state.copyWith(mode: mode);
    notifyListeners();
    _save();
  }

  void setLightVariant(ThemeVariantKey v) {
    if (v == _state.lightVariant) return;
    _state = _state.copyWith(lightVariant: v);
    notifyListeners();
    _save();
  }

  void setDarkVariant(ThemeVariantKey v) {
    if (v == _state.darkVariant) return;
    _state = _state.copyWith(darkVariant: v);
    notifyListeners();
    _save();
  }
}

/// Inherited access to the live theme controller.
class ThemeScope extends InheritedNotifier<ThemeController> {
  const ThemeScope({
    super.key,
    required ThemeController controller,
    required super.child,
  }) : super(notifier: controller);

  static ThemeController of(BuildContext context) {
    final scope = context.dependOnInheritedWidgetOfExactType<ThemeScope>();
    assert(scope != null, 'ThemeScope missing in widget tree');
    return scope!.notifier!;
  }
}
