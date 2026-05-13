import 'package:flutter/material.dart';

import 'branded_title.dart';
import 'daemon_controller.dart';
import 'settings.dart';
import 'settings_page.dart';
import 'status_watcher.dart';

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
      MaterialPageRoute(
        builder: (_) => SettingsPage(
          daemon: daemon,
          settings: settings,
          statusStream: statusStream,
          onSettingsChanged: () {},
        ),
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        leadingWidth: 220,
        leading: const Padding(
          padding: EdgeInsets.only(left: 16),
          child: BrandedTitle(),
        ),
        title: const SizedBox.shrink(),
        actions: [
          IconButton(
            tooltip: 'Settings',
            icon: const Icon(Icons.settings),
            onPressed: () => _openSettings(context),
          ),
        ],
      ),
      body: Column(
        children: [
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
    );
  }
}

class _ChatInputBar extends StatelessWidget {
  const _ChatInputBar();

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
        child: TextField(
          enabled: false,
          decoration: InputDecoration(
            hintText: 'Message kwaainet…',
            filled: true,
            fillColor: cs.surfaceContainerHighest,
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
