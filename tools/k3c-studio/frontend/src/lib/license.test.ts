import { describe, expect, it } from 'vitest';
import { formatSize, licenseMark, MARK_LABEL } from './license';

// B-677 (AC-03, AC-05): Lizenz-Marke einer Datei, gleiche Regeln wie im Go-Backend (Q70).
describe('Lizenz-Marke (B-677)', () => {
  it.each(['CC0 1.0', 'CC BY 4.0', 'CC-BY 3.0', 'OGA-BY 3.0'])('stuft %s als frei ein', (lizenz) => {
    expect(licenseMark(lizenz)).toBe('frei');
  });

  it.each(['selbst erzeugt (kein Credit nötig)', 'eigene Dateien', 'Public Domain'])('stuft die eigene bzw. gemeinfreie Datei "%s" als frei ein', (lizenz) => {
    expect(licenseMark(lizenz)).toBe('frei');
  });

  it('stuft Share-Alike als sa ein, auch wenn BY im Text steht', () => {
    expect(licenseMark('CC BY-SA 4.0')).toBe('sa');
    expect(licenseMark('CC-BY-SA 3.0')).toBe('sa');
  });

  it.each(['CC BY-NC 4.0', 'CC BY-ND 4.0', 'CC BY-NC-SA 4.0', 'Sampling Plus 1.0'])('sperrt %s', (lizenz) => {
    expect(licenseMark(lizenz)).toBe('gesperrt');
  });

  it('prüft leere und unbekannte Lizenzen', () => {
    expect(licenseMark('')).toBe('pruefen');
    expect(licenseMark('unbekannt')).toBe('pruefen');
    expect(licenseMark('Pixabay')).toBe('pruefen');
  });

  it('lässt das Katalog-Kennzeichen vor dem Lizenztext gelten', () => {
    expect(licenseMark('CC0 1.0', 'NC')).toBe('gesperrt');
    expect(licenseMark('CC0 1.0', 'ND')).toBe('gesperrt');
    expect(licenseMark('CC0 1.0', 'SA')).toBe('sa');
    expect(licenseMark('', 'NC')).toBe('gesperrt');
    expect(licenseMark('CC BY 4.0', 'nc')).toBe('gesperrt');
  });

  it('wertet ein leeres Kennzeichen nicht aus', () => {
    expect(licenseMark('CC BY 4.0', '')).toBe('frei');
  });

  it('hat zu jeder Marke eine Beschriftung', () => {
    expect(MARK_LABEL).toEqual({ frei: 'CC0/BY', sa: 'SA', pruefen: 'prüfen', gesperrt: 'gesperrt' });
  });
});

describe('Dateigröße (B-677)', () => {
  it('zeigt einen Strich ohne Angabe', () => {
    expect(formatSize(0)).toBe('–');
    expect(formatSize(-5)).toBe('–');
  });

  it('zeigt kleine Dateien in kB, mindestens 1 kB', () => {
    expect(formatSize(86_000)).toBe('86 kB');
    expect(formatSize(300)).toBe('1 kB');
  });

  it('zeigt ab 1 MB mit einer Nachkommastelle und Komma', () => {
    expect(formatSize(1_400_000)).toBe('1,4 MB');
    expect(formatSize(1_000_000)).toBe('1,0 MB');
  });
});
