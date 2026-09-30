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

/** Ereignisse von Go an die Oberfläche; die Seiten ab M4.2 ergänzen ihre Nutzdaten. */
export interface Events {
  'mcp:state': McpState;
}

export type EventName = keyof Events;

export interface Backend {
  /** true ohne Wails-Laufzeit: erfundene Daten, Badge `Mock`. */
  readonly mock: boolean;
  info(): Promise<Info>;
  /** Abonniert ein Ereignis; die Rückgabe meldet wieder ab. */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
