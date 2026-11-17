import { useEffect, useState } from 'react';
import { backend, type TaskCatalog, type TaskRun } from '../api';
import { errorText } from '@webkit/lib/errors';
import { pushHistory, upsertRun } from './tasks';

/** Daten der Tasks-Seite aus Go: Katalog, Läufe (mit Verlauf dieser Sitzung). */
export function useTasks() {
  const [catalog, setCatalog] = useState<TaskCatalog | null>(null);
  const [loadError, setLoadError] = useState('');
  const [runs, setRuns] = useState<TaskRun[]>([]);
  const [history, setHistory] = useState<TaskRun[]>([]);
  const [reloading, setReloading] = useState(false);

  useEffect(() => {
    const off = backend.on('task:state', (run) => {
      setRuns((list) => upsertRun(list, run));
      setHistory((h) => pushHistory(h, run));
    });
    backend.taskRuns().then((r) => setRuns((cur) => r.reduce(upsertRun, cur)), () => undefined);
    backend.tasks().then(setCatalog, (e) => setLoadError(errorText(e)));
    return off;
  }, []);

  const reload = async () => {
    setReloading(true);
    setLoadError('');
    try {
      setCatalog(await backend.tasksReload());
    } catch (e) {
      setLoadError(errorText(e));
    } finally {
      setReloading(false);
    }
  };
  return {
    catalog, loadError, runs, history, reloading,
    reload: () => void reload(),
  };
}
