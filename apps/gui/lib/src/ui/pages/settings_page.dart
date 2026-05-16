import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../daemon/config_file.dart';
import '../../daemon/daemon_controller.dart';
import '../../daemon/daemon_state.dart';
import '../../daemon/features_state.dart';
import '../../daemon/status_watcher.dart';
import '../../settings.dart';
import '../../tray/tray.dart';
import '../../window/window_focus.dart';
import '../theme/kwaai_theme.dart';
import '../theme/theme_controller.dart';
import '../theme/theme_variants.dart';
import '../widgets/app_shell.dart';
import '../widgets/branded_title.dart';
import '../widgets/kwaai_button.dart';
import '../widgets/kwaai_dropdown.dart';
import '../widgets/kwaai_heading.dart';
import '../widgets/kwaai_status_bar.dart';
import '../widgets/kwaai_text_field.dart';

/// Fill color for unselected/secondary controls — segmented-button segments
/// and secondary buttons (e.g. Stop daemon).
const Color kUnselectedFill = Color(0xFFEFEFEF);

/// Fill color for *selected* controls when the app window is not focused —
/// the accent tint desaturates to gray, matching native macOS behaviour.
const Color kSelectedUnfocusedFill = Color(0xFFD4D4D4);

class SettingsPage extends ConsumerStatefulWidget {
  const SettingsPage({
    super.key,
    required this.daemon,
    required this.settings,
    required this.tray,
    required this.onSettingsChanged,
  });

  final DaemonController daemon;
  final Settings settings;
  final TrayController tray;
  final VoidCallback onSettingsChanged;

  @override
  ConsumerState<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends ConsumerState<SettingsPage> {
  int _selectedTab = 0;

  Future<void> _start() => ref.read(daemonTransitionProvider.notifier).start();

  Future<void> _stop() => ref.read(daemonTransitionProvider.notifier).stop();

  Widget _buildStatusTab() {
    return SingleChildScrollView(
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          _FeatureCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const _StatusHeader(),
                const SizedBox(height: 12),
                Consumer(
                  builder: (context, ref, _) {
                    final status = ref.watch(daemonStatusProvider).valueOrNull;
                    final running = status?.running ?? false;
                    final busy =
                        ref.watch(daemonTransitionProvider) !=
                        DaemonTransition.none;
                    return Row(
                      children: [
                        KwaaiButton(
                          label: 'Start service',
                          icon: Icons.play_arrow,
                          onPressed: (running || busy) ? null : _start,
                        ),
                        const SizedBox(width: 8),
                        KwaaiButton(
                          label: 'Stop service',
                          icon: Icons.stop,
                          variant: KwaaiButtonVariant.destructive,
                          onPressed: (!running || busy) ? null : _stop,
                        ),
                      ],
                    );
                  },
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),
          _FeatureCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const KwaaiHeading('KwaaiNet binary location'),
                const SizedBox(height: 8),
                _DaemonSourcePicker(
                  daemon: widget.daemon,
                  settings: widget.settings,
                  onChanged: () {
                    setState(() {});
                    widget.onSettingsChanged();
                    // The user changed the binary source — assume the
                    // previous "not found" / spawn error no longer
                    // applies. They can re-trigger by clicking Start.
                    ref.read(daemonErrorProvider.notifier).clear();
                  },
                ),
              ],
            ),
          ),
          const SizedBox(height: 12),
          _FeatureCard(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                const KwaaiHeading('Service'),
                const SizedBox(height: 4),
                _StartOnStartupToggle(
                  settings: widget.settings,
                  onChanged: () {
                    setState(() {});
                    widget.onSettingsChanged();
                  },
                ),
                _KeepInTrayToggle(
                  settings: widget.settings,
                  tray: widget.tray,
                  onChanged: () {
                    setState(() {});
                    widget.onSettingsChanged();
                  },
                ),
              ],
            ),
          ),
        ],
      ),
    );
  }

  @override
  Widget build(BuildContext context) {
    return AppShell(
      child: LayoutBuilder(
        builder: (context, constraints) {
          final width = constraints.maxWidth;
          // Two layouts only: a side rail (icon + label) on wider/desktop
          // viewports, and a bottom nav bar on narrow/mobile-sized ones.
          final bottomNav = width < 600;

          final settingsContent = Column(
            children: [
              _SettingsTopBar(
                onClose: () => Navigator.of(context).pop(),
                clearTrafficLights: bottomNav,
              ),
              Expanded(
                child: IndexedStack(
                  index: _selectedTab,
                  children: [
                    _buildStatusTab(),
                    const _FeaturesTab(),
                    const _AppearanceTab(),
                  ],
                ),
              ),
              // Pinned to the bottom of the Settings card: surfaces when
              // the user has applied feature changes that need a daemon
              // restart. Persistent across the Status / Features /
              // Appearance tabs.
              const _RestartNeededBar(),
            ],
          );

          if (bottomNav) {
            return Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Expanded(
                  child: ShellCard(
                    gutter: const EdgeInsets.fromLTRB(
                      kShellGutter,
                      kShellGutter,
                      kShellGutter,
                      0,
                    ),
                    // Top-left/-right touch the window edge → rounded;
                    // the bottom (against the nav bar) stays sharp.
                    borderRadius: shellRadius(topLeft: true, topRight: true),
                    // No content inset here — the top bar clears the traffic
                    // lights itself (via clearTrafficLights), matching
                    // MainPage. The side-rail layout insets the *nav* card
                    // instead, since its top bar lives in the other card.
                    child: settingsContent,
                  ),
                ),
                ShellCard(
                  gutter: const EdgeInsets.all(kShellGutter),
                  borderRadius: shellRadius(
                    bottomLeft: true,
                    bottomRight: true,
                  ),
                  child: _SettingsNav(
                    axis: Axis.horizontal,
                    extended: true,
                    selectedIndex: _selectedTab,
                    onSelect: (i) => setState(() => _selectedTab = i),
                  ),
                ),
              ],
            );
          }

          return Row(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              ShellCard(
                gutter: const EdgeInsets.all(kShellGutter),
                // Left edge touches the window → rounded; right edge faces
                // the content card → sharp.
                borderRadius: shellRadius(topLeft: true, bottomLeft: true),
                contentPadding: const EdgeInsets.only(
                  top: kMacOSTitlebarHeight,
                ),
                child: _SettingsNav(
                  axis: Axis.vertical,
                  extended: true,
                  selectedIndex: _selectedTab,
                  onSelect: (i) => setState(() => _selectedTab = i),
                ),
              ),
              Expanded(
                child: ShellCard(
                  // Left gutter is half the full margin — splits the gap
                  // between the nav card and the content card.
                  gutter: const EdgeInsets.fromLTRB(
                    kShellGutter,
                    kShellGutter,
                    kShellGutter,
                    kShellGutter,
                  ),
                  // Right edge touches the window → rounded; left edge
                  // faces the nav card → sharp (kShellInnerRadius). The
                  // asymmetric corners visually echo the nav/content
                  // split.
                  borderRadius: shellRadius(topRight: true, bottomRight: true),
                  child: settingsContent,
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}

class _SettingsNavEntry {
  const _SettingsNavEntry(this.icon, this.selectedIcon, this.label);
  final IconData icon;
  final IconData selectedIcon;
  final String label;
}

const _settingsNavEntries = <_SettingsNavEntry>[
  _SettingsNavEntry(
    Icons.monitor_heart_outlined,
    Icons.monitor_heart,
    'Status',
  ),
  _SettingsNavEntry(Icons.tune_outlined, Icons.tune, 'Features'),
  _SettingsNavEntry(Icons.palette_outlined, Icons.palette, 'Appearance'),
];

class _SettingsNav extends StatelessWidget {
  const _SettingsNav({
    required this.axis,
    required this.extended,
    required this.selectedIndex,
    required this.onSelect,
  });

  final Axis axis;
  final bool extended;
  final int selectedIndex;
  final ValueChanged<int> onSelect;

  @override
  Widget build(BuildContext context) {
    // Bottom nav (horizontal) uses fully-rounded pill buttons; the side
    // rail uses the card-concentric small radius.
    final pill = axis == Axis.horizontal;
    final items = [
      for (var i = 0; i < _settingsNavEntries.length; i++)
        _SettingsNavItem(
          entry: _settingsNavEntries[i],
          selected: i == selectedIndex,
          extended: extended,
          pill: pill,
          onTap: () => onSelect(i),
        ),
    ];

    if (axis == Axis.horizontal) {
      return Padding(
        padding: const EdgeInsets.all(kShellInset),
        child: Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            for (final item in items)
              Padding(
                padding: const EdgeInsets.symmetric(horizontal: 4),
                child: item,
              ),
          ],
        ),
      );
    }

    return SizedBox(
      width: extended ? 172 : 64,
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const SizedBox(height: 8),
          for (final item in items)
            Padding(
              padding: const EdgeInsets.symmetric(
                horizontal: kShellInset,
                vertical: 2,
              ),
              child: item,
            ),
        ],
      ),
    );
  }
}

class _SettingsNavItem extends StatelessWidget {
  const _SettingsNavItem({
    required this.entry,
    required this.selected,
    required this.extended,
    required this.onTap,
    this.pill = false,
  });

  final _SettingsNavEntry entry;
  final bool selected;
  final bool extended;

  /// When true, render as a fully-rounded pill (bottom nav bar) rather than
  /// the small card-concentric radius (side rail).
  final bool pill;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final accent = context.kwaai.accentPrimary;
    final fg = selected ? accent : Theme.of(context).colorScheme.onSurface;
    final icon = Icon(
      selected ? entry.selectedIcon : entry.icon,
      size: 20,
      color: fg,
    );

    final radius = pill
        ? BorderRadius.circular(999)
        : BorderRadius.circular(concentricRadius(kShellRadius, kShellInset));
    return Material(
      color: selected ? accent.withValues(alpha: 0.12) : Colors.transparent,
      borderRadius: radius,
      child: InkWell(
        borderRadius: radius,
        onTap: onTap,
        child: Padding(
          padding: EdgeInsets.symmetric(
            horizontal: extended ? 12 : 0,
            vertical: 8,
          ),
          child: extended
              ? Row(
                  children: [
                    icon,
                    const SizedBox(width: 12),
                    Text(
                      entry.label,
                      style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                        color: fg,
                        fontWeight: selected
                            ? FontWeight.w600
                            : FontWeight.w400,
                      ),
                    ),
                  ],
                )
              : Center(child: icon),
        ),
      ),
    );
  }
}

class _SettingsTopBar extends StatelessWidget {
  const _SettingsTopBar({
    required this.onClose,
    this.clearTrafficLights = false,
  });

  final VoidCallback onClose;

  /// When this bar is the top-left card (bottom-nav layout), the native
  /// traffic lights overlap it — inset the brand to clear them, matching
  /// MainPage's top bar.
  final bool clearTrafficLights;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 52,
      child: Stack(
        children: [
          // Back arrow. clearTrafficLights pushes it right of the macOS
          // window controls in bottom-nav layout.
          Positioned(
            top: 2,
            left: clearTrafficLights ? 72 : 8,
            child: IconButton(
              tooltip: 'Back',
              icon: const Icon(Icons.arrow_back),
              onPressed: onClose,
            ),
          ),
          // Brand sits to the right of the back arrow. Same top inset as
          // MainPage's brand so it doesn't drift between pages.
          Positioned(
            top: 11,
            left: clearTrafficLights ? 116 : 52,
            child: const BrandedTitle(),
          ),
          Align(
            alignment: Alignment.topCenter,
            child: Padding(
              padding: const EdgeInsets.only(top: 14),
              child: Text(
                'Settings',
                style: Theme.of(
                  context,
                ).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w600),
              ),
            ),
          ),
        ],
      ),
    );
  }
}

class _StatusHeader extends ConsumerWidget {
  const _StatusHeader();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final cs = Theme.of(context).colorScheme;
    final transition = ref.watch(daemonTransitionProvider);
    final status =
        ref.watch(daemonStatusProvider).valueOrNull ?? NodeStatus.stopped();

    // While transitioning, the orange indicator + "Starting"/"Stopping"
    // override the daemon's reported running/stopped state.
    final Color color;
    final List<String> bits;
    switch (transition) {
      case DaemonTransition.starting:
        color = context.kwaai.statusTransitioning;
        bits = ['Starting…'];
      case DaemonTransition.stopping:
        color = context.kwaai.statusTransitioning;
        bits = ['Stopping…'];
      case DaemonTransition.none:
        color = status.running
            ? context.kwaai.statusRunning
            : context.kwaai.statusStopped;
        bits = <String>[
          status.running ? 'Running' : 'Stopped',
          if (status.running && status.pid != null) 'pid ${status.pid}',
          if (status.uptimeSecs != null)
            'up ${_fmtDuration(status.uptimeSecs!)}',
          if (status.memoryMb != null)
            '${status.memoryMb!.toStringAsFixed(0)} MB',
          if (status.cpuPercent != null)
            '${status.cpuPercent!.toStringAsFixed(1)}% CPU',
        ];
    }

    return Row(
      children: [
        Icon(Icons.circle, color: color, size: 14),
        const SizedBox(width: 8),
        Expanded(
          child: Text(
            bits.join('  •  '),
            style: Theme.of(context).textTheme.bodyMedium,
          ),
        ),
        if (status.source == 'pid' && status.running)
          Tooltip(
            message:
                'Status from PID probe only — full stats appear once daemon writes kwaainet.status',
            child: Icon(
              Icons.info_outline,
              size: 16,
              color: cs.onSurfaceVariant,
            ),
          ),
      ],
    );
  }

  String _fmtDuration(int secs) {
    final h = secs ~/ 3600;
    final m = (secs % 3600) ~/ 60;
    final s = secs % 60;
    if (h > 0) return '${h}h ${m}m';
    if (m > 0) return '${m}m ${s}s';
    return '${s}s';
  }
}

/// Toggle for the "start the service automatically at app launch"
/// preference. Reads / writes [Settings.startServiceOnStartup].
class _StartOnStartupToggle extends StatefulWidget {
  const _StartOnStartupToggle({
    required this.settings,
    required this.onChanged,
  });
  final Settings settings;
  final VoidCallback onChanged;

  @override
  State<_StartOnStartupToggle> createState() => _StartOnStartupToggleState();
}

class _StartOnStartupToggleState extends State<_StartOnStartupToggle> {
  late bool _value = widget.settings.startServiceOnStartup;

  Future<void> _set(bool v) async {
    setState(() => _value = v);
    await widget.settings.setStartServiceOnStartup(v);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    return _SwitchRow(
      label: 'Start service on startup',
      value: _value,
      onChanged: _set,
    );
  }
}

/// Toggle for the "keep running in the menu bar when the window is closed"
/// preference. Reads / writes [Settings.keepInTrayOnClose].
class _KeepInTrayToggle extends StatefulWidget {
  const _KeepInTrayToggle({
    required this.settings,
    required this.tray,
    required this.onChanged,
  });
  final Settings settings;
  final TrayController tray;
  final VoidCallback onChanged;

  @override
  State<_KeepInTrayToggle> createState() => _KeepInTrayToggleState();
}

class _KeepInTrayToggleState extends State<_KeepInTrayToggle> {
  late bool _value = widget.settings.keepInTrayOnClose;

  Future<void> _set(bool v) async {
    setState(() => _value = v);
    await widget.settings.setKeepInTrayOnClose(v);
    // Install / remove the menu-bar icon to match. Off ⇒ no tray.
    await widget.tray.setEnabled(v);
    widget.onChanged();
  }

  @override
  Widget build(BuildContext context) {
    return _SwitchRow(
      label: 'Keep running in tray when window is closed',
      value: _value,
      onChanged: _set,
    );
  }
}

/// Compact switch row — label on the left, scaled-down Switch on the right,
/// the whole row tappable. Mirrors `_RadioRow`'s visual rhythm so radios
/// and switches read as siblings.
class _SwitchRow extends StatelessWidget {
  const _SwitchRow({
    required this.label,
    required this.value,
    required this.onChanged,
  });

  final String label;
  final bool value;
  final ValueChanged<bool> onChanged;

  @override
  Widget build(BuildContext context) {
    final focused = WindowFocusScope.of(context);
    final activeFill = focused
        ? context.kwaai.accentPrimary
        : kSelectedUnfocusedFill;
    return InkWell(
      borderRadius: BorderRadius.circular(6),
      onTap: () => onChanged(!value),
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 2),
        child: Row(
          children: [
            Expanded(
              child: Text(label, style: Theme.of(context).textTheme.bodyMedium),
            ),
            const SizedBox(width: 6),
            // Wrap the scaled Switch in a SizedBox so it occupies the
            // shrunken layout height — matching the radio rows' ~32px
            // height so consecutive switch rows pack at the same
            // vertical rhythm as radio rows.
            SizedBox(
              height: 32,
              child: Transform.scale(
                scale: 0.67,
                child: Switch(
                  value: value,
                  onChanged: onChanged,
                  materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                  activeThumbColor: Colors.white,
                  activeTrackColor: activeFill,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _DaemonSourcePicker extends StatefulWidget {
  const _DaemonSourcePicker({
    required this.daemon,
    required this.settings,
    required this.onChanged,
  });
  final DaemonController daemon;
  final Settings settings;
  final VoidCallback onChanged;

  @override
  State<_DaemonSourcePicker> createState() => _DaemonSourcePickerState();
}

class _DaemonSourcePickerState extends State<_DaemonSourcePicker> {
  late final TextEditingController _pathController = TextEditingController(
    text: widget.settings.customPath ?? '',
  );

  @override
  void dispose() {
    _pathController.dispose();
    super.dispose();
  }

  Future<void> _set(DaemonMode m) async {
    await widget.settings.setMode(m);
    widget.onChanged();
  }

  Future<void> _commitCustomPath(String path) async {
    await widget.settings.setCustomPath(path.isEmpty ? null : path);
    widget.onChanged();
  }

  Future<void> _browseForPath() async {
    try {
      final res = await FilePicker.platform.pickFiles();
      if (res == null || res.files.isEmpty) return;
      final path = res.files.single.path;
      if (path == null) return;
      _pathController.text = path;
      await _commitCustomPath(path);
    } catch (e) {
      if (!mounted) return;
      ScaffoldMessenger.of(
        context,
      ).showSnackBar(SnackBar(content: Text('File picker failed: $e')));
    }
  }

  @override
  Widget build(BuildContext context) {
    final mode = widget.settings.mode;
    final systemBinaryFound = widget.daemon.findSystemBinary() != null;
    return RadioGroup<DaemonMode>(
      groupValue: mode,
      onChanged: (m) {
        if (m == null) return;
        _set(m);
      },
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const _RadioRow(
            value: DaemonMode.builtIn,
            label: 'Use built-in (dev: core/target/debug/kwaainet)',
          ),
          // "Use system" is only selectable when kwaainet is on PATH.
          _RadioRow(
            value: DaemonMode.system,
            label: 'Use system',
            enabled: systemBinaryFound,
          ),
          _PathRow(child: _SystemPathResult(daemon: widget.daemon)),
          const _RadioRow(value: DaemonMode.custom, label: 'Use other…'),
          // Always rendered, disabled until "Use other…" is selected. The
          // disabled state greys both the field and the browse button.
          _PathRow(
            child: Builder(
              builder: (context) {
                final isCustom = mode == DaemonMode.custom;
                return KwaaiTextField(
                  controller: _pathController,
                  hintText: '/path/to/kwaainet',
                  enabled: isCustom,
                  onSubmitted: _commitCustomPath,
                  onEditingComplete: () =>
                      _commitCustomPath(_pathController.text),
                  trailing: IconButton(
                    tooltip: 'Browse…',
                    icon: const Icon(Icons.folder_open, size: 18),
                    onPressed: isCustom ? _browseForPath : null,
                    visualDensity: VisualDensity.compact,
                  ),
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

/// Compact radio row — radio control with its label pulled right up against
/// it, the whole row tappable. Must sit inside a `RadioGroup<DaemonMode>`.
/// When [enabled] is false the row is greyed out and not selectable.
class _RadioRow extends StatelessWidget {
  const _RadioRow({
    required this.value,
    required this.label,
    this.enabled = true,
  });

  final DaemonMode value;
  final String label;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    final group = RadioGroup.maybeOf<DaemonMode>(context);
    final disabledColor = Theme.of(
      context,
    ).colorScheme.onSurface.withValues(alpha: 0.38);
    return InkWell(
      borderRadius: BorderRadius.circular(6),
      onTap: enabled ? () => group?.onChanged(value) : null,
      child: Padding(
        padding: const EdgeInsets.symmetric(vertical: 2),
        child: Row(
          children: [
            // Shrink the Material Radio visually. Radio has no size prop,
            // so scale the rendered painter; the hit-testing inside the
            // InkWell keeps the row tap-friendly. Fill desaturates to
            // gray when the app window is unfocused, matching buttons +
            // toggles.
            Transform.scale(
              scale: 0.8,
              child: Radio<DaemonMode>(
                value: value,
                enabled: enabled,
                materialTapTargetSize: MaterialTapTargetSize.shrinkWrap,
                visualDensity: VisualDensity.compact,
                fillColor: WidgetStateProperty.resolveWith((states) {
                  if (!states.contains(WidgetState.selected)) return null;
                  return WindowFocusScope.of(context)
                      ? context.kwaai.accentPrimary
                      : kSelectedUnfocusedFill;
                }),
              ),
            ),
            const SizedBox(width: 6),
            Flexible(
              child: Text(
                label,
                style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                  color: enabled ? null : disabledColor,
                ),
              ),
            ),
          ],
        ),
      ),
    );
  }
}

class _PathRow extends StatelessWidget {
  const _PathRow({required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    // Left inset matches where a _RadioRow's label starts: the compact
    // Radio's layout box (~32px) + the 6px SizedBox after it.
    return Padding(
      padding: const EdgeInsets.fromLTRB(38, 0, 16, 12),
      child: Align(alignment: Alignment.centerLeft, child: child),
    );
  }
}

class _SystemPathResult extends StatelessWidget {
  const _SystemPathResult({required this.daemon});
  final DaemonController daemon;

  @override
  Widget build(BuildContext context) {
    final path = daemon.findSystemBinary();
    if (path == null) {
      return Text(
        'No kwaainet binary found',
        style: Theme.of(
          context,
        ).textTheme.bodyMedium?.copyWith(color: context.kwaai.error),
      );
    }
    return SelectableText.rich(
      TextSpan(
        style: Theme.of(context).textTheme.bodyMedium,
        children: [
          const TextSpan(text: 'Binary found at: '),
          TextSpan(
            text: path,
            style: const TextStyle(fontFamily: 'monospace'),
          ),
        ],
      ),
    );
  }
}

/// Curated list of HuggingFace model IDs the GUI offers via the Features
/// dropdown. The daemon doesn't expose a catalog API yet, so this is a
/// hand-maintained list — swap to a runtime fetch when one lands.
const List<String> _knownModels = [
  'unsloth/Llama-3.1-8B-Instruct',
  'unsloth/Llama-3-8B',
  'unsloth/Llama-3.2-3B-Instruct',
  'mistralai/Mistral-7B-Instruct-v0.3',
  'Qwen/Qwen2.5-7B-Instruct',
  'microsoft/Phi-3-mini-4k-instruct',
];

/// Sentinel value used by the Model dropdown to represent "Other…" — when
/// chosen, a free-form text field appears for the user to type an
/// arbitrary HuggingFace model id.
const String _otherModelSentinel = '__other__';

class _FeaturesTab extends ConsumerWidget {
  const _FeaturesTab();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loaded = ref.watch(featuresProvider);
    return loaded.when(
      loading: () => const Center(child: CircularProgressIndicator()),
      error: (e, _) => Padding(
        padding: const EdgeInsets.all(16),
        child: Text('Failed to load config: $e'),
      ),
      data: (snapshot) {
        // Seed the draft on first render after the load. Doing it here
        // (instead of in build) is safe because the draft is a Notifier
        // whose seed() is idempotent.
        WidgetsBinding.instance.addPostFrameCallback((_) {
          ref.read(featuresDraftProvider.notifier).seed(snapshot);
        });
        final draft = ref.watch(featuresDraftProvider) ?? snapshot;
        final dirty = draft.model != snapshot.model ||
            draft.shardingEnabled != snapshot.shardingEnabled ||
            draft.storageEnabled != snapshot.storageEnabled ||
            draft.storageCapacityGb != snapshot.storageCapacityGb;

        return SingleChildScrollView(
          padding: const EdgeInsets.fromLTRB(16, 16, 16, 16),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: [
              _FeatureCard(child: _ShardingSection(draft: draft)),
              const SizedBox(height: 12),
              _FeatureCard(child: _StorageSection(draft: draft)),
              const SizedBox(height: 16),
              // One Apply for both sections. Disabled when the draft
              // matches what's on disk.
              Align(
                alignment: Alignment.centerRight,
                child: KwaaiButton(
                  label: 'Apply',
                  onPressed: dirty ? () => _apply(ref) : null,
                ),
              ),
            ],
          ),
        );
      },
    );
  }

  Future<void> _apply(WidgetRef ref) async {
    await ref.read(featuresDraftProvider.notifier).apply();
    ref.read(restartNeededProvider.notifier).mark();
    ref.invalidate(featuresProvider);
  }
}

/// Sectional card used to group related feature controls. Uses the
/// theme's [KwaaiThemeExtension.elevatedSurface] so it reads as one step
/// above the content card it sits in.
class _FeatureCard extends StatelessWidget {
  const _FeatureCard({required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: context.kwaai.elevatedSurface,
        borderRadius: BorderRadius.circular(8),
      ),
      padding: const EdgeInsets.fromLTRB(14, 12, 14, 14),
      child: child,
    );
  }
}

/// "Sharding" section: enable toggle + model dropdown.
class _ShardingSection extends ConsumerWidget {
  const _ShardingSection({required this.draft});

  final ConfigSnapshot draft;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.read(featuresDraftProvider.notifier);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const KwaaiHeading('Sharding'),
        const SizedBox(height: 4),
        _SwitchRow(
          label: 'Serve transformer shards to the network',
          value: draft.shardingEnabled,
          onChanged: notifier.setShardingEnabled,
        ),
        const SizedBox(height: 12),
        Text(
          'Model',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 4),
        // Align prevents the parent Column(stretch) from forcing the
        // picker to fill the section card; the picker's own children
        // (dropdown + optional Other field) self-size.
        Align(
          alignment: Alignment.centerLeft,
          child: _ModelPicker(
            current: draft.model,
            enabled: draft.shardingEnabled,
            onChanged: notifier.setModel,
          ),
        ),
      ],
    );
  }
}

/// "Storage" section: enable toggle + capacity_gb field.
class _StorageSection extends ConsumerWidget {
  const _StorageSection({required this.draft});

  final ConfigSnapshot draft;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final notifier = ref.read(featuresDraftProvider.notifier);
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const KwaaiHeading('Storage'),
        const SizedBox(height: 4),
        _SwitchRow(
          label: 'Offer vector storage to the network',
          value: draft.storageEnabled,
          onChanged: notifier.setStorageEnabled,
        ),
        const SizedBox(height: 12),
        Text(
          'Capacity (GB)',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
        const SizedBox(height: 4),
        // Align prevents the parent Column's stretch from forcing the
        // field to fill the section width — without it the ConstrainedBox
        // gets tight horizontal constraints and the maxWidth is ignored.
        Align(
          alignment: Alignment.centerLeft,
          child: SizedBox(
            width: 110,
            child: _CapacityField(
              initial: draft.storageCapacityGb,
              enabled: draft.storageEnabled,
              onChanged: notifier.setStorageCapacityGb,
            ),
          ),
        ),
      ],
    );
  }
}

/// Dropdown of curated HF models with an "Other…" escape hatch that
/// reveals a [KwaaiTextField] for arbitrary IDs.
class _ModelPicker extends StatefulWidget {
  const _ModelPicker({
    required this.current,
    required this.enabled,
    required this.onChanged,
  });

  final String current;
  final bool enabled;
  final ValueChanged<String> onChanged;

  @override
  State<_ModelPicker> createState() => _ModelPickerState();
}

class _ModelPickerState extends State<_ModelPicker> {
  late final TextEditingController _otherController = TextEditingController(
    text: _isKnown(widget.current) ? '' : widget.current,
  );
  late bool _isOther = !_isKnown(widget.current) && widget.current.isNotEmpty;

  static bool _isKnown(String v) => _knownModels.contains(v);

  @override
  void dispose() {
    _otherController.dispose();
    super.dispose();
  }

  String? get _dropdownValue {
    if (_isOther) return _otherModelSentinel;
    if (_knownModels.contains(widget.current)) return widget.current;
    return null;
  }

  void _onDropdownChanged(String? v) {
    if (v == null) return;
    if (v == _otherModelSentinel) {
      setState(() => _isOther = true);
      // Don't push a value yet — wait for the user to type in the field.
      return;
    }
    setState(() => _isOther = false);
    widget.onChanged(v);
  }

  @override
  Widget build(BuildContext context) {
    final dropdown = KwaaiDropdown<String>(
      value: _dropdownValue,
      enabled: widget.enabled,
      hintText: 'Select a model…',
      items: [
        for (final m in _knownModels)
          KwaaiDropdownItem(value: m, label: m),
        const KwaaiDropdownItem(
          value: _otherModelSentinel,
          label: 'Other…',
        ),
      ],
      onChanged: _onDropdownChanged,
    );

    if (!_isOther) return dropdown;

    // When "Other…" is selected, render the free-form input to the right
    // of the dropdown rather than below it.
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      children: [
        dropdown,
        const SizedBox(width: 8),
        SizedBox(
          width: 220,
          child: KwaaiTextField(
            controller: _otherController,
            enabled: widget.enabled,
            hintText: 'e.g. some-org/some-model',
            onSubmitted: widget.onChanged,
            onEditingComplete: () => widget.onChanged(_otherController.text),
          ),
        ),
      ],
    );
  }
}

class _CapacityField extends StatefulWidget {
  const _CapacityField({
    required this.initial,
    required this.enabled,
    required this.onChanged,
  });
  final double? initial;
  final bool enabled;
  final ValueChanged<double?> onChanged;

  @override
  State<_CapacityField> createState() => _CapacityFieldState();
}

class _CapacityFieldState extends State<_CapacityField> {
  late final TextEditingController _controller = TextEditingController(
    text: widget.initial?.toString() ?? '',
  );

  void _commit(String raw) {
    final trimmed = raw.trim();
    if (trimmed.isEmpty) {
      widget.onChanged(null);
      return;
    }
    final parsed = double.tryParse(trimmed);
    widget.onChanged(parsed);
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return KwaaiTextField(
      controller: _controller,
      enabled: widget.enabled,
      hintText: 'e.g. 10',
      onSubmitted: _commit,
      onEditingComplete: () => _commit(_controller.text),
    );
  }
}

/// Pinned bar at the bottom of the Settings shell card. Visible only when
/// the user has applied feature changes that need a service restart.
/// Bottom status bar of the Settings card. Shows either a daemon error
/// (red) or a restart-needed prompt (orange) — error takes priority.
/// Empty when neither applies. Replaces the old `_RestartNeededBar` and
/// the in-card error Card, so the user always sees these in the same
/// spot regardless of which Settings tab they're on.
class _RestartNeededBar extends ConsumerWidget {
  const _RestartNeededBar();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    // Bottom radii match the settings content card (bottom-left small
    // because the nav card sits to the left, bottom-right outer because
    // it touches the window). Top corners stay sharp — the bar meets
    // the card content above it with a flat top edge + divider line.
    final bottomRadius = BorderRadius.only(
      bottomLeft: const Radius.circular(kShellInnerRadius),
      bottomRight: const Radius.circular(kShellOuterRadius),
    );

    final err = ref.watch(daemonErrorProvider);
    if (err != null) {
      return KwaaiStatusBar(
        severity: KwaaiStatusSeverity.error,
        message: err,
        onDismiss: () => ref.read(daemonErrorProvider.notifier).clear(),
        bottomRadius: bottomRadius,
      );
    }

    final needsRestart = ref.watch(restartNeededProvider);
    if (!needsRestart) return const SizedBox.shrink();
    // No restart prompt when the service is stopped — the new config
    // will be picked up the next time the user starts it. Also clear the
    // flag so it doesn't reappear once they start the service again.
    final running = ref.watch(daemonStatusProvider).valueOrNull?.running
        ?? false;
    if (!running) {
      WidgetsBinding.instance.addPostFrameCallback((_) {
        ref.read(restartNeededProvider.notifier).clear();
      });
      return const SizedBox.shrink();
    }

    final transition = ref.watch(daemonTransitionProvider);
    final busy = transition != DaemonTransition.none;
    return KwaaiStatusBar(
      severity: KwaaiStatusSeverity.info,
      message: 'Restart the service to apply your changes.',
      action: KwaaiButton(
        label: 'Restart service',
        onPressed: busy ? null : () => _restart(ref),
      ),
      bottomRadius: bottomRadius,
    );
  }

  /// Stop the daemon, then start it. The transition provider clears its
  /// state when the watcher confirms each step, so the chat-area overlay
  /// and tray menu reflect Stopping… → Starting… → Running automatically.
  Future<void> _restart(WidgetRef ref) async {
    final transition = ref.read(daemonTransitionProvider.notifier);
    // If currently running, stop first. If already stopped, skip ahead.
    final status = ref.read(daemonStatusProvider).valueOrNull;
    if (status?.running ?? false) {
      await transition.stop();
      // Wait for the stopping transition to clear before kicking start.
      while (ref.read(daemonTransitionProvider) == DaemonTransition.stopping) {
        await Future<void>.delayed(const Duration(milliseconds: 200));
      }
    }
    await transition.start();
    ref.read(restartNeededProvider.notifier).clear();
  }
}

class _AppearanceTab extends StatelessWidget {
  const _AppearanceTab();

  @override
  Widget build(BuildContext context) {
    final theme = ThemeScope.of(context);
    final state = theme.state;
    final brightness = MediaQuery.platformBrightnessOf(context);
    final activeBrightness = state.effectiveBrightness(brightness);

    return SingleChildScrollView(
      padding: const EdgeInsets.all(16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const KwaaiHeading('Mode'),
          const SizedBox(height: 8),
          _ModePicker(current: state.mode, onChanged: theme.setMode),
          const SizedBox(height: 24),
          const KwaaiHeading('Light theme'),
          const SizedBox(height: 8),
          _VariantPicker(
            brightness: Brightness.light,
            current: state.lightVariant,
            isActive: activeBrightness == Brightness.light,
            onChanged: theme.setLightVariant,
          ),
          const SizedBox(height: 24),
          const KwaaiHeading('Dark theme'),
          const SizedBox(height: 8),
          _VariantPicker(
            brightness: Brightness.dark,
            current: state.darkVariant,
            isActive: activeBrightness == Brightness.dark,
            onChanged: theme.setDarkVariant,
          ),
        ],
      ),
    );
  }
}

class _ModePicker extends StatelessWidget {
  const _ModePicker({required this.current, required this.onChanged});
  final AppThemeMode current;
  final ValueChanged<AppThemeMode> onChanged;

  @override
  Widget build(BuildContext context) {
    final accent = context.kwaai.accentPrimary;
    final onSurface = Theme.of(context).colorScheme.onSurface;
    final focused = WindowFocusScope.of(context);
    // Selected fill desaturates to gray when the window is unfocused.
    final selectedFill = focused ? accent : kSelectedUnfocusedFill;
    final selectedFg = focused ? Colors.white : onSurface;
    // Unselected segments mirror a disabled FilledButton (onSurface @ 12%
    // fill) — but with lighter, still-legible text rather than the 38% a
    // real disabled button uses. Theme-aware: flips for light/dark.
    final unselectedFill = onSurface.withValues(alpha: 0.12);
    final unselectedFg = onSurface.withValues(alpha: 0.6);

    return SegmentedButton<AppThemeMode>(
      style: ButtonStyle(
        // Selected segment fills with the theme accent; unselected uses the
        // disabled-button fill. No border on any segment.
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
      ),
      showSelectedIcon: false,
      segments: const [
        ButtonSegment(
          value: AppThemeMode.auto,
          label: Text('Auto'),
          icon: Icon(Icons.brightness_auto),
        ),
        ButtonSegment(
          value: AppThemeMode.light,
          label: Text('Light'),
          icon: Icon(Icons.light_mode),
        ),
        ButtonSegment(
          value: AppThemeMode.dark,
          label: Text('Dark'),
          icon: Icon(Icons.dark_mode),
        ),
      ],
      selected: {current},
      onSelectionChanged: (s) => onChanged(s.first),
    );
  }
}

class _VariantPicker extends StatelessWidget {
  const _VariantPicker({
    required this.brightness,
    required this.current,
    required this.isActive,
    required this.onChanged,
  });
  final Brightness brightness;
  final ThemeVariantKey current;
  final bool isActive;
  final ValueChanged<ThemeVariantKey> onChanged;

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Wrap(
          spacing: 10,
          runSpacing: 10,
          children: ThemeVariantKey.values
              .map(
                (v) => _ThemeVariantDot(
                  color: v.swatch(brightness),
                  label: v.displayName,
                  isSelected: v == current,
                  onTap: () => onChanged(v),
                ),
              )
              .toList(),
        ),
        const SizedBox(height: 8),
        Text(
          isActive
              ? '${current.displayName} — ${current.description} (active)'
              : '${current.displayName} — ${current.description}',
          style: Theme.of(context).textTheme.bodyMedium?.copyWith(
            color: Theme.of(context).colorScheme.onSurfaceVariant,
          ),
        ),
      ],
    );
  }
}

class _ThemeVariantDot extends StatelessWidget {
  const _ThemeVariantDot({
    required this.color,
    required this.label,
    required this.isSelected,
    required this.onTap,
  });

  final Color color;
  final String label;
  final bool isSelected;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final outline = Theme.of(context).colorScheme.outline;
    final selectedColor = Theme.of(context).colorScheme.onSurface;
    return Tooltip(
      message: label,
      child: GestureDetector(
        onTap: onTap,
        child: Container(
          width: 28,
          height: 28,
          decoration: BoxDecoration(
            color: color,
            shape: BoxShape.circle,
            border: Border.all(
              color: isSelected
                  ? selectedColor
                  : outline.withValues(alpha: 0.3),
              width: isSelected ? 2.5 : 1,
            ),
          ),
        ),
      ),
    );
  }
}
