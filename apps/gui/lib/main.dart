import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:macos_window_utils/macos_window_utils.dart';
import 'package:window_manager/window_manager.dart';

import 'src/daemon/daemon_controller.dart';
import 'src/daemon/status_watcher.dart';
import 'src/settings.dart';
import 'src/tray/tray.dart';
import 'src/ui/pages/main_page.dart';
import 'src/ui/theme/theme_controller.dart';
import 'src/ui/theme/theme_variants.dart';
import 'src/window/window_focus.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await windowManager.ensureInitialized();
  await windowManager.setTitle('Kwaai AI');
  await windowManager.setMinimumSize(const Size(560, 400));

  if (!kIsWeb && Platform.isMacOS) {
    await WindowManipulator.initialize();
    await WindowManipulator.makeTitlebarTransparent();
    await WindowManipulator.enableFullSizeContentView();
    await WindowManipulator.addToolbar();
    await WindowManipulator.setToolbarStyle(
      toolbarStyle: NSWindowToolbarStyle.unified,
    );
    await WindowManipulator.hideTitle();
  }

  final settings = await Settings.load();
  final theme = await ThemeController.load();
  final daemon = DaemonController(settings);
  final watcher = StatusWatcher(daemon: daemon)..start();
  final tray = TrayController(daemon: daemon);
  await tray.init();
  watcher.stream.listen(tray.updateFromStatus);
  final windowFocus = WindowFocusNotifier()..attach();

  runApp(
    KwaainetGuiApp(
      daemon: daemon,
      settings: settings,
      watcher: watcher,
      theme: theme,
      windowFocus: windowFocus,
    ),
  );
}

class KwaainetGuiApp extends StatelessWidget {
  const KwaainetGuiApp({
    super.key,
    required this.daemon,
    required this.settings,
    required this.watcher,
    required this.theme,
    required this.windowFocus,
  });

  final DaemonController daemon;
  final Settings settings;
  final StatusWatcher watcher;
  final ThemeController theme;
  final WindowFocusNotifier windowFocus;

  @override
  Widget build(BuildContext context) {
    return WindowFocusScope(
      notifier: windowFocus,
      child: ThemeScope(
        controller: theme,
        child: AnimatedBuilder(
          animation: theme,
          builder: (context, _) {
            final state = theme.state;
            final lightTheme = buildKwaaiTheme(
              state.lightVariant,
              Brightness.light,
            );
            final darkTheme = buildKwaaiTheme(
              state.darkVariant,
              Brightness.dark,
            );
            final themeMode = switch (state.mode) {
              AppThemeMode.auto => ThemeMode.system,
              AppThemeMode.light => ThemeMode.light,
              AppThemeMode.dark => ThemeMode.dark,
            };
            return MaterialApp(
              title: 'Kwaai AI',
              theme: lightTheme,
              darkTheme: darkTheme,
              themeMode: themeMode,
              home: MainPage(
                daemon: daemon,
                settings: settings,
                statusStream: watcher.stream,
              ),
            );
          },
        ),
      ),
    );
  }
}
