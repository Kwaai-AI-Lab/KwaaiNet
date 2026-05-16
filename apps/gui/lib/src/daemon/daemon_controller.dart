import 'dart:convert';
import 'dart:io';

import '../settings.dart';
import 'paths.dart';

void _log(String msg) {
  stderr.writeln('[daemon-controller] $msg');
}

class DaemonResolution {
  DaemonResolution({
    required this.path,
    required this.exists,
    required this.source,
  });
  final String path;
  final bool exists;
  final String source;
}

class DaemonController {
  DaemonController(this._settings);

  final Settings _settings;

  DaemonResolution resolveBinary() {
    switch (_settings.mode) {
      case DaemonMode.builtIn:
        final p = builtInDebugDaemonPath;
        return DaemonResolution(
          path: p,
          exists: File(p).existsSync(),
          source: 'built-in (debug)',
        );
      case DaemonMode.system:
        final p = _whichKwaainet();
        return DaemonResolution(
          path: p ?? 'kwaainet',
          exists: p != null,
          source: 'system PATH',
        );
      case DaemonMode.custom:
        final p = _settings.customPath ?? '';
        return DaemonResolution(
          path: p,
          exists: p.isNotEmpty && File(p).existsSync(),
          source: 'custom path',
        );
      case DaemonMode.external:
        // The app neither launches nor manages the binary in this mode —
        // some external supervisor (launchd, systemd, Docker, manual
        // shell) runs it. resolveBinary() is still called by the
        // settings UI for display, so return a stable descriptor.
        return DaemonResolution(
          path: '',
          exists: false,
          source: 'externally managed',
        );
    }
  }

  String? findSystemBinary() => _whichKwaainet();

  String? _whichKwaainet() {
    final exeName = Platform.isWindows ? 'kwaainet.exe' : 'kwaainet';
    final paths = (Platform.environment['PATH'] ?? '').split(
      Platform.isWindows ? ';' : ':',
    );
    for (final dir in paths) {
      if (dir.isEmpty) continue;
      final candidate = '$dir${Platform.pathSeparator}$exeName';
      if (File(candidate).existsSync()) return candidate;
    }
    return null;
  }

  int? readPid() {
    final f = File(KwaainetPaths.pidFile);
    if (!f.existsSync()) return null;
    return int.tryParse(f.readAsStringSync().trim());
  }

  Future<bool> isAlive() async {
    final pid = readPid();
    if (pid == null) return false;
    return _processExists(pid);
  }

  Future<bool> _processExists(int pid) async {
    if (Platform.isWindows) {
      final r = await Process.run('tasklist', ['/FI', 'PID eq $pid', '/NH']);
      return r.stdout.toString().contains('$pid');
    }
    final r = await Process.run('kill', ['-0', '$pid']);
    return r.exitCode == 0;
  }

  Future<DaemonStartResult> start() async {
    _log('start() invoked');
    if (_settings.mode == DaemonMode.external) {
      _log('external mode — refusing to spawn (managed by user)');
      return DaemonStartResult.error(
        'Service is managed externally — the app will not start it.',
      );
    }
    if (await isAlive()) {
      final pid = readPid()!;
      _log('daemon already running (pid $pid) — attaching');
      return DaemonStartResult.alreadyRunning(pid);
    }
    final res = resolveBinary();
    _log('resolved daemon: ${res.path} (${res.source}, exists=${res.exists})');
    if (!res.exists) {
      _log('ABORT: binary not found');
      final where = res.path.isEmpty ? '(none)' : res.path;
      return DaemonStartResult.error(
        'Daemon binary not found at $where (${res.source})',
      );
    }
    Directory(KwaainetPaths.runDir).createSync(recursive: true);

    try {
      _log('spawning: ${res.path} start --daemon');
      // Suppress the daemon's in-process auto-updater. The GUI will own
      // notifying the user about new versions (forthcoming) — without
      // this the daemon silently swaps its own binary for the latest
      // upstream release ~5 minutes into a session and exits to
      // restart, which loses any locally-built feature work (e.g. a
      // newly-added rpc crate not yet in any release). See the
      // KWAAINET_NO_AUTO_UPDATE guard in kwaainet's node.rs.
      final env = Map<String, String>.from(Platform.environment)
        ..['KWAAINET_NO_AUTO_UPDATE'] = '1';
      final p = await Process.start(
        res.path,
        ['start', '--daemon'],
        environment: env,
      );
      _log('spawned pid ${p.pid} (piped — stdout/stderr will appear below)');

      p.stdout
          .transform(utf8.decoder)
          .transform(const LineSplitter())
          .listen((line) => stderr.writeln('[daemon] $line'));
      p.stderr
          .transform(utf8.decoder)
          .transform(const LineSplitter())
          .listen((line) => stderr.writeln('[daemon] $line'));
      p.exitCode.then((code) {
        _log('daemon process exited with code $code');
      });

      return DaemonStartResult.spawned(p.pid);
    } catch (e, st) {
      _log('spawn failed: $e\n$st');
      return DaemonStartResult.error('Failed to spawn daemon: $e');
    }
  }

  Future<bool> stop() async {
    _log('stop() invoked');
    if (_settings.mode == DaemonMode.external) {
      _log('external mode — refusing to stop (managed by user)');
      return false;
    }
    final pid = readPid();
    if (pid == null) {
      _log('no pid file — nothing to stop');
      return false;
    }
    _log('sending SIGTERM to pid $pid');
    if (Platform.isWindows) {
      final r = await Process.run('taskkill', ['/PID', '$pid']);
      _log('taskkill exit ${r.exitCode}');
      return r.exitCode == 0;
    }
    final r = await Process.run('kill', ['$pid']);
    _log('kill exit ${r.exitCode}');
    return r.exitCode == 0;
  }
}

class DaemonStartResult {
  DaemonStartResult._(this.kind, {this.pid, this.error});
  final String kind;
  final int? pid;
  final String? error;

  factory DaemonStartResult.spawned(int pid) =>
      DaemonStartResult._('spawned', pid: pid);
  factory DaemonStartResult.alreadyRunning(int pid) =>
      DaemonStartResult._('alreadyRunning', pid: pid);
  factory DaemonStartResult.error(String message) =>
      DaemonStartResult._('error', error: message);

  bool get ok => kind == 'spawned' || kind == 'alreadyRunning';
}
