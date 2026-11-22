import { useEffect, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { backend, type BalanceSummary } from '../api';

/** Berichte zu mehreren Pfaden (Vergleich); die Reihenfolge der Pfade ändert nichts am Laden. */
export function useSummaries(paths: readonly string[]) {
  const key = [...paths].sort().join('\n');
  const [state, setState] = useState<{ key: string; byPath?: Record<string, BalanceSummary>; error?: string }>({ key: '' });
  useEffect(() => {
    if (!key) return;
    let live = true;
    const list = key.split('\n');
    Promise.all(list.map((p) => backend.balanceReport(p))).then(
      (all) => live && setState({ key, byPath: Object.fromEntries(list.map((p, i) => [p, all[i]])) }),
      (e) => live && setState({ key, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [key]);
  const cur = state.key === key ? state : { key };
  return { byPath: key ? cur.byPath : undefined, error: key ? cur.error : undefined };
}
