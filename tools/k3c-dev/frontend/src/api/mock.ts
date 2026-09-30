import type { Backend, EventName, Events, Info, McpState } from './types';

// Mock ohne Wails-Laufzeit (`npm run dev` im Frontend): erfundene Daten, damit die Oberfläche im Browser testbar ist.
// Die Seiten ab M4.2 lassen hier ihre Daten laufen (Konsole, Logs, Dienste).

type Listener = (data: never) => void;

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
  return {
    mock: true,
    info: async (): Promise<Info> => ({ version: 'mock', mcp }),
    on: (event, fn) => {
      const set = listeners.get(event) ?? new Set<Listener>();
      set.add(fn as Listener);
      listeners.set(event, set);
      return () => set.delete(fn as Listener);
    },
  };
}
