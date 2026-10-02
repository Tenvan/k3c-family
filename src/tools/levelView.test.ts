import { describe, expect, it } from 'vitest';
import golden from '../../testdata/golden/level-forest.json';
import { levelModel, seedFromBytes, startTarget, type LevelResponse } from './levelView';

/** Erstes Level der Golden-Datei (Seed "0", Wald, 900 Units, 18 Abschnitte). */
const layout = (golden as unknown as { levels: Omit<LevelResponse, 'warnings'>[] }).levels[0]!;
const response = (warnings: string[] = []): LevelResponse => ({ ...layout, warnings });

describe('levelModel (B-092/AC-02, AC-03)', () => {
  const m = levelModel(response());

  it('Abschnitte des Golden-Levels, lückenlos bis zur Levelbreite', () => {
    expect(m.chunks).toHaveLength(layout.chunks.length);
    expect(m.chunks.map((c) => c.kind)).toEqual(layout.chunks.map((c) => c.kind));
    expect(m.chunks[0]!.from).toBe(0);
    expect(m.chunks.at(-1)!.to).toBe(m.widthUnits);
    m.chunks.slice(1).forEach((c, i) => expect(c.from).toBe(m.chunks[i]!.to));
  });

  it('Objekte nach x sortiert, Anzahl je Art wie in der Golden-Datei', () => {
    expect(m.objects).toHaveLength(layout.entities.length);
    expect(m.objects.map((o) => o.x)).toEqual([...m.objects.map((o) => o.x)].sort((a, b) => a - b));
    expect(Object.fromEntries(m.counts)).toEqual({ tree: 29, skillPoint: 4, rock: 4, chest: 2, portal: 2, bush: 2, recruitCamp: 2, castle: 1, exit: 1 });
    expect(m.counts[0]).toEqual(['tree', 29]); // häufigste zuerst
  });

  it('Burg, zwei Portale, ein Ausgang und die Hub-Mitte sind da', () => {
    expect(m.objects.filter((o) => o.kind === 'castle')).toHaveLength(1);
    expect(m.objects.filter((o) => o.kind === 'portal')).toHaveLength(2);
    expect(m.objects.filter((o) => o.kind === 'exit')).toHaveLength(1);
    expect(m.hubCenterUnits).toBe(layout.hubCenterUnits);
    expect(m.seed).toBe('0');
    expect(m.biomeId).toBe('forest');
  });

  it('Warnungen werden durchgereicht', () => {
    expect(m.warnings).toEqual([]);
    expect(levelModel(response(['Level zu kurz', '2 Portale erwartet, 1 gefunden'])).warnings).toEqual(['Level zu kurz', '2 Portale erwartet, 1 gefunden']);
  });

  it('gleich häufige Arten stehen nach Namen', () => {
    const tie = levelModel({ ...response(), entities: [{ kind: 'rock', x: 2 }, { kind: 'bush', x: 1 }] });
    expect(tie.counts).toEqual([['bush', 1], ['rock', 1]]);
  });

  it('unvollständige Antwort: leeres Modell mit einer Warnung, kein Fehler', () => {
    for (const bad of [undefined, null, {}, { seed: 'x', warnings: ['a'] }, { ...response(), chunks: undefined }, { ...response(), widthUnits: 'breit' }] as never[]) {
      const empty = levelModel(bad);
      expect(empty.chunks).toEqual([]);
      expect(empty.objects).toEqual([]);
      expect(empty.warnings.at(-1)).toBe('Antwort unvollständig');
    }
    expect(levelModel({ seed: 'x', warnings: ['a'] } as never).warnings).toEqual(['a', 'Antwort unvollständig']);
  });
});

describe('startTarget (B-092/AC-07)', () => {
  it('Wald und gültiger Seed: URL für ein neues Spiel mit diesem Namen', () => {
    expect(startTarget('probe', 'forest')).toEqual({ url: 'game.html?autostart=1&fresh=1&save=probe' });
    expect(startTarget('a-1', 'forest')).toEqual({ url: 'game.html?autostart=1&fresh=1&save=a-1' });
    expect(startTarget('k'.repeat(32), 'forest')).toHaveProperty('url');
  });

  it('Seed außerhalb des Namensformats: Grund statt URL', () => {
    for (const seed of ['Probe', 'k'.repeat(33), '', 'mit leerzeichen', 'a/b', 'ä', 'a_b']) {
      expect(startTarget(seed, 'forest'), seed).toHaveProperty('reason');
    }
  });

  it('Höhle, Mine und unbekannte Biome: Grund (Start nur im Wald)', () => {
    for (const biome of ['cave', 'mine', 'xyz', '']) expect(startTarget('probe', biome)).toEqual({ reason: 'Start nur für den Wald (Tiefe 0)' });
  });
});

describe('seedFromBytes', () => {
  it('acht Zeichen aus Kleinbuchstaben und Ziffern, gleiche Bytes gleicher Seed', () => {
    const bytes = Uint8Array.from([0, 1, 25, 26, 35, 36, 255, 128]);
    expect(seedFromBytes(bytes)).toBe('abz09adu');
    expect(seedFromBytes(bytes)).toBe(seedFromBytes(Uint8Array.from(bytes)));
  });

  it('immer ein startbarer Name, auch bei zu wenigen Bytes', () => {
    for (const bytes of [[], [7], Array(8).fill(255), Array.from({ length: 8 }, (_, i) => i * 37)]) {
      const seed = seedFromBytes(bytes);
      expect(seed).toMatch(/^[a-z0-9]{8}$/);
      expect(startTarget(seed, 'forest')).toHaveProperty('url');
    }
  });
});
