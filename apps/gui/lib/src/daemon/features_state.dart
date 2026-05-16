import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'config_file.dart';

/// The currently-persisted feature config (mirrors what's in
/// ~/.kwaainet/config.yaml). Refresh via `ref.invalidate(featuresProvider)`
/// after writing.
final featuresProvider = FutureProvider<ConfigSnapshot>((ref) async {
  return ConfigFile.load();
});

/// Holds an in-memory draft of feature settings the user is editing in the
/// Features tab. Initialised from [featuresProvider] on first access; cleared
/// when [apply()] persists the draft to config.yaml.
class FeaturesDraftNotifier extends Notifier<ConfigSnapshot?> {
  @override
  ConfigSnapshot? build() => null;

  /// Seed the draft from the on-disk values if it hasn't been started,
  /// or if the prior draft was constructed against a different schema
  /// (e.g. across a hot-reload that added new ConfigSnapshot fields —
  /// without this, reads of the new fields throw a non-null assertion).
  void seed(ConfigSnapshot loaded) {
    final s = state;
    if (s == null || !_hasAllFields(s)) {
      state = loaded;
    }
  }

  /// Defensive: touch every field to surface any null-from-stale-shape
  /// errors here, so we can re-seed instead of crashing during build.
  bool _hasAllFields(ConfigSnapshot s) {
    try {
      s.model;
      s.shardingEnabled;
      s.storageEnabled;
      s.storageCapacityGb;
      s.port;
      s.publicIp;
      s.initialPeers;
      s.forcePrivate;
      s.healthEnabled;
      s.healthEndpoint;
      return true;
    } catch (_) {
      return false;
    }
  }

  void setModel(String v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(model: v);
  }

  void setShardingEnabled(bool v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(shardingEnabled: v);
  }

  void setStorageEnabled(bool v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(storageEnabled: v);
  }

  void setStorageCapacityGb(double? v) {
    final s = state;
    if (s == null) return;
    // copyWith treats null as "leave unchanged", so we have to rebuild
    // the snapshot to clear a previously-set capacity.
    state = ConfigSnapshot(
      model: s.model,
      shardingEnabled: s.shardingEnabled,
      storageEnabled: s.storageEnabled,
      storageCapacityGb: v,
      port: s.port,
      publicIp: s.publicIp,
      initialPeers: s.initialPeers,
      forcePrivate: s.forcePrivate,
      healthEnabled: s.healthEnabled,
      healthEndpoint: s.healthEndpoint,
    );
  }

  void setPort(int? v) {
    final s = state;
    if (s == null) return;
    state = ConfigSnapshot(
      model: s.model,
      shardingEnabled: s.shardingEnabled,
      storageEnabled: s.storageEnabled,
      storageCapacityGb: s.storageCapacityGb,
      port: v,
      publicIp: s.publicIp,
      initialPeers: s.initialPeers,
      forcePrivate: s.forcePrivate,
      healthEnabled: s.healthEnabled,
      healthEndpoint: s.healthEndpoint,
    );
  }

  void setPublicIp(String v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(publicIp: v);
  }

  void setInitialPeers(List<String> v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(initialPeers: v);
  }

  void setForcePrivate(bool v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(forcePrivate: v);
  }

  void setHealthEnabled(bool v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(healthEnabled: v);
  }

  void setHealthEndpoint(String v) {
    final s = state;
    if (s == null) return;
    state = s.copyWith(healthEndpoint: v);
  }

  /// True when the draft differs from the on-disk snapshot.
  bool isDirty(ConfigSnapshot onDisk) {
    final s = state;
    if (s == null) return false;
    return s.model != onDisk.model ||
        s.shardingEnabled != onDisk.shardingEnabled ||
        s.storageEnabled != onDisk.storageEnabled ||
        s.storageCapacityGb != onDisk.storageCapacityGb ||
        s.port != onDisk.port ||
        s.publicIp != onDisk.publicIp ||
        !_listsEqual(s.initialPeers, onDisk.initialPeers) ||
        s.forcePrivate != onDisk.forcePrivate ||
        s.healthEnabled != onDisk.healthEnabled ||
        s.healthEndpoint != onDisk.healthEndpoint;
  }

  static bool _listsEqual(List<String> a, List<String> b) {
    if (a.length != b.length) return false;
    for (var i = 0; i < a.length; i++) {
      if (a[i] != b[i]) return false;
    }
    return true;
  }

  /// Persist the draft to config.yaml. Returns the snapshot that was
  /// written so callers can refresh other state.
  Future<ConfigSnapshot> apply() async {
    final s = state;
    if (s == null) throw StateError('No draft to apply');
    await ConfigFile.save(s);
    return s;
  }
}

final featuresDraftProvider =
    NotifierProvider<FeaturesDraftNotifier, ConfigSnapshot?>(
      FeaturesDraftNotifier.new,
    );

/// True when the user has applied feature changes that need a service
/// restart to take effect. Cleared by [clear()] when the daemon next
/// reports it has been restarted (or the user manually dismisses it).
class RestartNeededNotifier extends Notifier<bool> {
  @override
  bool build() => false;

  void mark() => state = true;
  void clear() => state = false;
}

final restartNeededProvider = NotifierProvider<RestartNeededNotifier, bool>(
  RestartNeededNotifier.new,
);
