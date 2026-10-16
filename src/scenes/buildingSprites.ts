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
  ...stufen('wall', 'mauer'),
  ...stufen('tower', 'turm'),
  'grafik:gate': 'k3c-paletten/props/fort-closed-door.png',
  'grafik:workshop': 'gebaeude/werkstatt-32.png',
  'grafik:tavern': 'gebaeude/taverne-32.png',
  'grafik:healer': 'gebaeude/heilplatz-32.png',
  'grafik:smithy': 'gebaeude/schmiede-32.png',
  'grafik:armory': 'gebaeude/ruestkammer-32.png',
  'grafik:storage': 'gothicvania-town/props-einzeln/crate-stack.png',
  'grafik:farm': 'gothicvania-town/props-einzeln/house-a.png',
  'grafik:barracks': 'gothicvania-town/props-einzeln/house-c.png',
  'grafik:stairsDown': 'phantasy-dungeon-entrance/ebenen/dungeon-door.png',
  'grafik:stairs-left': 'gothicvania-town/tileset-einzeln/stairs-left.png',
  'grafik:stairs': 'gothicvania-town/tileset-einzeln/stairs.png',
  'grafik:stairs-right': 'gothicvania-town/tileset-einzeln/stairs-right.png',
};

/** Mauer und Turm in den Material-Stufen 1–5 aus `tools/gebaeude` (GEB3): `grafik:<kind>-<n>` → `gebaeude/<id>-<n>-32.png` */
function stufen(kind: string, id: string): Record<string, string> {
  return Object.fromEntries([1, 2, 3, 4, 5].map((n) => [`grafik:${kind}-${n}`, `gebaeude/${id}-${n}-32.png`]));
}

/** Treppe hoch aus 16×32-Kacheln, von unten links nach oben rechts je eine Kachel weiter und eine halbe höher */
export const STAIRS_UP_TILES = ['grafik:stairs-left', 'grafik:stairs', 'grafik:stairs', 'grafik:stairs', 'grafik:stairs-right'];

/** Material-Stufen 1–5 (Regel materialien-gebaeude.md § 3.1; Grafik GEB3, Turm 5 = Zaubertum) */
const STAGES: Partial<Record<SiteKind, string[]>> = {
  wall: [1, 2, 3, 4, 5].map((n) => `grafik:wall-${n}`),
  tower: [1, 2, 3, 4, 5].map((n) => `grafik:tower-${n}`),
};

/** Ohne Stufe: die Grundform (Mauer und Turm: Stufe 1, wie `Site.level` „fehlt = 1“) */
const BASE: Partial<Record<SiteKind, string>> = {
  wall: 'grafik:wall-1',
  tower: 'grafik:tower-1',
  gate: 'grafik:gate',
  workshop: 'grafik:workshop',
  tavern: 'grafik:tavern',
  healer: 'grafik:healer',
  smithy: 'grafik:smithy',
  armory: 'grafik:armory',
  storage: 'grafik:storage',
  farm: 'grafik:farm',
  barracks: 'grafik:barracks',
  stairsDown: 'grafik:stairsDown',
  /** steht für die ganze Kachel-Treppe (STAIRS_UP_TILES) */
  stairsUp: 'grafik:stairs',
};

/** Maßstab im Spiel: eigene Gebäude aus `tools/gebaeude` sind schon 32 px je Kachel (×1), die 16-px-Packs ×2 (GEB2) */
export function textureScale(key: string): number {
  return BUILDING_TEXTURES[key]?.startsWith('gebaeude/') ? 1 : 2;
}

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
