import { describe, expect, it } from 'vitest';
import { isOpenable } from '../core/shell';
import { PAGES } from '../landing/pages';
import { SCENARIOS } from './testScenarios';
import { LEVEL_TILES, nextFocus } from './testTiles';

describe('Abschnitt „Level“ der Testseite (B-092/AC-06)', () => {
  it('enthält die Kachel „Level-Betrachter“ mit Ziel leveltest.html', () => {
    expect(LEVEL_TILES.map((t) => [t.title, t.href])).toEqual([['Level-Betrachter', 'leveltest.html']]);
    for (const tile of LEVEL_TILES) expect(tile.description.length).toBeGreaterThan(0);
  });

  it('das Ziel ist eine Seite, die die Shell öffnet, und steht auf der Landingpage', () => {
    for (const tile of LEVEL_TILES) {
      expect(isOpenable(tile.href)).toBe(true);
      expect(PAGES.some((p) => p.href === tile.href)).toBe(true);
    }
  });
});

describe('nextFocus: Steuerkreuz über alle Kacheln der Seite', () => {
  const count = SCENARIOS.length + LEVEL_TILES.length; // Szenarien zuerst, dann der Abschnitt „Level“
  const lastScenario = SCENARIOS.length - 1;
  const levelTile = SCENARIOS.length;

  it('von der letzten Szenarien-Kachel „weiter“ erreicht die Level-Kachel, „zurück“ geht wieder hin', () => {
    expect(nextFocus(count, lastScenario, 'next')).toBe(levelTile);
    expect(nextFocus(count, levelTile, 'prev')).toBe(lastScenario);
  });

  it('an den Rändern bleibt der Fokus stehen', () => {
    expect(nextFocus(count, 0, 'prev')).toBe(0);
    expect(nextFocus(count, count - 1, 'next')).toBe(count - 1);
  });

  it('ungültige Stellen werden auf den Bereich begrenzt, ohne Kacheln bleibt 0', () => {
    expect(nextFocus(count, -3, 'next')).toBe(1);
    expect(nextFocus(count, 99, 'prev')).toBe(count - 2);
    expect(nextFocus(0, 0, 'next')).toBe(0);
  });
});
