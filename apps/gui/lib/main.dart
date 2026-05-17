import 'dart:async';
import 'dart:io';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:macos_window_utils/macos_window_utils.dart';
import 'package:window_manager/window_manager.dart';

import 'src/chat/kwaai_rpc_client.dart';
import 'src/daemon/daemon_controller.dart';
import 'src/daemon/daemon_state.dart';
import 'src/daemon/status_watcher.dart';
import 'src/settings.dart';
import 'src/tray/tray.dart';
import 'src/ui/pages/main_page.dart';
import 'src/ui/theme/theme_controller.dart';
import 'src/ui/theme/theme_variants.dart';
import 'src/window/close_handler.dart';
import 'src/window/dock_icon.dart';
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

  // Riverpod container — created here so the tray (non-widget) can read /
  // invoke provider actions, and shared with the widget tree via
  // UncontrolledProviderScope.
  final container = ProviderContainer(
    overrides: [
      daemonControllerProvider.overrideWithValue(daemon),
      statusWatcherProvider.overrideWithValue(watcher),
      // Seed the localChatEnabled provider from the durable setting so
      // the main page's tab bar reflects the user's last choice on
      // first paint.
      localChatEnabledProvider.overrideWith(
        (_) => settings.localChatEnabled,
      ),
    ],
  );

  // Gate the gRPC client's connection probe on the daemon status.
  // No point sending Ping every 3 s when we already know the daemon
  // isn't running — that just floods the logs with connect-refused.
  // Flip back on as soon as the daemon comes up.
  //
  // IMPORTANT: only act on a confirmed running/not-running reading,
  // never on AsyncValue.loading (which flickers between status polls).
  // Edge-trigger via lastKnown so flapping doesn't tear down and
  // rebuild the channel on every poll.
  bool? lastKnownRunning;
  container.listen<AsyncValue<NodeStatus>>(
    daemonStatusProvider,
    (_, next) {
      final v = next.valueOrNull;
      if (v == null) return; // not a confirmed reading yet — leave probe as-is
      final running = v.running;
      if (running == lastKnownRunning) return;
      lastKnownRunning = running;
      container.read(kwaaiRpcClientProvider).setProbingEnabled(running);
    },
    fireImmediately: true,
  );

  final tray = TrayController(container: container);
  // Only install the menu-bar icon when the user opts in. The toggle in
  // Settings → Status flips it at runtime via tray.setEnabled().
  if (settings.keepInTrayOnClose) {
    await tray.init();
  }
  final windowFocus = WindowFocusNotifier()..attach();
  await WindowCloseHandler(settings).attach();
  // Listen for Dock-icon re-clicks / Finder reopens while the app is
  // hidden — brings the window + Dock icon back.
  installReopenHandler();

  // Auto-start the service at boot if the user has it enabled and the
  // daemon isn't already running. Goes through the same transition
  // provider as the buttons/tray, so the main UI immediately shows
  // "Starting…" and the overlay engages until the watcher confirms.
  if (settings.startServiceOnStartup &&
      settings.mode != DaemonMode.external &&
      !(await daemon.isAlive())) {
    // Fire-and-forget — start() awaits the controller call but we don't
    // want to block the app from coming up while the daemon spins up.
    unawaited(container.read(daemonTransitionProvider.notifier).start());
  }

  runApp(
    UncontrolledProviderScope(
      container: container,
      child: KwaainetGuiApp(
        daemon: daemon,
        settings: settings,
        watcher: watcher,
        theme: theme,
        windowFocus: windowFocus,
        tray: tray,
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
    required this.theme,
    required this.windowFocus,
    required this.tray,
  });

  final DaemonController daemon;
  final Settings settings;
  final StatusWatcher watcher;
  final ThemeController theme;
  final WindowFocusNotifier windowFocus;
  final TrayController tray;

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
              home: MainPage(daemon: daemon, settings: settings, tray: tray),
            );
          },
        ),
      ),
    );
  }
}
