import type { LevelCounts, ServiceStatus, ServicesView } from './types';

// Erfundene Dienste für den Mock (B-068: der Mock liefert alle Zustände). Laufende bewegen alle 2 s CPU und
// Speicher, Befehle gehen über `startet`/`stoppt` mit kurzer Verzögerung.

type Emit = (st: ServiceStatus) => void;

const MB = 1024 * 1024;
const wait = (ms: number) => new Promise((r) => setTimeout(r, ms));

function initial(now: number): ServiceStatus[] {
  const base = { description: '', health: '', log: '', pid: 0, startedAt: '', restarts: 0, lastError: '', cpu: 0, memory: 0, seq: 1 };
  const svc = (name: string, port: number, rest: Partial<ServiceStatus>): ServiceStatus => ({
    ...base, name, port, health: `http://127.0.0.1:${port}/`, state: 'gestoppt', ...rest,
  });
  return [
    svc('Vite', 5173, { tags: ['Spiel', 'Client'], description: 'Vite-Dev-Server: liefert den Browser-Client mit Hot Reload aus.', state: 'läuft', pid: 41232, cpu: 3.1, memory: 480 * MB,
      startedAt: new Date(now - (2 * 60 + 13) * 60_000).toISOString() }),
    svc('Spielserver', 8080, { tags: ['Spiel', 'Backend'], log: 'server', description: 'Go-Spielserver (Engine, API, WebSocket).' }),
    svc('Go-Server', 8090, { tags: ['Spiel', 'Backend'], state: 'fehlgeschlagen', restarts: 2, log: 'k3c-dev',
      lastError: 'Port 8090 bereits belegt (PID 8812)' }),
    svc('Spielraum', 8091, { tags: ['Spiel', 'Backend'], state: 'übernommen', pid: 8812, cpu: 0.4, memory: 96 * MB,
      startedAt: new Date(now - 42 * 60_000).toISOString() }),
  ];
}

export function mockServices(emit: Emit) {
  const list = initial(Date.now());
  const find = (name: string) => {
    const s = list.find((x) => x.name === name);
    if (!s) throw new Error(`unbekannter Dienst "${name}"; gültig: ${list.map((x) => x.name).join(', ')}`);
    return s;
  };
  const set = (s: ServiceStatus, change: Partial<ServiceStatus>) => {
    Object.assign(s, change, { seq: s.seq + 1 });
    emit({ ...s });
    return { ...s };
  };
  let tick = 0;
  setInterval(() => {
    tick++;
    for (const s of list) {
      if (s.state !== 'läuft' && s.state !== 'übernommen') continue;
      set(s, { cpu: Math.abs(3 * Math.sin(tick + s.port)), memory: s.memory + Math.round(Math.sin(tick) * 4 * MB) });
    }
  }, 2000);

  const start = async (name: string) => {
    const s = find(name);
    if (['läuft', 'startet', 'übernommen'].includes(s.state)) throw new Error(`${name} läuft bereits (${s.state})`);
    set(s, { state: 'startet', lastError: '', pid: 0 });
    await wait(1500);
    return set(s, { state: 'läuft', pid: 30000 + s.port, startedAt: new Date().toISOString(), cpu: 1, memory: 120 * MB });
  };
  const stop = async (name: string, force: boolean) => {
    const s = find(name);
    if (s.state === 'übernommen' && !force) throw new Error(`${name} ist übernommen; nur mit force stoppen`);
    if (s.state !== 'läuft' && s.state !== 'startet' && s.state !== 'übernommen') return { ...s };
    set(s, { state: 'stoppt' });
    await wait(800);
    return set(s, { state: 'gestoppt', pid: 0, cpu: 0, memory: 0 });
  };

  return {
    services: async (): Promise<ServicesView> => ({ services: list.map((s) => ({ ...s })), error: '' }),
    serviceStart: start,
    serviceStop: stop,
    serviceRestart: async (name: string) => {
      await stop(name, false);
      return start(name);
    },
    servicesStartAll: async () => {
      const todo = list.filter((s) => s.state === 'gestoppt' || s.state === 'fehlgeschlagen');
      await Promise.all(todo.map((s) => start(s.name)));
    },
    servicesStopAll: async () => {
      for (const s of [...list].reverse()) if (s.state !== 'übernommen') await stop(s.name, false);
    },
    serviceLogLevels: async (name: string): Promise<LevelCounts> => {
      const s = find(name);
      if (!s.log) throw new Error(`${name} hat kein Log`);
      tick++;
      const counts = { DEBUG: 40 + (tick % 7), INFO: 120 + tick, WARN: 3 + (tick % 3), ERROR: tick % 2 };
      return { counts, total: Object.values(counts).reduce((a, b) => a + b, 0), budgetHit: false };
    },
  };
}
