import { useState, useEffect } from 'react';

const EDITABLE_KEYS = [
  { key: 'model', label: 'Model', type: 'text' },
  { key: 'blocks', label: 'Blocks', type: 'number' },
  { key: 'port', label: 'Port', type: 'number' },
  { key: 'use_gpu', label: 'Use GPU', type: 'boolean' },
  { key: 'public_name', label: 'Public name', type: 'text' },
  { key: 'public_ip', label: 'Public IP', type: 'text' },
  { key: 'log_level', label: 'Log level', type: 'text' },
];

function toInputValue(config, { key, type }) {
  const v = config[key];
  if (type === 'boolean') return v === true || v === 'true' ? 'true' : 'false';
  return v != null ? String(v) : '';
}

export default function Config() {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState(null);
  const [edit, setEdit] = useState({});

  const load = () => {
    fetch('/api/config')
      .then((r) => r.json())
      .then(setData)
      .catch(() => setData({ error: 'fetch_failed' }))
      .finally(() => setLoading(false));
  };

  useEffect(() => {
    load();
  }, []);

  useEffect(() => {
    if (data?.config) {
      const next = {};
      EDITABLE_KEYS.forEach(({ key, type }) => {
        next[key] = toInputValue(data.config, { key, type });
      });
      setEdit(next);
    }
  }, [data?.config]);

  const handleSave = async () => {
    setSaving(true);
    setMessage(null);
    const updates = {};
    EDITABLE_KEYS.forEach(({ key, type }) => {
      let v = edit[key];
      if (type === 'number') v = v === '' ? '' : Number(v);
      if (type === 'boolean') v = v === 'true' ? 'true' : 'false';
      if (v !== '' && (data?.config?.[key] == null || String(data.config[key]) !== String(v))) {
        updates[key] = v;
      }
    });
    if (Object.keys(updates).length === 0) {
      setMessage('No changes to save.');
      setSaving(false);
      return;
    }
    try {
      const res = await fetch('/api/config', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ updates }),
      });
      const json = await res.json();
      if (res.ok && json.ok) {
        setMessage('Config saved. Restart the node to apply: kwaainet restart');
        load();
      } else {
        setMessage(json.errors ? json.errors.map((e) => e.key + ': ' + (e.error || '')).join(', ') : 'Save failed');
      }
    } catch (e) {
      setMessage(e.message || 'Save failed');
    } finally {
      setSaving(false);
    }
  };

  if (loading && !data) return <div className="card">Loading config…</div>;
  if (data?.error === 'no_config') {
    return (
      <div className="card">
        <p>No config found. Run setup first (or use the setup wizard).</p>
      </div>
    );
  }

  const config = data?.config || {};

  return (
    <div className="card">
      <h2 style={{ marginTop: 0 }}>Configuration</h2>
      {message && (
        <p style={{ color: 'var(--color-accent-blue)', marginBottom: '1rem' }}>{message}</p>
      )}
      <div style={{ display: 'grid', gap: '0.75rem', maxWidth: 420 }}>
        {EDITABLE_KEYS.map(({ key, label, type }) => (
          <label key={key} style={{ display: 'grid', gridTemplateColumns: '120px 1fr', gap: '0.5rem', alignItems: 'center' }}>
            <span style={{ fontWeight: 500 }}>{label}</span>
            {type === 'boolean' ? (
              <select
                value={edit[key] ?? toInputValue(config, { key, type })}
                onChange={(e) => setEdit((p) => ({ ...p, [key]: e.target.value }))}
                style={{ padding: '0.35rem' }}
              >
                <option value="true">true</option>
                <option value="false">false</option>
              </select>
            ) : (
              <input
                type={type === 'number' ? 'number' : 'text'}
                value={edit[key] ?? toInputValue(config, { key, type })}
                onChange={(e) => setEdit((p) => ({ ...p, [key]: e.target.value }))}
                style={{ padding: '0.35rem' }}
              />
            )}
          </label>
        ))}
      </div>
      <div style={{ marginTop: '1rem' }}>
        <button className="btn-primary" onClick={handleSave} disabled={saving}>
          {saving ? 'Saving…' : 'Save config'}
        </button>
      </div>
    </div>
  );
}
