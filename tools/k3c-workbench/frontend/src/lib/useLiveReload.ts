import { useEffect, useRef } from 'react';
import { backend, type EventName, type Events } from '../api';
import { liveReload, type LiveReload } from './liveReload';

// React-Seite von liveReload (B-895): ein Abo je Ereignis, beim Aushängen abgemeldet.

/** Takt der Planungsseite: planning:changed feuert bei laufenden Agenten alle 4–5 s, PlanningData ist über 5 MiB groß
 *  (gemessen 2026-11-14); höchstens 4 Ladungen je Minute. */
export const PLAN_RELOAD_MS = 15_000;

/** Lädt beim Einhängen und gedrosselt bei jedem der Ereignisse neu; `run` darf sich zwischen Renders ändern.
 *  `onEvent` sieht jede Nutzlast sofort (ungedrosselt), z. B. um geänderte IDs zu sammeln, bis `run` sie abholt. */
export function useLiveReload<E extends EventName = EventName>(events: readonly E[], run: () => void, everyMs: number,
  onEvent?: (data: Events[E]) => void): () => void {
  const latest = useRef(run);
  latest.current = run;
  const seen = useRef(onEvent);
  seen.current = onEvent;
  const ctl = useRef<LiveReload | null>(null);
  useEffect(() => {
    const r = liveReload(() => latest.current(), everyMs);
    ctl.current = r;
    latest.current();
    const offs = events.map((e) => backend.on(e, (d) => { seen.current?.(d as Events[E]); r.soon(); }));
    return () => {
      r.stop();
      offs.forEach((off) => off());
      ctl.current = null;
    };
  }, [everyMs, ...events]);
  return () => ctl.current?.soon();
}
