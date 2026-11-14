// Asset-Oberfläche (WB1.1): Typen genau laut Vertrag docs/standards/paper-mcp.md › 3–4 (Go: internal/assetsui).
// RV1 importiert sie über src/assets/api.ts; Änderungen nur per Ticket an WB1 (Sprint-README WB1 › D-02).

/** Regelverstoß einer Spec (`paper validate`): Feld, Regel, Text. */
export interface AssetFehler {
  feld: string;
  regel: string;
  text: string;
}

/** Eintrag im Katalog; `geaendert` gegen `HEAD`. */
export interface AssetEintrag {
  id: string;
  rig: string;
  gruppe: string;
  pfad: string;
  gueltig: boolean;
  geaendert: boolean;
  fehler?: AssetFehler[];
  /** Noch ungeprüft (nur `assetsIndex`, B-732); gilt bis zur Prüfung als gültig. */
  offen?: boolean;
}

/** Spiel-Maßstab eines Assets (B-733): Spiel-Pixel je Einheit der Spec und woher der Wert kommt. */
export interface Groesse {
  pxProEinheit: number;
  quelle: string;
}

/** Hintergrund eines Bioms (Variante 1): Ebenen hinten → vorn als Data-URLs, je 768 × 480 Spiel-Pixel, Fuß unten. */
export interface Hintergrund {
  fern: string;
  mitte: string;
  nah: string;
}

/** Animation einer Spec (`docs/standards/paper-spec.md`): 1–64 Frames, `fps` Standard 8. */
export interface SpecAnimation {
  frames: number;
  fps?: number;
  [feld: string]: unknown;
}

/** Spec als Objekt (ohne `$schema`); typisiert sind nur die Felder, die die Oberfläche liest. */
export interface Spec {
  id: string;
  rig: string;
  version: number;
  beschreibung?: string;
  teile: { id: string; [feld: string]: unknown }[];
  animationen?: Record<string, SpecAnimation>;
  varianten?: Record<string, unknown>;
  /** Ausgabe: Frame-Höhe in Pixeln und Rahmen [x, y, Breite, Höhe] in Einheiten, Fuß bei 0/0 (B-733). */
  ausgabe?: { hoehe_px: number; rahmen: [number, number, number, number] };
  [feld: string]: unknown;
}

/** Drei Töne eines Papier-Materials. */
export interface MaterialToene {
  licht: string;
  basis: string;
  schatten: string;
}

/** Stil-Datei als Objekt (ohne `$schema`). */
export interface Stil {
  materialien: Record<string, MaterialToene>;
  einzeltoene?: Record<string, string>;
  papier?: Record<string, unknown>;
  [feld: string]: unknown;
}

/** Prompt-Vorlage aus `data/asset-prompts.json`. */
export interface Vorlage {
  id: string;
  titel: string;
  ort: 'asset' | 'teil' | 'review';
  text: string;
}

export type ReviewStatus = 'neu' | 'geaendert' | 'geloescht' | 'ungueltig';

export interface FeldAenderung {
  pfad: string;
  alt: unknown;
  neu: unknown;
}

export interface ReviewAsset {
  id: string;
  rig: string;
  pfad: string;
  status: ReviewStatus;
  fehler?: AssetFehler[];
  felder: FeldAenderung[];
  teile: string[];
  stil: boolean;
  pixelAnteil: number;
}

export interface Review {
  basis: string;
  basisCommit: string;
  basisStand: 'aktuell' | 'lokal';
  assets: ReviewAsset[];
}

/** Box `[x, y, b, h]` in Pixeln des Frames. */
export type Box = [number, number, number, number];

export interface Frame {
  /** Data-URL (`data:image/png;base64,…`). */
  bild: string;
  teile: Record<string, Box>;
}

export interface PixelDiff {
  anteil: number;
  boxen: Box[];
  overlay: string;
}

/** Sound-Slot aus data/sounds.json (Go: studioview.SoundEintrag, WB1.4). */
export interface SoundEintrag {
  id: string;
  kategorie: string;
  beschreibung: string;
  bus?: string;
  status: 'zugeordnet' | 'luecke' | 'fehlt';
  dateien: string[];
}

/** Datei des Katalogs unter public/ (Go: assetkit.Asset). */
export interface KatalogDatei {
  datei: string;
  art: string;
  pack: string;
  gruppe?: string;
  titel?: string;
  urheber?: string;
  lizenz: string;
  quelle?: string;
  kennzeichen?: string;
}

/** Recherche-Treffer eines Slots (Go: assetkit.Hit). */
export interface Treffer {
  id: string;
  titel: string;
  link: string;
  quelle: string;
  urheber?: string;
  lizenz?: string;
  lizenzStatus: string;
  kennzeichen?: string;
  erstellt: string;
  datei?: string;
}

/** Quellen: Katalog und Recherche-Treffer je Slot (Go: studioview.Quellen). */
export interface Quellen {
  katalog: KatalogDatei[];
  treffer: Record<string, Treffer[]>;
}

/** RPC-Methoden der Asset-Oberfläche (Wails-Bindings bzw. /api/dev/rpc/Assets…). */
export interface AssetsBackend {
  assetsList(): Promise<AssetEintrag[]>;
  /** Katalog ohne Prüfung (B-732): ungeprüfte Einträge mit `offen`, Ergebnis über `assetsCheck`. */
  assetsIndex(): Promise<AssetEintrag[]>;
  /** Prüft die Specs unter `pfade` (aus `assetsIndex`) und liefert ihre Einträge. */
  assetsCheck(pfade: string[]): Promise<AssetEintrag[]>;
  /** Spiel-Maßstab eines Assets für den Größenvergleich (B-733). */
  assetsGroesse(id: string): Promise<Groesse>;
  /** Frame des Arbeitsstands ohne die Teile in `ohne` (B-734: Teil ausblenden). */
  assetsFrameOhne(id: string, animation: string, frame: number, ohne: string[]): Promise<Frame>;
  /** Biome mit Hintergrund (Namensteil `hg-<teil>-…`, z. B. `wald`). */
  assetsHintergruende(): Promise<string[]>;
  /** Die drei Ebenen eines Biom-Hintergrunds als Data-URLs. */
  assetsHintergrund(teil: string): Promise<Hintergrund>;
  assetsGet(id: string): Promise<Spec>;
  assetsStyle(): Promise<Stil>;
  assetsPrompts(): Promise<Vorlage[]>;
  /** Basis `HEAD`, `HEAD~1`, Commit-SHA oder `origin/main`. */
  assetsReview(basis: string): Promise<Review>;
  /** `variante` und `basis` dürfen leer sein (leere Basis = Arbeitsstand). */
  assetsFrame(id: string, animation: string, frame: number, variante: string, basis: string): Promise<Frame>;
  assetsPixelDiff(id: string, animation: string, frame: number, basis: string): Promise<PixelDiff>;
  /** Lesende Teile des früheren Asset-Studios (WB1.4): Sound-Slots, Hörprobe als Data-URL, Quellen. */
  studioSounds(): Promise<SoundEintrag[]>;
  studioSoundData(pfad: string): Promise<string>;
  studioSources(): Promise<Quellen>;
  /** Legt den Testspielstand voll-<seed> an; Antwort ist die Adresse zum Öffnen im Browser. */
  studioGameOpen(seed: string): Promise<string>;
}
