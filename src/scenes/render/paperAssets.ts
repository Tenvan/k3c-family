import paperJson from '../../../data/paper.json';

/**
 * Papierschnitt-Grafik (Entscheidung 005, US1): Zuordnung Spielrolle → Asset aus `data/paper.json`, Dateien unter
 * `public/grafik/<gruppe>/<id>/` (Manifest `manifest.json`, je Animation ein Sheet, Gebäude-Stufen `stand.s<n>.png`).
 * Reine Logik ohne Phaser.
 */

/** Animations-Eintrag eines Manifests (`tools/paper`, Render-Vertrag PV1) */
export interface PaperAnim {
  frames: number;
  fps: number;
  datei: string;
  /** Rahmen [x, y, Breite, Höhe] in Einheiten, Fußpunkt bei 0/0; fehlt er, gilt der des Assets */
  rahmen?: [number, number, number, number];
  frameW?: number;
  frameH?: number;
}

export interface PaperManifest {
  id: string;
  rahmen: [number, number, number, number];
  frameW: number;
  frameH: number;
  animationen: Record<string, PaperAnim>;
}

/** Verweis `<gruppe>/<id>` mit optionaler Variante (`@spieler2`) */
export interface PaperRef {
  gruppe: string;
  id: string;
  variante?: string;
}

/** Maße je Rolle (`data/paper.json › figuren`): Zielhöhe in Spiel-Pixeln, Tönung, Höhe über dem Boden, Deckkraft */
export interface PaperFigure {
  hoehe: number;
  tint?: string;
  y?: number;
  alpha?: number;
}

const DATA = paperJson as unknown as {
  figuren: { spieler: PaperFigure[]; truppen: Record<string, PaperFigure>; gegner: Record<string, PaperFigure> };
  spieler: string[];
  beritten: string;
  reittiere: Record<string, string>;
  truppen: Record<string, string>;
  gegner: Record<string, string>;
  gebaeude: string;
  hintergruende: { gruppe: string; varianten: number };
  objekte: Record<string, PaperObject>;
};

export function parseRef(text: string): PaperRef {
  const [path, variante] = text.split('@');
  const [gruppe, id] = path!.split('/');
  return variante ? { gruppe: gruppe!, id: id! , variante } : { gruppe: gruppe!, id: id! };
}

export const PAPER_FIGURES = DATA.figuren;
export const PAPER_PLAYERS: PaperRef[] = DATA.spieler.map(parseRef);
export const PAPER_MOUNTED: PaperRef = parseRef(DATA.beritten);
/** Reittier-Schlüssel (`data/paper.json` › `reittiere`, `data/monarch.json` › `mount.sprite`) → Asset-Id in `reittiere/` (US2) */
export const PAPER_MOUNTS: Record<string, string> = DATA.reittiere;
export const PAPER_TROOPS: Record<string, PaperRef> = Object.fromEntries(Object.entries(DATA.truppen).map(([k, v]) => [k, parseRef(v)]));
export const PAPER_ENEMIES: Record<string, PaperRef> = Object.fromEntries(Object.entries(DATA.gegner).map(([k, v]) => [k, parseRef(v)]));

/** Asset-Schlüssel `<gruppe>/<id>` (ohne Variante): ein Manifest je Schlüssel */
export const assetKey = (r: Pick<PaperRef, 'gruppe' | 'id'>): string => `${r.gruppe}/${r.id}`;

/** Alle Figuren-Assets, die das Spiel lädt (einmal je Gruppe/Id) */
export function usedFigureAssets(): string[] {
  const refs = [...PAPER_PLAYERS, PAPER_MOUNTED, ...Object.values(PAPER_TROOPS), ...Object.values(PAPER_ENEMIES)];
  return [...new Set(refs.map(assetKey))].sort();
}

/** Spiel-Animation → Papier-Animation; ohne Entsprechung `null` (dann `idle`) */
const ANIM_NAMES: Record<string, string> = { idle: 'idle', run: 'lauf', attack: 'angriff' };
export function paperAnimName(anim: string): string | null {
  return ANIM_NAMES[anim] ?? null;
}

/** Name im Manifest: Animation mit Variante (`lauf@spieler2`) */
export const manifestAnim = (name: string, variante?: string): string => (variante ? `${name}@${variante}` : name);

/** Textur-Schlüssel eines Sheets */
export const paperTextureKey = (key: string, anim: string): string => `paper:${key}:${anim}`;

/** Rahmen und Frame-Größe einer Animation (fehlende Felder vom Asset) */
export function animFrame(m: PaperManifest, anim: string): { rahmen: [number, number, number, number]; frameW: number; frameH: number } {
  const a = m.animationen[anim];
  return { rahmen: a?.rahmen ?? m.rahmen, frameW: a?.frameW ?? m.frameW, frameH: a?.frameH ?? m.frameH };
}

/** Ursprung (Fußpunkt 0/0) als Anteil des Frames */
export function paperOrigin(rahmen: readonly number[]): { x: number; y: number } {
  return { x: -rahmen[0]! / rahmen[2]!, y: -rahmen[1]! / rahmen[3]! };
}

/**
 * Maßstab der Animation, damit die Figur `targetPx` hoch erscheint (Höhe des Asset-Rahmens über dem Fuß), und die Oberkante
 * relativ zum Fuß (negativ). Alle Animationen teilen dieselben Spiel-Pixel je Einheit.
 */
export function paperScale(m: PaperManifest, anim: string, targetPx: number): { scale: number; top: number } {
  const unitPx = targetPx / -m.rahmen[1];
  const f = animFrame(m, anim);
  return { scale: unitPx / (f.frameH / f.rahmen[3]), top: -targetPx };
}

// ---------- Gebäude und Hintergründe ----------

/** Gebäude mit Papierschnitt-Spec (MG6, Gruppe `gebaeude-paper`) */
export const PAPER_BUILDINGS = [
  'burg', 'burg-ausbau', 'farm', 'hafen', 'heilplatz', 'hintergrundmauer', 'kaserne', 'lager', 'lagerfeuer', 'mauer',
  'rekrutierungslager', 'ruestkammer', 'schiff', 'schmiede', 'taverne', 'tor', 'treppe-hoch', 'treppe-runter', 'turm', 'werkstatt',
  // Baugerüste je Größenklasse und vor der Burg (CU1.1, CU1.2)
  'geruest-1', 'geruest-2', 'geruest-3', 'geruest-4', 'geruest-6', 'hub-geruest',
] as const;
const BUILDINGS = new Set<string>(PAPER_BUILDINGS);

/** Datei unter `public/grafik/` eines Gebäudes in Stufe `stufe` (1 = ohne Zusatz, D-15); kein Papier-Gebäude `null` */
export function paperBuildingFile(id: string, stufe = 1): string | null {
  if (!BUILDINGS.has(id)) return null;
  return `${DATA.gebaeude}/${id}/stand${stufe > 1 ? `.s${stufe}` : ''}.png`;
}

export const isPaperFile = (file: string): boolean =>
  [DATA.gebaeude, DATA.hintergruende.gruppe, ...new Set(Object.values(DATA.objekte).map((o) => o.gruppe))].some((g) => file.startsWith(`${g}/`));

/** Hintergrund-Schicht `teil` (`wald` …), `ebene` (`fern`, `mitte`, `nah`), Variante v; Papier kennt weniger Varianten, sie wiederholen sich */
export function paperBackgroundFile(teil: string, ebene: string, v: number): string {
  const n = ((Math.max(1, Math.trunc(v)) - 1) % DATA.hintergruende.varianten) + 1;
  return `${DATA.hintergruende.gruppe}/hg-${teil}-${ebene}-v${n}/stand.png`;
}

// ---------- Natur, Objekte, Münzen und Icons (MG8, US2) ----------

/** Maße eines Objekt-Assets aus `data/paper.json` › `objekte` (ein Test hält sie gleich mit den Manifesten) */
export interface PaperObject {
  gruppe: string;
  /** Frame-Breite und -Höhe in Pixeln; das Sheet `idle.png` legt die Frames nebeneinander */
  frame: [number, number];
  frames: number;
  rahmen: [number, number, number, number];
}

export const PAPER_OBJECTS: Record<string, PaperObject> = DATA.objekte;

/** Datei des Objekts (`<gruppe>/<id>/idle.png`); kein Papier-Objekt `null` */
export function paperObjectFile(id: string): string | null {
  const o = PAPER_OBJECTS[id];
  return o ? `${o.gruppe}/${id}/idle.png` : null;
}

/** Objekt-Id zu einer Datei aus `paperObjectFile`, sonst `null` */
export function paperObjectOf(file: string): string | null {
  const m = /^[a-z]+\/([a-z-]+)\/idle\.png$/.exec(file);
  return m && PAPER_OBJECTS[m[1]!] && paperObjectFile(m[1]!) === file ? m[1]! : null;
}

/** Darstellung eines Objekts: Frames, Ursprung im Fußpunkt und Maßstab 1 Einheit = 1 Spiel-Pixel (32 je Kachel wie die Gebäude) */
export function paperObjectLook(id: string): { frameWidth: number; frameHeight: number; frames: number; originX: number; originY: number; zoom: number } | null {
  const o = PAPER_OBJECTS[id];
  if (!o) return null;
  const origin = paperOrigin(o.rahmen);
  return { frameWidth: o.frame[0], frameHeight: o.frame[1], frames: o.frames, originX: origin.x, originY: origin.y, zoom: o.rahmen[3] / o.frame[1] };
}
