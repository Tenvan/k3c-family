import { describe, expect, it } from 'vitest';
import type { Site } from '../model/types';
import { backWalls } from './backWallRules';

const wall = (id: number, x: number, level = 1, state: Site['state'] = 'built'): Pick<Site, 'id' | 'kind' | 'x' | 'state' | 'level'> => ({ id, kind: 'wall', x, state, level });

describe('Hintergrundmauer (GEB3 AC-08, B-442)', () => {
  it('verbindet benachbarte gebaute Mauern einer Seite in der Stufe der äußeren', () => {
    // Beispiel der Spec: links Linie 1 (Stufe 3) und Linie 2 (Stufe 1, weiter außen), Linie 3 nicht gebaut
    const sites = [wall(1, 480, 3), wall(2, 440, 1), wall(3, 400, 2, 'unpaid')];
    expect(backWalls(sites, 500)).toEqual([{ id: 2, from: 440, to: 480, stage: 1 }]);
  });

  it('beide Seiten getrennt, Abschnitte von innen nach außen', () => {
    const sites = [wall(1, 520, 2), wall(2, 560, 4), wall(3, 600, 5), wall(4, 480, 1), wall(5, 440, 2)];
    expect(backWalls(sites, 500)).toEqual([
      { id: 5, from: 440, to: 480, stage: 2 },
      { id: 2, from: 520, to: 560, stage: 4 },
      { id: 3, from: 560, to: 600, stage: 5 },
    ]);
  });

  it('eine ungebaute Mauer in der Mitte trennt die Abschnitte', () => {
    const sites = [wall(1, 520), wall(2, 560, 1, 'waitingWorker'), wall(3, 600)];
    expect(backWalls(sites, 500)).toEqual([]);
  });

  it('nur eine gebaute Mauer je Seite: keine Hintergrundmauer; level fehlt = Stufe 1; andere Bauplätze zählen nicht', () => {
    const tower = { id: 9, kind: 'tower' as const, x: 540, state: 'built' as const };
    expect(backWalls([wall(1, 520), tower], 500)).toEqual([]);
    expect(backWalls([wall(1, 520), { ...wall(2, 560), level: undefined }], 500)).toEqual([{ id: 2, from: 520, to: 560, stage: 1 }]);
  });
});
