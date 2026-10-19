import { describe, expect, it } from 'vitest';
import type { ModelChunk } from './levelView';
import { bandBox, bandChunks, bandMode, chunkInfo, slotOfIndex } from './bandLayout';

// Chunk-Infos im Bodenstreifen der Live-Vorschau (B-485, WZ17.4): reine Abbildung Welt -> Pixel der Bühne und Inhalt je Chunk.
const chunks: ModelChunk[] = Array.from({ length: 8 }, (_, i) => ({ index: i, kind: i === 3 || i === 4 ? 'hub' : i === 0 ? 'edge' : 'forest', from: i * 50, to: i * 50 + 50 }));

describe('Chunk-Infos auf der Bühne', () => {
  it('liegt fest am oberen Rand mit fester Höhe, unabhängig vom Zoom', () => {
    expect(bandBox()).toEqual({ top: 0, height: 42 });
  });

  it('Chunk -> Pixelbereich: (Anfang − linke Kante der Live-Vorschau) · Zoom, auf die Bühne beschnitten; nur sichtbare Chunks', () => {
    const boxes = bandChunks(chunks, { left: 120, scale: 4 }, 300);
    expect(boxes.map((b) => [b.chunk.index, b.x, b.w])).toEqual([[2, 0, 120], [3, 120, 180]]); // ungeschnitten: −80…120 und 120…320
  });
});

describe('ausführlich oder kompakt je nach Platz', () => {
  it('schmal oder flach = kompakt bzw. nur Farbe; mehrzeilig nur, wenn alle Zeilen hineinpassen', () => {
    expect(bandMode(400, 5)).toBe('color');
    expect(bandMode(20, 30)).toBe('color');
    expect(bandMode(50, 30)).toBe('compact');
    expect(bandMode(400, 10)).toBe('compact');
    expect(bandMode(200, 16)).toBe('line');
    expect(bandMode(200, 40, 2)).toBe('full');
    expect(bandMode(200, 40, 3)).toBe('line'); // drei Zeilen brauchen 42 px
    expect(bandMode(200, 42, 3)).toBe('full');
    expect(bandMode(200, 54)).toBe('line'); // nur eine Zeile Inhalt
    expect(bandMode(100, 40, 2)).toBe('line');
  });
});

describe('Inhalt je Chunk', () => {
  it('Platz relativ zu den Hub-Abschnitten, Hub und Rand ohne Platz', () => {
    expect([slotOfIndex(chunks, 5), slotOfIndex(chunks, 7), slotOfIndex(chunks, 2), slotOfIndex(chunks, 3)]).toEqual([1, 3, -1, 0]);
  });

  it('geplanter Chunk: Name, Platz, Seed, feste Objekte und die Hinweisfarbe', () => {
    const info = chunkInfo(chunks, chunks[5], [{ slot: 1, kind: 'clearing', seed: 'abc', objects: [{ kind: 'tree', at: 1 }, { kind: 'rock', at: 9 }] }], [{ slot: 1, kind: 'planned', label: '' }]);
    expect(info).toEqual({ color: 'planned', icon: '🌲', name: 'Wald', slot: '+1', seed: 'abc', objects: 2 });
  });

  it('Fehler am Platz färbt rot; Hub und Rand nennen nur den Abschnitt; sonst neutral', () => {
    expect(chunkInfo(chunks, chunks[2], [], [{ slot: -1, kind: 'error', label: '' }]).color).toBe('error');
    expect(chunkInfo(chunks, chunks[3], [], [])).toEqual({ color: 'hub', icon: '🏰', name: 'Hub', slot: '', seed: '', objects: 0 });
    expect(chunkInfo(chunks, chunks[0], [], [])).toMatchObject({ color: 'special', name: 'Rand', slot: '' });
    expect(chunkInfo(chunks, chunks[6], [], []).color).toBe('none');
  });
});
