import 'dart:io';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'daemon_controller.dart';
import 'status_watcher.dart';

void _log(String msg) {
  stderr.writeln('[daemon-state] $msg');
}

/// Transient daemon lifecycle phase, tracked UI-side. The daemon's
/// [NodeStatus] only reports running/stopped — "starting"/"stopping" is the
/// gap between issuing a command and the watcher confirming the new state.
enum DaemonTransition { none, starting, stopping }

/// One-line error message surfaced when start/stop fails. Null when no error
/// is current.
typedef DaemonError = String?;

/// The single [DaemonController] instance. Provided by overriding this in the
/// [ProviderScope] at app start.
final daemonControllerProvider = Provider<DaemonController>((ref) {
  throw UnimplementedError(
    'daemonControllerProvider must be overridden with the app-wide '
    'DaemonController instance via ProviderScope.overrides.',
  );
});

/// The single [StatusWatcher] instance. Provided by overriding this in the
/// [ProviderScope] at app start.
final statusWatcherProvider = Provider<StatusWatcher>((ref) {
  throw UnimplementedError(
    'statusWatcherProvider must be overridden with the app-wide '
    'StatusWatcher instance via ProviderScope.overrides.',
  );
});

/// Live daemon status stream. Updates whenever the [StatusWatcher] emits.
final daemonStatusProvider = StreamProvider<NodeStatus>((ref) {
  return ref.watch(statusWatcherProvider).stream;
});

/// The current transition phase (none/starting/stopping). Exposes start()
/// and stop() that drive both the daemon and this provider's state, and
/// auto-clear once the [daemonStatusProvider] confirms the target.
class DaemonTransitionNotifier extends Notifier<DaemonTransition> {
  DaemonError _lastError;

  /// Latest error message from a failed start/stop, or null. Consumers can
  /// watch [daemonErrorProvider] to read this reactively.
  DaemonError get lastError => _lastError;

  @override
  DaemonTransition build() {
    // Subscribe to the status stream so we can auto-clear the transition
    // once the daemon reaches the expected state. ref.listen cleans itself
    // up when the provider is disposed.
    ref.listen<AsyncValue<NodeStatus>>(daemonStatusProvider, (previous, next) {
      next.whenData((status) {
        if (state == DaemonTransition.starting && status.running) {
          _log('starting → confirmed running, clearing transition');
          state = DaemonTransition.none;
        } else if (state == DaemonTransition.stopping && !status.running) {
          _log('stopping → confirmed stopped, clearing transition');
          state = DaemonTransition.none;
        }
      });
    });
    return DaemonTransition.none;
  }

  Future<void> start() async {
    _log('start() called');
    _lastError = null;
    ref.read(daemonErrorProvider.notifier).clear();
    state = DaemonTransition.starting;
    final r = await ref.read(daemonControllerProvider).start();
    if (!r.ok) {
      _log('start failed: ${r.error}');
      _lastError = r.error ?? 'start failed';
      ref.read(daemonErrorProvider.notifier).set(_lastError);
      state = DaemonTransition.none;
    }
  }

  Future<void> stop() async {
    _log('stop() called');
    _lastError = null;
    ref.read(daemonErrorProvider.notifier).clear();
    state = DaemonTransition.stopping;
    final ok = await ref.read(daemonControllerProvider).stop();
    if (!ok) {
      _log('stop failed');
      _lastError = 'stop failed';
      ref.read(daemonErrorProvider.notifier).set(_lastError);
      state = DaemonTransition.none;
    }
  }
}

final daemonTransitionProvider =
    NotifierProvider<DaemonTransitionNotifier, DaemonTransition>(
      DaemonTransitionNotifier.new,
    );

/// Latest start/stop error, or null. Cleared on the next action.
class DaemonErrorNotifier extends Notifier<DaemonError> {
  @override
  DaemonError build() => null;

  void set(DaemonError v) => state = v;
  void clear() => state = null;
}

final daemonErrorProvider = NotifierProvider<DaemonErrorNotifier, DaemonError>(
  DaemonErrorNotifier.new,
);
