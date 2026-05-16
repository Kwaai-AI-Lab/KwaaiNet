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

  /// Seed the draft from the on-disk values if it hasn't been started.
  void seed(ConfigSnapshot loaded) {
    state ??= loaded;
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
    state = ConfigSnapshot(
      model: s.model,
      shardingEnabled: s.shardingEnabled,
      storageEnabled: s.storageEnabled,
      storageCapacityGb: v,
    );
  }

  /// True when the draft differs from the on-disk snapshot.
  bool isDirty(ConfigSnapshot onDisk) {
    final s = state;
    if (s == null) return false;
    return s.model != onDisk.model ||
        s.shardingEnabled != onDisk.shardingEnabled ||
        s.storageEnabled != onDisk.storageEnabled ||
        s.storageCapacityGb != onDisk.storageCapacityGb;
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
