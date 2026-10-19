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
 *  der Entwurf der geplanten Chunks (auch ungespeichert); fehlt er, gilt bei einer Insel die gespeicherte Planung.
 *  chunkCount ist der Entwurf der Chunk-Anzahl (0 = Biom-Länge); fehlt er, gilt bei einer Insel die gespeicherte. */
export interface WorldSource {
  biome?: string;
  island?: string;
  stage?: number;
  planned?: PlannedChunk[] | null;
  chunkCount?: number | null;
}

/** Inselgröße (Go: sim.IslandSize, data/island-sizes.json): deutscher Name und Chunk-Anzahl */
export interface IslandSize {
  name: string;
  chunkCount: number;
}

/** Insel mit Tiefen, Biom je Tiefe, gespeicherter Planung und Chunk-Anzahl je Tiefe (Go: PlanIsland); ohne Eintrag in
 *  chunkCounts gilt die Biom-Länge. */
export interface PlanIsland {
  id: string;
  name: string;
  depths: number[];
  biomes: string[];
  planned: Record<string, PlannedChunk[]>;
  chunkCounts: Record<string, number>;
}

/** data/islands.json für den Editor (Go: PlanView); stand ist der Hash der Datei beim Lesen, sizes die Inselgrößen */
export interface PlanView {
  islands: PlanIsland[];
  sizes: IslandSize[];
  stand: string;
  error: string;
}

/** Ergebnis der Prüfung eines Entwurfs (Go: PlanCheckView); slot nennt den Platz des Fehlers (0 = ohne Platz) */
export interface PlanCheckView {
  ok: boolean;
  slot: number;
  error: string;
}

/** Seed-Inhalt eines geplanten Chunks (Go: PlanObjectsView) zum Umwandeln in feste Objekte, oder der Grund in error */
export interface PlanObjectsView {
  objects: PlannedObject[];
  error: string;
}

/** Wählbare Welten des Welten Editors (Go: WorldsView): jede Welt eine JSON-Datei mit allen Inseln und Anpassungen */
export interface WorldsView {
  worlds: { id: string; name: string }[];
  active: string;
  error: string;
}

/** Ergebnis des Speicherns (Go: PlanSaved); stale = Datei seit dem Laden von außen geändert, der Entwurf bleibt */
export interface PlanSaved {
  stand: string;
  hint: string;
  error: string;
  stale: boolean;
}

/** Zustand eines Platzes in der Vorschau; fehlende Felder lassen den Platz, wie die Hub-Stufe ihn ausbaut. */
export interface SiteLook {
  level?: number;
  hpPercent?: number;
  unbuilt?: boolean;
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
  /** Zustand einzelner Plätze (Schlüssel kind@x): Baustufe, Trefferpunkte in Prozent, nicht gebaut (Go: sim.SiteLook) */
  sites?: Record<string, SiteLook>;
  /** Planer-Stand als Kandidaten-hub.json (Gebäude-Planer), auch ungespeichert; fehlt = eingebettete data/hub.json */
  hubCandidate?: string;
}

/** Zelle der Matrix des Gebäude-Planers (Go: HubCell): verfügbar ab der Stufe der Zeile, dann mit Platz und Tiefe. */
export interface HubCell {
  available: boolean;
  /** Platz zur Hub-Mitte in Units; bei Mauerlinie und Tor je Seite (±) */
  offsetUnits: number;
  fromDepth: number;
  needsDeeper: boolean;
  /** nur in dieser Tiefe (Händler); -1 = jede Tiefe */
  onlyDepth: number;
}

/** Herkunft einer Zeile: Liste bzw. Block in hub.json */
export type HubSource = 'sites' | 'islandSites' | 'wallLines' | 'merchant';

/** Bauplatz der Matrix (Go: HubRow): Hub-Platz, Insel-Platz, Mauerlinie mit Turm, Tor oder Händler. */
export interface HubRow {
  /** kind@offset wie room.PreviewSiteKey, Linien line@k, Tore gate@k */
  key: string;
  /** Art aus hub.json; wall, gate, merchantBuy, merchantSell */
  kind: string;
  source: HubSource;
  /** Eintrag in seiner Liste (sites, islandSites, wallUnits); Händler 0 */
  index: number;
  hubLevel: number;
  /** Mauerlinie bzw. Tor: Linie k; sonst 0 */
  line: number;
  bothSides: boolean;
  towerUnits: number;
  /** Streuung nach außen je Seed; 0 = fest */
  jitterUnits: number;
  /** je Hub-Stufe 1 … levels */
  cells: HubCell[];
}

/** Matrix Hub-Stufe × Bauplatz (Go: HubMatrix); levels = Anzahl hub.json › levels, keine feste Zahl. */
export interface HubMatrix {
  levels: number;
  rows: HubRow[];
}

/** Befund der Prüfung im Go-Backend (sim.ValidateHub) */
export interface HubFinding {
  rule: string;
  site: string;
  detail: string;
}

/** Stand des Gebäude-Planers (Go: HubPlanView): raw ist der Kandidat der Live-Vorschau; error nennt eine nicht lesbare
 *  oder ungültige Datei mit Pfad, dann ist die Matrix leer. */
export interface HubPlanView {
  path: string;
  raw: string;
  matrix: HubMatrix;
  findings: HubFinding[];
  notes: string[];
  error: string;
  /** Stand beider Dateien beim Laden (Schreibregel); leer bei einem geprüften Kandidaten */
  stand: HubStand;
  /** Arten, die als Platz hinzukommen dürfen (entscheidet die Prüfung im Go-Backend) */
  kinds: string[];
}

/** Stand von data/hub.json und Regeltabelle beim Laden (Go: HubStand, SHA-256) */
export interface HubStand {
  hub: string;
  rules: string;
}

/** Ergebnis des Speicherns (Go: HubSaveResult): stale = Datei extern geändert, nichts geschrieben; rulesError = hub.json
 *  gespeichert, Regeltabelle veraltet; notice = Hinweis auf die REG-Änderung. */
export interface HubSaveResult {
  saved: boolean;
  /** Stand bedeutet dasselbe wie die Datei: nichts geschrieben, keine REG-Änderung */
  unchanged: boolean;
  stale: string;
  findings: HubFinding[];
  error: string;
  rulesError: string;
  notice: string;
  stand: HubStand;
}

/** Abweichende Zeile des erzeugten Blocks (1-basiert); fehlt sie auf einer Seite, ist der Text dort leer. */
export interface HubRulesLine {
  line: number;
  ist: string;
  soll: string;
}

/** Abweichung der Regeltabelle (Go: HubRulesView): Block im Regeltext (ist) gegen den aus dem Planer-Stand erzeugten
 *  (soll), mit dem Task, der den Block neu schreibt. */
export interface HubRulesView {
  path: string;
  task: string;
  same: boolean;
  ist: string;
  soll: string;
  lines: HubRulesLine[];
  error: string;
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
  /** Geplante Chunks: data/islands.json lesen, einen Entwurf (Planung und Chunk-Anzahl, 0 = Biom-Länge) prüfen, ohne zu
   *  schreiben, und Planung und Anzahl einer Tiefe speichern (B-461, B-485) */
  planLoad(): Promise<PlanView>;
  planCheck(island: string, depth: number, chunks: PlannedChunk[], count: number): Promise<PlanCheckView>;
  planSave(island: string, depth: number, chunks: PlannedChunk[], count: number, stand: string): Promise<PlanSaved>;
  /** Weitere Insel anlegen (Kopie der letzten) bzw. eine Insel entfernen (die erste bleibt), jeweils auf dem Stand aus planLoad */
  planIslandAdd(stand: string): Promise<PlanSaved>;
  /** Welten: auflisten, wählen (danach liest planLoad ihre Datei), mit Namen anlegen (wird gewählt), entfernen (Standard bleibt) */
  planWorlds(): Promise<WorldsView>;
  planWorldSelect(id: string): Promise<WorldsView>;
  planWorldAdd(name: string): Promise<WorldsView>;
  planWorldRemove(id: string): Promise<WorldsView>;
  planIslandRemove(id: string, stand: string): Promise<PlanSaved>;
  /** Seed-Inhalt des geplanten Chunks am Platz slot im Level zu Seed und Quelle (Umwandlung in feste Objekte, B-485/AC-06) */
  planChunkObjects(seed: string, source: WorldSource, slot: number): Promise<PlanObjectsView>;
  /** Im Spiel öffnen: `new` (neues Spiel mit dem Seed) oder `full` (voll ausgebaut), über die Landingpage des Servers unter base, nur Loopback */
  worldOpenGame(base: string, seed: string, biome: string, mode: 'new' | 'full'): Promise<OpenGameResult>;
  /** Gebäude-Planer: data/hub.json als Matrix mit Befunden der Prüfung (B-462) */
  hubPlan(): Promise<HubPlanView>;
  /** Gebäude-Planer: Abweichung der Regeltabelle vom Planer-Stand (leer = data/hub.json) */
  hubRules(hubJSON: string): Promise<HubRulesView>;
  /** Gebäude-Planer: Matrix und Befunde eines geänderten Stands (Prüfung im Go-Backend), schreibt nichts */
  hubCheck(candidate: string): Promise<HubPlanView>;
  /** Gebäude-Planer: speichert den Stand, wenn beide Dateien noch den Stand `stand` haben (B-462/AC-02, AC-06) */
  hubSave(candidate: string, stand: HubStand): Promise<HubSaveResult>;
  openUrl(url: string): void;
  /** Abo eines Ereignisses; liefert die Abmeldung */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
