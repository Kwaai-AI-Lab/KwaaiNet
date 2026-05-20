import Cocoa
import FlutterMacOS

class MainFlutterWindow: NSWindow {
  override func awakeFromNib() {
    let flutterViewController = FlutterViewController()
    let windowFrame = self.frame
    self.contentViewController = flutterViewController
    self.setFrame(windowFrame, display: true)

    // Persist + restore window position/size via AppKit before first paint.
    // No flicker: the frame is set before the window is shown.
    self.setFrameAutosaveName("MainWindow")

    RegisterGeneratedPlugins(registry: flutterViewController)

    // Install the lifecycle channel (setDockIconVisible) before Dart's
    // first call — installing it later would race with WindowCloseHandler.
    if let delegate = NSApp.delegate as? AppDelegate {
      delegate.installLifecycleChannel(
        messenger: flutterViewController.engine.binaryMessenger
      )
    }

    super.awakeFromNib()
  }
}
