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
    expect(l.chunks!.missing).toContain('1 Inseln');
    expect(worldProgress(worldStats(d()))).toBeLessThan(100);
  });

  it('eine Welt ohne Inseln und mit Dateifehler meldet, was fehlt', () => {
    const l = worldStats(d({ islands: [], error: 'kaputt' }));
    expect(l[0]).toMatchObject({ key: 'datei', missing: 'Fehler in der Datei' });
    expect(l.find((x) => x.key === 'inseln')!.missing).toBe('keine Insel');
  });
});
