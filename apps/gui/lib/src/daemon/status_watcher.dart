import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'daemon_controller.dart';
import 'paths.dart';

void _log(String msg) {
  stderr.writeln('[status-watcher] $msg');
}

class NodeStatus {
  NodeStatus({
    required this.running,
    this.pid,
    this.uptimeSecs,
    this.cpuPercent,
    this.memoryMb,
    this.memoryPercent,
    this.connections,
    this.threads,
    this.startedAt,
    this.source = 'pid',
  });

  final bool running;
  final int? pid;
  final int? uptimeSecs;
  final double? cpuPercent;
  final double? memoryMb;
  final double? memoryPercent;
  final int? connections;
  final int? threads;
  final int? startedAt;

  /// Where the data came from: 'pid' (live PID probe only), 'status'
  /// (kwaainet.status JSON), or 'none' (nothing running).
  final String source;

  static NodeStatus stopped() => NodeStatus(running: false, source: 'none');

  static NodeStatus? fromJson(Map<String, dynamic> j) {
    try {
      return NodeStatus(
        running: j['running'] as bool? ?? false,
        pid: (j['pid'] as num?)?.toInt(),
        uptimeSecs: (j['uptime_secs'] as num?)?.toInt(),
        cpuPercent: (j['cpu_percent'] as num?)?.toDouble(),
        memoryMb: (j['memory_mb'] as num?)?.toDouble(),
        memoryPercent: (j['memory_percent'] as num?)?.toDouble(),
        connections: (j['connections'] as num?)?.toInt(),
        threads: (j['threads'] as num?)?.toInt(),
        startedAt: (j['started_at'] as num?)?.toInt(),
        source: 'status',
      );
    } catch (_) {
      return null;
    }
  }
}

class StatusWatcher {
  StatusWatcher({
    required this.daemon,
    this.interval = const Duration(seconds: 2),
  });

  final DaemonController daemon;
  final Duration interval;
  final _controller = StreamController<NodeStatus>.broadcast();
  Timer? _timer;
  bool _lastRunning = false;

  Stream<NodeStatus> get stream => _controller.stream;

  void start() {
    if (_timer != null) return;
    _log('starting (polling every ${interval.inSeconds}s)');
    _poll();
    _timer = Timer.periodic(interval, (_) => _poll());
  }

  void stop() {
    _timer?.cancel();
    _timer = null;
  }

  Future<void> dispose() async {
    stop();
    await _controller.close();
  }

  Future<void> _poll() async {
    final pid = daemon.readPid();
    final alive = pid != null && await daemon.isAlive();

    if (!alive) {
      if (_lastRunning) {
        _log('daemon no longer running (was pid ${pid ?? '?'})');
        _lastRunning = false;
      }
      _controller.add(NodeStatus.stopped());
      return;
    }

    final fromFile = _readStatusFile();
    if (fromFile != null && fromFile.running) {
      if (!_lastRunning) {
        _log(
          'daemon running (pid ${fromFile.pid ?? pid}) — status JSON present',
        );
        _lastRunning = true;
      }
      _controller.add(fromFile);
      return;
    }

    final fallback = NodeStatus(running: true, pid: pid, source: 'pid');
    if (!_lastRunning) {
      _log('daemon running (pid $pid) — no status JSON yet, using PID-only');
      _lastRunning = true;
    }
    _controller.add(fallback);
  }

  NodeStatus? _readStatusFile() {
    final f = File(KwaainetPaths.statusFile);
    if (!f.existsSync()) return null;
    try {
      final text = f.readAsStringSync();
      if (text.trim().isEmpty) return null;
      return NodeStatus.fromJson(json.decode(text) as Map<String, dynamic>);
    } catch (e) {
      _log('failed to parse status file: $e');
      return null;
    }
  }
}
