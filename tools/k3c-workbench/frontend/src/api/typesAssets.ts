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

/** RPC-Methoden der Asset-Oberfläche (Wails-Bindings bzw. /api/dev/rpc/Assets…). */
export interface AssetsBackend {
  assetsList(): Promise<AssetEintrag[]>;
  assetsGet(id: string): Promise<Spec>;
  assetsStyle(): Promise<Stil>;
  assetsPrompts(): Promise<Vorlage[]>;
  /** Basis `HEAD`, `HEAD~1`, Commit-SHA oder `origin/develop`. */
  assetsReview(basis: string): Promise<Review>;
  /** `variante` und `basis` dürfen leer sein (leere Basis = Arbeitsstand). */
  assetsFrame(id: string, animation: string, frame: number, variante: string, basis: string): Promise<Frame>;
  assetsPixelDiff(id: string, animation: string, frame: number, basis: string): Promise<PixelDiff>;
}
