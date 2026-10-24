import { useCallback, useEffect, useState } from 'react';
import { backend } from '../api';

/** Takt der Frage „hat sich das Repo geändert?“; der Fingerabdruck kostet nur ein paar stat-Aufrufe. */
export const STAMP_POLL_MS = 3_000;

/** Beobachtet den Repo-Zustand (B-580 Rev. 5): Ändert er sich, auch von außen (Terminal, Claude), ruft der Hook
 *  onChange und meldet `stale`, bis die Seite neu geladen hat. `mark` holt den Fingerabdruck vor dem Laden und gibt
 *  eine Funktion zurück, die ihn nach dem Laden als neuen Stand setzt; so löst das eigene Laden nichts aus. */
export function useRepoStamp(active: boolean, onChange: () => void) {
  const [base, setBase] = useState('');
  const [stale, setStale] = useState(false);
  const mark = useCallback(async () => {
    const s = await backend.gitStamp().catch(() => '');
    return () => {
      setBase(s);
      setStale(false);
    };
  }, []);
  useEffect(() => {
    if (!active || !base) return;
    const id = window.setInterval(() => {
      backend.gitStamp().then((s) => {
        if (!s || s === base) return;
        setBase(s);
        setStale(true);
        onChange();
      }, () => undefined);
    }, STAMP_POLL_MS);
    return () => window.clearInterval(id);
  }, [active, base, onChange]);
  return { stale, mark };
}
