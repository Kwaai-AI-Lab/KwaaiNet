import { useState, useEffect } from 'react';

export default function Identity() {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    let cancelled = false;
    fetch('/api/identity')
      .then((r) => r.json())
      .then((json) => {
        if (!cancelled) setData(json);
      })
      .catch(() => {
        if (!cancelled) setData({ error: 'fetch_failed' });
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });
    return () => { cancelled = true; };
  }, []);

  if (loading && !data) return <div className="card">Loading identity…</div>;
  if (data?.error === 'cli_not_found') {
    return (
      <div className="card" style={{ borderColor: 'var(--color-error)' }}>
        <p style={{ color: 'var(--color-error)' }}>KwaaiNet CLI not found.</p>
      </div>
    );
  }

  return (
    <div className="card">
      <h2 style={{ marginTop: 0 }}>Identity</h2>
      <dl style={{ margin: 0 }}>
        {data?.did && (
          <>
            <dt style={{ fontWeight: 500, marginTop: '0.5rem' }}>DID</dt>
            <dd style={{ margin: '0.25rem 0 0 0', wordBreak: 'break-all' }}><code>{data.did}</code></dd>
          </>
        )}
        {data?.peer_id && (
          <>
            <dt style={{ fontWeight: 500, marginTop: '0.5rem' }}>Peer ID</dt>
            <dd style={{ margin: '0.25rem 0 0 0', wordBreak: 'break-all' }}><code>{data.peer_id}</code></dd>
          </>
        )}
        {data?.trust_tier && (
          <>
            <dt style={{ fontWeight: 500, marginTop: '0.5rem' }}>Trust tier</dt>
            <dd style={{ margin: '0.25rem 0 0 0' }}>{data.trust_tier}</dd>
          </>
        )}
      </dl>
      {data?.raw && (
        <details style={{ marginTop: '1rem' }}>
          <summary style={{ cursor: 'pointer', color: 'var(--color-accent-blue)' }}>Raw output</summary>
          <pre style={{ background: 'var(--color-muted)', padding: '0.75rem', borderRadius: 6, fontSize: 12, overflow: 'auto', marginTop: '0.5rem' }}>
            {data.raw}
          </pre>
        </details>
      )}
    </div>
  );
}
