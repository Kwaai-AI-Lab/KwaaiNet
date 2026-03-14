import { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';

// Kwaai's official guide: https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md
const TROUBLESHOOTING_URL = 'https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md#troubleshooting-bootstrap-connect-failed-and-broken-pipe-os-error-32';

export default function Network() {
  const [network, setNetwork] = useState(null);
  const [loading, setLoading] = useState(true);
  const [reconnectLoading, setReconnectLoading] = useState(false);
  const [reconnectResult, setReconnectResult] = useState(null);

  useEffect(() => {
    let cancelled = false;
    async function fetchNetwork() {
      try {
        const res = await fetch('/api/network-status');
        const text = await res.text();
        let json = {};
        try {
          json = text ? JSON.parse(text) : {};
        } catch (_) {}
        if (!cancelled) setNetwork(json);
      } catch (_) {
        if (!cancelled) setNetwork({ error: 'fetch_failed' });
      } finally {
        if (!cancelled) setLoading(false);
      }
    }
    fetchNetwork();
    const t = setInterval(fetchNetwork, 60000);
    return () => {
      cancelled = true;
      clearInterval(t);
    };
  }, []);

  async function runReconnect() {
    setReconnectLoading(true);
    setReconnectResult(null);
    try {
      const res = await fetch('/api/reconnect', { method: 'POST' });
      const text = await res.text();
      let json = {};
      try {
        json = text ? JSON.parse(text) : {};
      } catch (_) {}
      setReconnectResult({ ok: res.ok, ...json });
      if (res.ok) {
        setNetwork((n) => (n ? { ...n, reconnect_just_ran: true } : n));
      }
    } catch (e) {
      setReconnectResult({ ok: false, error: e.message });
    } finally {
      setReconnectLoading(false);
    }
  }

  if (loading && !network) {
    return <div className="card">Loading network status…</div>;
  }

  const hasError = network?.error === 'map_unreachable' || network?.error === 'fetch_failed' || network?.error === 'invalid_response';
  const bootstrapStates = network?.bootstrap_states;
  const bootstrapOk = bootstrapStates != null && typeof bootstrapStates === 'object' && Object.keys(bootstrapStates).length > 0;

  return (
    <div>
      <div className="card">
        <h2 style={{ marginTop: 0 }}>Network &amp; bootstrap</h2>
        <p style={{ marginTop: 0, color: 'var(--color-text-on-light)' }}>
          Your node must connect to Kwaai bootstrap servers (<code>bootstrap-1.kwaai.ai</code>, <code>bootstrap-2.kwaai.ai</code>) on TCP port 8000 to appear on the map. If you see &quot;Bootstrap connect failed&quot; or &quot;Broken pipe (os error 32)&quot; in the node log, follow the steps below — aligned with{' '}
          <a href={TROUBLESHOOTING_URL} target="_blank" rel="noopener noreferrer">Kwaai&apos;s troubleshooting guide</a>.
        </p>
        <p style={{ fontSize: 14, margin: '0.5rem 0 1rem 0' }}>
          <strong>Logs:</strong> Bootstrap, reconnect, and map messages are in the <Link to="/logs">node log</Link> (kwaainet.log). Open Logs to see &quot;Bootstrap connect failed&quot;, &quot;Announced to 2 of 2&quot;, etc.
        </p>

        <div style={{ marginBottom: '1rem' }}>
          <strong>Map / bootstrap status: </strong>
          {hasError ? (
            <span style={{ color: 'var(--color-error)' }}>Unreachable — map.kwaai.ai could not be reached</span>
          ) : bootstrapOk ? (
            <span style={{ color: 'var(--color-success)' }}>OK (map API reachable)</span>
          ) : network != null ? (
            <span style={{ color: 'var(--color-success)' }}>Map API reachable</span>
          ) : (
            <span>Checking…</span>
          )}
        </div>

        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '0.75rem', alignItems: 'center' }}>
          <button
            type="button"
            className="btn-primary"
            disabled={reconnectLoading}
            onClick={runReconnect}
          >
            {reconnectLoading ? 'Reconnecting…' : 'Reconnect P2P'}
          </button>
          <a
            href="https://map.kwaai.ai"
            target="_blank"
            rel="noopener noreferrer"
            className="btn-secondary"
            style={{ display: 'inline-block', padding: '0.6rem 1.2rem', textDecoration: 'none' }}
          >
            View on map
          </a>
        </div>

        {reconnectResult && (
          <div
            style={{
              marginTop: '1rem',
              padding: '0.75rem',
              borderRadius: 6,
              background: reconnectResult.ok ? 'rgba(16, 185, 129, 0.1)' : 'rgba(239, 68, 68, 0.1)',
              color: reconnectResult.ok ? 'var(--color-success)' : 'var(--color-error)',
              fontSize: 14,
            }}
          >
            {reconnectResult.ok ? (
              <>Reconnect started. The node restarted and is re-establishing P2P connections. <Link to="/logs">Check node log</Link> for progress.</>
            ) : (
              <>
                Reconnect failed: {reconnectResult.message || reconnectResult.stderr || reconnectResult.stdout || (reconnectResult.error === 'cli_not_found' ? 'KwaaiNet CLI not found' : reconnectResult.error) || 'Unknown error'}
                {(reconnectResult.error === 'reconnect_failed' || !reconnectResult.message) && (
                  <span style={{ display: 'block', marginTop: '0.35rem' }}>
                    Ensure the node is running (Status). <Link to="/logs">Node log</Link> shows bootstrap/reconnect output.
                  </span>
                )}
              </>
            )}
          </div>
        )}
      </div>

      <div className="card" style={{ marginTop: '1rem' }}>
        <h3 style={{ marginTop: 0 }}>If the node doesn’t appear on the map (per Kwaai guide)</h3>
        <p style={{ fontSize: 14, color: 'var(--color-text-on-light)', margin: '0 0 0.5rem 0' }}>
          Until at least one bootstrap peer is reachable, the node will not appear on the map. The node process itself is fine; this is a network/infrastructure connectivity issue.
        </p>
        <ol style={{ marginBottom: 0, paddingLeft: '1.25rem' }}>
          <li><strong>Bootstrap service status</strong> — The &quot;Map / bootstrap status&quot; above uses map.kwaai.ai. If the map API is unreachable or shows bootstrap down, the issue may be on the infrastructure side; try again later. (CLI: <code>curl -s https://map.kwaai.ai/api/v1/state | jq '.bootstrap_states'</code>)</li>
          <li><strong>Outbound connectivity</strong> — From the machine running the node: test DNS and TCP to port 8000 (see <a href={TROUBLESHOOTING_URL} target="_blank" rel="noopener noreferrer">Kwaai troubleshooting</a> for exact commands).</li>
          <li><strong>Firewall / NAT</strong> — Ensure outbound <strong>TCP port 8000</strong> is allowed. Corporate or ISP firewalls often block non-HTTP ports.</li>
          <li><strong>Retry</strong> — The node retries DHT announcement every 2 minutes. Use <strong>Reconnect P2P</strong> above to restart and retry, or wait; temporary failures may resolve on their own.</li>
        </ol>
        <p style={{ marginTop: '0.75rem', fontSize: 14 }}>
          <Link to="/logs">Node log</Link> (kwaainet.log) shows &quot;Bootstrap connect failed&quot;, &quot;Announced to 2 of 2&quot;, etc. Full steps: <a href={TROUBLESHOOTING_URL} target="_blank" rel="noopener noreferrer">DEBUGGING_MAP_VISIBILITY.md — Troubleshooting</a>.
        </p>
      </div>
    </div>
  );
}
