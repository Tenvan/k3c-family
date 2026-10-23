import { useEffect, useRef, useState } from 'react';
import { backend, type ResourceView } from '../api';
import { errorText } from '@webkit/lib/errors';

// Daten der Seite Monitoring. Kein Verlauf im Browser: jede Antwort ersetzt die vorige (Go verdichtet auf höchstens
// 240 Punkte), und abgefragt wird nur, solange die Seite sichtbar ist. So kann die Seite selbst nicht wachsen.

/** Abstand der Abfragen; Go nimmt alle 10 s eine Probe. */
export const REFRESH_MS = 10_000;

export interface ResourcesData {
  view: ResourceView | null;
  /** Grund, falls die letzte Abfrage scheiterte; die letzten Zahlen bleiben stehen. */
  error: string;
}

/** active falsch (Seite verdeckt) oder Fenster im Hintergrund: keine Abfrage, kein Timer. */
export function useResources(windowMin: number, active: boolean): ResourcesData {
  const [view, setView] = useState<ResourceView | null>(null);
  const [error, setError] = useState('');
  const latest = useRef(0);

  useEffect(() => {
    if (!active) return;
    let stopped = false;
    let timer: ReturnType<typeof setTimeout> | undefined;
    const tick = async () => {
      const id = ++latest.current;
      if (!document.hidden) {
        try {
          const v = await backend.resources(windowMin);
          if (!stopped && id === latest.current) {
            setView(v);
            setError('');
          }
        } catch (e) {
          if (!stopped && id === latest.current) setError(errorText(e));
        }
      }
      if (!stopped) timer = setTimeout(() => void tick(), REFRESH_MS);
    };
    void tick();
    const onVisible = () => {
      if (!document.hidden && !stopped) {
        clearTimeout(timer);
        void tick();
      }
    };
    document.addEventListener('visibilitychange', onVisible);
    return () => {
      stopped = true;
      latest.current++;
      clearTimeout(timer);
      document.removeEventListener('visibilitychange', onVisible);
    };
  }, [windowMin, active]);

  return { view, error };
}
