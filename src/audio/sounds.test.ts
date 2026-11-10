import { describe, expect, it } from 'vitest';
import sounds from '../../data/sounds.json';

// CU1.2/CU1.5: Die Sound-Slots stehen in data/sounds.json (einzige Quelle seit dem Wegfall des Altregisters).
describe('data/sounds.json (CU1.2)', () => {
  it('nur Sound-Slots, 32 Stück (Altregister data/assets.json fiel mit CU1.5)', () => {
    expect(sounds.slots.every((s) => s.art === 'sound')).toBe(true);
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
