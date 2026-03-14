import { useState, useEffect } from 'react';
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import Header from './components/Header';
import SetupWizard from './components/SetupWizard';
import Status from './components/Status';
import Config from './components/Config';
import Logs from './components/Logs';
import Identity from './components/Identity';
import Network from './components/Network';

function App() {
  const [setupStatus, setSetupStatus] = useState(null);
  const [skipWizard, setSkipWizard] = useState(false);

  useEffect(() => {
    fetch('/api/setup-status')
      .then((r) => r.json())
      .then(setSetupStatus)
      .catch(() => setSetupStatus({ needs_setup: true, step: 'none' }));
  }, []);

  const showWizard =
    setupStatus &&
    (setupStatus.cli_not_found || (setupStatus.needs_setup && !skipWizard));
  const showDashboard = !showWizard || skipWizard;

  if (!setupStatus) {
    return (
      <div style={{ padding: '3rem', textAlign: 'center', color: 'var(--color-text-on-light)' }}>
        Loading…
      </div>
    );
  }

  return (
    <BrowserRouter>
      {!showWizard || !setupStatus.cli_not_found ? <Header /> : null}
      {showWizard ? (
        <SetupWizard
          onComplete={() => setSkipWizard(true)}
          cliNotFound={!!setupStatus.cli_not_found}
        />
      ) : (
        <main style={{ padding: '1.5rem', maxWidth: 900, margin: '0 auto' }}>
          <Routes>
            <Route path="/" element={<Status />} />
            <Route path="/network" element={<Network />} />
            <Route path="/config" element={<Config />} />
            <Route path="/logs" element={<Logs />} />
            <Route path="/identity" element={<Identity />} />
          </Routes>
        </main>
      )}
    </BrowserRouter>
  );
}

export default App;
