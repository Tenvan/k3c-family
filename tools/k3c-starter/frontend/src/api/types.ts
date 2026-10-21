/** Zustand des Servers wie `Status` in server.go (JSON). */
export interface ServerState {
  running: boolean;
  starting: boolean;
  port: string;
  urls: string[];
  message: string;
  log: string[];
}

/** Alles, was die Oberfläche vom Starter braucht: Wails-Bindung im Fenster, sonst der Mock. */
export interface Backend {
  /** true ohne Wails-Laufzeit (erfundene Daten). */
  mock: boolean;
  state(): Promise<ServerState>;
  start(): Promise<void>;
  stop(): Promise<void>;
}
