import 'package:flutter/services.dart' show rootBundle;
import 'package:path_provider/path_provider.dart';
import 'package:tray_manager/tray_manager.dart';
import 'package:window_manager/window_manager.dart';
import 'dart:io';

import '../daemon/daemon_controller.dart';
import '../daemon/status_watcher.dart';

enum TrayState { running, stopped, error }

class TrayController with TrayListener {
  TrayController({required this.daemon});

  final DaemonController daemon;
  TrayState _state = TrayState.stopped;
  String _statusText = 'Stopped';
  String? _runningIcon;
  String? _stoppedIcon;
  String? _errorIcon;

  Future<void> init() async {
    _runningIcon = await _materializeIcon(
      'assets/tray_running.png',
      'tray_running.png',
    );
    _stoppedIcon = await _materializeIcon(
      'assets/tray_stopped.png',
      'tray_stopped.png',
    );
    _errorIcon = await _materializeIcon(
      'assets/tray_error.png',
      'tray_error.png',
    );
    await trayManager.setIcon(_stoppedIcon!);
    await trayManager.setToolTip('kwaainet — stopped');
    trayManager.addListener(this);
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

  Future<void> updateFromStatus(NodeStatus s) async {
    final newState = s.running ? TrayState.running : TrayState.stopped;
    final newText = s.running ? 'Running (pid ${s.pid ?? '?'})' : 'Stopped';
    if (newState == _state && newText == _statusText) return;
    _state = newState;
    _statusText = newText;
    final icon = switch (_state) {
      TrayState.running => _runningIcon,
      TrayState.stopped => _stoppedIcon,
      TrayState.error => _errorIcon,
    };
    if (icon != null) await trayManager.setIcon(icon);
    await trayManager.setToolTip('kwaainet — ${_statusText.toLowerCase()}');
    await _rebuildMenu();
  }

  Future<void> showError(String message) async {
    _state = TrayState.error;
    _statusText = message;
    if (_errorIcon != null) await trayManager.setIcon(_errorIcon!);
    await trayManager.setToolTip('kwaainet — error: $message');
    await _rebuildMenu();
  }

  Future<void> _rebuildMenu() async {
    final running = _state == TrayState.running;
    await trayManager.setContextMenu(
      Menu(
        items: [
          MenuItem(label: _statusText, disabled: true),
          MenuItem.separator(),
          MenuItem(key: 'start', label: 'Start service', disabled: running),
          MenuItem(key: 'stop', label: 'Stop service', disabled: !running),
          MenuItem.separator(),
          MenuItem(key: 'show', label: 'Open kwaainet-gui…'),
          MenuItem.separator(),
          MenuItem(key: 'quit', label: 'Quit kwaainet-gui'),
        ],
      ),
    );
  }

  @override
  void onTrayIconMouseDown() {
    trayManager.popUpContextMenu();
  }

  @override
  void onTrayIconRightMouseDown() {
    trayManager.popUpContextMenu();
  }

  @override
  void onTrayMenuItemClick(MenuItem menuItem) async {
    switch (menuItem.key) {
      case 'start':
        final r = await daemon.start();
        if (!r.ok) await showError(r.error ?? 'start failed');
        break;
      case 'stop':
        await daemon.stop();
        break;
      case 'show':
        await windowManager.show();
        await windowManager.focus();
        break;
      case 'quit':
        await dispose();
        await windowManager.destroy();
        exit(0);
    }
  }

  Future<void> dispose() async {
    trayManager.removeListener(this);
    await trayManager.destroy();
  }
}
