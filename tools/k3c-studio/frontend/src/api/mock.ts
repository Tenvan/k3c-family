import { mockAssets } from './mockAssets';
import { mockWorldLevel } from './mockWorld';
import type { AssetChoice, Backend, EventName, Events } from './types';

/** Backend ohne Wails (Browser, Tests): erfundene Daten; `external` spielt einen fremden Schreiber nach. */
export function mockBackend(): Backend & { external: (id: string, wahl: AssetChoice[], quiet?: boolean) => void } {
  const subs = new Map<EventName, Set<(data: never) => void>>();
  const emit = <E extends EventName>(event: E, data: Events[E]) => subs.get(event)?.forEach((fn) => (fn as (d: Events[E]) => void)(data));
  return {
    mock: true,
    ...mockAssets(() => emit('assets:changed', null)),
    worldLevel: async (seed, source) => mockWorldLevel(seed, source),
    openUrl: (url) => window.open(url, '_blank', 'noopener'),
    on: <E extends EventName>(event: E, fn: (data: Events[E]) => void) => {
      const set = subs.get(event) ?? new Set();
      set.add(fn as (data: never) => void);
      subs.set(event, set);
      return () => set.delete(fn as (data: never) => void);
    },
  };
}
