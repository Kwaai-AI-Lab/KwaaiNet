import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../chat/chat_state.dart';
import '../../chat/kwaai_rpc_client.dart';
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
            padding: EdgeInsets.only(left: 80, right: 16, top: 11),
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

/// The chat surface. Gated on whether we have a live gRPC connection
/// to the daemon — not whether the daemon's PID is alive. The two
/// can diverge (the daemon may be starting up, restarting, or have
/// its socket gone stale), and what actually matters for the chat
/// UI is whether a Chat call would succeed right now.
class _ChatBody extends ConsumerWidget {
  const _ChatBody({required this.onOpenSettings});

  final VoidCallback onOpenSettings;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // The chat surface only renders when the daemon is actually
    // reachable via gRPC — but the *status copy* below reflects the
    // daemon lifecycle (starting / running / stopping / stopped), so
    // the user gets useful detail rather than a generic "connecting".
    final conn = ref.watch(kwaaiRpcConnectionProvider).valueOrNull
        ?? RpcConnection.connecting;
    if (conn == RpcConnection.connected) {
      return const _ChatTranscript();
    }

    final ext = context.kwaai;
    final transition = ref.watch(daemonTransitionProvider);
    final status = ref.watch(daemonStatusProvider).valueOrNull;

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
        if (status == null) {
          // Haven't received the first status reading yet — distinguish
          // this muted "Checking…" from a confident "Service is stopped"
          // that would flicker the moment the probe lands.
          spinnerColor = Theme.of(context)
              .colorScheme
              .onSurfaceVariant
              .withValues(alpha: 0.4);
          headline = 'Checking service…';
          spinner = true;
          showStoppedSub = false;
        } else if (status.running) {
          // PID is alive but the RPC channel isn't ready yet — either
          // the daemon is mid-init (DHT bootstrap / inference setup
          // before the gRPC listener binds) or the socket was just
          // recreated and we're about to reconnect.
          spinnerColor = ext.statusTransitioning;
          headline = 'Connecting to service…';
          spinner = true;
          showStoppedSub = false;
        } else {
          spinnerColor = ext.statusStopped;
          headline = 'Service is stopped';
          spinner = false;
          showStoppedSub = true;
        }
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
                  const TextSpan(text: ' to start or configure the service.'),
                ],
              ),
            ),
          ],
        ],
      ),
    );
  }
}

/// Chat input. Enabled only when the service is up AND there's no
/// in-flight stream. While stopped / starting / stopping it stays
/// visible but inert.
class _ChatInputBar extends ConsumerStatefulWidget {
  const _ChatInputBar();

  @override
  ConsumerState<_ChatInputBar> createState() => _ChatInputBarState();
}

class _ChatInputBarState extends ConsumerState<_ChatInputBar> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();

  @override
  void dispose() {
    _controller.dispose();
    _focusNode.dispose();
    super.dispose();
  }

  void _send() {
    final text = _controller.text.trim();
    if (text.isEmpty) return;
    _controller.clear();
    ref.read(chatTranscriptProvider.notifier).send(text);
    _focusNode.requestFocus();
  }

  @override
  Widget build(BuildContext context) {
    final conn = ref.watch(kwaaiRpcConnectionProvider).valueOrNull;
    final streaming = ref.watch(chatStreamingProvider);
    final canSend = conn == RpcConnection.connected && !streaming;

    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
        child: TextField(
          controller: _controller,
          focusNode: _focusNode,
          enabled: canSend,
          onSubmitted: (_) => canSend ? _send() : null,
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
              onPressed: canSend ? _send : null,
            ),
          ),
        ),
      ),
    );
  }
}

/// Scrolling transcript of chat messages. Auto-scrolls to the bottom
/// on every rebuild so streaming tokens stay in view.
class _ChatTranscript extends ConsumerStatefulWidget {
  const _ChatTranscript();

  @override
  ConsumerState<_ChatTranscript> createState() => _ChatTranscriptState();
}

class _ChatTranscriptState extends ConsumerState<_ChatTranscript> {
  final _scrollController = ScrollController();

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  void _scrollToBottom() {
    if (!_scrollController.hasClients) return;
    _scrollController.animateTo(
      _scrollController.position.maxScrollExtent,
      duration: const Duration(milliseconds: 120),
      curve: Curves.easeOut,
    );
  }

  @override
  Widget build(BuildContext context) {
    final messages = ref.watch(chatTranscriptProvider);
    WidgetsBinding.instance.addPostFrameCallback((_) => _scrollToBottom());

    if (messages.isEmpty) {
      return Center(
        child: Text(
          'Send a message to start.',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
      );
    }

    return ListView.builder(
      controller: _scrollController,
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
      itemCount: messages.length,
      itemBuilder: (context, i) {
        final msg = messages[i];
        final isUser = msg.role == 'user';
        final accent = context.kwaai.accentPrimary;
        final scheme = Theme.of(context).colorScheme;
        return Padding(
          padding: const EdgeInsets.symmetric(vertical: 6),
          child: Row(
            mainAxisAlignment:
                isUser ? MainAxisAlignment.end : MainAxisAlignment.start,
            children: [
              Flexible(
                child: Column(
                  crossAxisAlignment: isUser
                      ? CrossAxisAlignment.end
                      : CrossAxisAlignment.start,
                  children: [
                    if (msg.text.isEmpty && msg.streaming)
                      // No tokens yet — show the animated Kwaai walk
                      // so the user can see we're actively waiting on
                      // the daemon. Replaced by the bubble below as
                      // soon as the first token arrives.
                      Padding(
                        padding: const EdgeInsets.symmetric(vertical: 4),
                        child: Image.asset(
                          'assets/kwaaiai-walk-anim.gif',
                          width: 32,
                          height: 32,
                        ),
                      )
                    else if (msg.text.isNotEmpty)
                      Container(
                        padding: const EdgeInsets.symmetric(
                          horizontal: 14,
                          vertical: 10,
                        ),
                        decoration: BoxDecoration(
                          color: isUser
                              ? accent.withValues(alpha: 0.12)
                              : context.kwaai.elevatedSurface,
                          borderRadius: BorderRadius.circular(14),
                        ),
                        child: SelectableText(
                          msg.text,
                          style:
                              Theme.of(context).textTheme.bodyMedium?.copyWith(
                            color: scheme.onSurface,
                          ),
                        ),
                      ),
                    if (msg.error != null)
                      _ChatErrorBadge(
                        error: msg.error!,
                        // Pad above only when there's a bubble (or the
                        // thinking indicator) above; a pure error
                        // message hugs the top of the row.
                        topPad: msg.text.isNotEmpty ? 6 : 0,
                      ),
                  ],
                ),
              ),
            ],
          ),
        );
      },
    );
  }
}

/// Compact error badge rendered below (or instead of) an assistant
/// bubble when its stream failed. Red icon + message in the theme's
/// destructive color so it reads as distinct from normal model output.
class _ChatErrorBadge extends StatelessWidget {
  const _ChatErrorBadge({required this.error, required this.topPad});

  final String error;
  final double topPad;

  @override
  Widget build(BuildContext context) {
    final dest = context.kwaai.buttonDestructive;
    return Padding(
      padding: EdgeInsets.only(top: topPad),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        decoration: BoxDecoration(
          color: dest.withValues(alpha: 0.10),
          border: Border.all(color: dest.withValues(alpha: 0.35)),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Icon(Icons.error_outline, size: 16, color: dest),
            const SizedBox(width: 8),
            Flexible(
              child: SelectableText(
                error,
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: dest,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}
