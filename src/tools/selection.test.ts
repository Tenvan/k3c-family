import { describe, expect, it } from 'vitest';
import { formatSelection, parseSaved } from './selection';

describe('Auswahl auf den Referenzseiten', () => {
  it('formatiert die Auswahl als Satz, ohne Doppelte, in der Reihenfolge der Wahl', () => {
    expect(formatSelection('Figuren', ['goblin', 'skeleton', 'goblin'])).toBe('Auswahl Figuren: goblin, skeleton');
    expect(formatSelection('Grafiken', ['gothicvania-town/props/houses.png'])).toBe('Auswahl Grafiken: gothicvania-town/props/houses.png');
  });

  it('ohne Auswahl bleibt der Text leer', () => {
    expect(formatSelection('Figuren', [])).toBe('');
  });

  it('liest gespeicherte IDs und ignoriert kaputte oder fremde Werte', () => {
    expect(parseSaved('["a","b"]')).toEqual(['a', 'b']);
    expect(parseSaved(null)).toEqual([]);
    expect(parseSaved('kaputt')).toEqual([]);
    expect(parseSaved('{"a":1}')).toEqual([]);
    expect(parseSaved('["a",1,null,"b"]')).toEqual(['a', 'b']);
  });
});
