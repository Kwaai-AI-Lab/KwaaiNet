import { useState, useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';

const KWAAI_TROUBLESHOOTING_URL = 'https://github.com/Kwaai-AI-Lab/KwaaiNet/blob/main/docs/DEBUGGING_MAP_VISIBILITY.md#troubleshooting-bootstrap-connect-failed-and-broken-pipe-os-error-32';

export default function Logs() {
  const [searchParams, setSearchParams] = useSearchParams();
  const logParam = searchParams.get('log') || 'node';
  const [lines, setLines] = useState([]);
  const [loading, setLoading] = useState(true);
  const [tail, setTail] = useState(true);
  const [n, setN] = useState(200);
  const [logSource, setLogSource] = useState(logParam === 'shard' ? 'shard' : 'node');

  useEffect(() => {
    const log = searchParams.get('log') || 'node';
    setLogSource(log === 'shard' ? 'shard' : 'node');
  }, [searchParams]);

  const setLogSourceAndUrl = (source) => {
    setLogSource(source);
    setSearchParams(source === 'node' ? {} : { log: source });
  };

  useEffect(() => {
    if (!tail) return;
    let cancelled = false;
    const fetchLogs = () => {
      fetch(`/api/logs?lines=${n}&log=${logSource}`)
        .then((r) => r.json())
        .then((data) => {
          if (!cancelled) setLines(data.lines || []);
        })
        .catch(() => {
          if (!cancelled) setLines([]);
        })
        .finally(() => {
          if (!cancelled) setLoading(false);
        });
    };
    fetchLogs();
    const t = setInterval(fetchLogs, 3000);
    return () => {
      cancelled = true;
      clearInterval(t);
    };
  }, [tail, n, logSource]);

  const logHint = logSource === 'shard'
    ? 'Shard server output. Only has content when the shard is running (Start shard on Status).'
    : 'Node log (as KwaaiNet intends): main process, bootstrap, DHT, map announcements, shard_ready, startup/shutdown. Health monitoring and map connectivity are part of this process. For "Bootstrap connect failed" or "Broken pipe", see ';

  return (
    <div className="card">
      <h2 style={{ marginTop: 0 }}>Logs</h2>
      <p style={{ fontSize: 14, color: 'var(--color-text-on-light)', margin: '0 0 0.75rem 0' }}>
        {logHint}
        {logSource === 'node' && (
          <a href={KWAAI_TROUBLESHOOTING_URL} target="_blank" rel="noopener noreferrer">Kwaai&apos;s troubleshooting guide</a>
        )}
        {logSource === 'node' && '.'}
      </p>
      <div style={{ marginBottom: '0.75rem', display: 'flex', alignItems: 'center', gap: '1rem', flexWrap: 'wrap' }}>
        <label style={{ display: 'flex', alignItems: 'center', gap: '0.35rem' }}>
          Log:
          <select value={logSource} onChange={(e) => setLogSourceAndUrl(e.target.value)}>
            <option value="node">Node (kwaainet.log)</option>
            <option value="shard">Shard (shard.log)</option>
          </select>
        </label>
        <label style={{ display: 'flex', alignItems: 'center', gap: '0.35rem' }}>
          <input
            type="checkbox"
            checked={tail}
            onChange={(e) => setTail(e.target.checked)}
          />
          Auto-refresh
        </label>
        <label style={{ display: 'flex', alignItems: 'center', gap: '0.35rem' }}>
          Lines:
          <select value={n} onChange={(e) => setN(Number(e.target.value))}>
            <option value={50}>50</option>
            <option value={200}>200</option>
            <option value={500}>500</option>
          </select>
        </label>
      </div>
      {loading && lines.length === 0 ? (
        <p>Loading…</p>
      ) : (
        <pre
          style={{
            background: '#1a1a1a',
            color: '#e5e5e5',
            padding: '1rem',
            borderRadius: 6,
            fontSize: 13,
            overflow: 'auto',
            maxHeight: '70vh',
            margin: 0,
          }}
        >
          {lines.length === 0 ? (logSource === 'shard' ? 'No shard log yet. Start the shard from the Status page.' : 'No log lines.') : lines.join('\n')}
        </pre>
      )}
    </div>
  );
}
