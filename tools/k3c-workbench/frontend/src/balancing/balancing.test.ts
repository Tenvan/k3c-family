import { describe, expect, it } from 'vitest';
import { exampleSummary } from '../api/mockBalance';
import type { BalanceRun } from '../api';
import { compareSummaries, tippedRows } from './compare';
import { comparisonMarkdown, corridorText, summaryJSON, summaryMarkdown } from './export';
import { goldSeries, resourceBars, survivalSeries } from './figures';
import { dropPreset, EMPTY_FORM, formFromSpec, isPresetList, mergeRuns, progressOf, savePreset, specFromForm } from './spec';

describe('Spec aus dem Formular', () => {
  it('lässt leere Felder weg und behält Tiefe 0 (Wald)', () => {
    expect(specFromForm(EMPTY_FORM).spec).toEqual({});
    const { spec } = specFromForm({ ...EMPTY_FORM, seeds: '4', depth: '0', bot: ' standard ', vary: 'a.json:x.y=1\n\n b.json:z=2 ' });
    expect(spec).toEqual({ seeds: 4, depth: 0, bot: 'standard', vary: ['a.json:x.y=1', 'b.json:z=2'] });
  });

  it('meldet Fehler auf Deutsch', () => {
    expect(specFromForm({ ...EMPTY_FORM, seeds: '0' }).error).toMatch(/Seeds/);
    expect(specFromForm({ ...EMPTY_FORM, days: 'x' }).error).toMatch(/Tage/);
    expect(specFromForm({ ...EMPTY_FORM, vary: 'kaputt' }).error).toMatch(/datei\.json:pfad/);
  });

  it('nimmt die Welt für die ganze Insel mit und kennt alte Voreinstellungen ohne Welt (BS11)', () => {
    expect(specFromForm({ ...EMPTY_FORM, world: 'standard', players: '1', bot: 'wirtschaft', days: '40' }).spec).toEqual({ world: 'standard', players: 1, bot: 'wirtschaft', days: 40 });
    const { world: _w, ...alt } = EMPTY_FORM;
    expect(specFromForm(alt as typeof EMPTY_FORM).spec).toEqual({});
    expect(isPresetList([{ name: 'alt', form: alt }])).toBe(true);
    expect(formFromSpec({ world: 'standard' }).world).toBe('standard');
  });

  it('nimmt die Hilfen des Stufen-Tests mit (helps)', () => {
    const form = { ...EMPTY_FORM, world: 'standard', helps: 'voll' };
    expect(specFromForm(form).spec).toEqual({ world: 'standard', helps: 'voll' });
    expect(formFromSpec({ world: 'standard', helps: 'boss' }).helps).toBe('boss');
    expect(formFromSpec({}).helps).toBe('');
  });

  it('formFromSpec ist die Umkehrung', () => {
    const form = { ...EMPTY_FORM, seeds: '3', depth: '0', vary: 'a.json:x=1', label: 'L' };
    expect(formFromSpec(specFromForm(form).spec)).toEqual(form);
  });
});

describe('Voreinstellungen und Läufe', () => {
  it('speichert, ersetzt und löscht nach Namen', () => {
    const one = savePreset([], ' A ', EMPTY_FORM);
    const two = savePreset(one, 'A', { ...EMPTY_FORM, seeds: '2' });
    expect(two).toHaveLength(1);
    expect(two[0].form.seeds).toBe('2');
    expect(savePreset(two, '  ', EMPTY_FORM)).toBe(two);
    expect(dropPreset(two, 'A')).toEqual([]);
    expect(isPresetList(two)).toBe(true);
    expect(isPresetList([{ name: 'x' }])).toBe(false);
  });

  it('Fortschritt und Zusammenführen', () => {
    const run = (id: string, done: number): BalanceRun => ({ id, label: id, where: '', state: 'läuft', phase: '', done, total: 4, started: '', verdict: '', report: '', lines: [] });
    expect(progressOf(run('a', 1))).toBe(25);
    expect(progressOf({ ...run('a', 1), total: 0 })).toBe(0);
    expect(mergeRuns([run('a', 1), run('b', 0)], [run('a', 2)]).map((r) => `${r.id}${r.done}`)).toEqual(['a2', 'b0']);
  });
});

describe('Vergleich', () => {
  const a = exampleSummary();
  const b = exampleSummary(0.15, true);

  it('findet gekippte Ziele und Differenzen', () => {
    const c = compareSummaries(a, b);
    expect(c.passA).toBe(true);
    expect(c.passB).toBe(false);
    expect(tippedRows(c).map((r) => r.statusA + '>' + r.statusB)).toEqual(['knapp>verletzt']);
    const surv = c.targets[0];
    expect(surv.delta).toBeCloseTo(-0.15);
    expect(c.figures.find((r) => r.key === 'normal|1|surv')?.delta).toBeCloseTo(-0.15);
  });

  it('kennzeichnet neue und entfallene Ziele wie Go', () => {
    const c = compareSummaries({ ...a, targets: a.targets.slice(0, 1) }, { ...a, targets: a.targets.slice(1) });
    expect(c.targets.map((r) => `${r.statusA}>${r.statusB}`)).toEqual(['neu>ok', 'neu>knapp', 'ok>entfällt']);
    expect(c.targets.map((r) => r.tipped)).toEqual([false, false, true]);
  });

  it('ohne Kennzahlen keine Kennzahl-Zeilen', () => {
    expect(compareSummaries({ ...a, kennzahlen: undefined }, { ...b, kennzahlen: undefined }).figures).toEqual([]);
  });
});

describe('Export', () => {
  const s = exampleSummary(0, false, ['bot.json:a=1']);

  it('Markdown mit Zielen, Korridor und Kennzahlen', () => {
    const md = summaryMarkdown(s, 'Test');
    expect(md).toContain('# Test');
    expect(md).toContain('- Ergebnis: Pass');
    expect(md).toContain('`bot.json:a=1`');
    expect(md).toContain('| Überleben Nacht 3 | 0.7 | 0.6 … ∞ | ok |');
    expect(md).toContain('| normal | 1 | 8 |');
  });

  it('JSON ist eingerückt und lesbar zurück', () => {
    const json = summaryJSON(s);
    expect(json.endsWith('\n')).toBe(true);
    expect(JSON.parse(json)).toEqual(s);
  });

  it('Vergleichs-Markdown nennt den Statuswechsel', () => {
    const md = comparisonMarkdown(compareSummaries(exampleSummary(), exampleSummary(0.15, true)), 'alt', 'neu');
    expect(md).toContain('# Vergleich alt → neu');
    expect(md).toContain('knapp → verletzt (!)');
  });

  it('Korridor ohne Ziel', () => {
    expect(corridorText({ lower: null, upper: null })).toBe('kein Ziel');
    expect(corridorText({ lower: null, upper: 3 })).toBe('−∞ … 3');
  });
});

describe('Diagrammdaten', () => {
  const f = exampleSummary().kennzahlen!;
  it('Linien je Grad, Gold Tag/Nacht, Rohstoffe', () => {
    expect(survivalSeries(f)).toHaveLength(1);
    expect(survivalSeries(f)[0].points).toHaveLength(5);
    expect(goldSeries(f).map((x) => x.name)).toEqual(['Tag', 'Nacht']);
    expect(resourceBars(f).map((x) => x.label)).toEqual(['Holz', 'Stein', 'Kupfer', 'Eisen', 'Kristall']);
    expect(resourceBars(f)[0].value).toBeGreaterThan(0);
  });
});
