import { mockAssets } from './mockAssets';
import { mockBalance } from './mockBalance';
import { mockBalanceData } from './mockBalanceData';
import { mockHub } from './mockHub';
import { mockWorld } from './mockWorld';
import type { Backend, EventName, Events, Info, McpState } from './types';

// Mock ohne Wails-Laufzeit (`?mock=1`, Vitest): erfundene Daten, damit die Oberfläche im Browser testbar ist. Die
// Bereiche lassen ihre Daten in eigenen Dateien laufen (mockWorld.ts, mockAssets.ts, mockHub.ts,
// mockBalance*.ts).

type Listener = (data: never) => void;

export function mockBackend(): Backend {
  const listeners = new Map<EventName, Set<Listener>>();
  const emit = <E extends EventName>(event: E, data: Events[E]) => {
    for (const fn of listeners.get(event) ?? []) (fn as (d: Events[E]) => void)(data);
  };
  let mcp: McpState = { addr: '127.0.0.1:5181', listening: false, error: '' };
  // Der Server „startet“ kurz nach dem Laden, wie in Go das Ereignis nach Start().
  setTimeout(() => {
    mcp = { addr: mcp.addr, listening: true, error: '' };
    emit('mcp:state', mcp);
  }, 800);
  return {
    mock: true,
    info: async (): Promise<Info> => ({ version: 'mock', mcp }),
    ...mockAssets(),
    ...mockWorld(),
    ...mockHub(),
    ...mockBalance({ run: (run) => emit('balance:runs', run), reports: () => emit('balance:reports', null) }),
    ...mockBalanceData(),
    openUrl: (url) => {
      window.open(url, '_blank', 'noopener');
    },
    on: (event, fn) => {
      const set = listeners.get(event) ?? new Set<Listener>();
      set.add(fn as Listener);
      listeners.set(event, set);
      return () => set.delete(fn as Listener);
    },
  };
}
