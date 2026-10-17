import { describe, expect, it } from 'vitest';
import { SCENARIOS } from './testScenarios';
import { nextFocus } from './testTiles';

describe('nextFocus: Steuerkreuz über alle Szenarien-Kacheln der Seite', () => {
  const count = SCENARIOS.length;

  it('„weiter“ und „zurück“ wandern um eine Kachel', () => {
    expect(nextFocus(count, 0, 'next')).toBe(1);
    expect(nextFocus(count, 1, 'prev')).toBe(0);
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
