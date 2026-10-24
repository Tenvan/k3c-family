import type { AssetKind, AssetStatus } from './types';

// Slot-Details der Grafik-Seite (WZ28.1), Form wie Go assets.SlotDetails.
/** Anzeigedaten eines Slots (Go: assets.SlotDetails, WZ28.1); fehlende Quellen stehen in hinweise */
export interface AssetSlotDetails {
  id: string;
  art: AssetKind;
  kategorie: string;
  beschreibung: string;
  status: AssetStatus;
  wahl?: { datei: string; sheet?: string };
  sheet?: SlotSheet;
  atlas?: { anims: SlotAtlasAnim[] };
  pack?: { id: string; quelle: string; lizenz: string };
  zuordnung: SlotZuordnung[];
  verwendung: string[];
  hinweise: string[];
}

/** Sheet aus data/sprites.json samt Tönung des Gegner-, Truppen- oder Spieler-Eintrags */
export interface SlotSheet {
  name: string;
  label: string;
  pack: string;
  frameW: number;
  frameH: number;
  scale: number;
  height: number;
  originX: number;
  originY: number;
  tint: string;
  anims: { name: string; frames: number; fps: number }[];
}

/** Animation im Atlas: Frame 0 als Rechteck auf der Seite */
export interface SlotAtlasAnim {
  name: string;
  frames: number;
  page: string;
  x: number;
  y: number;
  w: number;
  h: number;
}

/** Zeile aus einer Zuordnungstabelle unter docs/assets/ */
export interface SlotZuordnung {
  datei: string;
  abschnitt: string;
  zellen: Record<string, string>;
}
