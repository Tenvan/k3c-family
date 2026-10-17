import type { LevelLayout } from '@game/model/types';

// Typen des Asset-Studios (B-298, B-459): Slot-Register, Katalog und Recherche, wie Go sie liefert
// (tools/assetkit, tools/k3c-studio/app_assets.go). Kopie aus k3c-dev, ergänzt um Stand und Backend.
/** Ausschnitt in Bild-Pixeln (Go: assets.Rect) */
export interface AssetRect {
  x: number;
  y: number;
  w: number;
  h: number;
}

/** Zuordnung eines Asset-Slots (Go: assets.Choice); datei relativ zu public/ */
export interface AssetChoice {
  datei: string;
  sheet?: string;
  ausschnitt?: AssetRect;
  raster?: { w: number; h: number };
  sprite?: string;
}

export type AssetKind = 'grafik' | 'sound';
export type AssetStatus = 'zugeordnet' | 'luecke' | 'fehlt';

/** Asset-Slot des Slot-Registers mit Status (Go: AssetSlotView); wahl in Präferenz-Reihenfolge */
export interface AssetSlot {
  id: string;
  art: AssetKind;
  kategorie: string;
  beschreibung: string;
  bus?: string;
  wahl: AssetChoice[];
  status: AssetStatus;
}

/** Slot-Register; preview = Basis der Bild- und Sound-Adressen ("/" im Studio), stand = Hash von data/assets.json beim
 *  Lesen, AssetsSet braucht ihn (Schreibregel, B-459/AC-07) */
export interface AssetsView {
  slots: AssetSlot[];
  error: string;
  preview: string;
  stand: string;
}

/** Antwort von AssetsSet: gespeicherter Slot und neuer Stand */
export interface AssetSaved {
  slot: AssetSlot;
  stand: string;
}

/** Datei des Asset-Katalogs (Go: assets.Asset); kennzeichen NC, SA oder ND nach Q70 */
export interface CatalogAsset {
  datei: string;
  art: AssetKind;
  pack: string;
  gruppe?: string;
  w?: number;
  h?: number;
  titel?: string;
  urheber?: string;
  lizenz: string;
  quelle?: string;
  kennzeichen?: '' | 'NC' | 'SA' | 'ND';
}

export type LicenseStatus = 'erkannt' | 'pruefen' | 'ausgeschlossen' | 'bestaetigt';

/** Recherche-Treffer (Go: assets.Hit): Vorschlag mit Link, Quelle und Lizenz-Status nach Q70 */
export interface AssetHit {
  id: string;
  titel: string;
  link: string;
  quelle: string;
  urheber?: string;
  lizenz?: string;
  vorschau?: string;
  download?: string;
  lizenzStatus: LicenseStatus;
  kennzeichen?: '' | 'NC' | 'SA' | 'ND';
  erstellt: string;
  /** nach dem Import: Datei relativ zu public/ */
  datei?: string;
}

/** Ergebnis einer Recherche: alle gespeicherten Treffer des Slots, neu angelegte, Hinweise ausgefallener Quellen */
export interface ResearchResult {
  treffer: AssetHit[];
  neu: number;
  hinweise: string[];
}

/** Level des Welt-Betrachters (Go: WorldLevelView): das Level wie im Spiel plus Warnungen der Prüfung; error ist der
 *  Hinweis für die Seite (dann sind Abschnitte und Objekte leer). */
/** Festes Objekt eines geplanten Chunks (Go: level.PlannedObject); at zählt in Units ab Chunk-Anfang. */
export interface PlannedObject {
  kind: string;
  at: number;
}

/** Geplanter Chunk (Go: level.PlannedChunk): Platz relativ zum Hub (−n links, +n rechts, nie 0), Art, Chunk-Seed, Objekte. */
export interface PlannedChunk {
  slot: number;
  kind: string;
  seed?: string;
  objects?: PlannedObject[];
}

/** Quelle des Levels als Wert (Go: WorldSource): ein Biom oder eine Insel mit Stufe (Index in ihren Tiefen). planned ist
 *  der Entwurf der geplanten Chunks (auch ungespeichert); fehlt er, gilt bei einer Insel die gespeicherte Planung. */
export interface WorldSource {
  biome?: string;
  island?: string;
  stage?: number;
  planned?: PlannedChunk[] | null;
}

/** Insel mit Tiefen, Biom je Tiefe und gespeicherter Planung je Tiefe (Go: PlanIsland) */
export interface PlanIsland {
  id: string;
  name: string;
  depths: number[];
  biomes: string[];
  planned: Record<string, PlannedChunk[]>;
}

/** data/islands.json für den Editor (Go: PlanView); stand ist der Hash der Datei beim Lesen */
export interface PlanView {
  islands: PlanIsland[];
  stand: string;
  error: string;
}

/** Ergebnis der Prüfung eines Entwurfs (Go: PlanCheckView); slot nennt den Platz des Fehlers (0 = ohne Platz) */
export interface PlanCheckView {
  ok: boolean;
  slot: number;
  error: string;
}

/** Ergebnis des Speicherns (Go: PlanSaved); stale = Datei seit dem Laden von außen geändert, der Entwurf bleibt */
export interface PlanSaved {
  stand: string;
  hint: string;
  error: string;
  stale: boolean;
}

/** Optionen der Live-Vorschau (Go: WorldPreviewArgs); Nullwerte: höchste Hub-Stufe, Tag, alle Gebäude. */
export interface WorldPreviewArgs {
  seed: string;
  source: WorldSource;
  hubLevel: number;
  /** Sim-Phase day, dusk, night */
  phase: string;
  /** Fortschritt in der Phase, [0, 1) */
  progress: number;
  /** false schaltet einen Platz aus (Schlüssel kind@x) */
  buildings: Record<string, boolean>;
}

/** Vorschau-Welt (Go: WorldPreviewView): Level wie die Nachricht level, Zustand wie snap, oder ein Hinweis in error. */
export interface WorldPreviewView {
  level: LevelLayout;
  /** null bei einem Hinweis in error (Go: nil) */
  state: (Record<string, unknown> & { sites: { kind: string; x: number }[] }) | null;
  error: string;
}

/** Ergebnis von „Im Spiel öffnen“ (Go: OpenGameResult): geöffnete Adresse und geladener Spielstand, oder ein Hinweis in error. */
export interface OpenGameResult {
  url: string;
  save: string;
  error: string;
}

export type WorldLevelView = LevelLayout & { warnings: string[]; error: string };

/** Ereignisse des Backends: assets:changed = data/assets.json geändert (MCP, Editor, eigenes Speichern) */
export interface Events {
  'assets:changed': null;
}
export type EventName = keyof Events;

/** Bindings des Asset-Studios (Wails) bzw. ihr Mock im Browser */
export interface Backend {
  mock: boolean;
  assetsList(): Promise<AssetsView>;
  assetsSet(id: string, wahl: AssetChoice[], stand: string): Promise<AssetSaved>;
  assetsCatalog(query: string, art: string, limit: number): Promise<CatalogAsset[]>;
  assetsHits(id: string): Promise<AssetHit[]>;
  assetsResearch(id: string, query: string): Promise<ResearchResult>;
  assetsHitDrop(id: string, hit: string): Promise<void>;
  assetsImport(id: string, hit: string): Promise<string>;
  assetsDownload(id: string, hit: string): Promise<string>;
  assetsLicenseOk(id: string, hit: string): Promise<void>;
  /** Welt-Betrachter: Level zu Seed und Biom, ohne Spielserver (B-460) */
  worldLevel(seed: string, source: WorldSource): Promise<WorldLevelView>;
  /** Live-Vorschau: Welt zu den Optionen, aus der Vorschau-Funktion des Servers (B-460) */
  worldPreview(args: WorldPreviewArgs): Promise<WorldPreviewView>;
  /** Geplante Chunks: data/islands.json lesen, einen Entwurf prüfen (ohne zu schreiben), die Planung einer Tiefe speichern (B-461) */
  planLoad(): Promise<PlanView>;
  planCheck(island: string, depth: number, chunks: PlannedChunk[]): Promise<PlanCheckView>;
  planSave(island: string, depth: number, chunks: PlannedChunk[], stand: string): Promise<PlanSaved>;
  /** Im Spiel öffnen: `new` (neues Spiel mit dem Seed) oder `full` (voll ausgebaut), über die Landingpage des Servers unter base, nur Loopback */
  worldOpenGame(base: string, seed: string, biome: string, mode: 'new' | 'full'): Promise<OpenGameResult>;
  openUrl(url: string): void;
  /** Abo eines Ereignisses; liefert die Abmeldung */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
