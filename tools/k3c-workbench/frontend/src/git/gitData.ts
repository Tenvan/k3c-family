import { useCallback, useEffect, useRef, useState } from 'react';
import { errorText } from '@webkit/lib/errors';
import { backend, type GitPRList, type GitView } from '../api';
import { usePrPoll } from './prPoll';
import { useRepoStamp } from './repoStamp';

/** Stand der Seite Git und wann er neu kommt (B-580 Rev. 6): beim ersten Öffnen, mit „Neu laden“, nach eigenen
 *  Aktionen und bei jeder Änderung am Repo, auch von außen (repoStamp.ts); die PRs zusätzlich im Takt von GitHub
 *  (prPoll.ts). Läuft schon ein Laden, folgt danach genau eines; so sieht die Seite nie einen halben Stand als letzten. */
export function useGitData(active: boolean) {
  const [view, setView] = useState<GitView | null>(null);
  const [prs, setPrs] = useState<GitPRList | null>(null);
  const [error, setError] = useState('');
  const [loadedAt, setLoadedAt] = useState(0);
  const started = useRef(false);
  const busy = useRef(false);
  const again = useRef(false);
  const loadRef = useRef<() => Promise<void>>(async () => undefined);

  // Ein Fehler beim Nachladen lässt die zuletzt gesehenen PRs stehen und zeigt nur den Hinweis.
  const refreshPrs = useCallback(() => backend.gitPRs().then(setPrs,
    (e: unknown) => setPrs((cur) => ({ prs: cur?.prs ?? [], error: errorText(e), fetched: cur?.fetched }))), []);
  usePrPoll(active, prs, refreshPrs);
  const { stale, mark } = useRepoStamp(active, useCallback(() => void loadRef.current(), []));

  const load = useCallback(async () => {
    if (busy.current) {
      again.current = true;
      return;
    }
    busy.current = true;
    try {
      do {
        again.current = false;
        const settled = await mark(); // vor dem Lesen: eine Änderung währenddessen fällt beim nächsten Takt auf
        const prsTask = refreshPrs();
        await backend.gitOverview().then((v) => {
          setView(v);
          setError('');
        }, (e: unknown) => setError(errorText(e)));
        await prsTask;
        settled();
        setLoadedAt(Date.now());
      } while (again.current);
    } finally {
      busy.current = false;
    }
  }, [mark, refreshPrs]);
  loadRef.current = load;

  useEffect(() => {
    if (!active || started.current) return;
    started.current = true;
    void load();
  }, [active, load]);
  return { view, setView, prs, error, loadedAt, stale, load };
}
