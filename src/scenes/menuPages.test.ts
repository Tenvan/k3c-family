import { afterEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { DEV_TILES } from '../tools/devTiles';
import { categoryLabel, categorySub, menuCategories, runParamsOf, targetOf, uniqueRun, type CategoryId } from './menuPages';
import { GRID_MAX, HUB_LAYOUT, gridTiles, moveHub, type HubTile } from './pauseHub';

afterEach(() => setLanguage('de'));

const DIRS = ['up', 'down', 'left', 'right'] as const;

/** Zahl der Kacheln, die ab der ersten mit Richtungsdrücken erreichbar sind */
function reachable(tiles: readonly HubTile<string>[]): number {
  const seen = new Set([0]);
  const queue = [0];
  while (queue.length > 0) {
    const from = queue.shift()!;
    for (const to of DIRS.map((dir) => moveHub(tiles, from, dir)).filter((i) => !seen.has(i))) {
      seen.add(to);
      queue.push(to);
    }
  }
  return seen.size;
}

const all = menuCategories(true).flatMap((c) => c.entries);

describe('Ebenen Seiten (B-768/AC-02)', () => {
  it('vier Kategorien in fester Reihenfolge; ohne Dev-Menü nur Info und Hilfe', () => {
    expect(menuCategories(true).map((c) => c.id)).toEqual(['info', 'dev', 'perf', 'balance']);
    expect(menuCategories(false).map((c) => c.id)).toEqual(['info']);
  });

  it('jede Kachel der Werkzeugseite und die Lizenzen haben genau einen Eintrag', () => {
    const ids = all.map((e) => e.id);
    expect(new Set(ids).size).toBe(ids.length);
    expect(ids.sort()).toEqual([...DEV_TILES.map((tile) => tile.href), 'lizenzen.html'].sort());
  });

  it('Gamepad-Test steht unter Info und Hilfe, nicht unter Entwicklung', () => {
    const info = menuCategories(true).find((c) => c.id === 'info')!;
    expect(info.entries.map((e) => e.id)).toEqual(['lizenzen.html', 'gamepad-test.html']);
    expect(menuCategories(true).find((c) => c.id === 'dev')!.entries.some((e) => e.id === 'gamepad-test.html')).toBe(false);
  });

  it('Performance enthält Monitor, Stress Live-Grafik, Textur-Grenze und die drei Stress-echt-Szenarien', () => {
    const perf = menuCategories(true).find((c) => c.id === 'perf')!;
    expect(perf.entries.map((e) => e.target.kind)).toEqual(['page', 'page', 'page', 'run', 'run', 'run']);
  });

  it('jeder Eintrag hat Titel und Beschreibung, deutsch und englisch', () => {
    for (const lang of ['de', 'en']) {
      setLanguage(lang);
      for (const e of all) {
        expect(e.title.length).toBeGreaterThan(0);
        expect(e.description.length).toBeGreaterThan(0);
      }
      for (const c of menuCategories(true)) expect(categoryLabel(c.id).length).toBeGreaterThan(0);
    }
  });

  it('Kategorie-Zeile: Zahl der Einträge, leere Kategorie „noch leer“', () => {
    const [info, , , balance] = menuCategories(true);
    expect(categorySub(info!)).toContain(String(info!.entries.length));
    expect(categorySub(balance!)).toBe('noch leer');
    expect(categoryLabel('perf' satisfies CategoryId)).toBe('Performance');
  });
});

describe('Ziele der Einträge (B-768/AC-06)', () => {
  it('Spiel-URL wird zur Konfiguration, ohne dass die Seite neu geladen wird', () => {
    const run = targetOf({ href: 'game.html?dev=1&autostart=1&fresh=1&save=stress-wald&seed=stress&players=1&depth=0&stress=typisch' });
    expect(run).toEqual({ kind: 'run', params: { dev: '1', autostart: '1', fresh: '1', save: 'stress-wald', seed: 'stress', players: '1', depth: '0', stress: 'typisch' } });
  });

  it('andere Seiten bleiben Seiten, Dungeon Master bleibt extern, game.html ohne Abfrage ist keine Konfiguration', () => {
    expect(targetOf({ href: 'monitor.html' })).toEqual({ kind: 'page', href: 'monitor.html' });
    expect(targetOf({ href: 'dm.html', external: true })).toEqual({ kind: 'external', href: 'dm.html' });
    expect(runParamsOf('game.html')).toBeNull();
    expect(runParamsOf('stress.html?x=1')).toBeNull();
  });

  it('die drei Stress-echt-Szenarien tragen Spieler, Welt-Stufe und Szenario in der Konfiguration', () => {
    const runs = all.flatMap((e) => (e.target.kind === 'run' && e.target.params.stress ? [e.target.params] : []));
    expect(runs.map((p) => p.stress)).toEqual(['typisch', 'worstCase', 'worstCase']);
    expect(runs.map((p) => p.players)).toEqual(['1', '2', '2']);
    for (const p of runs) expect([p.dev, p.autostart, p.fresh]).toEqual(['1', '1', '1']);
  });
});

describe('Eigener Spielstand je Start', () => {
  it('fresh mit Namen bekommt die Uhrzeit angehängt, damit der Server den Namen nicht ablehnt', () => {
    const run = uniqueRun({ fresh: '1', save: 'stress-wald2', players: '2' }, 1_700_000_000_000);
    expect(run.save).toMatch(/^stress-wald2-[a-z0-9]+$/);
    expect(run.players).toBe('2');
    expect(uniqueRun({ fresh: '1', save: 'stress-wald2' }, 1)).not.toEqual(uniqueRun({ fresh: '1', save: 'stress-wald2' }, 2));
  });

  it('der Name bleibt gültig: höchstens 32 Zeichen aus a–z, 0–9 und Bindestrich', () => {
    const save = uniqueRun({ fresh: '1', save: 'x'.repeat(40) }, Date.now()).save!;
    expect(save.length).toBeLessThanOrEqual(32);
    expect(save).toMatch(/^[a-z0-9-]{1,32}$/);
  });

  it('ohne fresh oder ohne Namen bleibt alles wie es ist', () => {
    const plain = { dev: '1' };
    expect(uniqueRun(plain)).toBe(plain);
    const resume = { autostart: '1', save: 'familie' };
    expect(uniqueRun(resume)).toBe(resume);
  });
});

describe('Raster für Listen von Kacheln', () => {
  const items = (n: number) => Array.from({ length: n }, (_, i) => ({ id: `t${i}`, color: 1 }));

  it('bis 4 Einträge in 2 Spalten mit großer Schrift, bis 9 in 3 Spalten mit 44 px', () => {
    expect(gridTiles(items(4)).map((t) => [t.x, t.y].join(',')).length).toBe(4);
    expect(new Set(gridTiles(items(4)).map((t) => t.x)).size).toBe(2);
    expect(gridTiles(items(4))[0]!.labelPx).toBe(52);
    expect(new Set(gridTiles(items(7)).map((t) => t.x)).size).toBe(3);
    expect(gridTiles(items(7))[0]!.labelPx).toBe(44);
  });

  it('alle Kacheln liegen im Bild, über dem Hinweis und überlappen nicht (1 bis GRID_MAX)', () => {
    for (let n = 1; n <= GRID_MAX; n++) {
      const tiles = gridTiles(items(n));
      for (const t of tiles) {
        expect(t.x + t.w).toBeLessThanOrEqual(1800 + 1);
        expect(t.y + t.h).toBeLessThan(HUB_LAYOUT.hintY - HUB_LAYOUT.hintPx);
        expect(t.h).toBeGreaterThan(150);
      }
      tiles.forEach((a, i) =>
        tiles.slice(i + 1).forEach((b) => expect(a.x + a.w <= b.x + 0.01 || b.x + b.w <= a.x + 0.01 || a.y + a.h <= b.y + 0.01 || b.y + b.h <= a.y + 0.01).toBe(true)),
      );
    }
  });

  it('jede Kachel ist mit dem Pad erreichbar (3 × 3 und 2 × 2)', () => {
    for (const n of [4, 5, 7, 9]) expect(reachable(gridTiles(items(n)))).toBe(n);
  });
});
