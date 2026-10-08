import { useEffect, useState } from 'react';
import { backend, type TaskCatalog, type TaskGates, type TaskRun } from '../api';
import { errorText } from '../lib/errors';
import { pushHistory, upsertRun } from './tasks';

/** Daten der Tasks-Seite aus Go: Katalog, Läufe (mit Verlauf dieser Sitzung) und Freigabe-Schlösser. */
export function useTasks() {
  const [catalog, setCatalog] = useState<TaskCatalog | null>(null);
  const [loadError, setLoadError] = useState('');
  const [runs, setRuns] = useState<TaskRun[]>([]);
  const [history, setHistory] = useState<TaskRun[]>([]);
  const [gates, setGates] = useState<TaskGates>({ states: {}, pending: 0 });
  const [gateError, setGateError] = useState('');
  const [reloading, setReloading] = useState(false);

  useEffect(() => {
    const off = backend.on('task:state', (run) => {
      setRuns((list) => upsertRun(list, run));
      setHistory((h) => pushHistory(h, run));
    });
    backend.taskRuns().then((r) => setRuns((cur) => r.reduce(upsertRun, cur)), () => undefined);
    backend.tasks().then(setCatalog, (e) => setLoadError(errorText(e)));
    backend.taskGates().then(setGates, () => undefined);
    return off;
  }, []);

  const reload = async () => {
    setReloading(true);
    setLoadError('');
    try {
      setCatalog(await backend.tasksReload());
      setGates(await backend.taskGates());
    } catch (e) {
      setLoadError(errorText(e));
    } finally {
      setReloading(false);
    }
  };
  const gate = (fn: () => Promise<TaskGates>) => {
    setGateError('');
    fn().then(setGates, (e) => setGateError(errorText(e)));
  };
  return {
    catalog, loadError, runs, history, gates, gateError, reloading,
    reload: () => void reload(),
    toggleGate: (name: string) => gate(() => backend.taskGateToggle(name)),
    saveGates: () => gate(() => backend.taskGateSave()),
  };
}
