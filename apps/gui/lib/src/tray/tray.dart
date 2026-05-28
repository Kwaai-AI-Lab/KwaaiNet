import 'dart:io';

import 'package:flutter/services.dart' show rootBundle;
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:path_provider/path_provider.dart';
import 'package:tray_manager/tray_manager.dart';
import 'package:window_manager/window_manager.dart';

import '../daemon/daemon_state.dart';
import '../daemon/status_watcher.dart';
import '../window/dock_icon.dart';

enum TrayState { running, stopped, starting, stopping, error }

extension on TrayState {
  /// Coloured status emoji used in the menu's status row. macOS menu items
  /// don't support per-character colour, so the emoji is the only way to
  /// get a real coloured indicator inline.
  String get emoji => switch (this) {
    TrayState.running => '🟢',
    TrayState.stopped => '🔴',
    TrayState.starting => '🟠',
    TrayState.stopping => '🟠',
    TrayState.error => '🟠',
  };
}

/// Drives the macOS menu-bar tray icon + menu. Reads daemon status and
/// transition state from Riverpod providers, and dispatches start/stop
/// actions through them so the main UI sees the same Starting…/Stopping…
/// state as the tray.
class TrayController with TrayListener {
  TrayController({required ProviderContainer container})
    : _container = container;

  final ProviderContainer _container;
  TrayState _state = TrayState.stopped;
  String _statusText = 'Stopped';
  String? _iconPath;
  bool _enabled = false;
  ProviderSubscription<AsyncValue<NodeStatus>>? _statusSub;
  ProviderSubscription<DaemonTransition>? _transitionSub;

  /// True when the tray icon is currently installed in the menu bar.
  bool get enabled => _enabled;

  /// Install or remove the tray icon. Use this to honour the
  /// `keepInTrayOnClose` preference at runtime.
  Future<void> setEnabled(bool on) async {
    if (on == _enabled) return;
    if (on) {
      await init();
    } else {
      await dispose();
    }
  }

  Future<void> init() async {
    if (_enabled) return;
    // Monochrome logo. macOS menu-bar treats this as a template image and
    // auto-inverts for dark mode. State is communicated via the menu's
    // coloured emoji, not by swapping the tray icon.
    _iconPath = await _materializeIcon(
      'assets/tray_kwaai_logo@2x.png',
      'tray_kwaai_logo.png',
    );
    await trayManager.setIcon(_iconPath!, isTemplate: true);
    await trayManager.setToolTip('Kwaai AI — ${_statusText.toLowerCase()}');
    trayManager.addListener(this);
    // Flip _enabled before _rebuildMenu() — it guards on !_enabled and
    // would otherwise no-op during the initial setup.
    _enabled = true;

    // Subscribe to status + transition providers. Whenever either changes,
    // recompute the tray state and rebuild the menu.
    _statusSub = _container.listen<AsyncValue<NodeStatus>>(
      daemonStatusProvider,
      (_, _) => _refresh(),
      fireImmediately: true,
    );
    _transitionSub = _container.listen<DaemonTransition>(
      daemonTransitionProvider,
      (_, _) => _refresh(),
      fireImmediately: true,
    );

    await _rebuildMenu();
  }

  Future<String> _materializeIcon(String asset, String fileName) async {
    final bytes = await rootBundle.load(asset);
    final dir = await getTemporaryDirectory();
    await dir.create(recursive: true);
    final path = '${dir.path}${Platform.pathSeparator}$fileName';
    final f = File(path);
    await f.writeAsBytes(bytes.buffer.asUint8List());
    return path;
  }

  /// Recomputes [_state] / [_statusText] from current provider values and
  /// refreshes the menu + tooltip.
  Future<void> _refresh() async {
    final transition = _container.read(daemonTransitionProvider);
    final status = _container.read(daemonStatusProvider).valueOrNull;

    final TrayState newState;
    final String newText;
    switch (transition) {
      case DaemonTransition.starting:
        newState = TrayState.starting;
        newText = 'Starting…';
      case DaemonTransition.stopping:
        newState = TrayState.stopping;
        newText = 'Stopping…';
      case DaemonTransition.none:
        if (status != null && status.running) {
          newState = TrayState.running;
          newText = 'Running (pid ${status.pid ?? '?'})';
        } else {
          newState = TrayState.stopped;
          newText = 'Stopped';
        }
    }

    if (newState == _state && newText == _statusText) return;
    _state = newState;
    _statusText = newText;
    if (!_enabled) return;
    await trayManager.setToolTip('Kwaai AI — ${_statusText.toLowerCase()}');
    await _rebuildMenu();
  }

  Future<void> _rebuildMenu() async {
    if (!_enabled) return;
    final running = _state == TrayState.running;
    final transitioning =
        _state == TrayState.starting || _state == TrayState.stopping;
    await trayManager.setContextMenu(
      Menu(
        items: [
          MenuItem(label: '${_state.emoji}  $_statusText', disabled: true),
          MenuItem.separator(),
          MenuItem(
            key: 'start',
            label: 'Start service',
            disabled: running || transitioning,
          ),
          MenuItem(
            key: 'stop',
            label: 'Stop service',
            disabled: !running || transitioning,
          ),
          MenuItem.separator(),
          MenuItem(key: 'show', label: 'Open Kwaai AI…'),
          MenuItem.separator(),
          MenuItem(key: 'quit', label: 'Quit Kwaai AI'),
        ],
      ),
    );
  }

  void _log(String msg) {
    stderr.writeln('[tray] $msg');
  }

  @override
  void onTrayIconMouseDown() {
    _log('icon mouseDown → popping menu');
    trayManager.popUpContextMenu();
  }

  @override
  void onTrayIconRightMouseDown() {
    _log('icon rightMouseDown → popping menu');
    trayManager.popUpContextMenu();
  }

  @override
  void onTrayMenuItemClick(MenuItem menuItem) async {
    _log('menu click: key=${menuItem.key}');
    switch (menuItem.key) {
      case 'start':
        // Goes through the same provider action the settings page uses, so
        // the main UI reflects Starting… immediately.
        await _container.read(daemonTransitionProvider.notifier).start();
        break;
      case 'stop':
        await _container.read(daemonTransitionProvider.notifier).stop();
        break;
      case 'show':
        // The window may be hidden to tray and the Dock icon may be hidden
        // too (activation policy = .accessory). Restore both.
        try {
          await DockIcon.setVisible(true);
          await windowManager.setSkipTaskbar(false);
          await windowManager.show();
          await windowManager.focus();
          _log('window restored');
        } catch (e, st) {
          _log('show window failed: $e\n$st');
        }
        break;
      case 'quit':
        _log('quit → tearing down');
        await dispose();
        // Bypass the close-handler's prevent-close so Quit actually quits.
        try {
          await windowManager.setPreventClose(false);
          await windowManager.destroy();
        } catch (e) {
          _log('quit teardown error (proceeding to exit): $e');
        }
        exit(0);
    }
  }

  Future<void> dispose() async {
    if (!_enabled) return;
    _statusSub?.close();
    _transitionSub?.close();
    _statusSub = null;
    _transitionSub = null;
    trayManager.removeListener(this);
    await trayManager.destroy();
    _enabled = false;
  }
}
