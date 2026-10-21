import { errorText } from '@webkit/lib/errors';
import { useCallback, useEffect, useState } from 'react';
import { backend, type ServerState } from '../api';

const POLL_MS = 1000;

const CONNECTING: ServerState = { running: false, starting: false, port: '', urls: [], message: 'Verbinde …', log: [] };

/** Zustand des Servers, jede Sekunde neu abgefragt; `refresh` fragt sofort (nach Start und Beenden). */
export function useServerState(): { state: ServerState; refresh: () => Promise<void> } {
  const [state, setState] = useState<ServerState>(CONNECTING);
  const refresh = useCallback(async () => {
    try {
      setState(await backend.state());
    } catch (e) {
      setState((cur) => ({ ...cur, message: errorText(e) }));
    }
  }, []);
  // Nächste Abfrage erst nach der Antwort (wie die alte Seite), damit sich langsame Antworten nicht überholen.
  useEffect(() => {
    let timer: ReturnType<typeof setTimeout> | undefined;
    let alive = true;
    const loop = async () => {
      await refresh();
      if (alive) timer = setTimeout(() => void loop(), POLL_MS);
    };
    void loop();
    return () => {
      alive = false;
      clearTimeout(timer);
    };
  }, [refresh]);
  return { state, refresh };
}
