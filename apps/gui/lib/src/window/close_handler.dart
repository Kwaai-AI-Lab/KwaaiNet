import 'dart:io';

import 'package:window_manager/window_manager.dart';

import '../settings.dart';

void _log(String msg) {
  stderr.writeln('[window-close] $msg');
}

/// Decides what happens when the user closes the window. Reads
/// [Settings.keepInTrayOnClose] live, so the choice can be toggled in
/// preferences without restarting the app.
class WindowCloseHandler with WindowListener {
  WindowCloseHandler(this._settings);

  final Settings _settings;

  Future<void> attach() async {
    await windowManager.setPreventClose(true);
    windowManager.addListener(this);
  }

  void dispose() {
    windowManager.removeListener(this);
  }

  @override
  void onWindowClose() async {
    if (_settings.keepInTrayOnClose) {
      _log('keepInTrayOnClose=true → hiding window, app stays in tray');
      await windowManager.hide();
    } else {
      _log('keepInTrayOnClose=false → quitting');
      await windowManager.setPreventClose(false);
      await windowManager.destroy();
    }
  }
}
