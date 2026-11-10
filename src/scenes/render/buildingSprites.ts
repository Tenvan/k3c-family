import { clientLog } from '../../core/clientLog';
import { BUILDINGS } from '../../model/data';
import type { SiteKind, SiteState } from '../../model/types';
import { HUB_GROWTH, HUB_SCAFFOLD, type HubLook } from './hubGrowth';
import { paperBuildingFile } from './paperAssets';
import { SCAFFOLD_CLASSES, scaffoldKey } from './scaffoldView';

/**
 * Welche Grafik ein Gebäude zeigt (GR3.1, Zuordnung `docs/assets/zuordnung-objekte.md` und `zuordnung-welt.md`).
 * Reine Auswahl ohne Phaser: `null` = keine Grafik, dann zeichnet der Platzhalter. Lücken stehen hier nicht drin.
 */

/** Hub-Gebäude (GEB9): Bauplatz → Grafik-Id aus `tools/buildings`; ohne eigene `levels` in `data/buildings.json` wechseln sie mit der Hub-Stufe (`World.hubLevel`), mit `levels` folgen sie der Gebäudestufe (`Site.level`, B-569) */
const HUB_BUILDINGS: Partial<Record<SiteKind, string>> = {
  workshop: 'werkstatt', tavern: 'taverne', healer: 'heilplatz', smithy: 'schmiede',
  armory: 'ruestkammer', storage: 'lager', nursery: 'farm', barracks: 'kaserne',
};

/** Abschnitte des Schiffs aus `data/ship.json` (B-637): Kiel, Rumpf, Segel */
const SHIP_STAGES = [1, 2, 3];

/** Datei eines Gebäudes in Stufe n: Papierschnitt (`gebaeude-paper/`); jede Id steht in `PAPER_BUILDINGS` (ein Test prüft alle Schlüssel) */
const missing = new Set<string>();
const file = (id: string, n?: number): string => {
  const f = paperBuildingFile(id, n ?? 1);
  if (f) return f;
  if (!missing.has(id)) {
    missing.add(id);
    clientLog('warn', '🤒 Papier-Asset fehlt, keine Grafik', { id }); // kein Rückfall auf alte Pixel-Grafik (CU1.2)
  }
  return '';
};

/** Burg-Ausbauschritt (Stufe 6–13 der Wachstumsreihe) → Hub-Stufe 2–5 und Schritt 2–3 (D-MG6-05) */
export function castleStep(stufe: number): { level: number; step: number } | null {
  if (stufe < 6 || stufe > 13) return null;
  return { level: 2 + Math.floor((stufe - 6) / 2), step: 2 + ((stufe - 6) % 2) };
}

/** Überlage `burg-ausbau` über der Burg im Ausbauschritt (Stufe 2 = Banner und Fackeln, 3 = Wehrtürmchen), sonst `null` */
export function castleOverlay(look: Pick<HubLook, 'id' | 'stufe'>): string | null {
  const s = look.id === 'burg' ? castleStep(look.stufe) : null;
  return s ? `grafik:burg-ausbau-${s.step}` : null;
}

/** Textur-Schlüssel → Datei unter `public/grafik/` (nur zugeordnete Dateien; LoadScene lädt genau diese) */
export const BUILDING_TEXTURES: Record<string, string> = {
  ...stufen('hub', 'burg'),
  /** Wachstumsreihe ohne die Burgen der Hub-Stufen (GEB11, GEB17.6) und Baugerüst: `grafik:<id>-<stufe>`, `grafik:hub-geruest` */
  ...Object.fromEntries(HUB_GROWTH.filter((l) => !isHubStage(l)).map((l) => [`grafik:${l.id}-${l.stufe}`, growthFile(l)])),
  /** Überlage der Burg-Ausbauschritte (US1, D-MG6-05) */
  'grafik:burg-ausbau-2': file('burg-ausbau', 2),
  'grafik:burg-ausbau-3': file('burg-ausbau', 3),
  [`grafik:${HUB_SCAFFOLD}`]: file(HUB_SCAFFOLD),
  /** Gerüst im Bau je Größenklasse (GEB16, B-586): `grafik:geruest-<breite>` */
  ...Object.fromEntries(SCAFFOLD_CLASSES.map((c) => [scaffoldKey(c), file(`geruest-${c}`)])),
  ...stufen('wall', 'mauer'),
  ...stufen('tower', 'turm'),
  ...stufen('backwall', 'hintergrundmauer'),
  /** Schiff am Hafen (BS10, B-641): ein Abschnitt je Stufe (Kiel, Rumpf, Segel = fertig), Datei `schiff-<n>` */
  ...Object.fromEntries(SHIP_STAGES.map((n) => [`grafik:ship-${n}`, file('schiff', n)])),
  ...Object.assign({}, ...Object.entries(HUB_BUILDINGS).map(([kind, id]) => stufen(kind, id!))),
  'grafik:stairsUp': file('treppe-hoch'),
  'grafik:stairsDown': file('treppe-runter'),
};

/** Burg, Mauer, Turm, Tor und Hub-Gebäude in den Stufen 1–5 aus `tools/buildings` (GEB3, GEB4, GEB9): `grafik:<kind>-<n>` → `gebaeude/<id>-<n>-32.png` */
function stufen(kind: string, id: string): Record<string, string> {
  return Object.fromEntries([1, 2, 3, 4, 5].map((n) => [`grafik:${kind}-${n}`, file(id, n)]));
}

/** Datei eines Looks der Wachstumsreihe: Burg-Ausbauschritte zeigen die Burg ihrer Hub-Stufe (Überlage `castleOverlay`) */
function growthFile(l: Pick<HubLook, 'id' | 'stufe'>): string {
  const s = l.id === 'burg' ? castleStep(l.stufe) : null;
  return s ? file('burg', s.level) : file(l.id, l.stufe);
}

/** Material-Stufen 1–5 (Regel materialien-gebaeude.md § 3.1; Grafik GEB3, Turm 5 = Zaubertum; Tor GEB4 in der Stufe seiner Mauerlinie) */
const stageKeys = (kind: string): string[] => [1, 2, 3, 4, 5].map((n) => `grafik:${kind}-${n}`);
const STAGES: Partial<Record<SiteKind, string[]>> = {
  wall: stageKeys('wall'),
  tower: stageKeys('tower'),
  ship: SHIP_STAGES.map((n) => `grafik:ship-${n}`),
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
    // Hub-Gebäude und Schiff: fehlt die Stufe, gilt die höchste vorhandene (B-458 kann die Stufenzahl ändern); Mauer, Turm, Tor: Lücke = null
    const n = kind in HUB_BUILDINGS || kind === 'ship' ? Math.min(Math.max(Math.trunc(stage) || 1, 1), stages.length) : stage;
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
