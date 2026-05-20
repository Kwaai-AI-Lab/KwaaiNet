import 'package:flutter_test/flutter_test.dart';

import 'package:kwaainet_gui/src/daemon/status_watcher.dart';

void main() {
  test('NodeStatus parses daemon JSON', () {
    final s = NodeStatus.fromJson({
      'running': true,
      'pid': 1234,
      'uptime_secs': 90,
      'cpu_percent': 5.5,
      'memory_mb': 128.0,
      'connections': 7,
    });
    expect(s, isNotNull);
    expect(s!.running, true);
    expect(s.pid, 1234);
    expect(s.uptimeSecs, 90);
    expect(s.connections, 7);
  });

  test('NodeStatus.stopped() is not running', () {
    final s = NodeStatus.stopped();
    expect(s.running, false);
    expect(s.pid, isNull);
  });
}
