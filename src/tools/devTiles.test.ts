import { describe, expect, it } from 'vitest';
import { DEV_SECTIONS, DEV_TILES } from './devTiles';

/** Werkzeug-Seiten, die bis LP1 auf der Landingpage standen, dazu Dungeon Master (B-335/AC-02). */
const TOOLS = ['testing.html', 'leveltest.html', 'gamepad-test.html', 'aufstellung.html', 'figuren.html', 'grafiken.html', 'soundtest.html', 'monitor.html', 'dm.html'];

describe('Entwicklerseite (B-335)', () => {
  it('gliedert nach Entwicklung, Performance und Balancing', () => {
    expect(DEV_SECTIONS.map((s) => s.title)).toEqual(['Entwicklung', 'Performance', 'Balancing']);
  });

  it('enthält jede Werkzeug-Seite genau einmal', () => {
    const hrefs = DEV_TILES.map((t) => t.href);
    expect([...hrefs].sort()).toEqual([...TOOLS].sort());
  });

  it('Monitor liegt unter Performance, Balancing ist noch leer und erklärt das', () => {
    const [, perf, balance] = DEV_SECTIONS;
    expect(perf!.tiles.map((t) => t.href)).toEqual(['monitor.html']);
    expect(balance!.tiles).toEqual([]);
    expect(balance!.empty).not.toBe('');
  });

  it('nur Dungeon Master öffnet in einem neuen Fenster', () => {
    expect(DEV_TILES.filter((t) => t.external).map((t) => t.href)).toEqual(['dm.html']);
  });
});
