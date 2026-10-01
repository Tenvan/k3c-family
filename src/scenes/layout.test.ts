import { describe, expect, it } from 'vitest';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { computeLayout, sharedAnchor, SHARED_LINE_HEIGHT, type Cell } from './layout';

const area = (cells: Cell[]) => cells.reduce((sum, c) => sum + c.w * c.h, 0);

describe('computeLayout (B-016)', () => {
  it('1 Spieler: Vollbild, mit Mitspieler oben ein Drittel', () => {
    expect(computeLayout(1, false)).toEqual([{ x: 0, y: 0, w: GAME_WIDTH, h: GAME_HEIGHT, kind: 'player', seat: 0 }]);
    const [top, bottom] = computeLayout(1, true);
    expect(top).toMatchObject({ kind: 'partner', y: 0, h: GAME_HEIGHT / 3 });
    expect(bottom).toMatchObject({ kind: 'player', seat: 0, y: GAME_HEIGHT / 3, h: (GAME_HEIGHT * 2) / 3 });
  });

  it('2 Spieler: zwei Streifen übereinander, Mitspieler eines anderen Geräts ändert nichts', () => {
    const cells = computeLayout(2, false);
    expect(cells.map((c) => [c.seat, c.y, c.h])).toEqual([[0, 0, GAME_HEIGHT / 2], [1, GAME_HEIGHT / 2, GAME_HEIGHT / 2]]);
    expect(computeLayout(2, true)).toEqual(cells);
  });

  it('3 Spieler: zwei oben, einer breit unten, ohne Lücke und Überlappung', () => {
    const cells = computeLayout(3, false);
    expect(cells.map((c) => [c.seat, c.x, c.y, c.w, c.h])).toEqual([
      [0, 0, 0, GAME_WIDTH / 2, GAME_HEIGHT / 2],
      [1, GAME_WIDTH / 2, 0, GAME_WIDTH / 2, GAME_HEIGHT / 2],
      [2, 0, GAME_HEIGHT / 2, GAME_WIDTH, GAME_HEIGHT / 2],
    ]);
    expect(cells.every((c) => c.kind === 'player')).toBe(true);
    expect(area(cells)).toBe(GAME_WIDTH * GAME_HEIGHT);
  });

  it('4 Spieler: 2×2-Raster aus Vierteln ohne Lücke und Überlappung', () => {
    const cells = computeLayout(4, true);
    expect(cells.map((c) => [c.x, c.y])).toEqual([[0, 0], [GAME_WIDTH / 2, 0], [0, GAME_HEIGHT / 2], [GAME_WIDTH / 2, GAME_HEIGHT / 2]]);
    expect(cells.every((c) => c.kind === 'player' && c.w === GAME_WIDTH / 2 && c.h === GAME_HEIGHT / 2)).toBe(true);
    expect(area(cells)).toBe(GAME_WIDTH * GAME_HEIGHT);
  });

  it('0 und mehr als 4 werden auf 1 bis 4 begrenzt', () => {
    expect(computeLayout(0, false)).toHaveLength(1);
    expect(computeLayout(9, false)).toHaveLength(4);
  });
});

describe('sharedAnchor (B-084)', () => {
  it('Vollbild und Streifen: oben rechts', () => {
    for (const n of [1, 2]) expect(sharedAnchor(computeLayout(n, false))).toEqual({ x: GAME_WIDTH - 24, y: 16, originX: 1 });
  });

  it.each([3, 4])('%i Spieler: mittig am Kreuzpunkt, nicht in einer Feldecke', (n) => {
    const a = sharedAnchor(computeLayout(n, false));
    expect(a.x).toBe(GAME_WIDTH / 2);
    expect(a.originX).toBe(0.5);
    expect(a.y + (SHARED_LINE_HEIGHT * 3) / 2).toBe(GAME_HEIGHT / 2);
  });
});
