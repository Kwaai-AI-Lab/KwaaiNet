import 'package:flutter/material.dart';
import 'package:window_manager/window_manager.dart';

/// Tracks whether the native app window is currently focused. Used to drive
/// shadow + tint changes on themed surfaces.
class WindowFocusNotifier extends ChangeNotifier with WindowListener {
  bool _focused = true;
  bool get focused => _focused;

  void attach() {
    windowManager.addListener(this);
    windowManager.isFocused().then((v) {
      if (v != _focused) {
        _focused = v;
        notifyListeners();
      }
    });
  }

  @override
  void dispose() {
    windowManager.removeListener(this);
    super.dispose();
  }

  @override
  void onWindowFocus() {
    if (_focused) return;
    _focused = true;
    notifyListeners();
  }

  @override
  void onWindowBlur() {
    if (!_focused) return;
    _focused = false;
    notifyListeners();
  }
}

class WindowFocusScope extends InheritedNotifier<WindowFocusNotifier> {
  const WindowFocusScope({
    super.key,
    required WindowFocusNotifier notifier,
    required super.child,
  }) : super(notifier: notifier);

  static bool of(BuildContext context) {
    final scope = context
        .dependOnInheritedWidgetOfExactType<WindowFocusScope>();
    return scope?.notifier?.focused ?? true;
  }
}
