import { Link, useLocation } from 'react-router-dom';

const NAV = [
  { path: '/', label: 'Status' },
  { path: '/network', label: 'Network' },
  { path: '/config', label: 'Config' },
  { path: '/logs', label: 'Logs' },
  { path: '/identity', label: 'Identity' },
];

export default function Header() {
  const location = useLocation();

  return (
    <header
      style={{
        background: 'var(--color-header)',
        color: 'var(--color-text-on-dark)',
        padding: '0.75rem 1.5rem',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
      }}
    >
      <div style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
        <span style={{ fontSize: '1.25rem' }} aria-hidden>🌳</span>
        <span style={{ fontWeight: 600, fontSize: '1.1rem' }}>KwaaiNet</span>
      </div>
      <nav style={{ display: 'flex', alignItems: 'center', gap: '1.5rem' }}>
        {NAV.map(({ path, label }) => (
          <Link
            key={path}
            to={path}
            style={{
              color: 'var(--color-text-on-dark)',
              textDecoration: 'none',
              fontWeight: location.pathname === path ? 600 : 400,
              opacity: location.pathname === path ? 1 : 0.85,
            }}
          >
            {label}
          </Link>
        ))}
        <a
          href="https://map.kwaai.ai"
          target="_blank"
          rel="noopener noreferrer"
          style={{ color: 'var(--color-accent-blue)', marginLeft: '0.5rem' }}
        >
          View on map
        </a>
      </nav>
    </header>
  );
}
