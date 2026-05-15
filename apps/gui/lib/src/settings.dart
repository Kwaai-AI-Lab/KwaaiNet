import 'package:shared_preferences/shared_preferences.dart';

enum DaemonMode { builtIn, system, custom }

DaemonMode _parse(String? s) => switch (s) {
  'system' => DaemonMode.system,
  'custom' => DaemonMode.custom,
  _ => DaemonMode.builtIn,
};

String _serialize(DaemonMode m) => switch (m) {
  DaemonMode.builtIn => 'builtin',
  DaemonMode.system => 'system',
  DaemonMode.custom => 'custom',
};

class Settings {
  Settings._(this._prefs);

  static const _modeKey = 'daemon.mode';
  static const _pathKey = 'daemon.customPath';
  static const _keepInTrayKey = 'window.keepInTrayOnClose';
  static const _startOnStartupKey = 'daemon.startOnStartup';

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
}
