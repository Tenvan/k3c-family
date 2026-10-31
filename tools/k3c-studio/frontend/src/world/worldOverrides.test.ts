import { describe, expect, it } from 'vitest';
import type { Overrides, WorldValueEntry } from '../api';
import { canonical, filterRows, overrideChanges, parseValue, resetOverride, setOverride, valueRows } from './worldOverrides';
import { sameDraft } from './worldDraft';

// WZ37.1 (B-682): reine Logik der Werte-Overrides im Welten Editor.
const entries: WorldValueEntry[] = [
  { file: 'buildings', path: ['wall', 'cost', 'wood'], base: 10 },
  { file: 'buildings', path: ['wall', 'levels', '1', 'hp'], base: 500, override: 650 },
  { file: 'waves', path: ['perExtraPlayer'], base: 0.3 },
  { file: 'enemies', path: ['wolf', 'gold'], base: [3, 5] },
];
const saved = { buildings: { wall: { levels: { '1': { hp: 650 } } } } };

describe('Werte-Overrides (B-682/AC-01)', () => {
  it('setzt einen Override am Pfad, ohne andere zu verlieren', () => {
    const ov = setOverride(saved, 'buildings', ['wall', 'cost', 'wood'], 7, 10);
    expect(ov).toEqual({ buildings: { wall: { levels: { '1': { hp: 650 } }, cost: { wood: 7 } } } });
    expect(saved.buildings.wall).not.toHaveProperty('cost'); // unverändert
  });

  it('zurücksetzen entfernt den Override und leere Eltern', () => {
    expect(resetOverride(saved, 'buildings', ['wall', 'levels', '1', 'hp'])).toEqual({});
    const two = setOverride(saved, 'waves', ['perExtraPlayer'], 0.37, 0.3);
    expect(resetOverride(two, 'buildings', ['wall', 'levels', '1', 'hp'])).toEqual({ waves: { perExtraPlayer: 0.37 } });
  });

  it('speichert nur Abweichungen: Wert gleich dem Basiswert ist kein Override', () => {
    expect(setOverride(saved, 'buildings', ['wall', 'levels', '1', 'hp'], 500, 500)).toEqual({});
    expect(setOverride({}, 'enemies', ['wolf', 'gold'], [3, 5], [3, 5])).toEqual({});
    expect(setOverride({}, 'enemies', ['wolf', 'gold'], [4, 6], [3, 5])).toEqual({ enemies: { wolf: { gold: [4, 6] } } });
  });

  it('zählt geänderte Felder; gleiche Overrides in anderer Reihenfolge sind kein Entwurf', () => {
    const a = setOverride(saved, 'waves', ['perExtraPlayer'], 0.37, 0.3);
    expect(overrideChanges(a, saved)).toBe(1);
    expect(overrideChanges(resetOverride(a, 'buildings', ['wall', 'levels', '1', 'hp']), saved)).toBe(2);
    const reordered = { waves: { perExtraPlayer: 0.37 }, ...saved };
    expect(canonical(reordered)).toBe(canonical(a));
    const draft = (overrides: Overrides) => ({ name: 'W', notes: '', islands: [], overrides });
    expect(sameDraft(draft(reordered), draft(a))).toBe(true);
    expect(sameDraft(draft(saved), draft(a))).toBe(false);
  });

  it('liest Eingaben: Zahl, Liste, Text zum Text-Basiswert; Unlesbares bleibt Text (die Prüfung meldet den Typ)', () => {
    expect(parseValue('7', 10)).toBe(7);
    expect(parseValue('[4, 6]', [3, 5])).toEqual([4, 6]);
    expect(parseValue('7', 'Holz')).toBe('7');
    expect(parseValue('viel', 10)).toBe('viel');
  });
});

describe('geerbt gegen überschrieben (B-682/AC-02)', () => {
  it('Status je Feld folgt dem Entwurf, nicht der Datei', () => {
    const rows = valueRows(entries, saved);
    expect(rows.map((r) => [r.label, r.status])).toEqual([
      ['wall.cost.wood', 'geerbt'], ['wall.levels.1.hp', 'überschrieben'], ['perExtraPlayer', 'geerbt'], ['wolf.gold', 'geerbt'],
    ]);
    expect(rows[1]).toMatchObject({ base: 500, override: 650 });
    const reset = valueRows(entries, resetOverride(saved, 'buildings', ['wall', 'levels', '1', 'hp']));
    expect(reset[1]).toMatchObject({ status: 'geerbt', override: undefined, base: 500 });
  });

  it('filtert nach Datei, Suchtext und nur überschriebenen', () => {
    const rows = valueRows(entries, saved);
    expect(filterRows(rows, 'buildings', '', false).map((r) => r.label)).toEqual(['wall.cost.wood', 'wall.levels.1.hp']);
    expect(filterRows(rows, 'buildings', 'COST', false).map((r) => r.label)).toEqual(['wall.cost.wood']);
    expect(filterRows(rows, 'buildings', '', true).map((r) => r.label)).toEqual(['wall.levels.1.hp']);
  });
});
