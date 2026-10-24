import { useCallback, useEffect, useState } from 'react';
import { backend, type SourceProgress, type SourcesView } from '../api';
import { running } from './sourcesLogic';

const POLL_MS = 400;

export interface SourcesState {
  view: SourcesView | null;
  error: string;
  /** Fortschritt je Eintrag (laufend und eben beendet) */
  jobs: Map<string, SourceProgress>;
  reload: () => Promise<void>;
  download: (ids: string[]) => Promise<void>;
  adopt: (id: string) => Promise<void>;
}

/** Quellen-Zustand der Seite: Liste laden, Download starten und seinen Fortschritt abfragen, Datei übernehmen. */
export function useSources(): SourcesState {
  const [view, setView] = useState<SourcesView | null>(null);
  const [error, setError] = useState('');
  const [jobs, setJobs] = useState<Map<string, SourceProgress>>(new Map());
  const [active, setActive] = useState(false);

  const reload = useCallback(async () => {
    try {
      setView(await backend.sources());
      setError('');
    } catch (e) {
      setError((e as Error).message);
    }
  }, []);
  useEffect(() => void reload(), [reload]);

  useEffect(() => {
    if (!active) return;
    let stop = false;
    const timer = setInterval(() => {
      void backend.sourceProgress().then((list) => {
        if (stop) return;
        setJobs(new Map(list.map((j) => [j.id, j])));
        if (!running(list)) {
          setActive(false);
          void reload();
        }
      }, (e: Error) => {
        setError(e.message);
        setActive(false);
      });
    }, POLL_MS);
    return () => {
      stop = true;
      clearInterval(timer);
    };
  }, [active, reload]);

  const download = useCallback(async (ids: string[]) => {
    try {
      await backend.sourceDownload(ids);
      setError('');
      setActive(true);
    } catch (e) {
      setError((e as Error).message);
    }
  }, []);

  const adopt = useCallback(async (id: string) => {
    try {
      await backend.sourceAdopt(id);
      await reload();
    } catch (e) {
      setError((e as Error).message);
    }
  }, [reload]);

  return { view, error, jobs, reload, download, adopt };
}
