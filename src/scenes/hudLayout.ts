import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { SHARED_LINE_HEIGHT, sharedAnchor, type Cell } from './layout';
import { RADAR_BOTTOM_OFFSET, RADAR_HEIGHT } from './radar';

/**
 * Layout der HUD-Elemente (B-337): reine Rechnung ohne Phaser und DOM. Jedes Element hat einen Anker in seiner Fläche
 * (Zelle oder Bildschirm) und einen Rang; die Funktion legt sie ohne Überschneidung untereinander und mit den
 * Freiflächen ab. Was nicht passt, fällt nach Rang weg, Rang 0 (Pflicht) nie.
 */

export interface Rect {
  x: number;
  y: number;
  w: number;
  h: number;
}

export type HAlign = 'left' | 'center' | 'right';
export type VAlign = 'top' | 'middle' | 'bottom';

export interface HudItem {
  id: string;
  /** Fläche, in der das Element bleiben muss: Zelle oder Bildschirm */
  area: Rect;
  h: HAlign;
  v: VAlign;
  /** Abstand vom Rand der Fläche (links/rechts/oben/unten), bei `center`/`middle` Verschiebung aus der Mitte */
  dx: number;
  dy: number;
  w: number;
  ht: number;
  /** 0 = Pflicht (nie ausgeblendet), größer = fällt früher weg */
  rank: number;
}

/** Bereich der Diagnose links unten (`debugOverlayView.ts`, Text bis `GAME_HEIGHT - 70`, gemessen in U5.1): Freifläche, solange sie sichtbar ist */
export const DEBUG_AREA: Rect = { x: 20, y: 780, w: 570, h: 230 };

/** Abstand zwischen Elementen und zu Freiflächen */
export const GAP = 8;

/** Rangfolge laut 🧑 (2026-10-10): Pflicht Spielerzeile, Meldung, Skill-Leiste; der Steuerhinweis fällt zuerst weg. */
export const RANK = { player: 0, banner: 0, skills: 0, shared: 1, travel: 2, join: 3, radar: 4, info: 5, controls: 6 } as const;

export const overlaps = (a: Rect, b: Rect): boolean => a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h;

const inside = (r: Rect, a: Rect): boolean => r.x >= a.x && r.y >= a.y && r.x + r.w <= a.x + a.w && r.y + r.h <= a.y + a.h;

/** Wunschlage eines Elements an seinem Anker */
export function anchorRect(it: HudItem): Rect {
  const { area: a, w, ht } = it;
  const x = it.h === 'left' ? a.x + it.dx : it.h === 'right' ? a.x + a.w - w - it.dx : a.x + (a.w - w) / 2 + it.dx;
  const y = it.v === 'top' ? a.y + it.dy : it.v === 'bottom' ? a.y + a.h - ht - it.dy : a.y + (a.h - ht) / 2 + it.dy;
  return { x, y, w, h: ht };
}

/**
 * Nächste freie Lage zur Wunschlage: Kandidaten sind die Wunschlage und die Lagen direkt neben jedem Hindernis
 * (rechts, links, darunter, darüber); gewählt wird die mit dem kürzesten Weg. Gleicher Anker stapelt sich so von selbst.
 * ponytail: O(n²) Kandidaten je Element, bei ~40 Elementen und Neuberechnung nur bei Änderung genug.
 */
function place(it: HudItem, obstacles: readonly Rect[]): Rect | null {
  const base = anchorRect(it);
  const xs = [base.x, ...obstacles.flatMap((o) => [o.x + o.w + GAP, o.x - GAP - it.w])];
  const ys = [base.y, ...obstacles.flatMap((o) => [o.y + o.h + GAP, o.y - GAP - it.ht])];
  let best: Rect | null = null;
  let bestD = Infinity;
  for (const x of xs) {
    for (const y of ys) {
      const r = { x, y, w: it.w, h: it.ht };
      const d = Math.abs(x - base.x) + Math.abs(y - base.y);
      if (d >= bestD || !inside(r, it.area) || obstacles.some((o) => overlaps(r, o))) continue;
      best = r;
      bestD = d;
    }
  }
  return best;
}

/** Lage je Element-Id, `null` = ausgeblendet. Höherer Rang (kleinere Zahl) zuerst, bei Gleichstand die Reihenfolge der Liste. */
export function hudLayout(items: readonly HudItem[], free: readonly Rect[]): Map<string, Rect | null> {
  const taken: Rect[] = [...free];
  const out = new Map<string, Rect | null>();
  const order = items.map((it, i) => ({ it, i })).sort((a, b) => a.it.rank - b.it.rank || a.i - b.i);
  for (const { it } of order) {
    // ponytail: passt eine Pflichtanzeige nirgends, steht sie an ihrem Anker; die Tests zeigen, dass das in keinem Layout vorkommt
    const r = place(it, taken) ?? (it.rank === 0 ? anchorRect(it) : null);
    out.set(it.id, r);
    if (r) taken.push(r);
  }
  return out;
}

export interface Size {
  w: number;
  h: number;
}

const SCREEN: Rect = { x: 0, y: 0, w: GAME_WIDTH, h: GAME_HEIGHT };

/** Lage der Radar-Leiste in ihrer Zelle: Vollbreite mittig, in schmalen Zellen am äußeren Rand (B-090) */
function radarAnchor(cell: Cell): Pick<HudItem, 'h' | 'dx'> {
  if (cell.w >= GAME_WIDTH) return { h: 'center', dx: 0 };
  return { h: cell.x < GAME_WIDTH / 2 ? 'left' : 'right', dx: 24 };
}

/**
 * Die HUD-Elemente mit Anker und Rang. `sizes` nennt die Größe je Id (`player:<i>`, `skills:<i>`, `radar:<i>` mit
 * Zellen-Index `i`, sonst `shared`, `clock`, `fight`, `banner`, `join`, `travel`, `info`, `controls`); fehlt eine Id,
 * wird das Element gerade nicht gezeigt. `waiting`: der Beitritts-Hinweis steht groß in der Mitte.
 */
export function hudItems(cells: readonly Cell[], sizes: ReadonlyMap<string, Size>, waiting: boolean): HudItem[] {
  const items: HudItem[] = [];
  const add = (id: string, area: Rect, rank: number, a: Pick<HudItem, 'h' | 'v' | 'dx' | 'dy'>) => {
    const s = sizes.get(id);
    if (s) items.push({ id, area, rank, ...a, w: s.w, ht: s.h });
  };
  cells.forEach((cell, i) => {
    add(`player:${i}`, cell, RANK.player, { h: 'left', v: 'top', dx: 24, dy: 16 });
    add(`skills:${i}`, cell, RANK.skills, { h: 'left', v: 'bottom', dx: 24, dy: 24 });
    add(`radar:${i}`, cell, RANK.radar, { ...radarAnchor(cell), v: 'bottom', dy: RADAR_BOTTOM_OFFSET - RADAR_HEIGHT });
  });
  add('banner', SCREEN, RANK.banner, { h: 'center', v: 'middle', dx: 0, dy: 0 });
  const corner = sharedAnchor(cells).originX === 1; // B-084: oben rechts, bei 3 und 4 Spielern mittig am Kreuzpunkt
  ['shared', 'clock', 'fight'].forEach((id, line) =>
    add(id, SCREEN, RANK.shared, corner ? { h: 'right', v: 'top', dx: 24, dy: 16 + line * SHARED_LINE_HEIGHT } : { h: 'center', v: 'middle', dx: 0, dy: (line - 1) * SHARED_LINE_HEIGHT }),
  );
  add('travel', SCREEN, RANK.travel, { h: 'center', v: 'bottom', dx: 0, dy: 150 });
  add('join', SCREEN, RANK.join, waiting ? { h: 'center', v: 'middle', dx: 0, dy: 120 } : { h: 'center', v: 'bottom', dx: 0, dy: 85 });
  add('info', SCREEN, RANK.info, { h: 'left', v: 'top', dx: 20, dy: 60 });
  add('controls', SCREEN, RANK.controls, { h: 'right', v: 'bottom', dx: 20, dy: 10 });
  return items;
}
