import { describe, expect, it } from 'vitest';
import paperJson from '../../data/paper.json';
import stressJson from '../../data/stress.json';
import { KNOWN } from './known';
import { countAt, distinctTypes, figuresAt, hashText, motionFor, parseScenario, parseScenarios, seeded, stageCount, withScenery, type Scenario } from './stressPlan';

const INDEX = import.meta.glob('../../public/grafik/{gegner,truppen}/index.paper.json', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;
const FILE = stressJson as Record<string, unknown>;
const scenarios = parseScenarios(FILE, KNOWN);
const clone = (id: string): Record<string, unknown> => JSON.parse(JSON.stringify(FILE[id])) as Record<string, unknown>;

describe('Szenario-Datei (AC-11)', () => {
  it('enthält typisch und worstCase', () => {
    expect(Object.keys(scenarios).sort()).toEqual(['typisch', 'worstCase']);
    expect(scenarios.typisch!.name).toBe('Typisch');
    expect(scenarios.worstCase!.name).toBe('Worst Case');
  });

  it('alle Typen haben ein Papierschnitt-Asset in data/paper.json und einen Eintrag im Gruppen-Index', () => {
    const gegner = INDEX['../../public/grafik/gegner/index.paper.json'] ?? '';
    const truppen = INDEX['../../public/grafik/truppen/index.paper.json'] ?? '';
    expect(gegner).not.toBe('');
    expect(truppen).not.toBe('');
    for (const type of KNOWN.enemies) {
      const asset = (paperJson.gegner as Record<string, string>)[type];
      expect(asset, `Gegner ${type} fehlt in data/paper.json`).toBeDefined();
      expect(gegner, `Gegner ${type}: ${asset} fehlt im Index`).toContain(asset!.split('/')[1]);
    }
    for (const type of KNOWN.troops) {
      const asset = (paperJson.truppen as Record<string, string>)[type];
      expect(asset, `Truppe ${type} fehlt in data/paper.json`).toBeDefined();
      expect(truppen, `Truppe ${type}: ${asset} fehlt im Index`).toContain(asset!.split('/')[1]);
    }
  });

  it('beide Szenarien haben Hintergrund (Wald) und Vordergrund', () => {
    for (const s of Object.values(scenarios)) {
      expect(s.scenery).toMatchObject({ biome: 'forest', background: true });
      expect(s.scenery.foreground).toBeGreaterThan(0);
    }
    expect(KNOWN.biomes).toContain('forest');
  });

  it('Spielerzahl ist 1 oder 2, Rampe 10 s', () => {
    for (const s of Object.values(scenarios)) {
      expect([1, 2]).toContain(s.players);
      expect(s.ramp.everyMs).toBe(10_000);
    }
  });
});

describe('ungültige Szenarien (AC-12)', () => {
  it.each([
    ['unbekannter Typ', (s: Record<string, unknown>) => ((s.enemies as unknown[])[0] = { type: 'drache', count: 1 }), /unbekannter Typ drache/],
    ['players 3', (s: Record<string, unknown>) => (s.players = 3), /players muss 1 oder 2/],
    ['negative Anzahl', (s: Record<string, unknown>) => ((s.enemies as unknown[])[0] = { type: 'greed', count: -1 }), /count von greed/],
    ['Anzahl mit Komma', (s: Record<string, unknown>) => ((s.enemies as unknown[])[0] = { type: 'greed', count: 1.5 }), /count von greed/],
    ['Zoom 0', (s: Record<string, unknown>) => ((s.camera as Record<string, unknown>).zoom = 0), /camera braucht/],
    ['Mix unbekannt', (s: Record<string, unknown>) => ((s.sprites as Record<string, unknown>).mix = 'viel'), /sprites\.mix/],
    ['Biom unbekannt', (s: Record<string, unknown>) => ((s.scenery as Record<string, unknown>).biome = 'mond'), /scenery\.biome/],
    ['Vordergrund negativ', (s: Record<string, unknown>) => ((s.scenery as Record<string, unknown>).foreground = -2), /scenery\.foreground/],
    ['Hintergrund kein Wahrheitswert', (s: Record<string, unknown>) => ((s.scenery as Record<string, unknown>).background = 'ja'), /scenery\.background/],
    ['Stufe kürzer als die Aufwärmzeit', (s: Record<string, unknown>) => ((s.ramp as Record<string, unknown>).everyMs = 2000), /ramp\.everyMs/],
    ['Faktor unter 1', (s: Record<string, unknown>) => ((s.ramp as Record<string, unknown>).factor = 0.5), /ramp\.factor/],
    ['keine Figuren', (s: Record<string, unknown>) => ((s.enemies = []), (s.troops = [])), /keine Figuren/],
  ])('%s wird abgelehnt', (_name, mutate, message) => {
    const s = clone('typisch');
    mutate(s);
    expect(() => parseScenario('x', s, KNOWN)).toThrow(message);
  });

  it('eine Datei ohne Szenarien wird abgelehnt', () => {
    expect(() => parseScenarios({ _hinweis: 'x' }, KNOWN)).toThrow(/keine Szenarien/);
  });
});

describe('Rampe und Verteilung (AC-13)', () => {
  const typ = scenarios.typisch!;
  const worst = scenarios.worstCase!;
  const total = (s: Scenario, stage: number, players: 1 | 2 = s.players) => figuresAt(s, KNOWN, stage, players).length;

  it('Anzahl je Stufe: ceil(Anzahl × Faktor^(Stufe−1))', () => {
    expect(countAt(12, 1.5, 1)).toBe(12);
    expect(countAt(12, 1.5, 2)).toBe(18);
    expect(countAt(6, 1.5, 3)).toBe(14);
    expect(countAt(8, 1.5, 2)).toBe(12);
  });

  it('typisch startet mit 26 Figuren und wächst höchstens `steps` Stufen', () => {
    expect(total(typ, 1)).toBe(26);
    expect(total(typ, 2)).toBeGreaterThan(total(typ, 1));
    expect(stageCount(typ, true)).toBe(6);
    expect(stageCount(typ, false)).toBe(1);
  });

  it('worstCase startet mit 60 Figuren über alle 23 Typen', () => {
    const figs = figuresAt(worst, KNOWN, 1, 2);
    expect(figs).toHaveLength(60);
    expect(distinctTypes(figs)).toBe(23);
    expect(stageCount(worst, true)).toBe(8);
  });

  it('eine höhere Stufe enthält die Figuren der niedrigeren unverändert', () => {
    for (const s of [typ, worst]) {
      const low = figuresAt(s, KNOWN, 2, 2);
      const high = figuresAt(s, KNOWN, 3, 2);
      const byKey = new Map(high.map((f) => [f.key, f]));
      for (const f of low) expect(byKey.get(f.key), `${s.name} key ${f.key}`).toEqual(f);
    }
  });

  it('zwei Spieler teilen die Figuren, ein Spieler bekommt alle', () => {
    const two = figuresAt(typ, KNOWN, 1, 2);
    expect(two.filter((f) => f.player === 0).length + two.filter((f) => f.player === 1).length).toBe(two.length);
    expect(figuresAt(typ, KNOWN, 1, 1).every((f) => f.player === 0)).toBe(true);
  });

  it('distinctSheets begrenzt die Typen im typischen Mix', () => {
    const s: Scenario = { ...typ, sprites: { mix: 'typical', distinctSheets: 2 } };
    expect(distinctTypes(figuresAt(s, KNOWN, 1, 1))).toBe(2);
  });

  it('gleicher Seed, gleiche Verteilung; anderer Seed, andere', () => {
    expect(motionFor(1, 5, 0, 1920)).toEqual(motionFor(1, 5, 0, 1920));
    expect(motionFor(2, 5, 0, 1920)).not.toEqual(motionFor(1, 5, 0, 1920));
    const a = seeded(7);
    const b = seeded(7);
    expect([a(), a(), a()]).toEqual([b(), b(), b()]);
  });

  it('Werte liegen im Bildausschnitt des Spielers', () => {
    for (let key = 0; key < 200; key++) {
      const m = motionFor(3, key, 5000, 1920);
      expect(Math.abs(m.x - 5000)).toBeLessThanOrEqual(1920 * 0.45);
    }
  });

  it('?bg=0 und ?fg=0 schalten Hintergrund und Vordergrund ab', () => {
    const s = scenarios.typisch!;
    expect(withScenery(s, '').scenery).toEqual(s.scenery);
    expect(withScenery(s, '?bg=0').scenery).toEqual({ ...s.scenery, background: false });
    expect(withScenery(s, '?fg=0').scenery).toEqual({ ...s.scenery, foreground: 0 });
  });

  it('Datei-Hash ändert sich mit dem Inhalt', () => {
    expect(hashText('a')).not.toBe(hashText('b'));
    expect(hashText('a')).toHaveLength(8);
  });
});
