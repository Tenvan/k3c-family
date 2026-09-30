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

/** Ereignisse von Go an die Oberfläche; die Seiten ab M4.3 ergänzen ihre Nutzdaten. */
export interface Events {
  'mcp:state': McpState;
  'service:state': ServiceStatus;
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
  /** Abonniert ein Ereignis; die Rückgabe meldet wieder ab. */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
