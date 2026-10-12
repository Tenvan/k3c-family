import { describe, expect, it } from 'vitest';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { BAR_HEIGHT, computeLayout, type Cell } from './layout';

/** Höhe der Felder unter der Leiste oben */
const H = GAME_HEIGHT - BAR_HEIGHT;

const area = (cells: Cell[]) => cells.reduce((sum, c) => sum + c.w * c.h, 0);

describe('computeLayout (B-016)', () => {
  it('1 Spieler: ganzes Feld unter der Leiste, mit Mitspieler oben ein Drittel', () => {
    expect(computeLayout(1, false)).toEqual([{ x: 0, y: BAR_HEIGHT, w: GAME_WIDTH, h: H, kind: 'player', seat: 0 }]);
    const [top, bottom] = computeLayout(1, true);
    expect(top).toMatchObject({ kind: 'partner', y: BAR_HEIGHT, h: H / 3 });
    expect(bottom).toMatchObject({ kind: 'player', seat: 0, y: BAR_HEIGHT + H / 3, h: (H * 2) / 3 });
  });

  it('2 Spieler: zwei Streifen übereinander, Mitspieler eines anderen Geräts ändert nichts', () => {
    const cells = computeLayout(2, false);
    expect(cells.map((c) => [c.seat, c.y, c.h])).toEqual([[0, BAR_HEIGHT, H / 2], [1, BAR_HEIGHT + H / 2, H / 2]]);
    expect(computeLayout(2, true)).toEqual(cells);
  });

  it('3 Spieler: zwei oben, einer breit unten, ohne Lücke und Überlappung', () => {
    const cells = computeLayout(3, false);
    expect(cells.map((c) => [c.seat, c.x, c.y, c.w, c.h])).toEqual([
      [0, 0, BAR_HEIGHT, GAME_WIDTH / 2, H / 2],
      [1, GAME_WIDTH / 2, BAR_HEIGHT, GAME_WIDTH / 2, H / 2],
      [2, 0, BAR_HEIGHT + H / 2, GAME_WIDTH, H / 2],
    ]);
    expect(cells.every((c) => c.kind === 'player')).toBe(true);
    expect(area(cells)).toBe(GAME_WIDTH * H);
  });

  it('4 Spieler: 2×2-Raster aus Vierteln ohne Lücke und Überlappung', () => {
    const cells = computeLayout(4, true);
    expect(cells.map((c) => [c.x, c.y])).toEqual([[0, BAR_HEIGHT], [GAME_WIDTH / 2, BAR_HEIGHT], [0, BAR_HEIGHT + H / 2], [GAME_WIDTH / 2, BAR_HEIGHT + H / 2]]);
    expect(cells.every((c) => c.kind === 'player' && c.w === GAME_WIDTH / 2 && c.h === H / 2)).toBe(true);
    expect(area(cells)).toBe(GAME_WIDTH * H);
  });

  it('0 und mehr als 4 werden auf 1 bis 4 begrenzt', () => {
    expect(computeLayout(0, false)).toHaveLength(1);
    expect(computeLayout(9, false)).toHaveLength(4);
  });
});
