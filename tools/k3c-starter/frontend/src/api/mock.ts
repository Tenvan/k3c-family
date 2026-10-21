import type { Backend, ServerState } from './types';

// Mock ohne Wails-Laufzeit (`npx vite` im Frontend): simuliert Start, Lauf und Beenden mit erfundenen Adressen und Meldungen.
// Wie im Fenster startet der Server beim Laden von selbst (Go: App.start in OnStartup).

const STARTUP_MS = 1200;
const STOP_MS = 800;
const BEAT_MS = 3000;
const LOG_LINES = 40;
const PORT = '8080';
const URLS = [`http://localhost:${PORT}/`, `http://192.168.178.20:${PORT}/`];

export function mockBackend(): Backend {
  let phase: 'stopped' | 'starting' | 'running' | 'stopping' = 'stopped';
  let message = 'Server ist gestoppt.';
  let log: string[] = [];
  const add = (line: string) => {
    log = [...log, line].slice(-LOG_LINES);
  };
  const clock = () => new Date().toLocaleTimeString('de-DE');

  const start = async () => {
    if (phase !== 'stopped') return;
    phase = 'starting';
    message = 'Server startet …';
    add(`${clock()} 🚀 Server startet (Mock)`);
    setTimeout(() => {
      if (phase !== 'starting') return;
      phase = 'running';
      message = 'Server läuft.';
      add(`${clock()} ✅ Server läuft auf Port ${PORT}`);
    }, STARTUP_MS);
  };
  const stop = async () => {
    if (phase !== 'running' && phase !== 'starting') return;
    phase = 'stopping';
    message = 'Server wird beendet …';
    setTimeout(() => {
      phase = 'stopped';
      message = 'Server ist gestoppt.';
      add(`${clock()} 🛑 Server gestoppt`);
    }, STOP_MS);
  };
  setInterval(() => {
    if (phase === 'running') add(`${clock()} 💓 Takt`);
  }, BEAT_MS);
  void start();

  return {
    mock: true,
    state: async (): Promise<ServerState> => ({
      // Wie Go: beim Beenden bleibt „läuft“ gesetzt, bis der Prozess weg ist.
      running: phase === 'running' || phase === 'stopping',
      starting: phase === 'starting',
      port: PORT,
      urls: phase === 'running' || phase === 'stopping' ? URLS : [],
      message,
      log: [...log],
    }),
    start,
    stop,
  };
}
