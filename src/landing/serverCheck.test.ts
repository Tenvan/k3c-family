import { describe, expect, it } from 'vitest';
import { PAGES } from './pages';
import { needsServer } from './serverCheck';

describe('Kacheln ohne Server (B-032)', () => {
  it('alle Spiel-Kacheln brauchen den Server (AC-01)', () => {
    const play = PAGES.filter((p) => p.section === 'play');
    expect(play.length).toBeGreaterThan(0);
    expect(play.every(needsServer)).toBe(true);
  });

  it('Infoseiten und der Weg zur Entwicklerseite bleiben ohne Server nutzbar (AC-02)', () => {
    const rest = PAGES.filter((p) => p.section !== 'play');
    expect(rest.map((p) => p.href)).toEqual(expect.arrayContaining(['lizenzen.html', 'dev.html']));
    expect(rest.some(needsServer)).toBe(false);
  });
});
