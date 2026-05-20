import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:shared_preferences/shared_preferences.dart';

enum DaemonMode { builtIn, system, custom, external }

DaemonMode _parse(String? s) => switch (s) {
  'system' => DaemonMode.system,
  'custom' => DaemonMode.custom,
  'external' => DaemonMode.external,
  _ => DaemonMode.builtIn,
};

String _serialize(DaemonMode m) => switch (m) {
  DaemonMode.builtIn => 'builtin',
  DaemonMode.system => 'system',
  DaemonMode.custom => 'custom',
  DaemonMode.external => 'external',
};

class Settings {
  Settings._(this._prefs);

  static const _modeKey = 'daemon.mode';
  static const _pathKey = 'daemon.customPath';
  static const _keepInTrayKey = 'window.keepInTrayOnClose';
  static const _startOnStartupKey = 'daemon.startOnStartup';
  static const _localChatKey = 'dev.localChatEnabled';

  final SharedPreferences _prefs;

  static Future<Settings> load() async {
    return Settings._(await SharedPreferences.getInstance());
  }

  DaemonMode get mode => _parse(_prefs.getString(_modeKey));
  String? get customPath => _prefs.getString(_pathKey);

  /// When true, closing the window hides it to the menu-bar tray and the
  /// app keeps running. When false, closing the window quits the app.
  /// Defaults to true (Slack/Discord/Docker Desktop convention).
  bool get keepInTrayOnClose => _prefs.getBool(_keepInTrayKey) ?? true;

  /// When true, the app starts the kwaainet service at boot if it's not
  /// already running. Defaults to true.
  bool get startServiceOnStartup => _prefs.getBool(_startOnStartupKey) ?? true;

  /// Developer preference — when true, the main page shows a second
  /// tab "Local chat" that drives `kwaainet generate` (single-node
  /// local inference). Off by default; meant for development /
  /// fallback when you want to bypass the shard mesh.
  bool get localChatEnabled => _prefs.getBool(_localChatKey) ?? false;

  Future<void> setMode(DaemonMode m) async {
    await _prefs.setString(_modeKey, _serialize(m));
  }

  Future<void> setCustomPath(String? p) async {
    if (p == null || p.isEmpty) {
      await _prefs.remove(_pathKey);
    } else {
      await _prefs.setString(_pathKey, p);
    }
  }

  Future<void> setKeepInTrayOnClose(bool v) async {
    await _prefs.setBool(_keepInTrayKey, v);
  }

  Future<void> setStartServiceOnStartup(bool v) async {
    await _prefs.setBool(_startOnStartupKey, v);
  }

  Future<void> setLocalChatEnabled(bool v) async {
    await _prefs.setBool(_localChatKey, v);
  }
}

/// Riverpod-visible mirror of [Settings.localChatEnabled]. Widgets that
/// need to react to the toggle (e.g. the main page's tab bar) watch
/// this provider rather than re-reading the prefs object. Initial
/// value is seeded by main.dart at startup; the Settings UI both
/// writes through to Settings.setLocalChatEnabled() and updates this
/// provider so subscribers see the change immediately.
final localChatEnabledProvider = StateProvider<bool>((_) => false);
