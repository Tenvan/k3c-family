import { useCallback, useEffect, useState, type RefObject } from 'react';
import { backend, type LogQuery, type LogView } from '../api';
import { errorText } from '../lib/errors';

const POLL_MS = 2000; // B-064: alle 2 s nachladen, solange die Ansicht unten steht

export interface LogQueryState {
  view: LogView | null;
  /** Ablehnung aus Go (z. B. ungültige Suche); die letzten Treffer bleiben stehen. */
  error: string;
  reload: () => Promise<void>;
}

/** Lädt den Reiter Log bei jeder Filteränderung und läuft mit, solange bottom.current wahr ist. */
export function useLogQuery(name: string, q: LogQuery, bottom: RefObject<boolean>): LogQueryState {
  const [view, setView] = useState<LogView | null>(null);
  const [error, setError] = useState('');
  const { minLevel, ns, pattern, limit } = q;

  const reload = useCallback(async () => {
    try {
      setView(await backend.logsQuery(name, { minLevel, ns, pattern, limit }));
      setError('');
    } catch (e) {
      setError(errorText(e));
    }
  }, [name, minLevel, ns, pattern, limit]);

  useEffect(() => {
    void reload();
    const t = setInterval(() => {
      if (bottom.current) void reload();
    }, POLL_MS);
    return () => clearInterval(t);
  }, [reload, bottom]);

  return { view, error, reload };
}
