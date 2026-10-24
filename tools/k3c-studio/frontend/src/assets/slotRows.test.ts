import { describe, expect, it } from 'vitest';
import type { AssetSlotDetails } from '../api';
import { infoGroups } from './slotRows';

const base: AssetSlotDetails = { id: 'tavern', art: 'grafik', kategorie: 'gebaeude', beschreibung: 'Gebäude: tavern', status: 'luecke', zuordnung: [], verwendung: [], hinweise: ['keine Grafik'] };

const full: AssetSlotDetails = {
  ...base,
  status: 'zugeordnet',
  wahl: { datei: 'sprites/goblin/idle.png', sheet: 'goblin' },
  sheet: { name: 'goblin', label: 'Goblin', pack: 'goblin', frameW: 150, frameH: 150, scale: 1.5, height: 40, originX: 0.5, originY: 0.7, tint: '#c77dff', anims: [{ name: 'idle', frames: 4, fps: 8 }] },
  atlas: { anims: [{ name: 'idle', frames: 4, page: 'atlas-0.png', x: 10, y: 20, w: 150, h: 150 }] },
  pack: { id: 'goblin', quelle: 'Q', lizenz: 'CC0 1.0' },
  zuordnung: [{ datei: 'zuordnung-objekte.md', abschnitt: 'Gegner', zellen: { Objekt: '`greed`' } }],
  verwendung: ['data/biomes.json › forest.enemies'],
  hinweise: [],
};

describe('infoGroups', () => {
  it('liefert alle Gruppen in fester Reihenfolge', () => {
    expect(infoGroups(full).map((g) => g.title)).toEqual(['Definition', 'Sheet', 'Atlas', 'Zuordnung', 'Verwendung']);
    const sheet = infoGroups(full)[1]!.rows;
    expect(sheet).toContainEqual({ label: 'Frame', value: '150×150' });
    expect(sheet).toContainEqual({ label: 'Animation idle', value: '4 Frames, 8 fps' });
  });

  it('lässt leere Gruppen weg', () => {
    expect(infoGroups(base).map((g) => g.title)).toEqual(['Definition']);
  });

  it('Hinweise gehören nicht in die Gruppen', () => {
    const all = infoGroups(base).flatMap((g) => g.rows.map((r) => r.value));
    expect(all).not.toContain('keine Grafik');
  });
});
