import 'dart:io';

class KwaainetPaths {
  static String get home {
    final override = Platform.environment['KWAAINET_HOME'];
    if (override != null && override.isNotEmpty) return override;
    final base =
        Platform.environment['HOME'] ??
        Platform.environment['USERPROFILE'] ??
        '.';
    return '$base${Platform.pathSeparator}.kwaainet';
  }

  static String get runDir => '$home${Platform.pathSeparator}run';
  static String get pidFile => '$runDir${Platform.pathSeparator}kwaainet.pid';
  static String get statusFile =>
      '$runDir${Platform.pathSeparator}kwaainet.status';
  static String get configFile => '$home${Platform.pathSeparator}config.yaml';
  static String get logsDir => '$home${Platform.pathSeparator}logs';
}

String get builtInDebugDaemonPath {
  final exe = File(Platform.resolvedExecutable).absolute.path;
  final sep = Platform.pathSeparator;

  String? walk(String start) {
    var dir = Directory(start).parent;
    for (var i = 0; i < 16; i++) {
      final candidate =
          '${dir.path}${sep}core${sep}target${sep}debug${sep}kwaainet';
      if (File(candidate).existsSync()) return candidate;
      if (dir.parent.path == dir.path) break;
      dir = dir.parent;
    }
    return null;
  }

  final found = walk(exe);
  if (found != null) return found;

  final scriptDir = Directory.current.path;
  final guess =
      '$scriptDir$sep..$sep..${sep}core${sep}target${sep}debug${sep}kwaainet';
  return File(guess).existsSync() ? File(guess).absolute.path : guess;
}
