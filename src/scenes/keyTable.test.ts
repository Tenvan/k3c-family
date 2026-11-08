import { beforeEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { KEYBOARD_1, KEYBOARD_2 } from '../input/keyboardLayouts';
import { slotBindings } from '../input/slotBindings';
import { KEYS_LAYOUT } from './optionsLogic';
import { keyLabel, keyTable } from './keyTable';

beforeEach(() => setLanguage('de'));

describe('Tastenbelegung (B-741/AC-02, AC-03)', () => {
  const table = keyTable();
  const row = (name: string) => table.rows.find((r) => r[0] === name)!;

  it('Kopf: vier Geräte; jede Zeile hat eine Zelle je Spalte', () => {
    expect(table.head).toEqual(['', 'Controller', 'Tastatur 1', 'Tastatur 2', 'Touch']);
    expect(table.rows.map((r) => r[0])).toEqual(['Münzen geben (halten)', 'Sprinten', 'Laufen', 'Schlag', 'Skill 1', 'Skill 2', 'Skill 3', 'Skill 4', 'Skill-Menü', 'Optionen', 'Vollbild']);
    expect(table.rows.every((r) => r.length === table.head.length && r.every((c) => c !== ''))).toBe(true);
  });

  it('Schlag und Skills kommen aus slotBindings', () => {
    const pad = slotBindings('pad'), kb = slotBindings('keyboard'), touch = slotBindings('touch');
    ['Schlag', 'Skill 1', 'Skill 2', 'Skill 3', 'Skill 4', 'Skill-Menü'].forEach((name, i) => {
      expect(row(name)[1]).toBe(pad[i]!.label);
      expect(row(name)[2]).toBe(kb[i]!.label);
      expect(row(name)[4]).toBe(touch[i]!.label);
    });
  });

  it('Tastatur 1 und 2 aus den Layouts: Bestätigen, Sprinten, Laufen', () => {
    expect(row('Münzen geben (halten)').slice(1)).toEqual(['A', 'Leertaste', 'Enter', '🪙']);
    expect(row('Sprinten').slice(1)).toEqual(['RT', keyLabel(KEYBOARD_1.sprint[0]!), keyLabel(KEYBOARD_2.sprint[0]!), '»']);
    expect(row('Laufen')[3]).toBe('← / →');
    expect(row('Skill 1')[3]).toBe('Num 1');
  });

  it('Optionen und Vollbild: Tastatur 2 hat keine Taste', () => {
    expect(row('Optionen').slice(1)).toEqual(['MENU', 'Esc', '–', '☰']);
    expect(row('Vollbild')[3]).toBe('–');
  });

  it('Lage: Tabelle passt in 1080 px, Schrift ≥ 28 px (Q03)', () => {
    const L = KEYS_LAYOUT;
    expect(L.px).toBeGreaterThanOrEqual(28);
    expect(L.top + table.rows.length * L.rowH + L.px / 2).toBeLessThan(1080 - 60 - 14);
  });
});
