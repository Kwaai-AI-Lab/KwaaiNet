/**
 * KwaaiNet Node Dashboard — local API server
 * Binds 127.0.0.1:3456 only. Serves API + static frontend.
 */

import express from 'express';
import { execSync, execFileSync, spawn } from 'child_process';
import fs from 'fs';
import path from 'path';
import os from 'os';
import { fileURLToPath } from 'url';
import yaml from 'yaml';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const PORT = 3456;
const HOST = '127.0.0.1';

const app = express();
app.use(express.json());

const kwaainetDir = () => path.join(os.homedir(), '.kwaainet');
const configPath = () => path.join(kwaainetDir(), 'config.yaml');
const identityPath = () => path.join(kwaainetDir(), 'identity.key');
const logPath = () => path.join(kwaainetDir(), 'logs', 'kwaainet.log');
const shardLogPath = () => path.join(kwaainetDir(), 'logs', 'shard.log');

function kwaainetAvailable() {
  try {
    execSync('kwaainet --version', { encoding: 'utf8', stdio: ['pipe', 'pipe', 'pipe'] });
    return true;
  } catch {
    return false;
  }
}

function runKwaainet(args, options = {}) {
  const cmd = ['kwaainet', ...args];
  try {
    const out = execSync(cmd.join(' '), {
      encoding: 'utf8',
      stdio: ['pipe', 'pipe', 'pipe'],
      ...options,
    });
    return { ok: true, stdout: out || '', stderr: '' };
  } catch (e) {
    return { ok: false, stdout: e.stdout || '', stderr: e.stderr || e.message };
  }
}

// ---------------------------------------------------------------------------
// GET /api/setup-status
// If the user has already installed/set up (valid config exists), do not
// show the setup wizard again. Identity is created on first use if missing.
// ---------------------------------------------------------------------------
app.get('/api/setup-status', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.json({ needs_setup: true, step: 'none', cli_not_found: true });
  }
  const dir = kwaainetDir();
  const config = configPath();
  let hasConfig = false;
  let configValid = false;
  if (fs.existsSync(config)) {
    hasConfig = true;
    try {
      const raw = fs.readFileSync(config, 'utf8');
      yaml.parse(raw);
      configValid = true;
    } catch (_) {}
  }

  // Network already installed: valid config present → skip wizard
  if (fs.existsSync(dir) && hasConfig && configValid) {
    return res.json({ needs_setup: false, step: 'ready' });
  }
  return res.json({ needs_setup: true, step: 'none' });
});

// ---------------------------------------------------------------------------
// GET /api/status
// ---------------------------------------------------------------------------
app.get('/api/status', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.json({ error: 'cli_not_found', running: false });
  }
  const { ok, stdout } = runKwaainet(['status', '--json']);
  if (!ok && !stdout) {
    return res.json({ error: 'command_failed', running: false });
  }
  try {
    const data = JSON.parse(stdout.trim());
    return res.json({
      running: data.running === true,
      pid: data.pid ?? null,
      uptime: data.uptime_secs != null ? `${data.uptime_secs}s` : null,
      cpu: data.cpu_percent != null ? `${data.cpu_percent.toFixed(1)}%` : null,
      memory: data.memory_mb != null ? `${Math.round(data.memory_mb)} MB` : null,
      shard_running: data.shard_running === true,
      shard_pid: data.shard_pid ?? null,
    });
  } catch (_) {
    // Fallback: parse human output
    const running = /Status:\s*Running/i.test(stdout);
    const pidMatch = stdout.match(/PID:\s*(\d+)/);
    const uptimeMatch = stdout.match(/Uptime:\s*([^\n]+)/);
    const cpuMatch = stdout.match(/CPU:\s*([^\n]+)/);
    const memMatch = stdout.match(/Memory:\s*([^\n]+)/);
    const shardRunning = /Shard:\s*Running/i.test(stdout);
    const shardPidMatch = stdout.match(/Shard:\s*Running \(PID (\d+)\)/);
    res.json({
      running,
      pid: pidMatch ? parseInt(pidMatch[1], 10) : null,
      uptime: uptimeMatch ? uptimeMatch[1].trim() : null,
      cpu: cpuMatch ? cpuMatch[1].trim() : null,
      memory: memMatch ? memMatch[1].trim() : null,
      shard_running: shardRunning,
      shard_pid: shardPidMatch ? parseInt(shardPidMatch[1], 10) : null,
    });
  }
});

// ---------------------------------------------------------------------------
// GET /api/config
// ---------------------------------------------------------------------------
app.get('/api/config', (_req, res) => {
  const config = configPath();
  if (!fs.existsSync(config)) {
    return res.json({ error: 'no_config', config: null });
  }
  try {
    const raw = fs.readFileSync(config, 'utf8');
    const parsed = yaml.parse(raw);
    return res.json({ config: parsed });
  } catch (e) {
    return res.status(500).json({ error: 'parse_error', message: e.message });
  }
});

// Allowed keys for POST /api/config (allowlist only)
const CONFIG_SET_KEYS = new Set([
  'model', 'blocks', 'start_block', 'port', 'use_gpu', 'log_level',
  'public_name', 'public_ip', 'announce_addr', 'no_relay',
]);

// ---------------------------------------------------------------------------
// POST /api/config — update config via kwaainet config set KEY VALUE
// ---------------------------------------------------------------------------
app.post('/api/config', (req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const updates = req.body?.updates || req.body;
  if (typeof updates !== 'object' || Array.isArray(updates)) {
    return res.status(400).json({ error: 'bad_request', message: 'Expected { updates: { key: value } }' });
  }
  const errors = [];
  for (const [key, value] of Object.entries(updates)) {
    if (!CONFIG_SET_KEYS.has(key)) {
      errors.push({ key, error: 'disallowed_key' });
      continue;
    }
    const str = value == null ? '' : String(value);
    const { ok, stderr } = runKwaainet(['config', 'set', key, str]);
    if (!ok) errors.push({ key, error: stderr || 'set_failed' });
  }
  if (errors.length > 0) {
    return res.status(400).json({ ok: false, errors });
  }
  res.json({ ok: true });
});

// ---------------------------------------------------------------------------
// GET /api/logs?lines=200&log=node|shard  (default log=node)
// ---------------------------------------------------------------------------
app.get('/api/logs', (req, res) => {
  const lines = Math.min(parseInt(req.query.lines, 10) || 200, 2000);
  const logType = (req.query.log || 'node').toLowerCase();
  const log = logType === 'shard' ? shardLogPath() : logPath();
  if (!fs.existsSync(log)) {
    return res.json({ lines: [], truncated: false, log: logType, empty: true });
  }
  try {
    const content = fs.readFileSync(log, 'utf8');
    const all = content.split('\n').filter(Boolean);
    const last = all.slice(-lines);
    return res.json({ lines: last, truncated: all.length > lines, log: logType });
  } catch (e) {
    return res.status(500).json({ error: e.message, lines: [], log: logType });
  }
});

// ---------------------------------------------------------------------------
// GET /api/identity
// ---------------------------------------------------------------------------
app.get('/api/identity', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.json({ error: 'cli_not_found', did: null, peer_id: null, tier: null });
  }
  const { ok, stdout } = runKwaainet(['identity', 'show', '--json']);
  if (!ok && !stdout) {
    return res.json({ error: 'command_failed', raw: stdout || '' });
  }
  try {
    const data = JSON.parse(stdout.trim());
    return res.json({
      did: data.did ?? null,
      peer_id: data.peer_id ?? null,
      trust_tier: data.trust_tier ?? null,
      raw: stdout,
    });
  } catch (_) {}
  const didMatch = stdout.match(/DID[:\s]+([^\s\n]+)/i) || stdout.match(/did:peer:[^\s\n]+/);
  const peerMatch = stdout.match(/Peer\s*ID[:\s]+([^\s\n]+)/i);
  const tierMatch = stdout.match(/[Tt]rust\s*tier[:\s]+([^\n]+)/i);
  res.json({
    raw: stdout,
    did: didMatch ? (didMatch[1] || didMatch[0]).trim() : null,
    peer_id: peerMatch ? peerMatch[1].trim() : null,
    trust_tier: tierMatch ? tierMatch[1].trim() : null,
  });
});

// ---------------------------------------------------------------------------
// POST /api/setup — kwaainet setup
// ---------------------------------------------------------------------------
app.post('/api/setup', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['setup']);
  if (!ok) {
    return res.status(500).json({ error: 'setup_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// POST /api/setup-get-deps — kwaainet setup --get-deps (download p2pd if missing)
// ---------------------------------------------------------------------------
app.post('/api/setup-get-deps', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['setup', '--get-deps']);
  if (!ok) {
    return res.status(500).json({ error: 'get_deps_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// GET /api/health-status
// ---------------------------------------------------------------------------
app.get('/api/health-status', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.json({ error: 'cli_not_found', enabled: false });
  }
  const { ok, stdout } = runKwaainet(['health-status']);
  if (!ok) {
    return res.json({ enabled: false, raw: stdout || '' });
  }
  const enabled = /enabled|on|true/i.test(stdout);
  res.json({ enabled, raw: stdout });
});

// ---------------------------------------------------------------------------
// POST /api/health-enable | POST /api/health-disable
// ---------------------------------------------------------------------------
function runKwaainetHealth(subcommand) {
  try {
    const out = execFileSync('kwaainet', [subcommand], {
      encoding: 'utf8',
      stdio: ['pipe', 'pipe', 'pipe'],
    });
    return { ok: true, stdout: String(out || ''), stderr: '' };
  } catch (e) {
    const stdout = (e.stdout != null ? String(e.stdout) : '') || '';
    const stderr = (e.stderr != null ? String(e.stderr) : '') || (e.message ? String(e.message) : '');
    return { ok: false, stdout, stderr };
  }
}

app.post('/api/health-enable', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainetHealth('health-enable');
  if (!ok) {
    return res.status(500).json({ error: 'health_enable_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

app.post('/api/health-disable', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainetHealth('health-disable');
  if (!ok) {
    return res.status(500).json({ error: 'health_disable_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// POST /api/start — kwaainet start --daemon
// ---------------------------------------------------------------------------
app.post('/api/start', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['start', '--daemon']);
  if (!ok) {
    return res.status(500).json({ error: 'start_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// POST /api/stop — kwaainet stop
// ---------------------------------------------------------------------------
app.post('/api/stop', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['stop']);
  if (!ok) {
    return res.status(500).json({ error: 'stop_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// POST /api/restart — kwaainet restart
// ---------------------------------------------------------------------------
app.post('/api/restart', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['restart']);
  if (!ok) {
    return res.status(500).json({ error: 'restart_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// POST /api/restart-with-shard — stop then start --daemon --shard (so shard runs)
// ---------------------------------------------------------------------------
app.post('/api/restart-with-shard', async (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const stop = runKwaainet(['stop']);
  if (!stop.ok) {
    return res.status(500).json({ error: 'stop_failed', stdout: stop.stdout, stderr: stop.stderr });
  }
  await new Promise((r) => setTimeout(r, 3000));
  const start = runKwaainet(['start', '--daemon', '--shard']);
  if (!start.ok) {
    return res.status(500).json({ error: 'start_failed', stdout: start.stdout, stderr: start.stderr });
  }
  res.json({ ok: true, message: 'Node restarted with shard server.', stdout: start.stdout, stderr: start.stderr });
});

// ---------------------------------------------------------------------------
// POST /api/benchmark — kwaainet benchmark
// ---------------------------------------------------------------------------
app.post('/api/benchmark', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['benchmark']);
  if (!ok) {
    return res.status(500).json({ error: 'benchmark_failed', stdout, stderr });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// GET /api/network-status — bootstrap & map state from map.kwaai.ai
// ---------------------------------------------------------------------------
app.get('/api/network-status', async (_req, res) => {
  try {
    const resp = await fetch('https://map.kwaai.ai/api/v1/state');
    const text = await resp.text();
    let data = {};
    try {
      data = text ? JSON.parse(text) : {};
    } catch (_) {
      return res.json({ error: 'invalid_response', bootstrap_states: null });
    }
    const bootstrap_states = data.bootstrap_states ?? null;
    const node_count = data.model_reports?.length ?? null;
    res.json({ ok: resp.ok, bootstrap_states, node_count, raw: data });
  } catch (e) {
    res.json({ error: 'map_unreachable', message: e.message, bootstrap_states: null });
  }
});

// ---------------------------------------------------------------------------
// POST /api/reconnect — kwaainet reconnect (restart node to re-establish P2P)
// ---------------------------------------------------------------------------
app.post('/api/reconnect', (_req, res) => {
  if (!kwaainetAvailable()) {
    return res.status(400).json({ error: 'cli_not_found' });
  }
  const { ok, stdout, stderr } = runKwaainet(['reconnect']);
  if (!ok) {
    const msg = [stderr, stdout].filter(Boolean).join(' ').trim() || 'Node may not be running — try Start on the Status page first.';
    return res.status(500).json({ error: 'reconnect_failed', message: msg, stdout: stdout || '', stderr: stderr || '' });
  }
  res.json({ ok: true, stdout, stderr });
});

// ---------------------------------------------------------------------------
// Static frontend (built)
// ---------------------------------------------------------------------------
const frontendDist = path.join(__dirname, '..', 'frontend', 'dist');
if (fs.existsSync(frontendDist)) {
  app.use(express.static(frontendDist));
  app.get('*', (_req, res) => {
    res.sendFile(path.join(frontendDist, 'index.html'));
  });
} else {
  app.get('/', (_req, res) => {
    res.send(`
      <!DOCTYPE html>
      <html>
        <head><title>KwaaiNet Dashboard</title></head>
        <body style="font-family:system-ui;padding:2rem;background:#000;color:#fff;">
          <h1>KwaaiNet Node Dashboard</h1>
          <p>Build the frontend first: <code>cd frontend && npm install && npm run build</code></p>
          <p>API is available at <a href="/api/setup-status" style="color:#3063e9;">/api/setup-status</a>.</p>
        </body>
      </html>
    `);
  });
}

app.listen(PORT, HOST, () => {
  console.log(`KwaaiNet Dashboard: http://${HOST}:${PORT}`);
});
