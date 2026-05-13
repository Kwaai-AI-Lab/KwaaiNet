import 'package:file_picker/file_picker.dart';
import 'package:flutter/material.dart';

import 'branded_title.dart';
import 'daemon_controller.dart';
import 'settings.dart';
import 'status_watcher.dart';

class SettingsPage extends StatefulWidget {
  const SettingsPage({
    super.key,
    required this.daemon,
    required this.settings,
    required this.statusStream,
    required this.onSettingsChanged,
  });

  final DaemonController daemon;
  final Settings settings;
  final Stream<NodeStatus> statusStream;
  final VoidCallback onSettingsChanged;

  @override
  State<SettingsPage> createState() => _SettingsPageState();
}

class _SettingsPageState extends State<SettingsPage> {
  NodeStatus _status = NodeStatus.stopped();
  String? _lastError;
  int _selectedTab = 0;

  @override
  void initState() {
    super.initState();
    widget.statusStream.listen((s) {
      if (mounted) setState(() => _status = s);
    });
  }

  Future<void> _start() async {
    setState(() => _lastError = null);
    final r = await widget.daemon.start();
    if (!r.ok && mounted) {
      setState(() => _lastError = r.error ?? 'start failed');
    }
  }

  Future<void> _stop() async {
    setState(() => _lastError = null);
    final ok = await widget.daemon.stop();
    if (!ok && mounted) setState(() => _lastError = 'stop failed');
  }

  Widget _buildStatusTab() {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Padding(
          padding: const EdgeInsets.fromLTRB(16, 16, 16, 0),
          child: _StatusHeader(status: _status),
        ),
        Expanded(
          child: SingleChildScrollView(
            padding: const EdgeInsets.fromLTRB(16, 16, 16, 16),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Row(
                  children: [
                    FilledButton.icon(
                      onPressed: _status.running ? null : _start,
                      icon: const Icon(Icons.play_arrow),
                      label: const Text('Start daemon'),
                    ),
                    const SizedBox(width: 8),
                    OutlinedButton.icon(
                      onPressed: _status.running ? _stop : null,
                      icon: const Icon(Icons.stop),
                      label: const Text('Stop daemon'),
                    ),
                  ],
                ),
                if (_lastError != null) ...[
                  const SizedBox(height: 12),
                  Card(
                    color: Theme.of(context).colorScheme.errorContainer,
                    child: Padding(
                      padding: const EdgeInsets.all(12),
                      child: Text(
                        _lastError!,
                        style: TextStyle(
                          color: Theme.of(context).colorScheme.onErrorContainer,
                        ),
                      ),
                    ),
                  ),
                ],
                const SizedBox(height: 24),
                Text(
                  'Daemon source',
                  style: Theme.of(
                    context,
                  ).textTheme.bodyMedium?.copyWith(fontWeight: FontWeight.w600),
                ),
                const SizedBox(height: 8),
                _DaemonSourcePicker(
                  daemon: widget.daemon,
                  settings: widget.settings,
                  onChanged: () {
                    setState(() {});
                    widget.onSettingsChanged();
                  },
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        automaticallyImplyLeading: false,
        leadingWidth: 220,
        leading: const Padding(
          padding: EdgeInsets.only(left: 16),
          child: BrandedTitle(),
        ),
        centerTitle: true,
        title: Text(
          'Settings',
          style: Theme.of(
            context,
          ).textTheme.titleLarge?.copyWith(fontWeight: FontWeight.w600),
        ),
        actions: [
          IconButton(
            tooltip: 'Close',
            icon: const Icon(Icons.close),
            onPressed: () => Navigator.of(context).pop(),
          ),
        ],
      ),
      body: LayoutBuilder(
        builder: (context, constraints) {
          final extended = constraints.maxWidth >= 720;
          return Row(
            children: [
              NavigationRail(
                selectedIndex: _selectedTab,
                onDestinationSelected: (i) => setState(() => _selectedTab = i),
                extended: extended,
                labelType: extended ? null : NavigationRailLabelType.none,
                minWidth: 56,
                minExtendedWidth: 160,
                destinations: const [
                  NavigationRailDestination(
                    icon: Icon(Icons.monitor_heart_outlined),
                    selectedIcon: Icon(Icons.monitor_heart),
                    label: Text('Status'),
                  ),
                ],
              ),
              const VerticalDivider(width: 1),
              Expanded(
                child: IndexedStack(
                  index: _selectedTab,
                  children: [_buildStatusTab()],
                ),
              ),
            ],
          );
        },
      ),
    );
  }
}

class _StatusHeader extends StatelessWidget {
  const _StatusHeader({required this.status});
  final NodeStatus status;

  @override
  Widget build(BuildContext context) {
    final cs = Theme.of(context).colorScheme;
    final color = status.running ? Colors.green : Colors.grey;
    final bits = <String>[
      status.running ? 'Running' : 'Stopped',
      if (status.running && status.pid != null) 'pid ${status.pid}',
      if (status.uptimeSecs != null) 'up ${_fmtDuration(status.uptimeSecs!)}',
      if (status.memoryMb != null) '${status.memoryMb!.toStringAsFixed(0)} MB',
      if (status.cpuPercent != null)
        '${status.cpuPercent!.toStringAsFixed(1)}% CPU',
    ];

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
    return RadioGroup<DaemonMode>(
      groupValue: mode,
      onChanged: (m) {
        if (m == null) return;
        _set(m);
      },
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const RadioListTile<DaemonMode>(
            title: Text('Use built-in (dev: core/target/debug/kwaainet)'),
            value: DaemonMode.builtIn,
          ),
          const RadioListTile<DaemonMode>(
            title: Text('Use system'),
            value: DaemonMode.system,
          ),
          _PathRow(child: _SystemPathResult(daemon: widget.daemon)),
          const RadioListTile<DaemonMode>(
            title: Text('Use other…'),
            value: DaemonMode.custom,
          ),
          if (mode == DaemonMode.custom)
            _PathRow(
              child: TextField(
                controller: _pathController,
                decoration: InputDecoration(
                  hintText: '/path/to/kwaainet',
                  isDense: true,
                  border: const OutlineInputBorder(),
                  suffixIcon: IconButton(
                    tooltip: 'Browse…',
                    icon: const Icon(Icons.folder_open),
                    onPressed: _browseForPath,
                  ),
                ),
                onSubmitted: _commitCustomPath,
                onEditingComplete: () =>
                    _commitCustomPath(_pathController.text),
              ),
            ),
        ],
      ),
    );
  }
}

class _PathRow extends StatelessWidget {
  const _PathRow({required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(72, 0, 16, 12),
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
        style: Theme.of(context).textTheme.bodyMedium?.copyWith(
          color: Theme.of(context).colorScheme.error,
        ),
      );
    }
    return SelectableText(
      path,
      style: const TextStyle(fontFamily: 'monospace'),
    );
  }
}
