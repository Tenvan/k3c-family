import { describe, expect, it } from 'vitest';
import { deviation, parseEntry, stepText, valueKind } from './valueEdit';

describe('Werteingabe (B-1173)', () => {
  it('erkennt die Art des Basiswerts', () => {
    expect([3, true, 'x', [1], { a: 1 }].map(valueKind)).toEqual(['number', 'boolean', 'text', 'json', 'json']);
  });

  it('prüft beim Tippen: Zahl mit Komma, Ja/Nein, Text, JSON vom selben Typ', () => {
    expect(parseEntry('2,5', 3)).toEqual({ ok: true, value: 2.5 });
    expect(parseEntry('', 3)).toEqual({ ok: false, error: 'Zahl erwartet' });
    expect(parseEntry('viel', 3).ok).toBe(false);
    expect(parseEntry('Ja', false)).toEqual({ ok: true, value: true });
    expect(parseEntry('vielleicht', false).ok).toBe(false);
    expect(parseEntry(' Burg ', 'Monarch')).toEqual({ ok: true, value: ' Burg ' });
    expect(parseEntry('[3, 6]', [1, 2])).toEqual({ ok: true, value: [3, 6] });
    expect(parseEntry('5', [1, 2])).toEqual({ ok: false, error: 'Liste erwartet, z. B. [1, 2]' });
    expect(parseEntry('[1', [1, 2])).toEqual({ ok: false, error: 'kein gültiges JSON' });
    expect(parseEntry('[1]', { a: 1 }).ok).toBe(false);
  });

  it('Pfeiltasten ändern Zahlen: ganz um 1, dezimal um 0,1, Umschalt ×10, ohne Rundungsfehler', () => {
    expect(stepText('3', 3, 3, 1, false)).toBe('4');
    expect(stepText('3', 3, 3, -1, true)).toBe('-7');
    expect(stepText('0.2', 0.2, 0.5, 1, false)).toBe('0.3');
    expect(stepText('kaputt', 7, 7, 1, false)).toBe('8');
  });

  it('Abweichung: Unterschied und Prozent, leer bei gleichem Wert', () => {
    expect(deviation(3, 3)).toBe('');
    expect(deviation(3, 4)).toBe('+1 · +33 %');
    expect(deviation(40, 30)).toBe('−10 · −25 %');
    expect(deviation(0, 2)).toBe('+2');
    expect(deviation([3, 6], [4, 6])).toBe('geändert');
    expect(deviation('a', 'b')).toBe('geändert');
  });
});
