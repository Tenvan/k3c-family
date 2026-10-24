import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type BalanceReportInfo, type BalanceRun, type BalanceSpec, type BalanceSummary, type Tuning } from '../api';
import { errorText } from '@webkit/lib/errors';
import { isActive, mergeRuns } from './spec';

/** Abfrage-Takt, solange ein Lauf wartet oder läuft (Vertrag: 1–2 s). */
export const POLL_MS = 1500;

/** Läufe (mit Live-Status) und Berichtsliste der Balancing-Seite; die Seite blockiert nie, Fehler werden als Text gehalten. */
export function useBalancing(active: boolean) {
  const [runs, setRuns] = useState<BalanceRun[]>([]);
  const [reports, setReports] = useState<BalanceReportInfo[]>([]);
  const [error, setError] = useState('');
  const busy = useRef(0);

  const loadReports = useCallback(() => {
    backend.balanceReports().then(setReports, (e) => setError(errorText(e)));
  }, []);
  const loadRuns = useCallback(() => {
    backend.balanceRuns().then((r) => setRuns((cur) => mergeRuns(cur, r)), (e) => setError(errorText(e)));
  }, []);

  useEffect(() => {
    loadRuns();
    loadReports();
  }, [loadRuns, loadReports]);

  const open = runs.filter(isActive).length;
  useEffect(() => {
    // Ein Lauf ist fertig geworden: neue Berichte holen.
    if (open < busy.current) loadReports();
    busy.current = open;
  }, [open, loadReports]);
  useEffect(() => {
    if (!active || open === 0) return;
    const t = setInterval(loadRuns, POLL_MS);
    return () => clearInterval(t);
  }, [active, open, loadRuns]);

  const start = async (spec: BalanceSpec) => {
    setError('');
    try {
      const run = await backend.balanceStart(spec);
      setRuns((cur) => mergeRuns(cur, [run]));
    } catch (e) {
      setError(errorText(e));
    }
  };
  const stop = async (id: string) => {
    setError('');
    try {
      await backend.balanceStop(id);
      loadRuns();
    } catch (e) {
      setError(errorText(e));
    }
  };
  return { runs, reports, error, start, stop, reloadReports: loadReports };
}

/** Bericht zu einem Pfad; leerer Pfad = keiner. */
export function useSummary(path: string) {
  const [state, setState] = useState<{ path: string; summary?: BalanceSummary; error?: string }>({ path: '' });
  useEffect(() => {
    if (!path) return;
    let live = true;
    backend.balanceReport(path).then(
      (summary) => live && setState({ path, summary }),
      (e) => live && setState({ path, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [path]);
  const cur = state.path === path ? state : { path };
  return { summary: path ? cur.summary : undefined, error: path ? cur.error : undefined, loading: !!path && !cur.summary && !cur.error };
}

/** Abstimm-Ergebnis zu einem Pfad; leerer Pfad = keines. */
export function useTuning(path: string) {
  const [state, setState] = useState<{ path: string; tuning?: Tuning; error?: string }>({ path: '' });
  useEffect(() => {
    if (!path) return;
    let live = true;
    backend.balanceTuning(path).then(
      (tuning) => live && setState({ path, tuning }),
      (e) => live && setState({ path, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [path]);
  const cur = state.path === path ? state : { path };
  return { tuning: path ? cur.tuning : undefined, error: path ? cur.error : undefined, loading: !!path && !cur.tuning && !cur.error };
}
