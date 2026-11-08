import { describe, expect, it } from 'vitest';
import assets from '../../data/assets.json';
import sounds from '../../data/sounds.json';

// CU1.2: Die Sound-Slots stehen in data/sounds.json; data/assets.json (Altregister) fällt mit CU1.5, dann entfällt der Vergleich.
describe('data/sounds.json (CU1.2)', () => {
  it('nur Sound-Slots, mit denselben Ids und Wahlen wie im Altregister', () => {
    expect(sounds.slots.every((s) => s.art === 'sound')).toBe(true);
    const alt = (assets.slots as { art: string }[]).filter((s) => s.art === 'sound');
    expect(sounds.slots).toEqual(alt);
    expect(sounds.slots).toHaveLength(32);
  });

  it('Slot-Ids sind eindeutig, jeder Slot hat eine Wahl und einen Bus', () => {
    const ids = sounds.slots.map((s) => s.id);
    expect(new Set(ids).size).toBe(ids.length);
    for (const s of sounds.slots) {
      expect(s.wahl.length, s.id).toBeGreaterThan(0);
      expect(['sfx', 'music', 'ambient'], s.id).toContain(s.bus);
    }
  });
});
