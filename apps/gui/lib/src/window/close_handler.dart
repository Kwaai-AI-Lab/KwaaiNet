import 'dart:io';

import 'package:window_manager/window_manager.dart';

import '../settings.dart';
import 'dock_icon.dart';

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
      _log('keepInTrayOnClose=true → hiding window + Dock icon');
      await windowManager.hide();
      // Also hide the Dock icon and Cmd-Tab entry. The tray icon alone
      // signals the app is running.
      await DockIcon.setVisible(false);
    } else {
      _log('keepInTrayOnClose=false → quitting');
      await windowManager.setPreventClose(false);
      await windowManager.destroy();
    }
  }
}
