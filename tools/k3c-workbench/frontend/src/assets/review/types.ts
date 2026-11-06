// Typen der Asset-RPC laut docs/standards/paper-mcp.md › Oberfläche; Namen wie api/typesAssets.ts aus WB1, damit die
// Review-Seite nach dem Zusammenführen auf backend.assets* umziehen kann (B-701).

export interface AssetFehler {
  feld: string;
  regel: string;
  text: string;
}

export interface FeldAenderung {
  pfad: string;
  alt: unknown;
  neu: unknown;
}

export type ReviewStatus = 'neu' | 'geaendert' | 'geloescht' | 'ungueltig';

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

/** [x, y, b, h] in Pixeln des Frames. */
export type Box = [number, number, number, number];

export interface Frame {
  /** data:image/png;base64,… */
  bild: string;
  teile: Record<string, Box>;
}

export interface PixelDiff {
  anteil: number;
  boxen: Box[];
  overlay: string;
}

export interface Vorlage {
  id: string;
  titel: string;
  ort: 'asset' | 'teil' | 'review';
  text: string;
}

export interface SpecAnimation {
  frames: number;
  fps?: number;
}

export interface Spec {
  id: string;
  rig: string;
  version: number;
  animationen?: Record<string, SpecAnimation>;
}
