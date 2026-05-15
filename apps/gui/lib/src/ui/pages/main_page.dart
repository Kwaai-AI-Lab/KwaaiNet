import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../daemon/daemon_controller.dart';
import '../../daemon/daemon_state.dart';
import '../../settings.dart';
import '../../tray/tray.dart';
import '../theme/kwaai_theme.dart';
import '../widgets/app_shell.dart';
import '../widgets/branded_title.dart';
import 'settings_page.dart';

class MainPage extends StatelessWidget {
  const MainPage({
    super.key,
    required this.daemon,
    required this.settings,
    required this.tray,
  });

  final DaemonController daemon;
  final Settings settings;
  final TrayController tray;

  void _openSettings(BuildContext context) {
    Navigator.of(context).push(
      PageRouteBuilder(
        pageBuilder: (_, _, _) => SettingsPage(
          daemon: daemon,
          settings: settings,
          tray: tray,
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
            // Chat area swaps content based on whether the service is up;
            // the input bar stays visible but is disabled when it isn't.
            Expanded(
              child: _ChatBody(onOpenSettings: () => _openSettings(context)),
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

/// The chat surface. When the service is running it shows the chat (today
/// a placeholder); otherwise it shows the status message in the same spot,
/// no scrim/overlay — the chat just isn't there yet.
class _ChatBody extends ConsumerWidget {
  const _ChatBody({required this.onOpenSettings});

  final VoidCallback onOpenSettings;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final transition = ref.watch(daemonTransitionProvider);
    final status = ref.watch(daemonStatusProvider).valueOrNull;
    final running = status?.running ?? false;

    if (running && transition == DaemonTransition.none) {
      return Center(
        child: Text(
          'Chat here',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
      );
    }

    final ext = context.kwaai;
    final Color spinnerColor;
    final String headline;
    final bool spinner;
    final bool showStoppedSub;
    switch (transition) {
      case DaemonTransition.starting:
        spinnerColor = ext.statusTransitioning;
        headline = 'Starting service…';
        spinner = true;
        showStoppedSub = false;
      case DaemonTransition.stopping:
        spinnerColor = ext.statusTransitioning;
        headline = 'Stopping service…';
        spinner = true;
        showStoppedSub = false;
      case DaemonTransition.none:
        spinnerColor = ext.statusStopped;
        headline = 'Service is stopped';
        spinner = false;
        showStoppedSub = true;
    }

    final mutedStyle = Theme.of(context).textTheme.bodyMedium?.copyWith(
      color: Theme.of(context).colorScheme.onSurfaceVariant,
    );

    return Center(
      child: Column(
        mainAxisSize: MainAxisSize.min,
        children: [
          if (spinner) ...[
            SizedBox(
              width: 28,
              height: 28,
              child: CircularProgressIndicator(
                strokeWidth: 2.5,
                valueColor: AlwaysStoppedAnimation(spinnerColor),
              ),
            ),
            const SizedBox(height: 12),
          ],
          Text(headline, style: Theme.of(context).textTheme.titleMedium),
          if (showStoppedSub) ...[
            const SizedBox(height: 4),
            Text.rich(
              TextSpan(
                style: mutedStyle,
                children: [
                  const TextSpan(text: 'Open '),
                  WidgetSpan(
                    alignment: PlaceholderAlignment.baseline,
                    baseline: TextBaseline.alphabetic,
                    child: MouseRegion(
                      cursor: SystemMouseCursors.click,
                      child: GestureDetector(
                        onTap: onOpenSettings,
                        child: Text(
                          'settings',
                          style: mutedStyle?.copyWith(
                            color: ext.accentPrimary,
                            fontWeight: FontWeight.w600,
                          ),
                        ),
                      ),
                    ),
                  ),
                  const TextSpan(text: ' to start the service.'),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }
}

/// Chat input. Enabled only when the service is up. While stopped /
/// starting / stopping it stays visible but inert.
class _ChatInputBar extends ConsumerWidget {
  const _ChatInputBar();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final transition = ref.watch(daemonTransitionProvider);
    final status = ref.watch(daemonStatusProvider).valueOrNull;
    final enabled =
        (status?.running ?? false) && transition == DaemonTransition.none;

    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
        child: TextField(
          enabled: enabled,
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
              // Always null today (no chat logic yet); when send wiring
              // lands it should also respect `enabled`.
              onPressed: null,
            ),
          ),
        ),
      ),
    );
  }
}
