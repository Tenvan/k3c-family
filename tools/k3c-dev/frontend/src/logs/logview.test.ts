import { describe, expect, it } from 'vitest';
import type { LogEntry, LogView } from '../api';
import { budgetNote, entryKeys, footer, levelTone, oldestFirst, oneLine, pickTab, prettyData, tabEnabled, timeWindow } from './logview';

const view = (over: Partial<LogView> = {}): LogView => ({
  entries: [], bytesRead: 2048, budgetHit: false, skipped: 0, missing: false, ...over,
});
const entry = (msg: string): LogEntry => ({ time: '2026-09-30T08:03:00Z', level: 'INFO', ns: 'mcp', msg });

describe('Reiter Log und Fehler', () => {
  it('Log und Fehler nur für Log-Dateien, gemerkter Reiter fällt auf Konsole zurück', () => {
    expect(tabEnabled('konsole', false)).toBe(true);
    expect(tabEnabled('log', true)).toBe(true);
    expect(tabEnabled('log', false)).toBe(false);
    expect(pickTab('fehler', true)).toBe('fehler');
    expect(pickTab('fehler', false)).toBe('konsole');
  });

  it('älteste oben', () => {
    expect(oldestFirst([entry('neu'), entry('mitte'), entry('alt')]).map((e) => e.msg)).toEqual(['alt', 'mitte', 'neu']);
  });

  it('Fußzeile und Budget-Hinweis', () => {
    expect(footer(view(), 1234, true)).toBe('1.234 Einträge · 2,0 KB gelesen · läuft mit');
    expect(footer(view({ bytesRead: 300 }), 3, false)).toBe('3 Einträge · 300 B gelesen');
    expect(budgetNote(view())).toBe('');
    expect(budgetNote(view({ budgetHit: true }))).toContain('ältere Treffer möglich');
  });

  it('Zeitfenster', () => {
    const t = (h: number, m: number, d = 30) => new Date(2026, 8, d, h, m).toISOString();
    expect(timeWindow(t(8, 3), t(13, 51))).toBe('08:03–13:51');
    expect(timeWindow(t(8, 3), t(8, 3))).toBe('08:03');
    expect(timeWindow(t(23, 50), t(0, 10, 31))).toBe('30.09. 23:50–01.10. 00:10');
  });

  it('Beispiel in einer Zeile, gekürzt', () => {
    expect(oneLine('erste\nzweite')).toBe('erste');
    expect(oneLine('x'.repeat(200), 10)).toBe(`${'x'.repeat(9)}…`);
  });

  it('Daten eingerückt, ohne Daten leer', () => {
    expect(prettyData({ exit: 1 })).toBe('{\n  "exit": 1\n}');
    expect(prettyData({})).toBe('');
    expect(prettyData(undefined)).toBe('');
  });

  it('Zeilenschlüssel eindeutig, auch bei gleichen Einträgen', () => {
    const keys = entryKeys([entry('a'), entry('a'), entry('b')]);
    expect(new Set(keys).size).toBe(3);
    expect(keys[0]).toBe('2026-09-30T08:03:00Z|INFO|mcp|a');
  });

  it('Level-Töne', () => {
    expect(levelTone('ERROR')).toBe('error');
    expect(levelTone('WARN')).toBe('warn');
    expect(levelTone('TRACE')).toBe('neutral');
  });
});
