import type { WorldDetail, WorldIsland } from '../api';
import type { LevelModel } from './levelView';

// Querschnitt des Welten-Editors (B-963): reine Funktionen für Etappen-Gruppen, Biom-Farben, Streifen-Breite und Marker.
// Kein DOM, kein Netz. Die Etappe einer Insel steht optional in der Welt-Datei; fehlt sie, gilt Listenplatz + 1.

/** Biom-Palette (data/biomes/<id>.json › palette) */
export interface Palette {
  sky: string;
  far: string;
  near: string;
  ground: string;
}

/** Paletten der Biome; unbekannte Biome bekommen die neutrale Palette. */
const PALETTES: Record<string, Palette> = {
  forest: { sky: '#8ecae6', far: '#6a8caf', near: '#3a5a40', ground: '#588157' },
  cave: { sky: '#35354d', far: '#2e2e3a', near: '#44444f', ground: '#5c5c66' },
  mine: { sky: '#2b1a12', far: '#4a2f1d', near: '#7a4a2a', ground: '#8c5a3c' },
  ironhold: { sky: '#1f0f0a', far: '#3a1a10', near: '#5a2a18', ground: '#6b3a26' },
  crystal: { sky: '#0d1626', far: '#1a2a44', near: '#2a4466', ground: '#3a5a80' },
};
const NEUTRAL: Palette = { sky: '#9aa3ad', far: '#848d97', near: '#6f7882', ground: '#5a636d' };

export const paletteOf = (biomeId: string): Palette => PALETTES[biomeId] ?? NEUTRAL;

/** Himmel, zwei Hintergrundbänder und Boden als Verlauf von oben nach unten. */
export const gradientOf = (p: Palette): string =>
  `linear-gradient(${p.sky} 0 45%, ${p.far} 45% 65%, ${p.near} 65% 85%, ${p.ground} 85%)`;

/** Etappe einer Insel: das Feld `etappe`, sonst Listenplatz + 1. */
export const etappeOf = (island: WorldIsland, index: number): number => island.etappe ?? index + 1;

export interface EtappeIsland {
  island: WorldIsland;
  /** Listenplatz in der Welt (0-basiert) */
  index: number;
}

export interface EtappeGroup {
  etappe: number;
  islands: EtappeIsland[];
}

/** Inseln nach Etappe gruppiert, Etappen aufsteigend, Inseln in Listenreihenfolge. */
export function groupEtappen(detail: WorldDetail): EtappeGroup[] {
  const groups = new Map<number, EtappeIsland[]>();
  detail.islands.forEach((island, index) => {
    const n = etappeOf(island, index);
    groups.set(n, [...(groups.get(n) ?? []), { island, index }]);
  });
  return [...groups].sort((a, b) => a[0] - b[0]).map(([etappe, islands]) => ({ etappe, islands }));
}

/** Text des Bandes: „Etappe 2 · 2 Inseln · frei wählbar · 🔒 nach Etappe 1“. */
export function bandText(g: EtappeGroup, previous: EtappeGroup | undefined): string {
  const n = g.islands.length;
  const parts = [`Etappe ${g.etappe}`, n === 1 ? '1 Insel' : `${n} Inseln`];
  if (n > 1) parts.push('frei wählbar');
  parts.push(previous ? `🔒 nach Etappe ${previous.etappe}` : '🔓 ab Spielstart');
  return parts.join(' · ');
}

/** Anker der Insel im Querschnitt (Ziel der Seekarte). */
export const islandAnchor = (islandId: string): string => `insel-${islandId}`;

/** Schlüssel eines Levels in der Tabelle der geladenen Stufen. */
export const stageId = (island: string, stage: number): string => `${island}:${stage}`;

/** Breite eines Streifens in Prozent der längsten Stufe aller Inseln (mindestens 1). */
export function widthPct(model: LevelModel, models: ReadonlyMap<string, LevelModel>): number {
  let max = 1;
  for (const m of models.values()) max = Math.max(max, m.widthUnits);
  return Math.max(1, (model.widthUnits / max) * 100);
}

export type MarkKind = 'castle' | 'portal' | 'stairs' | 'boss' | 'harbor';

/** Marker im Streifen; Lage und Breite in Prozent der Streifenbreite. */
export interface Mark {
  kind: MarkKind;
  left: number;
  width?: number;
  title: string;
}

/** Marker eines Streifens: Burg (Hub-Chunks), Portale, Hafen-Chunks und Ausgang (Treppe, in der tiefsten Stufe Endboss). */
export function marksOf(model: LevelModel, deepest: boolean): Mark[] {
  const w = model.widthUnits || 1;
  const pct = (x: number) => (x / w) * 100;
  const marks: Mark[] = [];
  const hubs = model.chunks.filter((c) => c.kind === 'hub');
  if (hubs.length > 0) {
    const from = hubs[0]!.from;
    const to = hubs[hubs.length - 1]!.to;
    marks.push({ kind: 'castle', left: pct(from), width: pct(to - from), title: 'Hub / Burg' });
  }
  for (const c of model.chunks.filter((k) => k.kind === 'harbor')) {
    marks.push({ kind: 'harbor', left: pct((c.from + c.to) / 2), title: `Hafen-Chunk ${c.index}` });
  }
  for (const o of model.objects) {
    if (o.kind === 'portal') marks.push({ kind: 'portal', left: pct(o.x), title: 'Portal' });
    if (o.kind === 'exit') marks.push({ kind: deepest ? 'boss' : 'stairs', left: pct(o.x), title: deepest ? 'Endboss-Bau' : 'Treppe nach unten' });
  }
  return marks;
}

export const MARK_ICON: Record<MarkKind, string> = { castle: '🏰', portal: '🌀', stairs: '⬇', boss: '👹', harbor: '⚓' };
