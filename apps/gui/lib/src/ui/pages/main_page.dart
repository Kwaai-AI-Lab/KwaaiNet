import 'package:flutter/material.dart';

import '../../daemon/daemon_controller.dart';
import '../../daemon/status_watcher.dart';
import '../../settings.dart';
import '../theme/kwaai_theme.dart';
import '../widgets/app_shell.dart';
import '../widgets/branded_title.dart';
import 'settings_page.dart';

class MainPage extends StatelessWidget {
  const MainPage({
    super.key,
    required this.daemon,
    required this.settings,
    required this.statusStream,
  });

  final DaemonController daemon;
  final Settings settings;
  final Stream<NodeStatus> statusStream;

  void _openSettings(BuildContext context) {
    Navigator.of(context).push(
      PageRouteBuilder(
        pageBuilder: (_, _, _) => SettingsPage(
          daemon: daemon,
          settings: settings,
          statusStream: statusStream,
          onSettingsChanged: () {},
        ),
        transitionDuration: Duration.zero,
        reverseTransitionDuration: Duration.zero,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return AppShell(
      child: ShellCard(
        // Single card — every corner touches the window edge.
        borderRadius: shellRadius(
          topLeft: true,
          topRight: true,
          bottomLeft: true,
          bottomRight: true,
        ),
        child: Column(
          children: [
            _MainTopBar(onOpenSettings: () => _openSettings(context)),
            Expanded(
              child: Center(
                child: Text(
                  'Chat here',
                  style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                    color: Theme.of(context).colorScheme.onSurfaceVariant,
                  ),
                ),
              ),
            ),
            const _ChatInputBar(),
          ],
        ),
      ),
    );
  }
}

class _MainTopBar extends StatelessWidget {
  const _MainTopBar({required this.onOpenSettings});

  final VoidCallback onOpenSettings;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 52,
      child: Stack(
        children: [
          // Left padding clears the native macOS traffic lights, which
          // overlap the top-left of this card. Top-aligned so the brand
          // lines up with the traffic lights rather than the bar's centre.
          const Padding(
            padding: EdgeInsets.only(left: 80, right: 16, top: 9),
            child: Align(alignment: Alignment.topLeft, child: BrandedTitle()),
          ),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8),
            child: Align(
              alignment: Alignment.topRight,
              child: IconButton(
                tooltip: 'Settings',
                icon: const Icon(Icons.settings),
                onPressed: onOpenSettings,
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _ChatInputBar extends StatelessWidget {
  const _ChatInputBar();

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
        child: TextField(
          enabled: false,
          decoration: InputDecoration(
            hintText: 'Message kwaainet…',
            filled: true,
            fillColor: context.kwaai.inputBackground,
            contentPadding: const EdgeInsets.symmetric(
              horizontal: 16,
              vertical: 12,
            ),
            border: OutlineInputBorder(
              borderRadius: BorderRadius.circular(24),
              borderSide: BorderSide.none,
            ),
            suffixIcon: IconButton(
              icon: const Icon(Icons.arrow_forward),
              tooltip: 'Send',
              onPressed: null,
            ),
          ),
        ),
      ),
    );
  }
}
