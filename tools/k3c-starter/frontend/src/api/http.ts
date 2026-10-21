import type { Backend, ServerState } from './types';

// HTTP-Backend für den Entwicklungsmodus (`task k3c-starter:dev`): der Starter läuft als `-headless` ohne Fenster und
// bietet dieselben drei Aufrufe wie die Wails-Bindungen unter /api/starter/ (api.go). Vite leitet /api an ihn weiter
// (vite.config.ts); die Kopfzeile X-K3C-Starter verlangt der Starter bei jedem Aufruf. Die Oberfläche fragt den Zustand
// ohnehin im Sekundentakt ab (useServerState), eine Dauerverbindung braucht es nicht.

const BASE = '/api/starter';
const HEADERS = { 'X-K3C-Starter': '1' };

async function call<T>(method: 'GET' | 'POST', path: string): Promise<T> {
  const res = await fetch(`${BASE}/${path}`, { method, headers: HEADERS });
  if (!res.ok) {
    throw new Error(`Starter-API nicht erreichbar (${path}: HTTP ${res.status}). Läuft task k3c-starter:dev?`);
  }
  return (await res.json()) as T;
}

export function httpBackend(): Backend {
  return {
    mock: false,
    state: () => call<ServerState>('GET', 'state'),
    start: async () => void (await call<ServerState>('POST', 'start')),
    stop: async () => void (await call<ServerState>('POST', 'stop')),
  };
}
