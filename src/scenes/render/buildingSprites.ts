import type { SiteKind, SiteState } from '../../model/types';

/**
 * Welche Grafik ein Gebäude zeigt (GR3.1, Zuordnung `docs/assets/zuordnung-objekte.md` und `zuordnung-welt.md`).
 * Reine Auswahl ohne Phaser: `null` = keine Grafik, dann zeichnet der Platzhalter. Lücken stehen hier nicht drin.
 */

/** Textur-Schlüssel → Datei unter `public/grafik/` (nur zugeordnete Dateien; LoadScene lädt genau diese) */
export const BUILDING_TEXTURES: Record<string, string> = {
  ...stufen('hub', 'burg'),
  ...stufen('wall', 'mauer'),
  ...stufen('tower', 'turm'),
  ...stufen('backwall', 'hintergrundmauer'),
  ...stufen('gate', 'tor'),
  'grafik:workshop': 'gebaeude/werkstatt-32.png',
  'grafik:tavern': 'gebaeude/taverne-32.png',
  'grafik:healer': 'gebaeude/heilplatz-32.png',
  'grafik:smithy': 'gebaeude/schmiede-32.png',
  'grafik:armory': 'gebaeude/ruestkammer-32.png',
  'grafik:storage': 'gebaeude/lager-32.png',
  'grafik:farm': 'gebaeude/farm-32.png',
  'grafik:barracks': 'gebaeude/kaserne-32.png',
  'grafik:stairsUp': 'gebaeude/treppe-hoch-32.png',
  'grafik:stairsDown': 'gebaeude/treppe-runter-32.png',
  /** Plantage (GEB5): Setzling auf leerem Platz, Pappel auf gewachsenem Platz */
  'grafik:setzling': 'gebaeude/setzling-32.png',
  'grafik:pappel': 'gebaeude/pappel-32.png',
};

/** Burg, Mauer, Turm und Tor in den Stufen 1–5 aus `tools/buildings` (GEB3, GEB4): `grafik:<kind>-<n>` → `gebaeude/<id>-<n>-32.png` */
function stufen(kind: string, id: string): Record<string, string> {
  return Object.fromEntries([1, 2, 3, 4, 5].map((n) => [`grafik:${kind}-${n}`, `gebaeude/${id}-${n}-32.png`]));
}

/** Material-Stufen 1–5 (Regel materialien-gebaeude.md § 3.1; Grafik GEB3, Turm 5 = Zaubertum; Tor GEB4 in der Stufe seiner Mauerlinie) */
const STAGES: Partial<Record<SiteKind, string[]>> = {
  wall: [1, 2, 3, 4, 5].map((n) => `grafik:wall-${n}`),
  tower: [1, 2, 3, 4, 5].map((n) => `grafik:tower-${n}`),
  gate: [1, 2, 3, 4, 5].map((n) => `grafik:gate-${n}`),
};

/** Ohne Stufe: die Grundform (Mauer, Turm und Tor: Stufe 1, wie `Site.level` „fehlt = 1“) */
const BASE: Partial<Record<SiteKind, string>> = {
  wall: 'grafik:wall-1',
  tower: 'grafik:tower-1',
  gate: 'grafik:gate-1',
  workshop: 'grafik:workshop',
  tavern: 'grafik:tavern',
  healer: 'grafik:healer',
  smithy: 'grafik:smithy',
  armory: 'grafik:armory',
  storage: 'grafik:storage',
  farm: 'grafik:farm',
  barracks: 'grafik:barracks',
  stairsDown: 'grafik:stairsDown',
  stairsUp: 'grafik:stairsUp',
};

/** Maßstab im Spiel: eigene Gebäude aus `tools/buildings` sind schon 32 px je Kachel (×1), die 16-px-Packs ×2 (GEB2) */
export function textureScale(key: string): number {
  return BUILDING_TEXTURES[key]?.startsWith('gebaeude/') ? 1 : 2;
}

/** Textur der Hintergrundmauer in Stufe 1–5 (B-442), sonst `null` */
export function backWallTexture(stage: number): string | null {
  const key = `grafik:backwall-${stage}`;
  return key in BUILDING_TEXTURES ? key : null;
}

/** Grafik eines Bauplatzes; nur gebaute Gebäude, Baustellen behalten Umriss, Fortschritt und Preisschild */
export function siteTexture(kind: SiteKind, state: SiteState, stage?: number): string | null {
  if (state !== 'built') return null;
  const stages = STAGES[kind];
  if (stage !== undefined && stages) return stages[stage - 1] ?? null;
  return BASE[kind] ?? null;
}

/** Burg in der Hub-Stufe 1–5 (materialien-gebaeude.md § 2, GEB4), ohne Stufe 1; unbekannte Stufe `null` */
export function hubTexture(stage = 1): string | null {
  const key = `grafik:hub-${stage}`;
  return key in BUILDING_TEXTURES ? key : null;
}
