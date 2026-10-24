import { useEffect } from 'react';
import type { GitPRList } from '../api';

/** Takt der PR-Karte: ruhig 30 s, solange eine CI läuft oder GitHub die Mergebarkeit noch rechnet 10 s. */
export const PR_POLL_MS = 30_000;
export const PR_POLL_BUSY_MS = 10_000;

export function prPollMs(list: GitPRList): number {
  const busy = list.prs.some((p) => p.ci === 'läuft' || p.merge === 'unbekannt');
  return busy ? PR_POLL_BUSY_MS : PR_POLL_MS;
}

/** Holt die PRs im Takt neu, solange der Reiter offen ist (B-580 Rev. 4): CI, Merge-Stand und Merges von außen
 *  erscheinen ohne Knopf. Jede neue Liste stellt den nächsten Abruf; ohne Liste (noch nie geladen) wartet der Takt. */
export function usePrPoll(active: boolean, list: GitPRList | null, refresh: () => Promise<unknown>) {
  useEffect(() => {
    if (!active || !list) return;
    const id = window.setTimeout(() => void refresh(), prPollMs(list));
    return () => window.clearTimeout(id);
  }, [active, list, refresh]);
}
