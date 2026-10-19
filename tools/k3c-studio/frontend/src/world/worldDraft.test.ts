import { describe, expect, it } from 'vitest';
import type { PlanView } from '../api';
import { addIsland, draftOf, moveIsland, removeIsland, sameDraft, toggleDepth, updateIsland } from './worldDraft';

// Reine Logik des Welt-Entwurfs (WZ18.6, B-488/AC-05).
const view: PlanView = {
  name: 'Nordmeer', notes: '', sizes: [], stand: 's', error: '',
  islands: [
    { id: 'island1', name: 'Insel 1', depths: [0, 4], biomes: ['forest', 'crystal'], planned: {}, chunkCounts: {}, depthScaling: { hp: 1.5, damage: 1.3, speed: 1.1 } },
    { id: 'island3', name: 'Insel 3', depths: [0, 1, 4], biomes: [], planned: {}, chunkCounts: {}, depthScaling: { hp: 2, damage: 1, speed: 1 } },
  ],
};

describe('Welt-Entwurf', () => {
  it('beginnt beim geladenen Stand, jede Insel mit ihrer id als Herkunft', () => {
    const d = draftOf(view);
    expect(d.islands.map((i) => [i.from, i.id])).toEqual([['island1', 'island1'], ['island3', 'island3']]);
    expect(sameDraft(d, draftOf(view))).toBe(true);
  });

  it('neue Insel: Kopie der letzten mit freier id, ohne Herkunft', () => {
    const d = addIsland(addIsland(draftOf(view)));
    expect(d.islands.slice(2).map((i) => [i.from, i.id, i.name])).toEqual([['', 'island4', 'Insel 4'], ['', 'island5', 'Insel 5']]);
    expect(d.islands[2]!.depths).toEqual([0, 1, 4]);
    expect(d.islands[2]!.depthScaling).toEqual({ hp: 2, damage: 1, speed: 1 });
  });

  it('umordnen, entfernen und ändern', () => {
    const d = draftOf(view);
    expect(moveIsland(d, 1, -1).islands.map((i) => i.id)).toEqual(['island3', 'island1']);
    expect(moveIsland(d, 0, -1)).toBe(d);
    expect(removeIsland(d, 0).islands.map((i) => i.id)).toEqual(['island3']);
    expect(updateIsland(d, 1, { id: 'eis' }).islands[1]).toMatchObject({ from: 'island3', id: 'eis' });
  });

  it('Tiefen bleiben aufsteigend', () => {
    expect(toggleDepth([0, 4], 2)).toEqual([0, 2, 4]);
    expect(toggleDepth([0, 2, 4], 2)).toEqual([0, 4]);
  });
});
