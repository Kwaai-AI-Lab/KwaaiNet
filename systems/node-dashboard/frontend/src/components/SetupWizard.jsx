import { useState } from 'react';

const STEPS = [
  { id: 'init', label: 'Initialize', action: 'setup', description: 'Create config and identity for your node.' },
  { id: 'get-deps', label: 'Download p2pd', action: 'setup-get-deps', description: 'Download p2pd if missing (optional).' },
  { id: 'benchmark', label: 'Benchmark', action: 'benchmark', description: 'Measure your hardware throughput (optional).' },
  { id: 'start', label: 'Start node', action: 'start', description: 'Start the KwaaiNet node in the background.' },
];

export default function SetupWizard({ onComplete, cliNotFound }) {
  const [stepIndex, setStepIndex] = useState(0);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);
  const [message, setMessage] = useState('');

  const step = STEPS[stepIndex];
  const isLast = stepIndex === STEPS.length - 1;

  async function runAction(action) {
    setLoading(true);
    setError(null);
    setMessage('');
    try {
      const res = await fetch(`/api/${action}`, { method: 'POST' });
      const data = await res.json().catch(() => ({}));
      if (!res.ok) {
        setError(data.stderr || data.error || data.message || 'Request failed');
        return;
      }
      setMessage(data.stdout || 'Done.');
      if (isLast) {
        setTimeout(() => onComplete(), 1500);
      } else {
        setStepIndex((i) => i + 1);
      }
    } catch (e) {
      setError(e.message || 'Network error');
    } finally {
      setLoading(false);
    }
  }

  if (cliNotFound) {
    return (
      <div style={{ maxWidth: 560, margin: '3rem auto', padding: '0 1rem' }}>
        <div className="card" style={{ borderColor: 'var(--color-error)' }}>
          <h2 style={{ color: 'var(--color-error)', marginTop: 0 }}>KwaaiNet CLI not found</h2>
          <p>
            Install the <code>kwaainet</code> CLI first: build from source or use the install script, then ensure it is on your PATH.
          </p>
          <p>
            <a href="https://github.com/Kwaai-AI-Lab/KwaaiNet#readme" target="_blank" rel="noopener noreferrer">
              Installation instructions
            </a>
          </p>
        </div>
      </div>
    );
  }

  return (
    <div style={{ maxWidth: 560, margin: '3rem auto', padding: '0 1rem' }}>
      <div
        style={{
          background: 'var(--color-accent-blue)',
          color: 'var(--color-text-on-dark)',
          padding: '0.5rem 1rem',
          borderRadius: 8,
          marginBottom: '1.5rem',
          textAlign: 'center',
        }}
      >
        Set up your KwaaiNet node
      </div>
      <div className="card">
        <h2 style={{ marginTop: 0 }}>{step.label}</h2>
        <p style={{ color: 'var(--color-text-on-light)' }}>{step.description}</p>
        {error && (
          <p style={{ color: 'var(--color-error)', marginBottom: '1rem' }}>{error}</p>
        )}
        {message && (
          <pre
            style={{
              background: 'var(--color-muted)',
              padding: '0.75rem',
              borderRadius: 6,
              fontSize: 13,
              overflow: 'auto',
              minHeight: 80,
              maxHeight: 'min(70vh, 420px)',
              whiteSpace: 'pre-wrap',
              wordBreak: 'break-word',
              margin: 0,
            }}
          >
            {message}
          </pre>
        )}
        <div style={{ display: 'flex', gap: '0.75rem', flexWrap: 'wrap' }}>
          <button
            className="btn-primary"
            disabled={loading}
            onClick={() => runAction(step.action)}
          >
            {loading ? 'Running…' : step.id === 'init' ? 'Initialize KwaaiNet' : step.id === 'get-deps' ? 'Download p2pd' : step.id === 'benchmark' ? 'Run benchmark' : 'Start node'}
          </button>
          {(step.id === 'get-deps' || step.id === 'benchmark') && (
            <button
              className="btn-secondary"
              disabled={loading}
              onClick={() => setStepIndex((i) => i + 1)}
            >
              Skip
            </button>
          )}
          {stepIndex > 0 && (
            <button className="btn-secondary" disabled={loading} onClick={() => setStepIndex((i) => i - 1)}>
              Back
            </button>
          )}
        </div>
      </div>
      <p style={{ textAlign: 'center', marginTop: '1rem' }}>
        <button
          type="button"
          className="btn-secondary"
          onClick={onComplete}
          style={{ background: 'transparent', color: 'var(--color-accent-blue)' }}
        >
          I'm already set up
        </button>
      </p>
    </div>
  );
}
