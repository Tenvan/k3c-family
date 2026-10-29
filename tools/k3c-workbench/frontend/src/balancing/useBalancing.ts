import { useCallback, useEffect, useRef, useState } from 'react';
import { backend, type BalanceBaseline, type BalanceInsights, type BalanceReportInfo, type BalanceRun, type BalanceSpec, type BalanceSummary, type ReportRating, type Tuning } from '../api';
import { errorText } from '@webkit/lib/errors';
import { mergeRuns } from './spec';

interface Loaders {
  loadRuns: () => void;
  loadReports: () => void;
  loadBaselines: () => void;
}

/** Aufrufe der Seite: Fehler als Text, danach die betroffene Liste neu laden. Sammel-Aufrufe und Baseline liefern „geschafft“. */
function balanceActions(setError: (text: string) => void, { loadRuns, loadReports, loadBaselines }: Loaders) {
  const attempt = async (fn: () => Promise<unknown>, after: () => void): Promise<boolean> => {
    setError('');
    try {
      await fn();
      after();
      return true;
    } catch (e) {
      setError(errorText(e));
      return false;
    }
  };
  const call = async (fn: () => Promise<unknown>, after: () => void) => {
    await attempt(fn, after);
  };
  return {
    stop: (id: string) => call(() => backend.balanceStop(id), loadRuns),
    // Berichte bewerten und löschen (WZ28.13), auch gesammelt (B-662).
    rate: (path: string, rating: ReportRating) => call(() => backend.balanceReportRate(path, rating), loadReports),
    remove: (path: string) => call(() => backend.balanceReportDelete(path), loadReports),
    rateMany: (paths: string[], rating: ReportRating) => attempt(() => backend.balanceReportsRate(paths, rating), loadReports),
    removeMany: (paths: string[]) => attempt(() => backend.balanceReportsDelete(paths), loadReports),
    /** Durchgelaufenen Bericht als Baseline sichern (B-662), wie sim_test {action: baseline}. */
    saveBaseline: (path: string, name: string) => attempt(() => backend.balanceSaveBaseline(path, name), loadBaselines),
  };
}

/**
 * Läufe (mit Live-Status), Berichte und Baselines der Balancing-Seite; die Seite blockiert nie, Fehler werden als Text
 * gehalten. Kein Abfrage-Takt (B-662): geladen wird, wenn die Seite sichtbar wird, und bei den Ereignissen
 * `balance:runs` (Läufe), `balance:reports` (Berichte und Baselines) und `ui:reconnect` (alles); `reload` lädt von Hand.
 */
export function useBalancing(active: boolean) {
  const [runs, setRuns] = useState<BalanceRun[]>([]);
  const [reports, setReports] = useState<BalanceReportInfo[]>([]);
  const [baselines, setBaselines] = useState<BalanceBaseline[]>([]);
  const [error, setError] = useState('');

  const loadReports = useCallback(() => {
    backend.balanceReports().then(setReports, (e) => setError(errorText(e)));
  }, []);
  const loadBaselines = useCallback(() => {
    backend.balanceBaselines().then(setBaselines, (e) => setError(errorText(e)));
  }, []);
  // Die Antwort ersetzt die Liste: ein abgebrochener wartender Lauf fehlt darin und verschwindet. Eine ältere Antwort,
  // die nach einer neueren ankommt, wird verworfen (ohne Abfrage-Takt korrigierte sie sonst erst das nächste Ereignis).
  const runSeq = useRef(0);
  const loadRuns = useCallback(() => {
    const seq = ++runSeq.current;
    backend.balanceRuns().then((r) => {
      if (seq === runSeq.current) setRuns(r);
    }, (e) => setError(errorText(e)));
  }, []);
  const reload = useCallback(() => {
    loadRuns();
    loadReports();
    loadBaselines();
  }, [loadRuns, loadReports, loadBaselines]);

  useEffect(() => {
    // Erstes Laden und jeder Seitenwechsel; im Hintergrund hört die Seite nicht zu.
    if (!active) return;
    reload();
    const off = [
      backend.on('balance:runs', loadRuns),
      backend.on('balance:reports', () => {
        loadReports();
        loadBaselines();
      }),
      backend.on('ui:reconnect', reload),
    ];
    return () => off.forEach((f) => f());
  }, [active, reload, loadRuns, loadReports, loadBaselines]);

  const start = async (spec: BalanceSpec) => {
    setError('');
    try {
      const run = await backend.balanceStart(spec);
      setRuns((cur) => mergeRuns(cur, [run]));
    } catch (e) {
      setError(errorText(e));
    }
  };
  const actions = balanceActions(setError, { loadRuns, loadReports, loadBaselines });
  return { runs, reports, baselines, reloadBaselines: loadBaselines, error, start, reloadReports: loadReports, reload, ...actions };
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

/** Auswertung eines Berichts (B-632); leerer Pfad = keine. Ein Fehler (z. B. tuning-*.json) bleibt als Text. */
export function useInsights(path: string) {
  const [state, setState] = useState<{ path: string; insights?: BalanceInsights; error?: string }>({ path: '' });
  useEffect(() => {
    if (!path) return;
    let live = true;
    backend.balanceInsights(path).then(
      (insights) => live && setState({ path, insights }),
      (e) => live && setState({ path, error: errorText(e) }),
    );
    return () => {
      live = false;
    };
  }, [path]);
  const cur = state.path === path ? state : { path };
  return { insights: path ? cur.insights : undefined, error: path ? cur.error : undefined };
}
