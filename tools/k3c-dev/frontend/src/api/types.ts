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

/** Zähler eines Tools seit dem Start von k3c-dev (Go: mcpsrv.ToolStats). */
export interface ToolStats {
  name: string;
  description: string;
  calls: number;
  errors: number;
  avgMs: number;
  lastCall: string;
}

/** Zähler des MCP-Servers (Go: mcpsrv.Snapshot); startedAt ISO-Zeit. */
export interface McpStats {
  startedAt: string;
  clients: number;
  peakClients: number;
  inFlight: number;
  peakInFlight: number;
  totalCalls: number;
  errors: number;
  tools: ToolStats[];
}

/** Band 1 und 2 der MCP-Seite (Go: McpOverview). */
export interface McpOverview {
  mcp: McpState;
  stats: McpStats;
}

/** Ein Aufruf im Aufruf-Log (Go: mcpsrv.Call); ts ist das Ende, leer solange er läuft. */
export interface McpCall {
  id: number;
  ref: string;
  startedAt: string;
  ts: string;
  startSeq: number;
  endSeq: number;
  running: boolean;
  tool: string;
  args: string;
  durationMs: number;
  ok: boolean;
  error?: string;
  summary: string;
}

/** Eintrag einer Rangliste: Argument mit Laufzeiten oder Fehlermeldung mit Tool (Go: usage.Count). */
export interface UsageCount {
  tool?: string;
  value: string;
  count: number;
  avgMs?: number;
  p95Ms?: number;
  maxMs?: number;
}

/** Einzelner Aufruf in „Letzte Ausreißer“ bzw. „Langsamste Aufrufe“ (Go: usage.SlowCall). */
export interface SlowCall {
  at: string;
  tool: string;
  args: string;
  durationMs: number;
  ok: boolean;
  baselineMs: number;
}

/** Auswertung eines Tools in einem Bereich (Go: usage.ToolUsage). */
export interface ToolUsage {
  name: string;
  calls: number;
  errors: number;
  avgMs: number;
  p50Ms: number;
  p95Ms: number;
  maxMs: number;
  sumMs: number;
  outliers: number;
  args: UsageCount[];
  topErrors: UsageCount[];
}

/** Auswertung eines Bereichs, Sitzung oder Gesamtzeit (Go: usage.Scope). */
export interface UsageScope {
  since: string;
  calls: number;
  errors: number;
  avgMs: number;
  p50Ms: number;
  p95Ms: number;
  maxMs: number;
  sumMs: number;
  outliers: number;
  tools: ToolUsage[];
  topErrors: UsageCount[];
  recentOutliers: SlowCall[];
  slowest: SlowCall[];
}

/** Eine Minute der Zeitreihe (Go: usage.Bucket); ts Minutenanfang in ms, Werte je Tool. */
export interface UsageBucket {
  ts: number;
  calls: Record<string, number>;
  errors: number;
  ms: Record<string, number>;
  maxMs: Record<string, number>;
  outliers: Record<string, number>;
}

/** Regeln der Statistik für die Erklärzeile, nur in Go gepflegt (Go: UsageRules). */
export interface UsageRules {
  outlierFactor: number;
  outlierFloorMs: number;
  baselineCalls: number;
  percentileErrorPct: number;
}

/** Nutzungsstatistik (Go: McpUsageView). */
export interface McpUsage {
  session: UsageScope;
  allTime: UsageScope;
  minutes: UsageBucket[];
  rules: UsageRules;
}

/** Ein Task aus `task --list-all` (Go: taskcat.Task); file relativ zur Repo-Wurzel. */
export interface TaskInfo {
  name: string;
  namespace: string;
  leaf: string;
  desc: string;
  summary: string;
  aliases: string[];
  file: string;
  line: number;
}

/** Gruppe des Task-Baums, Namensraum vor dem ersten Doppelpunkt (Go: taskcat.Namespace). */
export interface TaskNamespace {
  name: string;
  tasks: TaskInfo[];
}

/** Katalog der Tasks-Seite (Go: TaskCatalog); error ist ein Feld, der alte Baum bleibt dann sichtbar. */
export interface TaskCatalog {
  namespaces: TaskNamespace[];
  count: number;
  loadedAt: string;
  error: string;
}

export type TaskRunState = 'running' | 'succeeded' | 'failed' | 'cancelled';

/** Aktueller oder letzter Lauf eines Tasks (Go: taskrun.Run); endedAt ist beim Laufen der Nullwert. */
export interface TaskRun {
  name: string;
  state: TaskRunState;
  pid: number;
  args: string[];
  startedAt: string;
  endedAt: string;
  exitCode: number;
  durationMs: number;
  reason: string;
}

/** Eine Session eines Sprints (Go: planning.Session); Status `entwurf` bei Stichpunkten ohne Tabelle. */
export interface PlanSession {
  nr: string;
  typ: string;
  agent: string;
  status: string;
  titel: string;
}

/** Aktiver oder geplanter Sprint (Go: planning.Sprint). */
export interface PlanSprint {
  id: string;
  title: string;
  domain: string;
  status: string;
  reife: string;
  spec: string;
  tickets: string[];
  sessions: PlanSession[];
}

/** Ticket aus docs/backlog (Go: planning.Ticket). */
export interface PlanTicket {
  nr: string;
  title: string;
  domain: string;
  typ: string;
  prio: string;
  status: string;
  sprint: string;
  spec: string;
}

/** Planung beim Laden (Go: planning.Data); done zählt die erledigten Sprints. */
export interface PlanningData {
  sprints: PlanSprint[];
  tickets: PlanTicket[];
  done: number;
}

/** Lesbare Planungs-Dokumente: Plan (docs/plan-weiterentwicklung.md) und Fragenkatalog (docs/fragenkatalog.md). */
export type PlanDoc = 'plan' | 'fragen';

/** Datei im Index (Status A, M, D, R, C, T) oder Arbeitsbaum (M, D, T, U Konflikt, ? untracked); Go: gitcommit.File. */
export interface GitFile {
  path: string;
  status: string;
}

/** Git-Seite beim Laden (Go: GitView); recent sind die letzten Commits als Zeile, types/domains füllen das Formular. */
export interface GitView {
  branch: string;
  staged: GitFile[];
  unstaged: GitFile[];
  recent: string[];
  types: string[];
  domains: string[];
}

/** Eingabe des Commit-Formulars (Go: gitcommit.Message). */
export interface CommitMessage {
  type: string;
  scope: string;
  subject: string;
  body: string;
}

/** Ergebnis eines Commits: neuer Hash und Stand danach (Go: CommitResult). */
export interface CommitResult {
  hash: string;
  view: GitView;
}

/** Ereignisse von Go an die Oberfläche. */
export interface Events {
  'mcp:state': McpState;
  'service:state': ServiceStatus;
  'source:state': Source;
  /** Eine Liste, damit Go später bündeln kann; vorerst je eine Zeile. */
  'console:line': ConsoleLine[];
  'mcp:start': McpCall;
  'mcp:call': McpCall;
  'task:state': TaskRun;
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
  mcpOverview(): Promise<McpOverview>;
  /** Startet den HTTP-Teil neu; ein Fehler steht im zurückgegebenen Zustand. */
  mcpRestart(): Promise<McpState>;
  mcpInstructions(): Promise<string>;
  mcpCalls(): Promise<McpCall[]>;
  mcpUsage(): Promise<McpUsage>;
  /** Katalog; wird beim ersten Aufruf geladen und dann gehalten, `tasksReload` liest neu. */
  tasks(): Promise<TaskCatalog>;
  tasksReload(): Promise<TaskCatalog>;
  /** Startet `task <name> -- args`; die Ausgabe liegt als Konsolen-Quelle `task:<name>` (consoleTail, console:line). */
  taskStart(name: string, args: string[]): Promise<TaskRun>;
  taskStop(name: string): Promise<TaskRun>;
  taskRuns(): Promise<TaskRun[]>;
  git(): Promise<GitView>;
  /** Staging verändert nur den Index; ein Fehler kommt als Ablehnung mit dem Text aus Go. */
  gitStage(paths: string[]): Promise<GitView>;
  gitUnstage(paths: string[]): Promise<GitView>;
  /** Eine verletzte Regel kommt als `feld: Grund` (feld: type, scope, subject), `nichts gestaged` ohne Feld. */
  gitCommit(m: CommitMessage): Promise<CommitResult>;
  /** Liest Sprints und Tickets frisch von der Platte. */
  planning(): Promise<PlanningData>;
  /** Markdown eines Planungs-Dokuments. */
  planningDoc(name: PlanDoc): Promise<string>;
  /** Abonniert ein Ereignis; die Rückgabe meldet wieder ab. */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
