import { BUILDINGS } from '../../model/data';
import type { SiteKind, SiteState } from '../../model/types';
import { HUB_GROWTH, HUB_SCAFFOLD, type HubLook } from './hubGrowth';
import { SCAFFOLD_CLASSES, scaffoldKey } from './scaffoldView';

/**
 * Welche Grafik ein Gebäude zeigt (GR3.1, Zuordnung `docs/assets/zuordnung-objekte.md` und `zuordnung-welt.md`).
 * Reine Auswahl ohne Phaser: `null` = keine Grafik, dann zeichnet der Platzhalter. Lücken stehen hier nicht drin.
 */

/** Hub-Gebäude (GEB9): Bauplatz → Grafik-Id aus `tools/buildings`; ohne eigene `levels` in `data/buildings.json` wechseln sie mit der Hub-Stufe (`World.hubLevel`), mit `levels` folgen sie der Gebäudestufe (`Site.level`, B-569) */
const HUB_BUILDINGS: Partial<Record<SiteKind, string>> = {
  workshop: 'werkstatt', tavern: 'taverne', healer: 'heilplatz', smithy: 'schmiede',
  armory: 'ruestkammer', storage: 'lager', farm: 'farm', barracks: 'kaserne',
};

/** Textur-Schlüssel → Datei unter `public/grafik/` (nur zugeordnete Dateien; LoadScene lädt genau diese) */
export const BUILDING_TEXTURES: Record<string, string> = {
  ...stufen('hub', 'burg'),
  /** Wachstumsreihe ohne die Burgen der Hub-Stufen (GEB11, GEB17.6) und Baugerüst: `grafik:<id>-<stufe>`, `grafik:hub-geruest` */
  ...Object.fromEntries(HUB_GROWTH.filter((l) => !isHubStage(l)).map((l) => [`grafik:${l.id}-${l.stufe}`, `gebaeude/${l.id}-${l.stufe}-32.png`])),
  [`grafik:${HUB_SCAFFOLD}`]: `gebaeude/${HUB_SCAFFOLD}-32.png`,
  /** Gerüst im Bau je Größenklasse (GEB16, B-586): `grafik:geruest-<breite>` */
  ...Object.fromEntries(SCAFFOLD_CLASSES.map((c) => [scaffoldKey(c), `gebaeude/geruest-${c}-32.png`])),
  ...stufen('wall', 'mauer'),
  ...stufen('tower', 'turm'),
  ...stufen('backwall', 'hintergrundmauer'),
  ...Object.assign({}, ...Object.entries(HUB_BUILDINGS).map(([kind, id]) => stufen(kind, id!))),
  'grafik:stairsUp': 'gebaeude/treppe-hoch-32.png',
  'grafik:stairsDown': 'gebaeude/treppe-runter-32.png',
  /** Plantage (GEB5, B-568): Setzling auf leerem Platz, niedrige Pappel auf gewachsenem Platz (vor der Farm) */
  'grafik:setzling': 'gebaeude/setzling-32.png',
  'grafik:pappel-niedrig': 'gebaeude/pappel-niedrig-32.png',
};

/** Burg, Mauer, Turm, Tor und Hub-Gebäude in den Stufen 1–5 aus `tools/buildings` (GEB3, GEB4, GEB9): `grafik:<kind>-<n>` → `gebaeude/<id>-<n>-32.png` */
function stufen(kind: string, id: string): Record<string, string> {
  return Object.fromEntries([1, 2, 3, 4, 5].map((n) => [`grafik:${kind}-${n}`, `gebaeude/${id}-${n}-32.png`]));
}

/** Material-Stufen 1–5 (Regel materialien-gebaeude.md § 3.1; Grafik GEB3, Turm 5 = Zaubertum; Tor GEB4 in der Stufe seiner Mauerlinie) */
const stageKeys = (kind: string): string[] => [1, 2, 3, 4, 5].map((n) => `grafik:${kind}-${n}`);
const STAGES: Partial<Record<SiteKind, string[]>> = {
  wall: stageKeys('wall'),
  tower: stageKeys('tower'),
  ...Object.fromEntries(Object.keys(HUB_BUILDINGS).map((kind) => [kind, stageKeys(kind)])),
};

/** Ohne Stufe: die Grundform (Mauer, Turm, Tor und Hub-Gebäude: Stufe 1, wie `Site.level` „fehlt = 1“) */
const BASE: Partial<Record<SiteKind, string>> = {
  ...Object.fromEntries(Object.keys(STAGES).map((kind) => [kind, `grafik:${kind}-1`])),
  stairsDown: 'grafik:stairsDown',
  stairsUp: 'grafik:stairsUp',
};

/** Hub-Gebäude ohne eigene Stufen zeigen die Grafik der Hub-Stufe (GEB9); der Aufrufer reicht `World.hubLevel` (fehlt = 1) als Stufe durch. Mit `levels` (B-569) zählt die Stufe des Platzes */
export function isHubBuilding(kind: SiteKind): boolean {
  return kind in HUB_BUILDINGS && !BUILDINGS[kind]?.levels?.length;
}

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
  if (stage !== undefined && stages) {
    // Hub-Gebäude (mit und ohne eigene Stufen): fehlt die Stufe, gilt die höchste vorhandene (B-458 kann die Stufenzahl ändern); Mauer, Turm, Tor: Lücke = null
    const n = kind in HUB_BUILDINGS ? Math.min(Math.max(Math.trunc(stage) || 1, 1), stages.length) : stage;
    return stages[n - 1] ?? null;
  }
  return BASE[kind] ?? null;
}

/** Burg in der Hub-Stufe 1–5 (materialien-gebaeude.md § 2, GEB4), ohne Stufe 1; unbekannte Stufe `null` */
export function hubTexture(stage = 1): string | null {
  const key = `grafik:hub-${stage}`;
  return key in BUILDING_TEXTURES ? key : null;
}

/** Burg einer Hub-Stufe 1–5 (`burg-<n>`, Schlüssel `grafik:hub-<n>`); die Ausbauschritte `burg-6` … liegen unter `grafik:burg-<n>` */
function isHubStage(look: Pick<HubLook, 'id' | 'stufe'>): boolean {
  return look.id === 'burg' && look.stufe >= 1 && look.stufe <= 5;
}

/** Textur eines Looks der Wachstumsreihe (GEB11, GEB17.6, `hubLookOf`): Burg der Hub-Stufe über `hubTexture`, sonst `grafik:<id>-<stufe>`; unbekannt `null` */
export function hubLookTexture(look: Pick<HubLook, 'id' | 'stufe'>): string | null {
  if (isHubStage(look)) return hubTexture(look.stufe);
  const key = `grafik:${look.id}-${look.stufe}`;
  return key in BUILDING_TEXTURES ? key : null;
}

/** Baugerüst vor der Burg, solange ein Bauer den Hub-Ausbau baut (GEB11) */
export const HUB_SCAFFOLD_TEXTURE = `grafik:${HUB_SCAFFOLD}`;
