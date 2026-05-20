import Cocoa
import FlutterMacOS

@main
class AppDelegate: FlutterAppDelegate {
  private var lifecycleChannel: FlutterMethodChannel?

  override func applicationShouldTerminateAfterLastWindowClosed(_ sender: NSApplication) -> Bool {
    // Let the Flutter-side WindowCloseHandler decide whether to quit or
    // hide to the menu-bar tray. Returning true here pre-empts that.
    return false
  }

  override func applicationSupportsSecureRestorableState(_ app: NSApplication) -> Bool {
    return true
  }

  /// Dock-icon click / Finder re-launch while the app is hidden. Do the
  /// window-front work natively so the user gets an immediate response
  /// even if the Dart isolate is busy (post-sleep wake, GC, etc.) —
  /// previously this round-tripped through Dart, which let AppKit decide
  /// the app was unresponsive and swap the Dock menu's Quit for Force
  /// Quit. Dart is still notified afterward so any in-Dart "window
  /// visible" state stays in sync.
  override func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
    NSApp.setActivationPolicy(.regular)
    NSApp.unhide(nil)
    NSApp.activate(ignoringOtherApps: true)
    for window in sender.windows {
      if window.isMiniaturized {
        window.deminiaturize(nil)
      }
      window.setIsVisible(true)
      window.makeKeyAndOrderFront(nil)
    }
    lifecycleChannel?.invokeMethod("reopenWindow", arguments: nil)
    return true
  }

  /// Wires the `kwaai/lifecycle` method channel. Called from
  /// MainFlutterWindow.awakeFromNib so the channel is registered before
  /// Dart's first call.
  func installLifecycleChannel(messenger: FlutterBinaryMessenger) {
    guard lifecycleChannel == nil else { return }
    let channel = FlutterMethodChannel(
      name: "kwaai/lifecycle",
      binaryMessenger: messenger
    )
    channel.setMethodCallHandler { call, result in
      switch call.method {
      case "setDockIconVisible":
        let visible = (call.arguments as? Bool) ?? true
        NSApp.setActivationPolicy(visible ? .regular : .accessory)
        if visible {
          NSApp.activate(ignoringOtherApps: true)
        }
        result(nil)
      default:
        result(FlutterMethodNotImplemented)
      }
    }
    lifecycleChannel = channel
  }
}
