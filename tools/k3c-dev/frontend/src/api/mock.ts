import { mockLogFiles } from './mockLogFiles';
import { mockLogs } from './mockLogs';
import { mockServices } from './mockServices';
import type { Backend, EventName, Events, Info, McpState, ServiceStatus, Source } from './types';

// Mock ohne Wails-Laufzeit (`npm run dev` im Frontend): erfundene Daten, damit die Oberfläche im Browser testbar ist.
// Die Seiten lassen ihre Daten in eigenen Dateien laufen (mockServices.ts, mockLogs.ts, mockLogFiles.ts).

type Listener = (data: never) => void;

/** Quelle eines Dienstes wie serviceSource in Go (app_logs.go). */
function serviceSource(st: ServiceStatus): Source {
  const pid = st.pid > 0 ? ` · PID ${st.pid}` : '';
  return { name: st.name, kind: 'service', state: st.state, detail: `Port ${st.port}${pid}` };
}

export function mockBackend(): Backend {
  const listeners = new Map<EventName, Set<Listener>>();
  const emit = <E extends EventName>(event: E, data: Events[E]) => {
    for (const fn of listeners.get(event) ?? []) (fn as (d: Events[E]) => void)(data);
  };
  let mcp: McpState = { addr: '127.0.0.1:5180', listening: false, error: '' };
  // Der Server „startet“ kurz nach dem Laden, wie in Go das Ereignis nach Start().
  setTimeout(() => {
    mcp = { addr: mcp.addr, listening: true, error: '' };
    emit('mcp:state', mcp);
  }, 800);
  const services = mockServices((st) => {
    emit('service:state', st);
    emit('source:state', serviceSource(st));
  });
  const logs = mockLogs((lines) => emit('console:line', lines), (src) => emit('source:state', src));
  return {
    mock: true,
    info: async (): Promise<Info> => ({ version: 'mock', mcp }),
    ...services,
    ...mockLogFiles(),
    sources: async () => [...(await services.services()).services.map(serviceSource), ...logs.logSources()],
    consoleTail: async (source) => {
      const names = (await services.services()).services.map((s) => s.name);
      if (!logs.knows(source) && !names.includes(source)) throw new Error(`unbekannte Quelle "${source}"`);
      return logs.consoleTail(source);
    },
    on: (event, fn) => {
      const set = listeners.get(event) ?? new Set<Listener>();
      set.add(fn as Listener);
      listeners.set(event, set);
      return () => set.delete(fn as Listener);
    },
  };
}
