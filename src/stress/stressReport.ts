import type { RunResult } from './stressRun';

export interface ReportMeta {
  runId: string;
  scenario: string;
  players: number;
  seed: number | string;
  startedAt: string;
  /** `canvas` (stress.html) oder `real` (echter Client mit Go-Server) */
  mode: 'canvas' | 'real';
}

/** Gerätekennung für Lauf-Kennung und Bericht; dieselbe wie im Client-Log (`k3c-client`) */
export function deviceTag(): string {
  try {
    return (localStorage.getItem('k3c-client') ?? '').slice(0, 6) || 'anon';
  } catch {
    return 'anon';
  }
}

/** Bericht über `/api/report` (`reports/*.json`), nur mit `?perf=1`; Fehler werden verworfen */
export function sendStressReport(meta: ReportMeta, result: RunResult, search: string = location.search): void {
  if (new URLSearchParams(search).get('perf') !== '1') return;
  const body = { kind: 'stress', version: 1, ...meta, device: deviceTag(), userAgent: navigator.userAgent, viewport: `${window.innerWidth}x${window.innerHeight}`, ...result };
  void fetch('/api/report', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body), keepalive: true }).catch(() => undefined);
}
