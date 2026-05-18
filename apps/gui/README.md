# kwaainet_gui

Flutter desktop GUI for KwaaiNet. Runs on macOS, Linux, and Windows. Talks
to the `kwaainet` daemon over gRPC and ships with a menu-bar tray, a chat
view backed by the daemon's shard mesh, and a developer-mode local-chat
tab that drives `kwaainet generate` directly.

## Prerequisites

- Flutter SDK **3.11.4** or newer (`flutter --version`)
- A working desktop toolchain for your platform:
  - macOS: Xcode command-line tools
  - Linux: GTK 3 / clang / ninja / pkg-config (see
    [Flutter Linux setup](https://docs.flutter.dev/get-started/install/linux/desktop))
  - Windows: Visual Studio with the "Desktop development with C++" workload
- A `kwaainet` binary on `PATH`, or a checkout of this repo if you want
  the GUI to drive a debug build of the daemon (see "Daemon modes" below)

## Build & run

```bash
cd apps/gui
flutter pub get
flutter run -d macos      # or: -d linux, -d windows
```

## Tests & lint

```bash
cd apps/gui
flutter analyze
flutter test
```

CI runs both on every push and PR (see `.github/workflows/ci.yml`).

## Daemon modes

The Settings page exposes four ways for the GUI to find the `kwaainet`
daemon (see `lib/src/settings.dart` and `lib/src/daemon/daemon_controller.dart`):

| Mode        | Behaviour                                                              |
| ----------- | ---------------------------------------------------------------------- |
| `builtIn`   | Run the debug binary from this checkout (`core/target/debug/kwaainet`) |
| `system`    | Find `kwaainet` on `PATH`                                              |
| `custom`    | Run a binary at a user-chosen path                                     |
| `external`  | Don't manage the daemon — assume something else (launchd, systemd,     |
|             | Docker, a shell) is already running it                                 |

The first three modes spawn and supervise the daemon process; the GUI
writes the PID to a state file and surfaces start/stop controls. In
`external` mode the GUI is read-only and only probes the daemon over
gRPC.

## gRPC bindings

The Dart bindings under `lib/src/chat/generated/` are generated from
`core/crates/kwaai-rpc/proto/kwaai.proto`. See
[`lib/src/chat/generated/README.md`](lib/src/chat/generated/README.md)
for how to regenerate them.

## Layout

```
apps/gui/
├── lib/
│   ├── main.dart
│   └── src/
│       ├── chat/        # gRPC client, chat state, generated bindings
│       ├── daemon/      # daemon lifecycle (spawn/probe/stop)
│       ├── settings.dart
│       ├── tray/        # menu-bar tray integration
│       ├── ui/          # widgets and pages
│       └── window/      # window/lifecycle wiring
├── macos/  linux/  windows/   # platform shells
└── test/
```
