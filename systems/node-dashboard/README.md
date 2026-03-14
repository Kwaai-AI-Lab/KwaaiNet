# KwaaiNet Node Dashboard

Local web UI to view and control your KwaaiNet node: status, config, logs, identity, and first-run setup.

**Purpose:** See what's going on with your node without using the terminal. Optional — the node runs fine with CLI only.

**Port:** Backend runs on `http://127.0.0.1:3456` (localhost only).

## Requirements

- **Node.js** 18+
- **kwaainet** CLI on your PATH (from [KwaaiNet](https://github.com/Kwaai-AI-Lab/KwaaiNet) build or install)

## Quick start

```bash
# From repo root
cd systems/node-dashboard
npm install
npm run dev
```

- **Development:** Backend runs at http://127.0.0.1:3456, frontend at http://127.0.0.1:5173 (proxies API to backend). Open **http://127.0.0.1:5173** in your browser.
- **Production (after build):** Run `npm run build` then `npm start`; open **http://127.0.0.1:3456** (backend serves the built frontend).

- First run (no `~/.kwaainet/` or missing config/identity): the UI guides you through setup (Initialize → Config → Benchmark → Start).
- Already set up: dashboard with Status, Network, Config, Logs, Identity and optional Start/Stop/Restart. **Network** helps fix bootstrap/map issues (reconnect P2P, troubleshooting).

## Structure

| Path | Purpose |
|------|--------|
| `backend/` | Node HTTP server on 127.0.0.1:3456; API and static frontend |
| `frontend/` | React SPA (Vite); build output served by backend |

## API (backend)

| Method | Path | Description |
|--------|------|--------------|
| GET | `/api/setup-status` | `{ needs_setup, step }` — whether to show setup wizard |
| GET | `/api/status` | Node status (running, PID, uptime, CPU, memory, shard) |
| GET | `/api/config` | Config (model, blocks, port, public_name, etc.) |
| GET | `/api/logs` | Last N log lines |
| GET | `/api/identity` | DID, Peer ID, trust tier (from `kwaainet identity show`) |
| POST | `/api/setup` | Run `kwaainet setup` |
| POST | `/api/start` | Run `kwaainet start --daemon` |
| POST | `/api/stop` | Run `kwaainet stop` |
| POST | `/api/restart` | Run `kwaainet restart` |
| POST | `/api/config` | Update config (body: `{ "updates": { "key": "value" } }`, allowlisted keys only) |
| POST | `/api/setup-get-deps` | Run `kwaainet setup --get-deps` (download p2pd if missing) |
| GET | `/api/health-status` | Health monitoring on/off |
| POST | `/api/health-enable` | Enable health monitoring |
| POST | `/api/health-disable` | Disable health monitoring |
| GET | `/api/network-status` | Bootstrap/map state from map.kwaai.ai (for Network page) |
| POST | `/api/reconnect` | Run `kwaainet reconnect` — restart node to re-establish P2P |
| POST | `/api/restart-with-shard` | Stop node then start with `--shard` so the shard server runs |

## Docs

- [Node UI Planning](../../docs/NODE_UI_PLANNING.md) — full plan, design, and TODO.
- [DEBUGGING_MAP_VISIBILITY](../../docs/DEBUGGING_MAP_VISIBILITY.md) — Kwaai’s guide; the Network page troubleshooting (bootstrap, Broken pipe) follows the [Troubleshooting section](https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md#troubleshooting-bootstrap-connect-failed-and-broken-pipe-os-error-32).
