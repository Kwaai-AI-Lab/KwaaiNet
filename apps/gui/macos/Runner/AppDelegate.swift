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

  /// Dock-icon click / Finder re-launch while the app is hidden. Tell Dart
  /// to show its window — the close handler had set the activation policy
  /// to .accessory which hid the Dock icon; restoring it goes back to
  /// .regular and brings the window to front.
  override func applicationShouldHandleReopen(_ sender: NSApplication, hasVisibleWindows flag: Bool) -> Bool {
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
