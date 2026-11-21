import { useCallback, useEffect, useState } from 'react';
import { formatWeltHash, parseWeltHash, type WeltFilterState } from './weltFilter';

/** Welt-Filter mit Zustand in der Adresse (`#welt=…&modus=…`): beim Start aus `location.hash`, jede Änderung schreibt ihn
 *  zurück (ohne neuen Verlaufseintrag), eine Änderung der Adresse von außen (Link, Zurück) wirkt sofort. Zugriff auf
 *  Adresse oder Verlauf kann scheitern (Vorschau, Test); dann gilt der Zustand nur im Speicher. */
export function useWeltFilter(): [WeltFilterState, (patch: Partial<WeltFilterState>) => void] {
  const [state, setState] = useState<WeltFilterState>(() => read());
  useEffect(() => {
    const onHash = () => setState(read());
    window.addEventListener('hashchange', onHash);
    return () => window.removeEventListener('hashchange', onHash);
  }, []);
  const change = useCallback((patch: Partial<WeltFilterState>) => {
    setState((cur) => {
      const next = { ...cur, ...patch };
      write(next);
      return next;
    });
  }, []);
  return [state, change];
}

function read(): WeltFilterState {
  try {
    return parseWeltHash(window.location.hash);
  } catch {
    return parseWeltHash('');
  }
}

function write(state: WeltFilterState) {
  try {
    const hash = formatWeltHash(state, window.location.hash);
    if (hash !== window.location.hash) window.history.replaceState(null, '', `${window.location.pathname}${window.location.search}${hash}`);
  } catch {
    // Adresse nicht beschreibbar: der Zustand lebt nur im Speicher
  }
}
