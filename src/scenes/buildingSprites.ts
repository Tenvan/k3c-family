import type { SiteKind, SiteState } from '../model/types';

/**
 * Welche Grafik ein Gebäude zeigt (GR3.1, Zuordnung `docs/assets/zuordnung-objekte.md` und `zuordnung-welt.md`).
 * Reine Auswahl ohne Phaser: `null` = keine Grafik, dann zeichnet der Platzhalter. Lücken stehen hier nicht drin.
 */

/** Textur-Schlüssel → Datei unter `public/grafik/` (nur zugeordnete Dateien; LoadScene lädt genau diese) */
export const BUILDING_TEXTURES: Record<string, string> = {
  'grafik:fort-tileset': 'k3c-paletten/ebenen/fort-tileset.png',
  'grafik:fort-banner': 'k3c-paletten/props/fort-banner.png',
  'grafik:hub-2': 'wooden-fortress-and-animated-doors/ebenen/wooden-castle.png',
  'grafik:wall-2': 'gothicvania-town/tileset-einzeln/wall.png',
  'grafik:wall-3': 'k3c-paletten/tileset-einzeln/wall-kupfer.png',
  'grafik:wall-4': 'k3c-paletten/tileset-einzeln/wall-eisen.png',
  'grafik:wall-5': 'k3c-paletten/tileset-einzeln/wall-kristall.png',
  'grafik:tower-2': 'k3c-paletten/props/tower-stein.png',
  'grafik:tower-3': 'k3c-paletten/props/tower-kupfer.png',
  'grafik:tower-4': 'k3c-paletten/props/tower-eisen.png',
  'grafik:gate': 'k3c-paletten/props/fort-closed-door.png',
  'grafik:workshop': 'gothicvania-town/props-einzeln/house-b.png',
  'grafik:storage': 'gothicvania-town/props-einzeln/crate-stack.png',
  'grafik:farm': 'gothicvania-town/props-einzeln/house-a.png',
  'grafik:barracks': 'gothicvania-town/props-einzeln/house-c.png',
  'grafik:stairsDown': 'phantasy-dungeon-entrance/ebenen/dungeon-door.png',
  'grafik:stairs-left': 'gothicvania-town/tileset-einzeln/stairs-left.png',
  'grafik:stairs': 'gothicvania-town/tileset-einzeln/stairs.png',
  'grafik:stairs-right': 'gothicvania-town/tileset-einzeln/stairs-right.png',
};

/** Treppe hoch aus 16×32-Kacheln, von unten links nach oben rechts je eine Kachel weiter und eine halbe höher */
export const STAIRS_UP_TILES = ['grafik:stairs-left', 'grafik:stairs', 'grafik:stairs', 'grafik:stairs', 'grafik:stairs-right'];

/** Material-Stufen 1–5 (Regel materialien-gebaeude.md § 3.1); Stufe 1 (Holz) und Turm 5 (Zaubertum) sind Lücken */
const STAGES: Partial<Record<SiteKind, (string | null)[]>> = {
  wall: [null, 'grafik:wall-2', 'grafik:wall-3', 'grafik:wall-4', 'grafik:wall-5'],
  tower: [null, 'grafik:tower-2', 'grafik:tower-3', 'grafik:tower-4', null],
};

/** Ohne Stufe im Snapshot: die Grafik der Objekt-Zeile (Mauer und Turm zeigen dort Stein = Stufe 2) */
const BASE: Partial<Record<SiteKind, string>> = {
  wall: 'grafik:wall-2',
  tower: 'grafik:tower-2',
  gate: 'grafik:gate',
  workshop: 'grafik:workshop',
  storage: 'grafik:storage',
  farm: 'grafik:farm',
  barracks: 'grafik:barracks',
  stairsDown: 'grafik:stairsDown',
  /** steht für die ganze Kachel-Treppe (STAIRS_UP_TILES) */
  stairsUp: 'grafik:stairs',
};

/** Grafik eines Bauplatzes; nur gebaute Gebäude, Baustellen behalten Umriss, Fortschritt und Preisschild */
export function siteTexture(kind: SiteKind, state: SiteState, stage?: number): string | null {
  if (state !== 'built') return null;
  const stages = STAGES[kind];
  if (stage !== undefined && stages) return stages[stage - 1] ?? null;
  return BASE[kind] ?? null;
}

/** Hub-Stufe (materialien-gebaeude.md § 2): 1 = Burg aus dem Fort-Tileset, 2 = Palisade; 3–5 sind Kachel-Bausätze, noch nicht gesetzt */
export function hubTexture(stage = 1): string | null {
  return ['grafik:fort-tileset', 'grafik:hub-2'][stage - 1] ?? null;
}
