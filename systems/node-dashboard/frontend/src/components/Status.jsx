import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';

export default function Status() {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [actionLoading, setActionLoading] = useState(null);
  const [health, setHealth] = useState(null);
  const [healthLoading, setHealthLoading] = useState(null);
  const [healthError, setHealthError] = useState(null);
  const [network, setNetwork] = useState(null);
  const [fixLoading, setFixLoading] = useState(null);
  const [fixMessage, setFixMessage] = useState(null);

  useEffect(() => {
    let cancelled = false;
    async function fetchStatus() {
      try {
        const res = await fetch('/api/status');
        const json = await res.json();
        if (!cancelled) setData(json);
      } catch (_) {
        if (!cancelled) setData({ error: 'fetch_failed' });
      } finally {
        if (!cancelled) setLoading(false);
      }
    }
    fetchStatus();
    const t = setInterval(fetchStatus, 5000);
    return () => {
      cancelled = true;
      clearInterval(t);
    };
  }, []);

  useEffect(() => {
    let cancelled = false;
    setHealthError(null);
    fetch('/api/health-status')
      .then((r) => r.text())
      .then((text) => {
        if (cancelled) return;
        try {
          const json = text ? JSON.parse(text) : {};
          setHealth(typeof json.enabled === 'boolean' ? json : { ...json, enabled: false });
        } catch (_) {
          setHealth({ enabled: false });
        }
      })
      .catch(() => { if (!cancelled) setHealth({ enabled: false }); });
    return () => { cancelled = true; };
  }, [actionLoading]);

  useEffect(() => {
    let cancelled = false;
    fetch('/api/network-status')
      .then((r) => r.text())
      .then((text) => {
        if (cancelled) return;
        try {
          const json = text ? JSON.parse(text) : {};
          setNetwork(json);
        } catch (_) {
          setNetwork({ error: 'fetch_failed' });
        }
      })
      .catch(() => { if (!cancelled) setNetwork({ error: 'fetch_failed' }); });
    return () => { cancelled = true; };
  }, []);

  async function runHealthAction(action) {
    setHealthLoading(action);
    setHealthError(null);
    try {
      const res = await fetch(`/api/health-${action}`, { method: 'POST' });
      const text = await res.text();
      let json = {};
      try {
        json = text ? JSON.parse(text) : {};
      } catch (_) {
        const hint = /^\s*</.test(text || '') ? ' Backend may not be running (got HTML).' : '';
        setHealthError('Invalid response from server.' + hint);
        return;
      }
      if (res.ok) {
        setHealth((h) => ({ ...h, enabled: action === 'enable' }));
        const r = await fetch('/api/health-status');
        const nextText = await r.text();
        try {
          const next = nextText ? JSON.parse(nextText) : {};
          if (!next.error) setHealth(next);
        } catch (_) {}
      } else {
        setHealthError(json.stderr || json.error || json.message || 'Request failed');
      }
    } catch (e) {
      setHealthError(e.message || 'Request failed');
    } finally {
      setHealthLoading(null);
    }
  }

  async function runFix(fix) {
    setFixLoading(fix);
    setFixMessage(null);
    try {
      if (fix === 'shard') {
        const res = await fetch('/api/restart-with-shard', { method: 'POST' });
        const text = await res.text();
        let json = {};
        try {
          json = text ? JSON.parse(text) : {};
        } catch (_) {}
        if (res.ok) {
          setFixMessage('Node restarting with shard. Status will update in a few seconds.');
          const t = setInterval(() => fetch('/api/status').then((r) => r.json()).then(setData), 2000);
          setTimeout(() => clearInterval(t), 15000);
        } else {
          setFixMessage(json.stderr || json.stdout || json.error || 'Failed');
        }
      } else if (fix === 'reconnect') {
        const res = await fetch('/api/reconnect', { method: 'POST' });
        const text = await res.text();
        let json = {};
        try {
          json = text ? JSON.parse(text) : {};
        } catch (_) {}
        if (res.ok) {
          setFixMessage('Reconnect started. Node restarted; P2P re-establishing.');
          fetch('/api/status').then((r) => r.json()).then(setData);
        } else {
          setFixMessage(json.message || json.stderr || json.stdout || json.error || 'Failed');
        }
      }
    } catch (e) {
      setFixMessage(e.message || 'Request failed');
    } finally {
      setFixLoading(null);
    }
  }

  async function runAction(action) {
    setActionLoading(action);
    try {
      const res = await fetch(`/api/${action}`, { method: 'POST' });
      const json = await res.json();
      if (res.ok) {
        const r = await fetch('/api/status');
        setData(await r.json());
      } else {
        setData((d) => ({ ...d, error: json.stderr || json.error }));
      }
    } finally {
      setActionLoading(null);
    }
  }

  if (loading && !data) {
    return <div className="card">Loading status…</div>;
  }

  if (data?.error === 'cli_not_found') {
    return (
      <div className="card" style={{ borderColor: 'var(--color-error)' }}>
        <p style={{ color: 'var(--color-error)' }}>KwaaiNet CLI not found. Install it and ensure it is on your PATH.</p>
      </div>
    );
  }

  const running = data?.running === true;
  const shardRunning = data?.shard_running === true;
  const healthEnabled = health?.enabled === true;
  const mapReachable = network != null && !network.error && (network.bootstrap_states != null || network.ok === true);

  const working = [];
  const needsAttention = [];
  if (running) working.push('Node running');
  else needsAttention.push({ id: 'node', label: 'Node not running', fix: null });
  if (shardRunning) working.push('Shard server running');
  else if (running) needsAttention.push({ id: 'shard', label: 'Shard not running', fix: 'shard' });
  if (healthEnabled) working.push('Health monitoring on');
  else needsAttention.push({ id: 'health', label: 'Health monitoring off', fix: 'health' });
  if (mapReachable) working.push('Map API reachable');
  else if (running) needsAttention.push({ id: 'map', label: 'Map / bootstrap unknown or unreachable', fix: 'reconnect' });

  return (
    <div>
      <div className="card" style={{ marginBottom: '1rem' }}>
        <h2 style={{ marginTop: 0 }}>Status overview</h2>
        <div style={{ display: 'grid', gridTemplateColumns: 'minmax(0, 1fr) minmax(0, 1fr)', gap: '1.5rem', marginTop: '0.5rem' }}>
          <div>
            <strong style={{ color: 'var(--color-success)', fontSize: 14 }}>Working</strong>
            <ul style={{ listStyle: 'none', paddingLeft: 0, margin: '0.25rem 0 0 0' }}>
              {working.length ? working.map((w) => (
                <li key={w} style={{ display: 'flex', alignItems: 'center', gap: '0.35rem', marginBottom: '0.25rem' }}>
                  <span style={{ color: 'var(--color-success)' }} aria-hidden>✓</span> {w}
                </li>
              )) : (
                <li style={{ color: 'var(--color-muted)' }}>—</li>
              )}
            </ul>
          </div>
          <div>
            <strong style={{ color: 'var(--color-error)', fontSize: 14 }}>Needs attention</strong>
            <ul style={{ listStyle: 'none', paddingLeft: 0, margin: '0.25rem 0 0 0' }}>
              {needsAttention.length ? needsAttention.map((item) => (
                <li key={item.id} style={{ display: 'flex', alignItems: 'center', flexWrap: 'wrap', gap: '0.5rem', marginBottom: '0.5rem' }}>
                  <span style={{ color: 'var(--color-error)' }} aria-hidden>!</span>
                  <span style={{ minWidth: 0 }}>{item.label}</span>
                  {item.fix === 'shard' && (
                    <button
                      type="button"
                      className="btn-primary"
                      style={{ padding: '0.35rem 0.65rem', fontSize: 13, flexShrink: 0 }}
                      disabled={fixLoading !== null}
                      onClick={() => runFix('shard')}
                    >
                      {fixLoading === 'shard' ? '…' : 'Start shard'}
                    </button>
                  )}
                  {item.fix === 'health' && (
                    <button
                      type="button"
                      className="btn-primary"
                      style={{ padding: '0.35rem 0.65rem', fontSize: 13, flexShrink: 0 }}
                      disabled={healthLoading !== null}
                      onClick={() => runHealthAction('enable')}
                    >
                      {healthLoading ? '…' : 'Enable'}
                    </button>
                  )}
                  {item.fix === 'reconnect' && (
                    <button
                      type="button"
                      className="btn-secondary"
                      style={{ padding: '0.35rem 0.65rem', fontSize: 13, flexShrink: 0 }}
                      disabled={fixLoading !== null}
                      onClick={() => runFix('reconnect')}
                    >
                      {fixLoading === 'reconnect' ? '…' : 'Reconnect P2P'}
                    </button>
                  )}
                </li>
              )) : (
                <li style={{ color: 'var(--color-success)' }}>All good</li>
              )}
            </ul>
            {fixMessage && (
              <p style={{ fontSize: 13, color: 'var(--color-text-on-light)', margin: '0.5rem 0 0 0' }}>{fixMessage}</p>
            )}
          </div>
        </div>
      </div>
      <div className="card">
        <h2 style={{ marginTop: 0 }}>Node status</h2>
        <p style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
          <span
            style={{
              width: 10,
              height: 10,
              borderRadius: '50%',
              background: running ? 'var(--color-success)' : 'var(--color-error)',
            }}
          />
          <strong>{running ? 'Running' : 'Not running'}</strong>
        </p>
        {running && (
          <ul style={{ listStyle: 'none', paddingLeft: 0 }}>
            {data.pid != null && <li>PID: {data.pid}</li>}
            {data.uptime && <li>Uptime: {data.uptime}</li>}
            {data.cpu && <li>CPU: {data.cpu}</li>}
            {data.memory && <li>Memory: {data.memory}</li>}
            {data.shard_running != null && (
              <li style={{ display: 'flex', alignItems: 'center', gap: '0.5rem', flexWrap: 'wrap' }}>
                Shard: {data.shard_running ? `Running (PID ${data.shard_pid || '—'})` : 'Not running'}
                <Link to="/logs?log=shard" style={{ fontSize: 13 }}>View shard log</Link>
              </li>
            )}
          </ul>
        )}
        <p style={{ margin: '0.5rem 0 0 0', fontSize: 13 }}>
          <Link to="/logs">View node log</Link>
          {' '}(bootstrap, map, shard_ready — as KwaaiNet intends)
        </p>
        <div style={{ display: 'flex', gap: '0.75rem', marginTop: '1rem' }}>
          {!running && (
            <button
              className="btn-primary"
              disabled={actionLoading !== null}
              onClick={() => runAction('start')}
            >
              {actionLoading === 'start' ? 'Starting…' : 'Start node'}
            </button>
          )}
          {running && (
            <>
              <button
                className="btn-secondary"
                disabled={actionLoading !== null}
                onClick={() => runAction('stop')}
              >
                {actionLoading === 'stop' ? 'Stopping…' : 'Stop'}
              </button>
              <button
                className="btn-primary"
                disabled={actionLoading !== null}
                onClick={() => runAction('restart')}
              >
                {actionLoading === 'restart' ? 'Restarting…' : 'Restart'}
              </button>
            </>
          )}
        </div>
      </div>
      {health && !data?.error && (
        <div className="card" style={{ marginTop: '1rem' }}>
          <h3 style={{ marginTop: 0 }}>Health monitoring</h3>
          {healthError && (
            <>
              <p style={{ color: 'var(--color-error)', fontSize: 14, margin: '0 0 0.5rem 0' }}>{healthError}</p>
              {/got HTML|Backend may not be running/i.test(healthError) && (
                <p style={{ fontSize: 13, color: 'var(--color-text-on-light)', margin: '0 0 0.5rem 0' }}>
                  Start the dashboard with both frontend and backend: from repo root run <code style={{ background: 'var(--color-muted)', padding: '0.1rem 0.3rem', borderRadius: 4 }}>./start-ui.sh</code> or from <code style={{ background: 'var(--color-muted)', padding: '0.1rem 0.3rem', borderRadius: 4 }}>systems/node-dashboard</code> run <code style={{ background: 'var(--color-muted)', padding: '0.1rem 0.3rem', borderRadius: 4 }}>npm run dev</code>. The API must be running on port 3456.
                </p>
              )}
            </>
          )}
          <p style={{ margin: 0 }}>
            {health.enabled ? 'Enabled' : 'Disabled'}
            {' — '}
            <button
              type="button"
              className="btn-primary"
              style={{ padding: '0.35rem 0.75rem', fontSize: 14, marginLeft: '0.25rem' }}
              disabled={healthLoading !== null}
              onClick={() => runHealthAction(health.enabled ? 'disable' : 'enable')}
            >
              {healthLoading ? '…' : health.enabled ? 'Disable' : 'Enable'}
            </button>
          </p>
          <p style={{ margin: '0.5rem 0 0 0', fontSize: 13 }}>
            <Link to="/logs">Node log</Link> (kwaainet.log) — health and map are part of the node process.
          </p>
        </div>
      )}
      {running && !data?.error && (
        <p style={{ marginTop: '1rem', fontSize: 14, color: 'var(--color-text-on-light)' }}>
          Connection or map issues? <Link to="/network">Network</Link> — reconnect P2P and troubleshooting. <Link to="/logs">View node log</Link> for bootstrap/map messages. Follow{' '}
          <a href="https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md#troubleshooting-bootstrap-connect-failed-and-broken-pipe-os-error-32" target="_blank" rel="noopener noreferrer">Kwaai&apos;s troubleshooting guide</a>.
        </p>
      )}
    </div>
  );
}
