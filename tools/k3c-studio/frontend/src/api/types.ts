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
  openUrl(url: string): void;
  /** Abo eines Ereignisses; liefert die Abmeldung */
  on<E extends EventName>(event: E, fn: (data: Events[E]) => void): () => void;
}
