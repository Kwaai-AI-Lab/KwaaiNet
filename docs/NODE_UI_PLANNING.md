# KwaaiNet Node UI – Planning

**Purpose:** A local UI so users can see what’s going on with their KwaaiNet node without using the terminal.

**Status:** Planning  
**Last updated:** 2025-03

---

## Alignment with Kwaai

The dashboard follows [Kwaai's official debugging and troubleshooting guide](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md) ([troubleshooting section](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md#troubleshooting-bootstrap-connect-failed-and-broken-pipe-os-error-32)):

- **Bootstrap / map visibility:** Terminology (bootstrap-1.kwaai.ai, bootstrap-2.kwaai.ai, TCP 8000, Broken pipe), the four-step checklist (bootstrap status, outbound connectivity, firewall, retry), and the "node process is fine; network/infrastructure issue" message match the doc.
- **Logs:** Node log (kwaainet.log) and shard log (shard.log) are the only logs, as KwaaiNet intends; bootstrap/DHT/map messages are in the node log.
- **Network page:** Reconnect P2P, map status, and troubleshooting links point to the same guide so the whole system is consistent with main Kwaai intention.

---

## 1. Goals

- **Single place** to see node status, config, logs, identity, and (optionally) network context.
- **First-run setup flow** — when the UI is started and the node is not yet set up, guide the user through setup (create config, identity, benchmark, then start node) instead of showing an empty dashboard.
- **No replacement** for [map.kwaai.ai](https://map.kwaai.ai) — that stays the network-wide view; this UI is **local node only**.
- **Optional** — node runs fine with CLI only; UI is a convenience layer.
- **Local-first** — runs on the same machine as the node (or same LAN); no required cloud account.

---

## 2. What the UI Should Show (Data Sources)

All of this is already available via the `kwaainet` CLI or local files. The UI would surface it in one place.

| Area | Source | Notes |
|------|--------|--------|
| **Node status** | `kwaainet status` | Running/not running, PID, uptime, CPU %, memory (MB), shard server status |
| **Configuration** | `~/.kwaainet/config.yaml` or `kwaainet config` | model, blocks, port, use_gpu, public_name, public_ip, announce_addr, etc. |
| **Logs** | `kwaainet logs` / `~/.kwaainet/logs/kwaainet.log` | Live tail or last N lines |
| **Identity** | `kwaainet identity show` | DID, Peer ID, trust tier, credential summary |
| **Health** | `kwaainet health-status` | Health monitoring on/off and status |
| **P2P / monitor** | `kwaainet monitor stats` | Connection stats (if exposed in a parseable way) |
| **Shard** | `kwaainet shard status` (if applicable) | Local shard block range, role |
| **Network context** | [map.kwaai.ai API](https://map.kwaai.ai) (e.g. `/api/v1/state`) | Optional: see where this node appears on the map, bootstrap state |

Actions the UI could trigger (by shelling out or later via a small local API):

- Start / stop / restart node: `kwaainet start --daemon`, `kwaainet stop`, `kwaainet restart`
- Open config in editor or “edit in UI” (write `config.yaml` and optionally restart)
- Link to “View on map” → open map.kwaai.ai (and optionally highlight this node by Peer ID if the map supports it)

---

## 3. First-Run Setup Flow (When the UI Starts)

When the user opens the UI for the first time — or when `~/.kwaainet/` does not exist or has no valid config/identity — **the UI should take them through setup** instead of showing an empty or broken dashboard.

### 3.1 Detection

- Backend checks: does `~/.kwaainet/config.yaml` exist and is it valid? Does identity exist (e.g. `~/.kwaainet/identity.key` or equivalent)?
- API: e.g. `GET /api/setup-status` returns `{ "needs_setup": true|false, "step": "none"|"config"|"identity"|"benchmark"|"ready" }` so the frontend can show the right screen.

### 3.2 Setup Steps (guided in the UI)

| Step | CLI equivalent | What the UI does |
|------|----------------|------------------|
| **1. Initialize** | `kwaainet setup` | Create `~/.kwaainet/`, default config, identity. Show short explanation; one button: "Initialize KwaaiNet". |
| **2. Config (optional edit)** | `kwaainet config set ...` | Show key settings (public name, public IP, model, blocks, GPU). User can accept defaults or edit; save writes config. |
| **3. Dependencies (if needed)** | `kwaainet setup --get-deps` | If `p2pd` is missing, offer "Download p2pd" and run it. |
| **4. Benchmark** | `kwaainet benchmark` | "Measure your hardware" — run benchmark, show progress and result (e.g. throughput tok/s). Optional "Skip for now". |
| **5. Start node** | `kwaainet start --daemon` | "Start your node" — one click to start; then redirect to main dashboard (status, logs, etc.). |

User can **skip** non-critical steps (e.g. benchmark, or use defaults for config) but must complete at least **Initialize** (and optionally get-deps) before the dashboard is useful. After setup is done, next time the UI starts it goes straight to the dashboard.

### 3.3 UX Details

- **First visit:** Show a single "Get started" or "Set up your node" flow (wizard or linear steps). Do not show Status/Config/Logs until setup is complete or user explicitly chooses "I'm already set up".
- **Return visit (already set up):** Open directly to dashboard (status, config, logs, identity). Optionally a "Run setup again" or "Re-run benchmark" in settings.
- **Partial setup:** If config exists but identity or benchmark is missing, show the next step in the flow (e.g. "Run benchmark" or "Create identity") with a clear "Continue setup" path.
- **Errors:** If `kwaainet` is not installed or not on PATH, show a clear message: "KwaaiNet CLI not found. Install it first (e.g. run the project's setup script or install from release)." with a link to docs.

This way, **starting the UI takes the user through setup** until the node is ready, then the UI becomes the normal dashboard.

---

## 4. Feature Phases

### Phase 1 – Read-only dashboard (MVP)

- **Status:** Running / not running, PID, uptime, CPU, memory; shard on/off.
- **Config:** Read-only view of main settings (model, blocks, port, public name, etc.).
- **Logs:** Last N lines + “tail” (auto-refresh); optional log level filter.
- **Identity:** DID, Peer ID, trust tier, number of credentials.

Data: run `kwaainet status`, `kwaainet config`, `kwaainet identity show`, and read log file; parse output or (better) add `--json` to these commands for stable parsing.

### Phase 2 – Control + config edit

- **Start / stop / restart** node (and optionally shard) from the UI.
- **Config edit:** Form or key-value edit; write `~/.kwaainet/config.yaml`; optional “restart node after change”.
- **Health:** Toggle health monitoring; show health status.

Requires: safe way to invoke `kwaainet` (e.g. local backend with strict validation and no arbitrary shell).

### Phase 3 – Richer monitoring and network

- **P2P / monitor:** Show connection count, relay vs direct, basic stats (from `kwaainet monitor` or future API).
- **Shard:** If shard is used, show block range, chain status, local role.
- **Network:** “View on map” link; optional embed or iframe of map.kwaai.ai; optional display of “this node” in network state (from map API).

### Phase 4 – Optional enhancements

- **Benchmark / calibrate:** Trigger and show results (e.g. throughput).
- **Updates:** Show version, “check for update”, link to release notes.
- **Alerts:** Notifications when node goes down or health fails (browser/OS notifications or optional system tray).

---

## 5. Architecture Options

### Option A – Web UI + local backend (recommended)

- **Frontend:** Single-page web app (React, Vue, Svelte, or similar); static or served by a tiny server.
- **Backend:** Small local HTTP server (Rust or Node) that:
  - Runs on `127.0.0.1` only (and optionally same-machine-only).
  - Calls `kwaainet` subcommands and/or reads `~/.kwaainet/*` and status files.
  - Exposes JSON API (e.g. `GET /api/status`, `GET /api/config`, `GET /api/logs`, `POST /api/restart`).
  - For actions: validates input and runs only allowlisted `kwaainet` commands (no arbitrary shell).
- **Delivery:** 
  - **Dev:** `npm run dev` (or equivalent) + run backend.
  - **Dist:** Backend can serve the built frontend (one binary or script); user opens `http://127.0.0.1:PORT`.

**Pros:** One clear place for logic and security; UI can be modern and responsive.  
**Cons:** Need to implement and ship the backend.

### Option B – TUI (terminal UI)

- Use a Rust crate (e.g. `ratatui`) inside the existing `kwaainet` CLI or a separate `kwaainet-dashboard` binary.
- Screens: status, config, logs, identity; refresh in place; key bindings for start/stop/restart.

**Pros:** No browser; fits terminal-only workflows.  
**Cons:** Less “dashboard” feel; no easy embedding of map or rich visuals.

### Option C – Electron / Tauri desktop app

- Desktop wrapper around the same web UI; backend can be bundled (e.g. Rust) or call `kwaainet` and read files.

**Pros:** Installable app, possible system tray.  
**Cons:** Heavier distribution and platform-specific builds.

**Recommendation:** Start with **Option A** (web UI + local backend). Option B can be a separate “TUI mode” later. Option C can wrap the same web UI if needed.

---

## 6. Data Access (How to get JSON)

Today the CLI prints human-readable text. For a robust UI we need machine-readable output.

**Short term (Phase 1):**

- **Option 1:** Add `--json` to relevant commands (`kwaainet status --json`, `kwaainet config --json`, `kwaainet identity show --json`). UI backend runs these and parses JSON.
- **Option 2:** Backend reads files only: `~/.kwaainet/config.yaml`, `~/.kwaainet/run/*.pid`, and status file if present; parses log file for “last N lines”. No CLI changes, but status might be less accurate (e.g. process dead but PID file left behind) unless we also probe the process.

**Medium term:**

- Small **local API** in the CLI or a companion process: e.g. `kwaainet ui` that starts the HTTP server and serves the UI + API. API uses existing Rust structs (`NodeStatus`, `KwaaiNetConfig`, etc.) and returns JSON. No parsing of CLI output.

**Recommendation:** Add `--json` for `status`, `config`, and `identity show` in the CLI (minimal change, big win for UI and scripting). Backend uses `--json` and file reads for logs; later replace with a native API if we add `kwaainet ui`.

**CLI JSON schemas (implemented):**
- `kwaainet status --json`: `{ running, pid?, uptime_secs?, cpu_percent?, memory_mb?, shard_running, shard_pid? }`
- `kwaainet config --json` (with show): full `KwaaiNetConfig` (model, blocks, port, use_gpu, public_name, etc.)
- `kwaainet identity show --json`: `{ did, peer_id, trust_tier, score, credential_count }`

---

## 7. Repo Layout (proposed)

Follow existing repo structure (e.g. `/systems` for apps):

```
systems/
  node-dashboard/          # or "node-ui"
    README.md              # Purpose, deps, how to run, ports
    frontend/              # SPA (e.g. Vite + React)
    backend/               # Optional: tiny Rust or Node server
    docs/                  # Optional: UI-specific notes
```

Or under `docs/` only for planning and design, and implementation lives in a separate repo until it’s merged. Either way, a single `README.md` in the UI component should state: purpose, dependencies, setup, which ports/APIs it uses, and that it’s optional.

---

## 8. UI Design (Visual Guidelines)

Align the Node dashboard with Kwaai branding so it feels part of the same ecosystem. Use the following palette and layout cues (reference: Kwaai website).

### 8.1 Color Palette

| Role | Hex | Usage |
|------|-----|--------|
| **Primary light** | `#FFFFFF` | Main content background, cards on dashboard |
| **Primary dark** | `#000000` | Header / navigation bar background |
| **Accent blue** | `#3063e9` | Top info bar, primary links, icon outlines, focus states, "View on map" |
| **Accent purple** | `#78479e` | Primary action buttons (Start, Stop, Restart, Initialize), footer or section bars, setup wizard CTAs |
| **Text on dark** | `#FFFFFF` | Navigation, header, any text on black or purple |
| **Text on light** | `#333333` | Body text on white; secondary copy |
| **Muted / borders** | `#f0f0f0` | Dividers, input borders, secondary panels |
| **Success** | Green tint | e.g. "Node running", success toasts |
| **Error / warning** | Red/amber | Errors, "Node stopped", validation |

Use **accent blue** for links and key info (e.g. "Open map"), **accent purple** for main actions (Start node, Run benchmark, Next in setup wizard).

### 8.2 Layout and Components

- **Header:** Black (`#000000`) bar; left: Kwaai/KwaaiNet logo (e.g. tree icon + "KwaaiNet"); right: nav (Status, Config, Logs, Identity) and optional "View on map" link. White text and icons.
- **Optional top bar:** If you show a single line of context (e.g. "Local node dashboard" or "Setup in progress"), use accent blue (`#3063e9`) with white text; keep it minimal.
- **Main content:** White (`#FFFFFF`) background. Use cards or panels for Status, Config, Logs, Identity with subtle borders or shadow.
- **Primary buttons:** Purple (`#78479e`), white text, slightly rounded corners (e.g. 6–8px). Examples: "Initialize KwaaiNet", "Start node", "Run benchmark", "Next".
- **Secondary actions:** White background, blue border and blue text (`#3063e9`), or text-only blue links.
- **Footer (optional):** Purple (`#78479e`) strip with white text for "KwaaiNet" + link to docs or map.kwaai.ai; or keep footer minimal.

### 8.3 Setup Wizard

- Reuse the same header; content area shows step title and one main CTA per step (e.g. "Initialize", "Run benchmark", "Start node") in purple.
- Use blue for secondary links (e.g. "Skip", "I'm already set up") and for progress or step indicators if needed.

### 8.4 Typography and Accessibility

- Clear hierarchy: one main heading per view; section headings for Status, Config, Logs, Identity.
- Ensure contrast: white on purple/black and dark grey on white meet WCAG AA where applicable.
- Prefer system UI or a simple sans-serif (e.g. Inter, system-ui) for readability.

---

## 9. Security and Safety

- **Binding:** Backend listens only on `127.0.0.1` (and optionally a configurable LAN IP for “view from another device on my network”).
- **No auth by default:** Assumption: only trusted users on the same machine (or LAN). If we ever add auth, it’s a later phase.
- **Actions:** Only allowlisted operations (start/stop/restart, config write with validation). No arbitrary shell or command injection.
- **Paths:** Resolve `~/.kwaainet` and other paths in a predictable way; don’t expose arbitrary file read.

---

## 10. Out of Scope (for this UI)

- **Replacing map.kwaai.ai** — network-wide map and discovery stay there; we only link to it or show “your node” from its API.
- **Running inference in the UI** — chat/completion stays in existing clients (e.g. Open WebUI) talking to `kwaainet serve` or `kwaainet shard api`.
- **Multi-node management** — one UI instance = one node (or one machine). Managing many nodes is a future/admin topic.
- **User/auth/accounts** — no sign-up; local node only.

---

## 11. Success Criteria

- **First run:** When the UI is started and the node is not set up, the user is guided through setup (initialize → config → benchmark → start) and then sees the dashboard.
- **Dashboard:** User can open the UI (e.g. `http://127.0.0.1:3456`) and immediately see:
  - Is my node running? (PID, uptime, CPU, memory)
  - What’s my config? (model, blocks, port, name, etc.)
  - Last log lines (and optionally tail)
  - Identity (DID, Peer ID, trust tier)
- Optionally: one-click start/stop/restart and safe config edit.
- No breaking changes to CLI behavior; existing scripts and docs remain valid.

---

## 12. Next Steps

1. **Decide:** Option A (web + backend) vs TUI-first; and where in repo the code lives.
2. **Backend:** Add `GET /api/setup-status` (needs_setup, step) and allowlisted setup actions (run `kwaainet setup`, `kwaainet benchmark`, `kwaainet start --daemon`, etc.).
3. **CLI:** Add `--json` to `kwaainet status`, `kwaainet config`, `kwaainet identity show` (and document schema).
4. **Backend (dashboard):** Minimal server (e.g. Rust axum or Node Express) on 127.0.0.1 that calls CLI with `--json` and reads log file; expose `/api/status`, `/api/config`, `/api/logs`, `/api/identity`.
5. **Frontend:** Setup wizard for first run (detect via setup-status; steps: initialize, config, benchmark, start); then main SPA with Status, Config, Logs, Identity views; optional Start/Stop/Restart and “Open map” link.
6. **Integration:** Document in main README and CONTRIBUTING how to run the UI (e.g. `cd systems/node-dashboard && ./run.sh` or `kwaainet ui` if we add it).

---

## References

- CLI commands: `kwaainet --help` (see [README](../README.md) for full list).
- Config: `~/.kwaainet/config.yaml` ([config.rs](../core/crates/kwaai-cli/src/config.rs)); [KwaaiNetConfig](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/core/crates/kwaai-cli/src/config.rs) schema.
- Status: [NodeStatus](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/core/crates/kwaai-cli/src/daemon.rs) in daemon.rs; `kwaainet status` output.
- Map API: e.g. `curl -s https://map.kwaai.ai/api/v1/state` (see [DEBUGGING_MAP_VISIBILITY.md](./DEBUGGING_MAP_VISIBILITY.md)).
- Deployment diagram (TUI mention): [DEPLOYMENT_ARCHITECTURE.md](./DEPLOYMENT_ARCHITECTURE.md).

---

## TODO

### Backend

- [x] Decide stack (Rust axum vs Node) and repo location (e.g. `systems/node-dashboard/backend`).
- [x] Implement local HTTP server on 127.0.0.1 only.
- [x] Add `GET /api/setup-status` (needs_setup, step: none | config | identity | benchmark | ready).
- [x] Add allowlisted setup actions: run `kwaainet setup`, `kwaainet setup --get-deps`, `kwaainet benchmark`, `kwaainet start --daemon`.
- [x] Expose `GET /api/status`, `GET /api/config`, `GET /api/logs`, `GET /api/identity`.
- [x] Expose `POST /api/restart` (and optionally start/stop) with strict validation.

### CLI

- [x] Add `--json` to `kwaainet status` and document JSON schema.
- [x] Add `--json` to `kwaainet config` (or equivalent) and document schema.
- [x] Add `--json` to `kwaainet identity show` and document schema.

### UI Design

- [x] Apply color palette: primary light `#FFFFFF`, dark `#000000`, accent blue `#3063e9`, accent purple `#78479e` (see §8).
- [x] Implement header: black bar, logo + "KwaaiNet", nav (Status, Config, Logs, Identity), "View on map" link.
- [x] Use purple for primary buttons (Start, Stop, Initialize, Next); blue for links and secondary actions.
- [x] Ensure contrast and WCAG AA where applicable; use system-ui or simple sans-serif.

### Frontend

- [x] Create SPA (e.g. Vite + React/Vue/Svelte) under `systems/node-dashboard/frontend`.
- [x] Implement setup wizard: detect setup-status, steps Initialize → Config → Benchmark → Start.
- [x] Implement dashboard: Status, Config (read-only at first), Logs (tail), Identity.
- [x] Add Start / Stop / Restart controls (Phase 2).
- [x] Add “View on map” link to map.kwaai.ai.
- [x] Handle “CLI not found” and other errors with clear messages and doc links.

### Setup flow

- [x] Backend: detect missing `~/.kwaainet/config.yaml` or invalid/empty config.
- [x] Backend: detect missing identity (e.g. `~/.kwaainet/identity.key`).
- [x] Frontend: first visit → show wizard; return visit (setup complete) → show dashboard.
- [x] Frontend: partial setup → show “Continue setup” with next step.

### Integration and docs

- [x] Add README in `systems/node-dashboard/` (purpose, deps, how to run, ports).
- [x] Document in main README how to run the UI.
- [x] Optional: add `kwaainet ui` subcommand that launches backend + opens browser.
