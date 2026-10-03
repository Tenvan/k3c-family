import type { ServiceStatus, Source } from '../api';

// Ordnet die Quellen der Seite Dienste & Logs: Log-Dateien eines Dienstes hängen am Dienst, der Rest steht in
// Gruppen mit einer Zeile Erklärung darunter.

/** Bekannte Log-Dateien ohne eigenen Dienst: Zweck und Einordnung. */
const LOG_INFO: Record<string, { text: string; tags: string[] }> = {
  'k3c-dev': { text: 'Log von k3c-dev selbst: Dienste, MCP-Aufrufe, Watch.', tags: ['Tool'] },
  'k3c-client': { text: 'Browser-Client des Spiels, über /api/log vom Go-Server mitgeschrieben.', tags: ['Spiel', 'Client'] },
  'k3c-server': { text: 'Strukturiertes Log des Go-Spielservers.', tags: ['Spiel', 'Backend'] },
};

const GROUPS = [
  { kind: 'log', title: 'Log-Dateien' },
  { kind: 'run', title: 'Läufe (MCP check)' },
  { kind: 'console', title: 'Weitere Konsolen' },
] as const;

export interface Group {
  title: string;
  items: Source[];
}

/** Mitgeschriebene Ausgabe eines Dienstes (Go: outlog.go, logs/<name klein>.jsonl). */
const outLog = (s: ServiceStatus) => s.name.toLowerCase();

/** Log eines Dienstes: sein strukturiertes Log, sonst seine mitgeschriebene Ausgabe, sofern es sie gibt. */
export function serviceLog(s: ServiceStatus, sources: Source[]): string {
  if (s.log) return s.log;
  return sources.some((x) => x.kind === 'log' && x.name === outLog(s)) ? outLog(s) : '';
}

/** Log-Name für die Reiter `Log` und `Fehler` der gewählten Quelle ('' = nur Konsole). */
export function logFor(src: Source, services: ServiceStatus[], sources: Source[]): string {
  if (src.kind === 'log') return src.name;
  const s = services.find((x) => x.name === src.name);
  return s ? serviceLog(s, sources) : '';
}

/** Alle Quellen außer Diensten und deren Logs, nach Art gruppiert; leere Gruppen fallen weg. */
export function otherGroups(sources: Source[], services: ServiceStatus[]): Group[] {
  const owned = new Set(services.flatMap((s) => [s.name, s.log, outLog(s)]));
  const rest = sources.filter((x) => x.kind !== 'service' && !owned.has(x.name));
  return GROUPS.map((g) => ({ title: g.title, items: rest.filter((x) => x.kind === g.kind) })).filter((g) => g.items.length);
}

/** Eine Zeile: wofür die Quelle da ist. */
export function describe(src: Source): string {
  if (src.kind === 'log') return LOG_INFO[src.name]?.text ?? `JSON-Log logs/${src.name}.jsonl`;
  if (src.kind === 'run') return `Letzter Lauf von ${src.name.replace(/^check:/, '')} über das MCP-Tool check`;
  return 'Konsolen-Ausgabe ohne Dienst oder Log-Datei';
}

/** Einordnung einer Quelle ohne Dienst: Läufe sind Werkzeug, Logs laut LOG_INFO, sonst keine. */
export function tagsOf(src: Source): string[] {
  if (src.kind === 'run') return ['Tool'];
  return (src.kind === 'log' && LOG_INFO[src.name]?.tags) || [];
}
