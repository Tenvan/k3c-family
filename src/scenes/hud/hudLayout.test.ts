import { describe, expect, it } from 'vitest';
import { GAME_HEIGHT, GAME_WIDTH } from '../../core/constants';
import { BAR_HEIGHT, computeLayout, type Cell } from '../layout';
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
  m.set('banner', { w: 640, h: 42 });
  m.set('shared', { w: 416, h: 42 });
  m.set('clock', { w: 352, h: 42 });
  m.set('fight', { w: 288, h: 42 });
  m.set('citizens', { w: 448, h: 42 });
  m.set('places', { w: 448, h: 120 });
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

  it('ohne Freifläche stehen die Pflichtanzeigen an ihrem Anker, alle gemeinsamen Anzeigen und Spielerwerte in der Leiste oben', () => {
    for (const cells of [computeLayout(1, false), computeLayout(4, false)]) {
      const items = hudItems(cells, sizes(cells), false);
      const out = hudLayout(items, []);
      if (cells.length === 1) for (const it of items.filter((i) => i.rank === 0 && i.id !== 'banner')) expect(out.get(it.id)).toEqual(anchorRect(it)); // die Meldung weicht der Spielerkarte aus (B-414)
      const [info, clock, fight, shared] = ['info', 'clock', 'fight', 'shared'].map((id) => out.get(id)!);
      expect(info.x).toBe(24);
      expect(fight.x + fight.w).toBe(GAME_WIDTH - 24);
      expect(info.x + info.w).toBeLessThanOrEqual(clock.x);
      expect(clock.x + clock.w).toBeLessThanOrEqual(fight.x);
      expect(shared.y).toBeGreaterThanOrEqual(info.y + info.h);
      const bar = { x: 0, y: 0, w: GAME_WIDTH, h: BAR_HEIGHT };
      for (const id of ['info', 'clock', 'fight', 'shared', ...cells.map((_, i) => `player:${i}`)]) {
        const r = out.get(id)!;
        expect(r.x >= bar.x && r.y >= bar.y && r.x + r.w <= bar.w && r.y + r.h <= bar.h, `${id} in der Leiste`).toBe(true);
      }
    }
  });

  it('Spielerzeilen stehen nebeneinander in der Leiste, nicht in den Feldern', () => {
    const cells = computeLayout(2, false);
    const out = hudLayout(hudItems(cells, sizes(cells), false), []);
    const [a, b] = [out.get('player:0')!, out.get('player:1')!];
    expect(a.y).toBeLessThan(BAR_HEIGHT);
    expect(b.y).toBeLessThan(BAR_HEIGHT);
    expect(overlaps(a, b)).toBe(false);
  });

  it('☰ oben links: der Vorrat in der zweiten Zeile der Leiste rückt neben den Knopf (B-337 › Beispiele)', () => {
    const cells = computeLayout(1, false);
    const items = hudItems(cells, sizes(cells), false);
    const shared = hudLayout(items, [HOME, PAUSE]).get('shared')!;
    expect(overlaps(shared, PAUSE)).toBe(false);
    expect(shared).not.toEqual(anchorRect(items.find((i) => i.id === 'shared')!));
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

describe('Welt-Meldungen im Band (B-414/AC-01)', () => {
  it.each([
    ['1', computeLayout(1, false)],
    ['1 mit Partner', computeLayout(1, true)],
    ['2', computeLayout(2, false)],
  ])('%s Spieler: die Meldung steht innerhalb der Leiste und überlagert keine Leistenzeile', (_name, cells) => {
    const out = hudLayout(hudItems(cells, sizes(cells), false), [HOME]);
    const banner = out.get('banner')!;
    expect(banner).not.toBeNull();
    expect(banner.y).toBeGreaterThanOrEqual(0);
    expect(banner.y + banner.h).toBeLessThanOrEqual(BAR_HEIGHT);
    for (const id of ['info', 'clock', 'fight', 'shared', 'citizens', 'places']) {
      const other = out.get(id);
      if (other) expect(overlaps(banner, other), `banner schneidet ${id}`).toBe(false);
    }
    cells.forEach((c, i) => {
      const card = out.get(`player:${i}`);
      if (c.kind === 'player' && card) expect(overlaps(banner, card), `banner schneidet Spielerkarte ${i}`).toBe(false);
    });
  });
});

describe('Gemeinsame Anzeigen nur in der Leiste, Kompass je Feld oben (B-413/AC-01, AC-02)', () => {
  const SHARED = ['info', 'clock', 'fight', 'shared', 'citizens', 'places'];

  it.each(LAYOUTS)('%s Spieler: Vorrat, Hub, Tageszeit, Kampf in der Leiste, in keinem Spielerfeld (AC-01)', (_name, cells) => {
    const out = hudLayout(hudItems(cells, sizes(cells), false), [HOME]);
    for (const id of SHARED) {
      const r = out.get(id);
      if (!r) continue;
      expect(r.y + r.h, `${id} in der Leiste`).toBeLessThanOrEqual(BAR_HEIGHT);
      for (const c of cells) expect(overlaps(r, c), `${id} schneidet ein Feld`).toBe(false);
    }
    for (const c of cells) expect(c.y, 'Felder beginnen unter der Leiste').toBeGreaterThanOrEqual(BAR_HEIGHT);
  });

  it.each(LAYOUTS)('%s Spieler: der Kompass steht oben in seinem Feld, ohne Überlagerung (AC-02)', (_name, cells) => {
    const out = hudLayout(hudItems(cells, sizes(cells), false), [HOME]);
    const placed = [...out.entries()].filter((e): e is [string, Rect] => e[1] !== null);
    cells.forEach((c, i) => {
      const r = out.get(`radar:${i}`);
      expect(r, `Kompass ${i}`).toBeTruthy();
      expect(r!.x >= c.x && r!.x + r!.w <= c.x + c.w, 'Kompass im Feld').toBe(true);
      expect(r!.y).toBeGreaterThanOrEqual(c.y); // im obersten Feld damit unter der Leiste
      expect(r!.y + r!.h).toBeLessThanOrEqual(c.y + c.h / 3); // im oberen Drittel
      for (const [id, q] of placed) if (id !== `radar:${i}`) expect(overlaps(r!, q), `Kompass ${i} schneidet ${id}`).toBe(false);
    });
  });
});
