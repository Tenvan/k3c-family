import { describe, expect, it } from 'vitest';
import type { PlanIsland, PlannedChunk } from '../api';
import { addChunk, addObject, isDirty, moveObject, normalized, removeChunk, removeObject, savedCount, savedPlan, slotLabel, updateChunk } from './plan';

// B-461/AC-09 (WZ8.5): reine Logik des Editors für geplante Chunks.

const portal: PlannedChunk = { slot: 6, kind: 'portal' };

describe('Entwurf bearbeiten', () => {
  it('Chunk hinzufügen sortiert nach Platz, links vor rechts', () => {
    const draft = addChunk(addChunk([portal], -3, 'clearing'), 2, 'rocks');
    expect(draft.map((c) => c.slot)).toEqual([-3, 2, 6]);
  });

  it('ein doppelter Platz bleibt stehen (die Prüfung nennt ihn)', () => {
    expect(addChunk([portal], 6, 'forest')).toHaveLength(2);
  });

  it('Platz, Art und Chunk-Seed ändern, die Liste bleibt nach Platz sortiert', () => {
    const draft = updateChunk([{ slot: 1, kind: 'forest' }, portal], 0, { slot: 9, kind: 'clearing', seed: 'x' });
    expect(draft).toEqual([portal, { slot: 9, kind: 'clearing', seed: 'x' }]);
  });

  it('Chunk löschen', () => {
    expect(removeChunk([portal, { slot: 7, kind: 'forest' }], 0)).toEqual([{ slot: 7, kind: 'forest' }]);
  });

  it('Objekte anlegen, verschieben und löschen, ohne die übrigen Chunks zu berühren', () => {
    const base: PlannedChunk[] = [{ slot: 2, kind: 'rocks' }, portal];
    const one = addObject(base, 0, { kind: 'rock', at: 10 });
    const two = addObject(one, 0, { kind: 'tree', at: 30 });
    expect(two[0]!.objects).toEqual([{ kind: 'rock', at: 10 }, { kind: 'tree', at: 30 }]);
    expect(moveObject(two, 0, 1, 40)[0]!.objects).toEqual([{ kind: 'rock', at: 10 }, { kind: 'tree', at: 40 }]);
    expect(removeObject(two, 0, 0)[0]!.objects).toEqual([{ kind: 'tree', at: 30 }]);
    expect(two[1]).toBe(base[1]);
  });

  it('Änderungen verändern die Eingabe nicht', () => {
    const base: PlannedChunk[] = [{ slot: 2, kind: 'rocks', objects: [{ kind: 'rock', at: 10 }] }];
    const copy = structuredClone(base);
    addObject(base, 0, { kind: 'tree', at: 3 });
    moveObject(base, 0, 0, 5);
    updateChunk(base, 0, { kind: 'x' });
    expect(base).toEqual(copy);
  });
});

describe('Form zum Prüfen und Speichern', () => {
  it('lässt leeren Seed und leere Objektliste weg und hält die Feldreihenfolge', () => {
    const draft: PlannedChunk[] = [{ slot: 1, kind: 'forest', seed: '', objects: [] }, { slot: 2, kind: 'rocks', seed: 's', objects: [{ kind: 'rock', at: 10 }] }];
    expect(JSON.stringify(normalized(draft))).toBe('[{"slot":1,"kind":"forest"},{"slot":2,"kind":"rocks","seed":"s","objects":[{"kind":"rock","at":10}]}]');
  });

  it('isDirty vergleicht die normalisierte Form', () => {
    expect(isDirty([{ slot: 1, kind: 'forest', seed: '' }], [{ slot: 1, kind: 'forest' }])).toBe(false);
    expect(isDirty([{ slot: 1, kind: 'forest', seed: 'a' }], [{ slot: 1, kind: 'forest' }])).toBe(true);
  });

  it('savedPlan liest die Planung der Stufe aus den Tiefen der Insel', () => {
    const island: PlanIsland = { id: 'i', name: 'I', depths: [0, 1], biomes: ['forest', 'cave'], planned: { '1': [portal] }, chunkCounts: { '1': 20 } };
    expect(savedPlan(island, 1)).toEqual([portal]);
    expect(savedPlan(island, 0)).toEqual([]);
    expect(savedPlan(island, 5)).toEqual([]);
    expect(savedPlan(undefined, 0)).toEqual([]);
  });

  it('savedCount liest die Chunk-Anzahl der Stufe, ohne Eintrag 0 (Biom-Länge)', () => {
    const island: PlanIsland = { id: 'i', name: 'I', depths: [0, 1], biomes: ['forest', 'cave'], planned: {}, chunkCounts: { '1': 20 } };
    expect([savedCount(island, 1), savedCount(island, 0), savedCount(island, 5), savedCount(undefined, 0)]).toEqual([20, 0, 0, 0]);
  });

  it('slotLabel', () => {
    expect([slotLabel(3), slotLabel(-3)]).toEqual(['+3', '-3']);
  });
});
