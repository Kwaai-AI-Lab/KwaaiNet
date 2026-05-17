import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../chat/chat_message.dart';
import '../../chat/chat_state.dart';
import '../../chat/kwaai_rpc_client.dart';
import '../../daemon/daemon_controller.dart';
import '../../daemon/daemon_state.dart';
import '../../settings.dart';
import '../../tray/tray.dart';
import '../../window/window_focus.dart';
import '../theme/kwaai_theme.dart';
import '../widgets/app_shell.dart';
import '../widgets/branded_title.dart';
import '../widgets/kwaai_chat_composer.dart';
import 'settings_page.dart';

class MainPage extends ConsumerStatefulWidget {
  const MainPage({
    super.key,
    required this.daemon,
    required this.settings,
    required this.tray,
  });

  final DaemonController daemon;
  final Settings settings;
  final TrayController tray;

  @override
  ConsumerState<MainPage> createState() => _MainPageState();
}

class _MainPageState extends ConsumerState<MainPage> {
  /// Index into the currently-visible tab list (`_visibleTabs(...)`).
  /// Reset to 0 when the visible tab list shrinks so we never point
  /// past the end.
  int _selectedTab = 0;

  /// Tabs that are visible based on current settings. The first entry
  /// is always the main chat (shard_run). Additional entries appear
  /// when developer/etc. toggles are on.
  List<_TabSpec> _visibleTabs(WidgetRef ref) {
    final localChatOn = ref.watch(localChatEnabledProvider);
    return [
      const _TabSpec(label: 'Chat', path: ChatPath.shardRun),
      if (localChatOn)
        const _TabSpec(label: 'Local chat', path: ChatPath.generateLocal),
    ];
  }

  void _openSettings() {
    Navigator.of(context).push(
      PageRouteBuilder(
        pageBuilder: (_, _, _) => SettingsPage(
          daemon: widget.daemon,
          settings: widget.settings,
          tray: widget.tray,
          onSettingsChanged: () {},
        ),
        transitionDuration: Duration.zero,
        reverseTransitionDuration: Duration.zero,
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    final tabs = _visibleTabs(ref);
    // Clamp selection in case the user just turned off the tab they
    // were on (e.g. disabled Local chat from Settings).
    if (_selectedTab >= tabs.length) _selectedTab = 0;

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
            // Top bar carries the tab strip inline when more than one
            // tab is visible — single-tab mode collapses back to a
            // plain brand + settings icon header.
            _MainTopBar(
              onOpenSettings: _openSettings,
              tabs: tabs.length > 1 ? tabs : const [],
              selectedTab: _selectedTab,
              onSelectTab: (i) => setState(() => _selectedTab = i),
            ),
            Expanded(
              child: IndexedStack(
                index: _selectedTab,
                children: [
                  for (final t in tabs)
                    _ChatTab(path: t.path, onOpenSettings: _openSettings),
                ],
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Per-tab metadata: display label + which gRPC operation drives the
/// chat surface.
class _TabSpec {
  const _TabSpec({required this.label, required this.path});
  final String label;
  final ChatPath path;
}

/// The contents of one tab — body + input — driven by a [ChatPath].
class _ChatTab extends StatelessWidget {
  const _ChatTab({required this.path, required this.onOpenSettings});

  final ChatPath path;
  final VoidCallback onOpenSettings;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        Expanded(
          child: _ChatBody(path: path, onOpenSettings: onOpenSettings),
        ),
        _ChatInputBar(path: path),
      ],
    );
  }
}

/// SegmentedButton-based tab selector. Same visual primitive as the
/// Mode picker on Settings → Appearance, so the two read as siblings.
class _MainTabSegmented extends StatelessWidget {
  const _MainTabSegmented({
    required this.tabs,
    required this.selectedIndex,
    required this.onSelect,
  });

  final List<_TabSpec> tabs;
  final int selectedIndex;
  final ValueChanged<int> onSelect;

  @override
  Widget build(BuildContext context) {
    final accent = context.kwaai.accentPrimary;
    final onSurface = Theme.of(context).colorScheme.onSurface;
    final focused = WindowFocusScope.of(context);
    final selectedFill = focused ? accent : kSelectedUnfocusedFill;
    final selectedFg = focused ? Colors.white : onSurface;
    final unselectedFill = onSurface.withValues(alpha: 0.12);
    final unselectedFg = onSurface.withValues(alpha: 0.6);

    return SegmentedButton<int>(
      style: ButtonStyle(
        backgroundColor: WidgetStateProperty.resolveWith(
          (states) => states.contains(WidgetState.selected)
              ? selectedFill
              : unselectedFill,
        ),
        foregroundColor: WidgetStateProperty.resolveWith(
          (states) =>
              states.contains(WidgetState.selected) ? selectedFg : unselectedFg,
        ),
        side: const WidgetStatePropertyAll(BorderSide.none),
        visualDensity: VisualDensity.compact,
        padding: const WidgetStatePropertyAll(
          EdgeInsets.symmetric(horizontal: 10, vertical: 4),
        ),
      ),
      showSelectedIcon: false,
      segments: [
        for (var i = 0; i < tabs.length; i++)
          ButtonSegment(value: i, label: Text(tabs[i].label)),
      ],
      selected: {selectedIndex},
      onSelectionChanged: (s) => onSelect(s.first),
    );
  }
}

class _MainTopBar extends StatelessWidget {
  const _MainTopBar({
    required this.onOpenSettings,
    this.tabs = const [],
    this.selectedTab = 0,
    this.onSelectTab,
  });

  final VoidCallback onOpenSettings;

  /// Visible tabs. When empty, the bar collapses to just the brand
  /// and the settings icon (single-chat mode).
  final List<_TabSpec> tabs;
  final int selectedTab;
  final ValueChanged<int>? onSelectTab;

  @override
  Widget build(BuildContext context) {
    // Top-align everything in the 52px bar so the brand lines up with
    // the macOS traffic lights (which sit ~top: 11). The segmented
    // button is taller than the brand text, so its top inset is
    // smaller — picked empirically to land the button label's text
    // baseline on the brand title's baseline.
    const brandTop = 11.0;
    const tabTop = 4.0;
    return SizedBox(
      height: 52,
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Left inset clears the native macOS traffic lights.
          const SizedBox(width: 80),
          const Padding(
            padding: EdgeInsets.only(top: brandTop),
            child: BrandedTitle(),
          ),
          if (tabs.isNotEmpty) ...[
            const SizedBox(width: 24),
            Padding(
              padding: const EdgeInsets.only(top: tabTop),
              child: _MainTabSegmented(
                tabs: tabs,
                selectedIndex: selectedTab,
                onSelect: (i) => onSelectTab?.call(i),
              ),
            ),
          ],
          const Spacer(),
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8),
            child: IconButton(
              tooltip: 'Settings',
              icon: const Icon(Icons.settings),
              onPressed: onOpenSettings,
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
  const _ChatBody({required this.path, required this.onOpenSettings});

  final ChatPath path;
  final VoidCallback onOpenSettings;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // The chat surface only renders when the daemon is actually
    // reachable via gRPC — but the *status copy* below reflects the
    // daemon lifecycle (starting / running / stopping / stopped), so
    // the user gets useful detail rather than a generic "connecting".
    final conn =
        ref.watch(kwaaiRpcConnectionProvider).valueOrNull ??
        RpcConnection.connecting;
    if (conn == RpcConnection.connected) {
      return _ChatTranscript(path: path);
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
          spinnerColor = Theme.of(
            context,
          ).colorScheme.onSurfaceVariant.withValues(alpha: 0.4);
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
  const _ChatInputBar({required this.path});

  final ChatPath path;

  @override
  ConsumerState<_ChatInputBar> createState() => _ChatInputBarState();
}

class _ChatInputBarState extends ConsumerState<_ChatInputBar> {
  final _controller = TextEditingController();
  final _focusNode = FocusNode();

  /// True once we've handed focus to the input. Used to gate the
  /// auto-focus to a single shot — without this the field would yank
  /// focus back from the user any time `canSend` rebuilds (e.g. they
  /// click somewhere else mid-stream).
  bool _autofocused = false;

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
    ref.read(chatTranscriptProvider(widget.path).notifier).send(text);
    _focusNode.requestFocus();
  }

  void _newChat() {
    _controller.clear();
    ref.read(chatTranscriptProvider(widget.path).notifier).newChat();
    _focusNode.requestFocus();
  }

  @override
  Widget build(BuildContext context) {
    final conn = ref.watch(kwaaiRpcConnectionProvider).valueOrNull;
    final streaming = ref.watch(chatStreamingProvider(widget.path));
    final canSend = conn == RpcConnection.connected && !streaming;
    final hasTranscript = ref
        .watch(chatTranscriptProvider(widget.path))
        .isNotEmpty;
    final onSurface = Theme.of(context).colorScheme.onSurface;

    // First time the input becomes usable, claim focus so the user
    // can start typing without clicking. Single-shot — doesn't yank
    // focus back if they navigate away mid-session.
    if (canSend && !_autofocused) {
      _autofocused = true;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (mounted) _focusNode.requestFocus();
      });
    }

    return SafeArea(
      top: false,
      child: Padding(
        padding: const EdgeInsets.fromLTRB(16, 8, 16, 16),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.end,
          children: [
            // New Chat sits outside the composer pill so it reads as a
            // distinct workspace action, not part of the input. Bottom-
            // aligned with the Send button when the composer expands.
            Padding(
              padding: const EdgeInsets.only(bottom: 4),
              child: IconButton(
                tooltip: 'New chat',
                icon: const Icon(Icons.add_comment_outlined),
                onPressed: hasTranscript ? _newChat : () {},
                color: onSurface.withValues(alpha: hasTranscript ? 0.85 : 0.45),
              ),
            ),
            const SizedBox(width: 6),
            Expanded(
              child: KwaaiChatComposer(
                controller: _controller,
                focusNode: _focusNode,
                enabled: canSend,
                onSend: canSend ? _send : null,
                hintText: 'Message kwaainet…',
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Scrolling transcript of chat messages. Auto-scrolls to the bottom
/// on every rebuild so streaming tokens stay in view.
class _ChatTranscript extends ConsumerStatefulWidget {
  const _ChatTranscript({required this.path});

  final ChatPath path;

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
    final messages = ref.watch(chatTranscriptProvider(widget.path));
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
            mainAxisAlignment: isUser
                ? MainAxisAlignment.end
                : MainAxisAlignment.start,
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
                        child: SelectableText.rich(
                          TextSpan(
                            style: Theme.of(context).textTheme.bodyMedium
                                ?.copyWith(color: scheme.onSurface),
                            children: [
                              TextSpan(text: msg.text),
                              if (!isUser && msg.streaming)
                                WidgetSpan(
                                  alignment: PlaceholderAlignment.baseline,
                                  baseline: TextBaseline.alphabetic,
                                  child: _StreamingDots(
                                    color: scheme.onSurface,
                                  ),
                                ),
                            ],
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
/// Friendly rendering of a [ChatError]. Switches on the structured
/// proto code rather than grepping the message — daemon strings are
/// fair game for the "show details" disclosure but never drive the
/// headline.
({String headline, String? details}) _friendlyChatError(ChatError err) {
  // Mirror kwaai.v1.Error.Code values. Kept inline here (rather than
  // importing the generated enum) so the dispatcher is readable at a
  // glance and so adding a new code only requires one switch arm.
  switch (err.code) {
    case 1: // INVALID_ARGUMENT
      return (headline: 'Invalid request.', details: err.message);
    case 2: // NOT_FOUND
      return (headline: 'Not found.', details: err.message);
    case 3: // UNAVAILABLE
      return (
        headline: 'Lost connection to the service.',
        details: err.message,
      );
    case 4: // CANCELLED
      return (headline: 'Cancelled.', details: null);
    case 6: // UNIMPLEMENTED
      return (
        headline: 'The daemon doesn\'t support this operation yet.',
        details: err.message,
      );
    case 7: // NO_PEERS_FOR_MODEL
      return (
        headline: 'No peers are serving this model on the network yet.',
        details: err.message,
      );
    case 8: // INSUFFICIENT_COVERAGE
      return (
        headline: 'Not enough peers — some model blocks aren\'t being served.',
        details: err.message,
      );
    case 9: // ALL_CANDIDATES_FAILED
      return (
        headline: 'Not enough peers available for distributed inference.',
        details: err.message,
      );
    case 10: // MODEL_LOAD_FAILED
      return (
        headline: 'The daemon couldn\'t load the configured model.',
        details: err.message,
      );
    case 5: // INTERNAL
    case 0: // UNKNOWN
    default:
      // Default: surface the raw error verbatim — caller likely
      // needs the detail to debug.
      return (headline: err.message, details: null);
  }
}

class _ChatErrorBadge extends StatefulWidget {
  const _ChatErrorBadge({required this.error, required this.topPad});

  final ChatError error;
  final double topPad;

  @override
  State<_ChatErrorBadge> createState() => _ChatErrorBadgeState();
}

class _ChatErrorBadgeState extends State<_ChatErrorBadge> {
  bool _expanded = false;

  @override
  Widget build(BuildContext context) {
    final dest = context.kwaai.buttonDestructive;
    final (:headline, :details) = _friendlyChatError(widget.error);
    return Padding(
      padding: EdgeInsets.only(top: widget.topPad),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 8),
        decoration: BoxDecoration(
          color: dest.withValues(alpha: 0.10),
          border: Border.all(color: dest.withValues(alpha: 0.35)),
          borderRadius: BorderRadius.circular(10),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisSize: MainAxisSize.min,
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Icon(Icons.error_outline, size: 16, color: dest),
                const SizedBox(width: 8),
                Flexible(
                  child: SelectableText(
                    headline,
                    style: Theme.of(
                      context,
                    ).textTheme.bodyMedium?.copyWith(color: dest),
                  ),
                ),
              ],
            ),
            if (details != null) ...[
              const SizedBox(height: 4),
              MouseRegion(
                cursor: SystemMouseCursors.click,
                child: GestureDetector(
                  onTap: () => setState(() => _expanded = !_expanded),
                  child: Padding(
                    // Align text under the icon-then-gap.
                    padding: const EdgeInsets.only(left: 24),
                    child: Text(
                      _expanded ? 'Hide details' : 'Show details',
                      style: Theme.of(context).textTheme.bodySmall?.copyWith(
                        color: dest.withValues(alpha: 0.7),
                        decoration: TextDecoration.underline,
                        decorationColor: dest.withValues(alpha: 0.4),
                      ),
                    ),
                  ),
                ),
              ),
              if (_expanded) ...[
                const SizedBox(height: 4),
                Padding(
                  padding: const EdgeInsets.only(left: 24),
                  child: SelectableText(
                    details,
                    style: Theme.of(context).textTheme.bodySmall?.copyWith(
                      color: dest.withValues(alpha: 0.85),
                      fontFamily: 'monospace',
                    ),
                  ),
                ),
              ],
            ],
          ],
        ),
      ),
    );
  }
}

/// Inline "…" animation appended to a streaming assistant bubble.
/// Each dot fades in on its own 1/3 of the cycle so the user can see
/// the stream is still alive even when the model pauses between tokens.
class _StreamingDots extends StatefulWidget {
  const _StreamingDots({required this.color});

  final Color color;

  @override
  State<_StreamingDots> createState() => _StreamingDotsState();
}

class _StreamingDotsState extends State<_StreamingDots>
    with SingleTickerProviderStateMixin {
  late final AnimationController _controller = AnimationController(
    vsync: this,
    duration: const Duration(milliseconds: 1200),
  )..repeat();

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _controller,
      builder: (context, _) {
        final t = _controller.value;
        return Padding(
          padding: const EdgeInsets.only(left: 2),
          child: Row(
            mainAxisSize: MainAxisSize.min,
            crossAxisAlignment: CrossAxisAlignment.end,
            children: [_dot(t, 0.0), _dot(t, 1 / 3), _dot(t, 2 / 3)],
          ),
        );
      },
    );
  }

  Widget _dot(double t, double phase) {
    // Triangle wave offset by `phase`: ramps 0→1 over the first half
    // of each dot's slot, then back to 0. Keeps min opacity above zero
    // so the dots remain visible (just dim) at their trough.
    final local = ((t - phase) % 1.0 + 1.0) % 1.0;
    final ramp = local < 0.5 ? local * 2 : (1 - local) * 2;
    final opacity = 0.25 + 0.75 * ramp;
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 1),
      child: Container(
        width: 4,
        height: 4,
        decoration: BoxDecoration(
          shape: BoxShape.circle,
          color: widget.color.withValues(alpha: opacity),
        ),
      ),
    );
  }
}
