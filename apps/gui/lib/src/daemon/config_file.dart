import 'dart:io';

import 'package:yaml/yaml.dart';
import 'package:yaml_edit/yaml_edit.dart';

import 'paths.dart';

void _log(String msg) {
  stderr.writeln('[config-file] $msg');
}

/// Subset of ~/.kwaainet/config.yaml that the GUI surfaces in the Features
/// settings. Other keys in the file are read but never touched on write.
class ConfigSnapshot {
  const ConfigSnapshot({
    required this.model,
    required this.shardingEnabled,
    required this.storageEnabled,
    required this.storageCapacityGb,
    required this.port,
    required this.publicIp,
    required this.initialPeers,
    required this.forcePrivate,
    required this.healthEnabled,
    required this.healthEndpoint,
  });

  /// Top-level `model` — the HuggingFace model id this node serves.
  final String model;

  /// `contribute.shards` — true if the daemon should auto-start shard
  /// serving on boot.
  final bool shardingEnabled;

  /// `contribute.storage` — true if the daemon should auto-start the
  /// storage API on boot.
  final bool storageEnabled;

  /// `storage.capacity_gb` — how much disk the storage feature offers.
  /// Null when the storage section hasn't been initialised yet.
  final double? storageCapacityGb;

  /// `port` — TCP port the libp2p listener binds. Null = key absent /
  /// daemon picks one.
  final int? port;

  /// `public_ip` — externally-reachable IP this node announces. Empty
  /// string means auto-detect (NAT-PMP / observed address).
  final String publicIp;

  /// `initial_peers` — list of multiaddrs used to bootstrap the DHT.
  final List<String> initialPeers;

  /// `force_private` — when true, the daemon assumes its reachability
  /// is "Private" and skips AutoNAT probing. Forces relayed connections
  /// (no hole-punching attempts on incoming dials).
  final bool forcePrivate;

  /// `health_monitoring.enabled`.
  final bool healthEnabled;

  /// `health_monitoring.api_endpoint`.
  final String healthEndpoint;

  ConfigSnapshot copyWith({
    String? model,
    bool? shardingEnabled,
    bool? storageEnabled,
    double? storageCapacityGb,
    int? port,
    String? publicIp,
    List<String>? initialPeers,
    bool? forcePrivate,
    bool? healthEnabled,
    String? healthEndpoint,
  }) {
    return ConfigSnapshot(
      model: model ?? this.model,
      shardingEnabled: shardingEnabled ?? this.shardingEnabled,
      storageEnabled: storageEnabled ?? this.storageEnabled,
      storageCapacityGb: storageCapacityGb ?? this.storageCapacityGb,
      port: port ?? this.port,
      publicIp: publicIp ?? this.publicIp,
      initialPeers: initialPeers ?? this.initialPeers,
      forcePrivate: forcePrivate ?? this.forcePrivate,
      healthEnabled: healthEnabled ?? this.healthEnabled,
      healthEndpoint: healthEndpoint ?? this.healthEndpoint,
    );
  }
}

class ConfigFile {
  ConfigFile._();

  /// Path to ~/.kwaainet/config.yaml.
  static String get path => KwaainetPaths.configFile;

  static const ConfigSnapshot _defaults = ConfigSnapshot(
    model: '',
    shardingEnabled: true,
    storageEnabled: true,
    storageCapacityGb: null,
    port: null,
    publicIp: '',
    initialPeers: [],
    forcePrivate: false,
    healthEnabled: true,
    healthEndpoint: '',
  );

  /// Load the current config, falling back to sensible defaults if the file
  /// is missing or fields are unset.
  static Future<ConfigSnapshot> load() async {
    final f = File(path);
    if (!f.existsSync()) {
      _log('config.yaml missing — returning defaults');
      return _defaults;
    }
    try {
      final raw = f.readAsStringSync();
      final doc = loadYaml(raw);
      if (doc is! YamlMap) {
        _log('config.yaml is not a map — returning defaults');
        return _defaults;
      }
      final model = (doc['model'] as String?) ?? '';
      final contribute = doc['contribute'];
      final shardingEnabled = contribute is YamlMap
          ? (contribute['shards'] as bool? ?? true)
          : true;
      final storageEnabled = contribute is YamlMap
          ? (contribute['storage'] as bool? ?? true)
          : true;
      final storage = doc['storage'];
      final storageCapacityGb = storage is YamlMap
          ? (storage['capacity_gb'] as num?)?.toDouble()
          : null;
      final port = (doc['port'] as num?)?.toInt();
      final publicIp = (doc['public_ip'] as String?) ?? '';
      final rawPeers = doc['initial_peers'];
      final initialPeers = rawPeers is YamlList
          ? rawPeers.map((e) => e.toString()).toList()
          : <String>[];
      final forcePrivate = (doc['force_private'] as bool?) ?? false;
      final health = doc['health_monitoring'];
      final healthEnabled = health is YamlMap
          ? (health['enabled'] as bool? ?? true)
          : true;
      final healthEndpoint = health is YamlMap
          ? (health['api_endpoint'] as String? ?? '')
          : '';
      return ConfigSnapshot(
        model: model,
        shardingEnabled: shardingEnabled,
        storageEnabled: storageEnabled,
        storageCapacityGb: storageCapacityGb,
        port: port,
        publicIp: publicIp,
        initialPeers: initialPeers,
        forcePrivate: forcePrivate,
        healthEnabled: healthEnabled,
        healthEndpoint: healthEndpoint,
      );
    } catch (e) {
      _log('failed to parse config.yaml: $e — returning defaults');
      return _defaults;
    }
  }

  /// Write [updated] back to ~/.kwaainet/config.yaml, preserving all other
  /// keys + comments + ordering via [YamlEditor].
  ///
  /// Only the keys controlled by the Features UI are touched.
  static Future<void> save(ConfigSnapshot updated) async {
    final f = File(path);
    final dir = Directory(KwaainetPaths.home);
    if (!dir.existsSync()) dir.createSync(recursive: true);

    final existing = f.existsSync() ? f.readAsStringSync() : '';
    final editor = YamlEditor(existing.isEmpty ? '{}\n' : existing);

    _setScalar(editor, ['model'], updated.model);
    _setScalar(editor, ['contribute', 'shards'], updated.shardingEnabled);
    _setScalar(editor, ['contribute', 'storage'], updated.storageEnabled);
    if (updated.storageCapacityGb != null) {
      _setScalar(editor, ['storage', 'capacity_gb'], updated.storageCapacityGb);
    }
    _setScalar(editor, ['port'], updated.port);
    _setScalar(editor, ['public_ip'], updated.publicIp);
    _setScalar(editor, ['initial_peers'], updated.initialPeers);
    _setScalar(editor, ['force_private'], updated.forcePrivate);
    _setScalar(
      editor,
      ['health_monitoring', 'enabled'],
      updated.healthEnabled,
    );
    _setScalar(
      editor,
      ['health_monitoring', 'api_endpoint'],
      updated.healthEndpoint,
    );

    f.writeAsStringSync(editor.toString());
    _log('wrote ${f.path}');
  }

  /// Update a single key path. yaml_edit's [YamlEditor.update] throws if any
  /// parent is missing — walk the path and create intermediate maps as
  /// needed.
  static void _setScalar(
    YamlEditor editor,
    List<String> keyPath,
    Object? value,
  ) {
    for (var i = 0; i < keyPath.length - 1; i++) {
      final parentPath = keyPath.sublist(0, i + 1);
      try {
        editor.parseAt(parentPath);
      } catch (_) {
        // Parent missing — create an empty map for it.
        editor.update(parentPath, <String, dynamic>{});
      }
    }
    editor.update(keyPath, value);
  }
}
