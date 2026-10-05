/** Abfrage von `/api/metrics` für die Monitoring-Seite (B-282, docs/protocol.md › Diagnose: Verläufe über /api/metrics). */
import type { MetricsResponse } from './monitorData';

export type MetricsResult =
  | { kind: 'ok'; data: MetricsResponse }
  /** 404: Diagnose aus, `K3C_STATUS_TOKEN` fehlt am Server. */
  | { kind: 'off' }
  /** 401: Token fehlt oder ist falsch. */
  | { kind: 'unauthorized' }
  /** Server nicht erreichbar oder andere Antwort. */
  | { kind: 'offline' };

export type FetchLike = (
  url: string,
  init: { headers: Record<string, string> },
) => Promise<{ ok: boolean; status: number; json(): Promise<unknown> }>;

const defaultFetch: FetchLike = (url, init) => fetch(url, init);

export const POLL_MS = 3000;
export const MAX_BACKOFF_MS = 30_000;

/** `since` = `now` der letzten Antwort; 0 = ganzer Puffer. */
export const metricsUrl = (since: number): string => (since > 0 ? `/api/metrics?since=${since}` : '/api/metrics');

/** Holt das Delta seit `since`; wirft nie. */
export async function fetchMetrics(token: string, since: number, f: FetchLike = defaultFetch): Promise<MetricsResult> {
  try {
    const res = await f(metricsUrl(since), { headers: { Authorization: `Bearer ${token}` } });
    if (res.status === 404) return { kind: 'off' };
    if (res.status === 401) return { kind: 'unauthorized' };
    if (!res.ok) return { kind: 'offline' };
    return { kind: 'ok', data: (await res.json()) as MetricsResponse };
  } catch {
    return { kind: 'offline' };
  }
}

/** Wartezeit bis zur nächsten Abfrage: 3 s, nach Fehlern verdoppelt bis 30 s. */
export const nextDelay = (failures: number): number => Math.min(MAX_BACKOFF_MS, POLL_MS * 2 ** Math.max(0, failures));
