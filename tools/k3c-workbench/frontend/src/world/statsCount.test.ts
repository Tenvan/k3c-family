import { describe, expect, it } from 'vitest';
import type { WorldDetail } from '../api';
import { worldProgress, worldStats } from './statsCount';

const d = (over: Partial<WorldDetail> = {}): WorldDetail => ({
  id: 'w', name: 'W', notes: '', extends: '', error: '',
  islands: [{ id: 'a', name: 'A', depths: [0, 1], biomes: ['forest', ''], plannedChunks: {}, chunkCounts: {}, story: { arrival: ['x'] } }],
  story: { start: ['s'] },
  ...over,
});

describe('Statistik der Welt', () => {
  it('zählt vorhandene und fehlende Teile', () => {
    const l = Object.fromEntries(worldStats(d()).map((x) => [x.key, x]));
    expect(l.stufen).toMatchObject({ have: 1, need: 2, missing: '1 ohne Biom' });
    expect(l.geschichte).toMatchObject({ have: 2, need: 6, missing: '4 Einträge' });
    expect(l.chunks).toMatchObject({ optional: true, missing: '' }); // reiner Zufall ist kein Mangel
    expect(worldProgress(worldStats(d()))).toBeLessThan(100);
  });

  it('eine Welt ohne Inseln und mit Dateifehler meldet, was fehlt', () => {
    const l = worldStats(d({ islands: [], error: 'kaputt' }));
    expect(l[0]).toMatchObject({ key: 'datei', missing: 'Fehler in der Datei' });
    expect(l.find((x) => x.key === 'inseln')!.missing).toBe('keine Insel');
  });

  it('eigene Assets: neue Welt braucht je Gruppe eines, ein Ableger nicht', () => {
    const own = { groups: ['figuren', 'gebaeude-paper'], count: 3 };
    const neu = worldStats(d(), own).find((x) => x.key === 'assets')!;
    expect(neu).toMatchObject({ have: 2, need: 5, missing: 'Gegner, Hintergründe, Reittiere' });
    const ableger = worldStats(d({ extends: 'standard' }), { groups: [], count: 0 }).find((x) => x.key === 'assets')!;
    expect(ableger).toMatchObject({ optional: true, missing: '' });
    expect(worldStats(d({ extends: 'standard', kind: 'new' }), { groups: [], count: 0 }).find((x) => x.key === 'assets')!.missing).not.toBe('');
    expect(worldStats(d()).some((x) => x.key === 'assets')).toBe(false); // Basis-Welt: keine Zeile
  });
});
