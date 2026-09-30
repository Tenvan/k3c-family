import { mockLogFiles } from './mockLogFiles';
import { mockLogs } from './mockLogs';
import { mockMcp } from './mockMcp';
import { mockServices } from './mockServices';
import type { Backend, EventName, Events, Info, McpState, ServiceStatus, Source } from './types';

// Mock ohne Wails-Laufzeit (`npm run dev` im Frontend): erfundene Daten, damit die Oberfläche im Browser testbar ist.
// Die Seiten lassen ihre Daten in eigenen Dateien laufen (mockServices.ts, mockLogs.ts, mockLogFiles.ts, mockMcp.ts).

type Listener = (data: never) => void;

const INSTRUCTIONS = `# k3c-dev (Mock)

Entwickler-Werkzeug des Spiels K3C. Diese Tools ersetzen **Shell-Befehle** und Dateilesen.

## Prüfen

- \`check_run\` statt \`npm run check\` in der Shell. Ziele: \`npm:check\`, \`npm:test\`,
  \`go:test\`.
- Mehr Kontext zu einem Lauf: \`console_tail\` mit \`check:<ziel>\`.

## Dienste

Dienste nie per Shell starten, sondern mit \`svc_start\`.`;

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
  const mcpTools = mockMcp((event, call) => emit(event, call), () => mcp);
  const logs = mockLogs((lines) => emit('console:line', lines), (src) => emit('source:state', src));
  return {
    mock: true,
    info: async (): Promise<Info> => ({ version: 'mock', mcp }),
    ...services,
    ...mockLogFiles(),
    ...mcpTools,
    mcpInstructions: async () => INSTRUCTIONS,
    mcpRestart: async () => {
      mcp = { ...mcp, listening: false, error: '' };
      emit('mcp:state', mcp);
      await new Promise((r) => setTimeout(r, 600));
      mcp = { ...mcp, listening: true };
      emit('mcp:state', mcp);
      return mcp;
    },
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
