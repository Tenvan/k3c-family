import { describe, expect, it } from 'vitest';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { computeLayout, sharedAnchor, SHARED_LINE_HEIGHT, type Cell } from './layout';
import { radarMarkers, radarRect, RADAR_HEIGHT, RADAR_MAX_WIDTH, TOP_STRIP, type RadarWorld } from './radar';

/** Kleines Level: 1000 Units breit, Burg in der Mitte, zwei Portale, ein Ausgang. */
function world(over: Partial<RadarWorld> = {}): RadarWorld {
  return {
    widthUnits: 1000,
    castle: { x: 500 },
    portals: [100, 900],
    sites: [],
    players: [{ index: 0, x: 500, respawnIn: 0 }],
    enemies: [],
    level: { entities: [{ kind: 'tree', x: 40 }, { kind: 'exit', x: 1000 }] },
    ...over,
  };
}
const view = { fromUnits: 450, spanUnits: 100 };
const of = (kind: string, w: RadarWorld = world(), self: number | null = 0) => radarMarkers(w, view, self).markers.filter((m) => m.kind === kind);

describe('radarMarkers (B-090)', () => {
  it('Burg, Portale und Ausgang stehen an ihrer Lage 0..1', () => {
    expect(of('castle').map((m) => m.pos)).toEqual([0.5]);
    expect(of('portal').map((m) => m.pos)).toEqual([0.1, 0.9]);
    expect(of('exit').map((m) => m.pos)).toEqual([1]);
  });

  it('Level ohne Portale hat keine Portal-Marker', () => {
    expect(of('portal', world({ portals: [] }))).toEqual([]);
  });

  it('Treppen (hoch und runter) werden zu Marken, andere Bauplätze nicht', () => {
    const sites = [{ kind: 'stairsUp' as const, x: 250 }, { kind: 'stairsDown' as const, x: 750 }, { kind: 'wall' as const, x: 300 }];
    expect(of('stairs', world({ sites })).map((m) => m.pos)).toEqual([0.25, 0.75]);
  });

  it('Gegner sind einzelne Marker, an den Rändern 0 und 1, außerhalb begrenzt', () => {
    const enemies = [{ x: 0 }, { x: 1000 }, { x: -20 }, { x: 1300 }, { x: 500 }];
    expect(of('enemy', world({ enemies })).map((m) => m.pos)).toEqual([0, 1, 0, 1, 0.5]);
  });

  it('zwei Spieler: beide mit Index, nur der des Feldes ist eigen', () => {
    const players = [{ index: 0, x: 200, respawnIn: 0 }, { index: 1, x: 800, respawnIn: 0 }];
    const w = world({ players });
    expect(of('player', w, 0).map((m) => [m.index, m.pos, m.own])).toEqual([[0, 0.2, true], [1, 0.8, false]]);
    expect(of('player', w, 1).map((m) => [m.index, m.own])).toEqual([[0, false], [1, true]]);
  });

  it('Feld des Mitspielers (self = null): kein Spieler ist eigen', () => {
    const players = [{ index: 0, x: 200, respawnIn: 0 }, { index: 1, x: 800, respawnIn: 0 }];
    expect(of('player', world({ players }), null).every((m) => m.own === false)).toBe(true);
  });

  it('gefallener Spieler bleibt als Marker, aber down', () => {
    const players = [{ index: 0, x: 300, respawnIn: 4.2 }, { index: 1, x: 700, respawnIn: 0 }];
    expect(of('player', world({ players })).map((m) => m.down)).toEqual([true, false]);
  });

  it('Monarchen werden zuletzt gezeichnet, Gegner davor', () => {
    const kinds = radarMarkers(world({ enemies: [{ x: 10 }] }), view, 0).markers.map((m) => m.kind);
    expect(kinds[kinds.length - 1]).toBe('player');
    expect(kinds.indexOf('enemy')).toBeLessThan(kinds.indexOf('player'));
  });

  it('Sichtfenster in 0..1 und an den Levelrändern begrenzt', () => {
    expect(radarMarkers(world(), { fromUnits: 450, spanUnits: 100 }, 0).view).toEqual({ from: 0.45, to: 0.55 });
    expect(radarMarkers(world(), { fromUnits: -50, spanUnits: 2000 }, 0).view).toEqual({ from: 0, to: 1 });
  });

  it('Level der Breite 0 rechnet nicht durch null', () => {
    const m = radarMarkers(world({ widthUnits: 0 }), view, 0);
    expect(m.markers.every((x) => x.pos === 0)).toBe(true);
    expect(m.view).toEqual({ from: 0, to: 0 });
  });
});

describe('radarRect (B-090/AC-02)', () => {
  const layouts: [string, Cell[]][] = [
    ['1 Spieler', computeLayout(1, false)],
    ['1 Spieler mit Partner', computeLayout(1, true)],
    ['2 Spieler', computeLayout(2, false)],
    ['3 Spieler', computeLayout(3, false)],
    ['4 Spieler', computeLayout(4, false)],
    ['4 Spieler mit Partner', computeLayout(4, true)],
  ];

  it.each(layouts)('%s: jede Leiste liegt in ihrer Zelle, unter dem Home-Button-Streifen', (_name, cells) => {
    for (const cell of cells) {
      const r = radarRect(cell);
      expect(r.x).toBeGreaterThanOrEqual(cell.x);
      expect(r.x + r.w).toBeLessThanOrEqual(cell.x + cell.w);
      expect(r.y).toBeGreaterThanOrEqual(cell.y);
      expect(r.y + r.h).toBeLessThanOrEqual(cell.y + cell.h);
      expect(r.y).toBeGreaterThanOrEqual(TOP_STRIP);
      expect(r.y + r.h).toBeLessThanOrEqual(GAME_HEIGHT);
      expect(r.w).toBeLessThanOrEqual(RADAR_MAX_WIDTH);
      expect(r.h).toBe(RADAR_HEIGHT);
    }
  });

  it.each(layouts)('%s: Leisten verschiedener Zellen überlappen sich nicht', (_name, cells) => {
    const rects = cells.map(radarRect);
    rects.forEach((a, i) =>
      rects.slice(i + 1).forEach((b) => {
        const apart = a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
        expect(apart).toBe(true);
      }),
    );
  });

  it('Vollbild: halbe Breite, mittig, höchstens 640 px', () => {
    expect(radarRect({ x: 0, y: 0, w: GAME_WIDTH, h: GAME_HEIGHT })).toEqual({ x: (GAME_WIDTH - RADAR_MAX_WIDTH) / 2, y: GAME_HEIGHT - 68, w: RADAR_MAX_WIDTH, h: RADAR_HEIGHT });
  });

  it('schmale Felder: Leiste rückt an den äußeren Bildschirmrand', () => {
    const [tl, tr] = computeLayout(4, false);
    expect(radarRect(tl).x).toBe(24);
    expect(radarRect(tr).x + radarRect(tr).w).toBe(GAME_WIDTH - 24);
  });

  /** Der gemeinsame HUD-Block (Vorrat, Tageszeit, Kampf) ist hier großzügig 900 px breit angesetzt. */
  it.each(layouts)('%s: Leisten überlappen den gemeinsamen HUD-Block nicht', (_name, cells) => {
    const a = sharedAnchor(cells);
    const block = { x: a.x - a.originX * 900, y: a.y, w: 900, h: 3 * SHARED_LINE_HEIGHT };
    for (const cell of cells) {
      const r = radarRect(cell);
      const apart = r.x + r.w <= block.x || block.x + block.w <= r.x || r.y + r.h <= block.y || block.y + block.h <= r.y;
      expect(apart).toBe(true);
    }
  });
});
