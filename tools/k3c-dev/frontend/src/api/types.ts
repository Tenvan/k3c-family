// Backend-Vertrag der Oberfläche (B-064 › Backend-Vertrag). Go-Seite: tools/k3c-dev/app.go.
// Wails-Laufzeit → wails.ts, ohne sie → mock.ts; welche gilt, entscheidet index.ts.

/** Zustand des MCP-Servers (Go: MCPState). */
export interface McpState {
  addr: string;
  listening: boolean;
  error: string;
}

/** Antwort von Info() beim Laden (Go: Info). */
export interface Info {
  version: string;
  mcp: McpState;
}

/** Zustände eines Dienstes (Go: services.State); unbekannte zeigt die Oberfläche neutral. */
export type ServiceState = 'gestoppt' | 'startet' | 'läuft' | 'übernommen' | 'stoppt' | 'fehlgeschlagen';

/** Ein Dienst (Go: services.Status). startedAt ist ISO-Zeit, cpu Prozent, memory Bytes. */
export interface ServiceStatus {
  name: string;
  port: number;
  health: string;
  log: string;
  state: ServiceState | string;
  pid: number;
  startedAt: string;
  restarts: number;
  lastError: string;
  cpu: number;
  memory: number;
  /** Zählt je Dienst jede Änderung; ein Ereignis mit kleinerer Seq als der gesehenen ist veraltet. */
  seq: number;
}

/** Dienste-Seite beim Laden (Go: ServicesView); error: services.json nicht geladen. */
export interface ServicesView {
  services: ServiceStatus[];
  error: string;
}

/** Log-Einträge eines Dienstes der letzten 60 min je Level (Go: services.LevelCounts). */
export interface LevelCounts {
  counts: Record<string, number>;
  total: number;
  budgetHit: boolean;
}

/** Art einer Quelle der Logs-Seite (Go: Source.Kind). */
export type SourceKind = 'service' | 'run' | 'log' | 'console';

/**
 * Eintrag der Quellenleiste (Go: Source). state: Dienst → ServiceState; Lauf → running, ok, failed, timeout;
 * Log → entries, empty. Der Name ist die Konsolen-Quelle (Lauf: check:<ziel>).
 */
export interface Source {
  name: string;
  kind: SourceKind | string;
  state: string;
  detail: string;
}

/** Zeile einer Konsolen-Quelle (Go: console.Line). seq zählt je Quelle, auch über einen neuen Lauf hinweg. */
export interface ConsoleLine {
  source: string;
  stream: string;
  text: string;
  seq: number;
}

/** Eintrag einer Log-Datei (Go: logs.Entry); time ist ISO-Zeit. */
export interface LogEntry {
  time: string;
  level: string;
  ns: string;
  msg: string;
  data?: Record<string, unknown>;
}

/** Filter des Reiters Log (Go: LogQuery); minLevel leer = alle, limit 100, 200 oder 500. */
export interface LogQuery {
  minLevel: string;
  ns: string;
  pattern: string;
  limit: number;
}

/** Ergebnis des Reiters Log (Go: LogView), neueste zuerst. */
export interface LogView {
  entries: LogEntry[];
  bytesRead: number;
  budgetHit: boolean;
  skipped: number;
  missing: boolean;
}

/** Gleichartige Meldungen (Go: logs.Group). */
export interface LogGroup {
  ns: string;
  fingerprint: string;
  level: string;
  count: number;
  first: string;
  last: string;
  example: string;
}

/** Ergebnis des Reiters Fehler (verdichtet) (Go: ErrorsView), häufigste zuerst. */
export interface ErrorsView {
  groups: LogGroup[];
  entries: number;
  bytesRead: number;
  budgetHit: boolean;
  missing: boolean;
}

/** Ereignisse von Go an die Oberfläche; M5 ergänzt seine Nutzdaten. */
export interface Events {
  'mcp:state': McpState;
  'service:state': ServiceStatus;
  'source:state': Source;
  /** Eine Liste, damit Go später bündeln kann; vorerst je eine Zeile. */
  'console:line': ConsoleLine[];
}

export type EventName = keyof Events;

export interface Backend {
  /** true ohne Wails-Laufzeit: erfundene Daten, Badge `Mock`. */
  readonly mock: boolean;
  info(): Promise<Info>;
  services(): Promise<ServicesView>;
  /** Befehle warten auf das Ergebnis; ein Fehler kommt als Ablehnung mit dem Text aus Go. */
  serviceStart(name: string): Promise<ServiceStatus>;
  serviceStop(name: string, force: boolean): Promise<ServiceStatus>;
  serviceRestart(name: string): Promise<ServiceStatus>;
  servicesStartAll(): Promise<void>;
  servicesStopAll(): Promise<void>;
  serviceLogLevels(name: string): Promise<LevelCounts>;
  sources(): Promise<Source[]>;
  /** Ganzer Puffer einer Quelle mit ANSI-Farben; eine unbekannte Quelle lehnt Go ab. */
  consoleTail(source: string): Promise<ConsoleLine[]>;
  /** Ein ungültiger regulärer Ausdruck in pattern kommt als Ablehnung zurück. */
  logsQuery(source: string, q: LogQuery): Promise<LogView>;
  /** level: WARN oder ERROR; Go liest die letzten 24 h. */
  logsErrors(source: string, level: string): Promise<ErrorsView>;
  /** Abonniert ein Ereignis; die Rückgabe meldet wieder ab. */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
