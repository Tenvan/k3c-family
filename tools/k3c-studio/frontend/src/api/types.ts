export type * from './typesWorld';
export type * from './typesHub';
export type * from './typesBalance';
import type { AssetsBackend } from './typesAssets';
import type { WorldBackend } from './typesWorld';
import type { HubBackend } from './typesHub';
import type { BalanceBackend, BalanceRun } from './typesBalance';

// Schnittstelle der Oberfläche von k3c-studio (Go: App in tools/k3c-studio, Bindings app_*.go und uiapi.go): Welten,
// Assets, Hub, Balancing und die Studio-Ansichten; die Typen der Bereiche liegen in typesWorld.ts, typesAssets.ts,
// typesHub.ts, typesBalance.ts.

/** Zustand des MCP-Servers k3c-studio für das Badge der Kopfzeile (Go: MCPState). */
export interface McpState {
  addr: string;
  listening: boolean;
  error: string;
}

/** Was die Oberfläche beim Laden abfragt (Go: Info). */
export interface Info {
  version: string;
  mcp: McpState;
}

/** Nutzlast von `world:changed` (Go: world.Changed, B-1216): IDs der Welten, deren Datei sich geändert hat; fehlt ids,
 *  hat sich eine Basis-Datei geändert (Werte aller Welten) und jede Welt lädt neu. */
export interface WorldChanged {
  ids?: string[];
}

export interface Events {
  'mcp:state': McpState;
  /** Eine Datei der Welten (data/worlds, Basis-Werte, Biome) hat sich geändert, durch Editor, MCP oder von Hand (B-1216). */
  'world:changed': WorldChanged | null;
  /** Ein Lauf der Balancing-Seite hat Zustand, Phase oder Fortschritt geändert (höchstens etwa 2 je Sekunde je Lauf, B-662). */
  'balance:runs': BalanceRun;
  /** Berichte oder Baselines haben sich geändert (neu, gelöscht, bewertet, Baseline gesichert); ohne Nutzlast. */
  'balance:reports': null;
  /** Nur HTTP-Client: Der Ereignis-Strom ist nach einem Abbruch wieder verbunden; Seiten laden ihre Listen einmal neu. */
  'ui:reconnect': null;
}
export type EventName = keyof Events;

export interface Backend extends AssetsBackend, WorldBackend, HubBackend, BalanceBackend {
  /** true ohne Wails-Laufzeit und ohne Backend: erfundene Daten (src/api/mock.ts). */
  mock: boolean;
  info(): Promise<Info>;
  openUrl(url: string): void;
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
