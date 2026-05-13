import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

import 'src/daemon_controller.dart';
import 'src/main_page.dart';
import 'src/settings.dart';
import 'src/status_watcher.dart';
import 'src/tray.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();
  await windowManager.setTitle('Kwaai AI');
  await windowManager.setMinimumSize(const Size(560, 400));

  final settings = await Settings.load();
  final daemon = DaemonController(settings);
  final watcher = StatusWatcher(daemon: daemon)..start();
  final tray = TrayController(daemon: daemon);
  await tray.init();
  watcher.stream.listen(tray.updateFromStatus);

  runApp(KwaainetGuiApp(daemon: daemon, settings: settings, watcher: watcher));
}

ThemeData _buildTheme() {
  final base = ThemeData(
    colorScheme: ColorScheme.fromSeed(seedColor: Colors.indigo),
    useMaterial3: true,
  );
  final bodyMediumStyle = base.textTheme.bodyMedium;
  return base.copyWith(
    navigationRailTheme: base.navigationRailTheme.copyWith(
      selectedLabelTextStyle: bodyMediumStyle,
      unselectedLabelTextStyle: bodyMediumStyle,
      indicatorColor: Colors.transparent,
      useIndicator: false,
    ),
    listTileTheme: base.listTileTheme.copyWith(
      titleTextStyle: bodyMediumStyle,
      subtitleTextStyle: bodyMediumStyle?.copyWith(
        color: base.colorScheme.onSurfaceVariant,
      ),
    ),
  );
}

class KwaainetGuiApp extends StatelessWidget {
  const KwaainetGuiApp({
    super.key,
    required this.daemon,
    required this.settings,
    required this.watcher,
  });

  final DaemonController daemon;
  final Settings settings;
  final StatusWatcher watcher;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'kwaainet-gui',
      theme: _buildTheme(),
      home: MainPage(
        daemon: daemon,
        settings: settings,
        statusStream: watcher.stream,
      ),
    );
  }
}
