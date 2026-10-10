import { describe, expect, it } from 'vitest';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { computeLayout, type Cell } from './layout';
import { anchorRect, DEBUG_AREA, hudItems, hudLayout, overlaps, RANK, type HudItem, type Rect, type Size } from './hudLayout';
import { radarRect } from './radar';

/** Größen wie im Spiel (28 px fett, Innenabstand 8/4 px), kompakt in schmalen Zellen */
function sizes(cells: readonly Cell[]): Map<string, Size> {
  const m = new Map<string, Size>();
  const narrow = cells.some((c) => c.w < GAME_WIDTH);
  cells.forEach((c, i) => {
    if (c.kind === 'player') {
      m.set(`player:${i}`, { w: narrow ? 384 : 704, h: 42 });
      m.set(`skills:${i}`, { w: narrow ? 640 : 832, h: 42 });
    }
    m.set(`radar:${i}`, { w: radarRect(c).w, h: 20 });
  });
  m.set('banner', { w: 640, h: 72 });
  m.set('shared', { w: 416, h: 42 });
  m.set('clock', { w: 352, h: 42 });
  m.set('fight', { w: 288, h: 42 });
  m.set('travel', { w: 512, h: 56 });
  m.set('join', { w: 480, h: 40 });
  m.set('info', { w: 448, h: 38 });
  m.set('controls', { w: 608, h: 38 });
  return m;
}

const HOME: Rect = { x: GAME_WIDTH / 2 - 90, y: 0, w: 180, h: 60 };
const PAUSE: Rect = { x: 12, y: 64, w: 158, h: 64 };
/** `.k3c-touch .grp` in Spielkoordinaten: 87 × 22 vmin; im 16:9-Fenster 1 vmin = 10,8, im 4:3-Fenster 14,4 Spiel-px */
const touch = (vmin: number): Rect => ({ x: GAME_WIDTH - 89 * vmin, y: GAME_HEIGHT - 24 * vmin, w: 87 * vmin, h: 22 * vmin });
const TOUCH: Record<string, Rect[]> = { aus: [], '16:9': [PAUSE, touch(10.8)], '4:3': [PAUSE, touch(14.4)] };

const LAYOUTS: [string, Cell[]][] = [
  ['1', computeLayout(1, false)],
  ['1 mit Partner', computeLayout(1, true)],
  ['2', computeLayout(2, false)],
  ['3', computeLayout(3, false)],
  ['4', computeLayout(4, false)],
];

const CASES = LAYOUTS.flatMap(([name, cells]) =>
  Object.keys(TOUCH).flatMap((touch) => [false, true].map((debug) => ({ name, cells, touch, debug }))),
);

describe('hudLayout (B-337/AC-01)', () => {
  it.each(CASES)('$name Spieler, Touch $touch, Debug $debug: nichts überlagert sich, Pflicht ist da', ({ cells, touch, debug }) => {
    const free = [HOME, ...TOUCH[touch]!, ...(debug ? [DEBUG_AREA] : [])];
    const items = hudItems(cells, sizes(cells), false);
    const out = hudLayout(items, free);
    const placed = [...out.entries()].filter((e): e is [string, Rect] => e[1] !== null);
    for (const [id, r] of placed) {
      for (const f of free) expect(overlaps(r, f), `${id} schneidet Freifläche`).toBe(false);
      for (const [other, q] of placed) if (other !== id) expect(overlaps(r, q), `${id} schneidet ${other}`).toBe(false);
    }
    for (const it of items.filter((i) => i.rank === 0)) expect(out.get(it.id), `Pflicht ${it.id}`).not.toBeNull();
    cells.forEach((c, i) => c.kind === 'player' && expect(out.get(`player:${i}`)).not.toBeNull());
  });

  it('ohne Freifläche stehen die Pflichtanzeigen an ihrem Anker, der gemeinsame Block stapelt sich oben rechts', () => {
    const cells = computeLayout(1, false);
    const items = hudItems(cells, sizes(cells), false);
    const out = hudLayout(items, []);
    for (const it of items.filter((i) => i.rank === 0)) expect(out.get(it.id)).toEqual(anchorRect(it));
    const [shared, clock, fight] = ['shared', 'clock', 'fight'].map((id) => out.get(id)!);
    expect(shared.x + shared.w).toBe(GAME_WIDTH - 24);
    expect(clock.y).toBeGreaterThanOrEqual(shared.y + shared.h);
    expect(fight.y).toBeGreaterThanOrEqual(clock.y + clock.h);
  });

  it('☰ oben links: die Raum-Zeile rückt neben oder unter den Knopf (B-337 › Beispiele)', () => {
    const cells = computeLayout(1, false);
    const items = hudItems(cells, sizes(cells), false);
    const info = hudLayout(items, [HOME, PAUSE]).get('info')!;
    expect(overlaps(info, PAUSE)).toBe(false);
    expect(info).not.toEqual(anchorRect(items.find((i) => i.id === 'info')!));
  });

  it('zu kleine Fläche: Elemente fallen nach Rang weg, Pflicht bleibt', () => {
    const area: Rect = { x: 0, y: 0, w: 400, h: 100 };
    const item = (id: string, rank: number): HudItem => ({ id, area, rank, h: 'left', v: 'top', dx: 0, dy: 0, w: 400, ht: 46 });
    const out = hudLayout([item('hint', RANK.controls), item('room', RANK.info), item('gold', RANK.player)], []);
    expect(out.get('gold')).not.toBeNull();
    expect(out.get('room')).not.toBeNull();
    expect(out.get('hint')).toBeNull();
  });

  it('wartend steht der Beitritts-Hinweis in der Mitte, sonst unten', () => {
    const cells = computeLayout(1, false);
    const s = sizes(cells);
    const mid = hudLayout(hudItems(cells, s, true), []).get('join')!;
    const low = hudLayout(hudItems(cells, s, false), []).get('join')!;
    expect(mid.y + mid.h / 2).toBe(GAME_HEIGHT / 2 + 120);
    expect(low.y).toBeGreaterThan(mid.y);
  });
});
