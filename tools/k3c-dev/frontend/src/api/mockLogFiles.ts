import type { ErrorsView, LogEntry, LogGroup, LogQuery, LogView } from './types';

// Erfundene Log-Datei k3c-dev für den Mock (B-064 › Log, Fehler): wächst alle 2 s, einige Einträge tragen Daten.
// Filter und Verdichtung wie in Go (logs.Scan, logs.Digest), nur grob: Zahlen und Texte in Anführungszeichen maskiert.

const RANKS: Record<string, number> = { DEBUG: 0, INFO: 1, WARN: 2, ERROR: 3 };

const SAMPLES: Omit<LogEntry, 'time'>[] = [
  { level: 'INFO', ns: 'mcp', msg: 'aufruf beendet', data: { tool: 'logs_query', ms: 12 } },
  { level: 'DEBUG', ns: 'usage', msg: 'statistik gespeichert' },
  { level: 'INFO', ns: 'svc', msg: 'dienst Vite: läuft', data: { pid: 41232 } },
  { level: 'WARN', ns: 'check', msg: 'lauf beendet', data: { target: 'task:test', exit: 1, ms: 5120 } },
  { level: 'INFO', ns: 'check', msg: 'lauf beendet', data: { target: 'task:lint', exit: 0, ms: 1442 } },
  { level: 'ERROR', ns: 'svc', msg: 'dienst Spielserver fehlgeschlagen: Port 8080 bereits belegt (PID 8812)' },
  { level: 'WARN', ns: 'svc', msg: 'dienst Vite antwortet nicht (3 Prüfungen in Folge)' },
  { level: 'ERROR', ns: 'mcp', msg: 'tool "logs_query": source "../geheim" unbekannt' },
];

const fingerprint = (msg: string) => msg.replace(/"[^"]*"/g, '<s>').replace(/\d+(?:[.,]\d+)?/g, '<n>');

export function mockLogFiles() {
  const entries: LogEntry[] = [];
  const add = (at: number, i: number) => entries.push({ ...SAMPLES[i % SAMPLES.length], time: new Date(at).toISOString() });
  const start = Date.now() - 6 * 3_600_000;
  for (let i = 0; i < 120; i++) add(start + i * 180_000, (i * 5) % SAMPLES.length);
  let tick = 0;
  setInterval(() => add(Date.now(), tick++), 2000);

  const file = (source: string) => {
    if (source === 'k3c-dev') return entries;
    if (source === 'server') return [];
    throw new Error(`unbekannte Log-Quelle "${source}"; gültig: k3c-dev, server`);
  };

  const logsQuery = async (source: string, q: LogQuery): Promise<LogView> => {
    const all = file(source);
    let re: RegExp | null = null;
    try {
      re = q.pattern ? new RegExp(q.pattern) : null;
    } catch (e) {
      throw new Error(`ungültige Suche, kein regulärer Ausdruck: ${(e as Error).message}`);
    }
    const min = RANKS[q.minLevel] ?? 0;
    const hits = [...all].reverse().filter((e) =>
      (RANKS[e.level] ?? 0) >= min && (!q.ns || e.ns === q.ns) && (!re || re.test(e.msg)));
    return { entries: hits.slice(0, q.limit), bytesRead: all.length * 140, budgetHit: false, skipped: 0, missing: false };
  };

  const logsErrors = async (source: string, level: string): Promise<ErrorsView> => {
    const since = Date.now() - 24 * 3_600_000;
    const hits = [...file(source)].reverse()
      .filter((e) => (RANKS[e.level] ?? 0) >= (RANKS[level] ?? 2) && Date.parse(e.time) >= since);
    const groups = new Map<string, LogGroup>();
    for (const e of hits) {
      const fp = fingerprint(e.msg);
      const g = groups.get(e.ns + fp) ?? { ns: e.ns, fingerprint: fp, level: e.level, count: 0, first: e.time, last: e.time, example: e.msg };
      g.count++;
      g.first = e.time; // hits laufen von neu nach alt
      groups.set(e.ns + fp, g);
    }
    const sorted = [...groups.values()].sort((a, b) => b.count - a.count || Date.parse(b.last) - Date.parse(a.last));
    return { groups: sorted, entries: hits.length, bytesRead: hits.length * 140, budgetHit: false, missing: false };
  };

  return { logsQuery, logsErrors };
}
