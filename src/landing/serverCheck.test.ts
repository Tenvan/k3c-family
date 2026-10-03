import { describe, expect, it } from 'vitest';
import { PAGES } from './pages';
import { needsServer } from './serverCheck';

describe('Kacheln ohne Server (B-032)', () => {
  it('alle Spiel-Kacheln brauchen den Server (AC-01)', () => {
    const play = PAGES.filter((p) => p.section === 'play');
    expect(play.length).toBeGreaterThan(0);
    expect(play.every(needsServer)).toBe(true);
  });

  it('Testseiten und Infoseiten bleiben ohne Server nutzbar (AC-02)', () => {
    const rest = PAGES.filter((p) => p.section !== 'play');
    expect(rest.map((p) => p.href)).toEqual(expect.arrayContaining(['gamepad-test.html', 'aufstellung.html', 'figuren.html', 'grafiken.html', 'lizenzen.html']));
    expect(rest.some(needsServer)).toBe(false);
  });
});
