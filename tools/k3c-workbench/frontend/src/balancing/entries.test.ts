import { describe, expect, it } from 'vitest';
import type { BalanceBaseline, BalanceReportInfo, BalanceRun } from '../api';
import { buildEntries, dayGroup, entryKind, entryLabel, entryLine2, entryMeta, groupByDay, resolveSelection, selectionAfterRemoval } from './entries';
import { leverLine, leverValue } from './variant';

// B-660: Einträge der Liste (Zusammenführen, Gruppen, Namen) und Varianten aus Hebeln.

const NOW = new Date(2026, 9, 29, 15, 0);
const at = (days: number, hour = 10) => new Date(2026, 9, 29 - days, hour, 0).toISOString();

const run = (id: string, over: Partial<BalanceRun> = {}): BalanceRun => ({
  id, label: id, where: '', state: 'läuft', phase: 'rechnen', done: 1, total: 4, started: at(0, 14), verdict: '', report: '', lines: [], ...over,
});
const report = (path: string, over: Partial<BalanceReportInfo> = {}): BalanceReportInfo => ({
  path, run: 'simtest-run-1', modified: at(0, 12), seeds: 8, pass: true, kind: 'balance', targetsPass: 3, targetsTotal: 16, ...over,
});

describe('Einträge', () => {
  it('führt einen Lauf mit seinem Bericht zusammen und sortiert aktive Läufe nach oben', () => {
    const entries = buildEntries(
      [run('run-1', { state: 'fertig', report: 'reports/a.json' }), run('run-2'), run('run-3', { state: 'wartet' })],
      [report('reports/a.json'), report('reports/b.json', { modified: at(1) })], [],
    );
    expect(entries.map((e) => e.id)).toEqual(['run:run-2', 'run:run-3', 'reports/a.json', 'reports/b.json']);
  });

  it('ein laufender Lauf mit alter Lauf-ID eines früheren Berichts bleibt sichtbar', () => {
    const entries = buildEntries([run('run-1')], [report('reports/old.json', { runId: 'run-1', modified: at(3) })], []);
    expect(entries.map((e) => e.id)).toContain('run:run-1');
  });

  it('der Bericht ersetzt die Auswahl des Laufs; unbekannte Auswahl wird leer', () => {
    const done = run('run-1', { state: 'fertig', report: 'reports/a.json' });
    const entries = buildEntries([done], [report('reports/a.json')], []);
    expect(resolveSelection('run:run-1', [done], entries)).toBe('reports/a.json');
    expect(resolveSelection('reports/a.json', [done], entries)).toBe('reports/a.json');
    expect(resolveSelection('reports/weg.json', [], entries)).toBe('');
  });

  it('nach dem Löschen springt die Auswahl auf den nächsten, sonst den vorigen Eintrag', () => {
    const entries = buildEntries([], [report('a', { modified: at(0, 12) }), report('b', { modified: at(0, 11) }), report('c', { modified: at(0, 10) })], []);
    expect(selectionAfterRemoval(entries, ['b'], 'b')).toBe('c');
    expect(selectionAfterRemoval(entries, ['b', 'c'], 'b')).toBe('a');
    expect(selectionAfterRemoval(entries, ['a', 'b', 'c'], 'b')).toBe('');
    expect(selectionAfterRemoval(entries, ['c'], 'a')).toBe('a');
  });

  it('Art, Name und Zeilen', () => {
    const base: BalanceBaseline = { name: 'M1', saved: at(1), run: 'simtest-run-1', seeds: 1, seedMode: 'dev', bots: [], worlds: [], pass: true, frozen: [] };
    const spec = { bot: 'wirtschaft', players: 1, world: 'standard', days: 40, seeds: 8 };
    const [e] = buildEntries([], [report('reports/a.json', { spec, varianten: ['a.json:x=1'] })], [base]);
    expect(entryKind(e)).toBe('Baseline');
    expect(entryLabel(e, NOW)).toBe('wirtschaft · Welt standard · 12:00');
    expect(entryMeta(e)).toBe('wirtschaft · 1 Sp. · Welt standard · 40 T. · 8 Seeds');
    expect(entryLine2(e, NOW)).toBe('12:00 · 3/16 Ziele · +1 Var. · Baseline M1');
    const [old] = buildEntries([], [report('reports/balance-1.json', { run: '' })], []);
    expect(entryLabel(old, NOW)).toBe('balance-1.json');
    expect(entryMeta(old)).toBe('8 Seeds');
  });

  it('gruppiert nach Läuft, Heute, Gestern, Älter', () => {
    const entries = buildEntries([run('run-9')], [report('a'), report('b', { modified: at(1) }), report('c', { modified: at(5) })], []);
    expect(entries.map((e) => dayGroup(e, NOW))).toEqual(['Läuft', 'Heute', 'Gestern', 'Älter']);
    expect(groupByDay(entries, NOW).map((g) => `${g.name}:${g.items.length}`)).toEqual(['Läuft:1', 'Heute:1', 'Gestern:1', 'Älter:1']);
  });
});

describe('Varianten aus Hebeln', () => {
  it('rechnet den Wert ± Schritt und lässt ganze Zahlen ganz', () => {
    expect(leverValue(1, '-', 0.5)).toBe(0.5);
    expect(leverValue(300, '+', 0.2)).toBe(360);
    expect(leverValue(20, '+', 0.01)).toBe(21);
  });

  it('baut die Zeile aus Datei und Pfad oder meldet null', () => {
    const lever = { path: 'buildings.json:wall.levels.0.hp', direction: '+' as const, step: 0.2, label: 'x' };
    expect(leverLine(lever, [{ file: 'buildings.json', path: 'wall.levels.0.hp', value: 300 }])).toBe('buildings.json:wall.levels.0.hp=360');
    expect(leverLine(lever, [])).toBeNull();
  });
});
