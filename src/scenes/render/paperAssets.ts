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

const DATA = paperJson as unknown as {
  spieler: string[];
  beritten: string;
  truppen: Record<string, string>;
  gegner: Record<string, string>;
  gebaeude: string;
  hintergruende: { gruppe: string; varianten: number };
};

export function parseRef(text: string): PaperRef {
  const [path, variante] = text.split('@');
  const [gruppe, id] = path!.split('/');
  return variante ? { gruppe: gruppe!, id: id! , variante } : { gruppe: gruppe!, id: id! };
}

export const PAPER_PLAYERS: PaperRef[] = DATA.spieler.map(parseRef);
export const PAPER_MOUNTED: PaperRef = parseRef(DATA.beritten);
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
] as const;
const BUILDINGS = new Set<string>(PAPER_BUILDINGS);

/** Datei unter `public/grafik/` eines Gebäudes in Stufe `stufe` (1 = ohne Zusatz, D-15); kein Papier-Gebäude `null` */
export function paperBuildingFile(id: string, stufe = 1): string | null {
  if (!BUILDINGS.has(id)) return null;
  return `${DATA.gebaeude}/${id}/stand${stufe > 1 ? `.s${stufe}` : ''}.png`;
}

export const isPaperFile = (file: string): boolean => file.startsWith(`${DATA.gebaeude}/`) || file.startsWith(`${DATA.hintergruende.gruppe}/`);

/** Hintergrund-Schicht `teil` (`wald` …), `ebene` (`fern`, `mitte`, `nah`), Variante v; Papier kennt weniger Varianten, sie wiederholen sich */
export function paperBackgroundFile(teil: string, ebene: string, v: number): string {
  const n = ((Math.max(1, Math.trunc(v)) - 1) % DATA.hintergruende.varianten) + 1;
  return `${DATA.hintergruende.gruppe}/hg-${teil}-${ebene}-v${n}/stand.png`;
}
