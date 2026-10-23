import { describe, expect, it } from 'vitest';
import { isOpenable } from '../core/shell';
import { DEV_SECTIONS, DEV_TILES } from './devTiles';

/** Werkzeug-Seiten, die bis LP1 auf der Landingpage standen, dazu Dungeon Master (B-335/AC-02) und Grafik-Pipeline (B-331). */
const TOOLS = ['testing.html', 'gamepad-test.html', 'grafiken.html', 'pipeline.html', 'soundtest.html', 'monitor.html', 'dm.html', 'game.html?dev=1'];

describe('Entwicklerseite (B-335)', () => {
  it('gliedert nach Entwicklung, Performance und Balancing', () => {
    expect(DEV_SECTIONS.map((s) => s.title)).toEqual(['Entwicklung', 'Performance', 'Balancing']);
  });

  it('enthält jede Werkzeug-Seite genau einmal', () => {
    const hrefs = DEV_TILES.map((t) => t.href);
    expect([...hrefs].sort()).toEqual([...TOOLS].sort());
  });

  it('Kachel „Spiel (Dev)“ öffnet game.html?dev=1 über die Shell (B-564/AC-01)', () => {
    const tile = DEV_TILES.find((t) => t.href === 'game.html?dev=1');
    expect(tile?.title).toBe('Spiel (Dev)');
    expect(tile?.external).toBeUndefined();
    expect(isOpenable(tile!.href)).toBe(true);
  });

  it('Monitor liegt unter Performance, Balancing ist noch leer und erklärt das', () => {
    const [, perf, balance] = DEV_SECTIONS;
    expect(perf!.tiles.map((t) => t.href)).toEqual(['monitor.html']);
    expect(balance!.tiles).toEqual([]);
    expect(balance!.empty).not.toBe('');
  });

  it('nur Dungeon Master öffnet in einem neuen Fenster (das Asset-Studio ist ein eigenes Fenster, B-507)', () => {
    expect(DEV_TILES.filter((t) => t.external).map((t) => t.href)).toEqual(['dm.html']);
  });
});
