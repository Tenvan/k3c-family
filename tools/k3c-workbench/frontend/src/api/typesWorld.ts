// Typen des Welten-Editors (B-879, Go: internal/world). Port aus dem früheren Asset-Studio (api/worldTypes.ts).

/** Eine Welt der Liste (Go: world.Info) */
export interface WorldInfo {
  id: string;
  name: string;
}

/** Festes Objekt eines geplanten Chunks; at zählt in Units ab Chunk-Anfang (Go: level.PlannedObject) */
export interface PlannedObject {
  kind: string;
  at: number;
}

/** Geplanter Chunk; slot zählt relativ zu den Hub-Chunks (−n links, +n rechts), Go: level.PlannedChunk */
export interface PlannedChunk {
  slot: number;
  kind: string;
  seed?: string;
  objects?: PlannedObject[];
  buildings?: { kind: string; at?: number }[];
  enemies?: unknown[];
}

/** Insel einer Welt mit dem Biom je Tiefe (gleiche Reihenfolge wie depths), Go: world.Island */
export interface WorldIsland {
  id: string;
  name: string;
  depths: number[];
  biomes: string[];
  plannedChunks: Record<string, PlannedChunk[]>;
  chunkCounts: Record<string, number>;
}

/** Welt mit Kopf und Inseln; error nennt, warum sie nicht lesbar ist (Go: world.Detail) */
export interface WorldDetail {
  id: string;
  name: string;
  notes: string;
  extends: string;
  islands: WorldIsland[];
  error: string;
}

/** Level einer Insel-Stufe wie im Spiel plus Warnungen und geplante Chunks (Go: world.LevelView) */
export interface WorldLevel {
  seed: string;
  biomeId: string;
  widthUnits: number;
  chunkWidthUnits: number;
  hubCenterUnits: number;
  chunks: { index: number; kind: string; startUnits: number }[];
  entities: { kind: string; x: number }[];
  planned: PlannedChunk[];
  warnings: string[];
  error: string;
}

/** Ein Feld einer Basis-Datei: base ist null, wenn die Basis das Feld nicht nennt; override fehlt, wenn die Welt erbt. */
export interface WorldValueEntry {
  file: string;
  path: string[];
  base: unknown;
  override?: unknown;
}

/** Werte der Welt (Go: world.ValuesView): Eltern-Welt (leer = Basis data/), Nachfolger und Felder. */
export interface WorldValuesView {
  world: string;
  extends: string;
  children: string[];
  entries: WorldValueEntry[];
  warnings?: string[];
  error: string;
}

/** Aufrufe des Welten-Editors (Go: app_world.go) */
export interface WorldBackend {
  worldList(): Promise<WorldInfo[]>;
  worldDetail(id: string): Promise<WorldDetail>;
  /** Level der Insel island in der Stufe stage (Index in ihren Tiefen) zum Seed; rechnet ohne Spielserver und ohne Simulation */
  worldLevel(id: string, island: string, stage: number, seed: string): Promise<WorldLevel>;
  worldValues(id: string): Promise<WorldValuesView>;
  /** Feld übersteuern; null setzt es zurück. Geschrieben wird nur eine gültige Welt, sonst steht der Grund in error. */
  worldOverrideSet(id: string, file: string, path: string[], value: unknown): Promise<WorldValuesView>;
}
