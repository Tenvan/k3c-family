import { describe, expect, it } from 'vitest';
import type { BalanceDataFileValue } from '../../api';
import { EXAMPLES, formFromExample } from '../help';
import { EMPTY_FORM, formFromSpec, specFromForm } from '../spec';
import { durationText, formatMinutes, minutesPerSeed, seedCount, summaryText, workOf } from './estimate';
import { buildIndex, checkRows } from './variantCheck';
import { buildSpec, formWithRows, normalizeForm, rowsFromForm } from './variantForm';
import { countKinds, deviation, newRow, parseLine, rowsFromText, rowsToText, switchKind } from './variants';

// B-661/AC-02 bis AC-04 (WZ32.4): Varianten-Modell, Prüfung, Text ↔ Tabelle ↔ Auftrag.

const DATA: BalanceDataFileValue[] = [
  { file: 'economy.json', path: 'purse.startGold', value: 14 },
  { file: 'economy.json', path: 'gatherables.tree.markCost', value: 1 },
  { file: 'economy.json', path: 'dawnGoldPerPlayer', value: 0 },
  { file: 'buildings.json', path: 'wall.cost.gold', value: 2 },
  { file: 'buildings.json', path: 'tower.cost.gold', value: 4 },
  { file: 'waves.json', path: 'perExtraPlayer', value: 0.25 },
];
const INDEX = buildIndex(DATA);
const check = (...lines: string[]) => checkRows(lines.map(parseLine), INDEX);

describe('Basis und Abweichung (AC-02)', () => {
  it('fest: (neu − Basis) / Basis, Bereich als Spanne, Basis 0 ergibt „–“', () => {
    expect(deviation(parseLine('economy.json:gatherables.tree.markCost=0.5'), 1)).toEqual({ text: '−50 %', tone: 'down' });
    expect(deviation(parseLine('buildings.json:wall.cost.gold=3'), 2)).toEqual({ text: '+50 %', tone: 'up' });
    expect(deviation(parseLine('buildings.json:wall.cost.gold=2'), 2).text).toBe('±0 %');
    expect(deviation(parseLine('buildings.json:wall.cost.gold=1..4'), 2)).toEqual({ text: '−50 % … +100 %', tone: 'range' });
    expect(deviation(parseLine('economy.json:dawnGoldPerPlayer=3'), 0).text).toBe('–');
    expect(deviation(parseLine('economy.json:purse.startGold=abc'), 14).text).toBe('–');
  });

  it('Umschalten fest ↔ Bereich ohne Neueingabe (Bereich ±25 % um die Basis wie rangeAround)', () => {
    const fest = newRow('buildings.json', 'wall.cost.gold', 2, 'fest');
    const range = switchKind({ ...fest, lo: '', hi: '' }, 'bereich', 2);
    expect([range.kind, range.lo, range.hi]).toEqual(['bereich', '2', '3']);
    const back = switchKind(range, 'fest', 2);
    expect([back.kind, back.value]).toEqual(['fest', '2']);
    expect(switchKind(parseLine('economy.json:dawnGoldPerPlayer=1'), 'bereich', 0)).toMatchObject({ lo: '0', hi: '1' });
  });

  it('prüft gegen data/: falscher Pfad mit Vorschlag, falsche Datei, Zahl, von ≥ bis', () => {
    const [path, file, nan, comma, order, ok] = check(
      'buildings.json:tower.cost.gld=3', 'building.json:wall.cost.gold=3', 'economy.json:purse.startGold=viel',
      'economy.json:gatherables.tree.markCost=1,5', 'buildings.json:wall.cost.gold=4..2', 'buildings.json:tower.cost.gold=3',
    );
    expect(path.error).toBe('Pfad unbekannt. Meinten Sie tower.cost.gold (4)?');
    expect(path.fix).toEqual({ file: 'buildings.json', path: 'tower.cost.gold', base: 4 });
    expect(file.fix).toMatchObject({ file: 'buildings.json', path: 'wall.cost.gold' });
    expect(nan.error).toContain('kein Zahlenwert');
    expect(comma.error).toContain('Dezimalpunkt');
    expect(order.error).toContain('kleiner');
    expect(ok).toMatchObject({ base: 4 });
    expect(ok.error).toBeUndefined();
  });

  it('ohne geladene Werte prüft sie nur die Form; unlesbare Zeilen und Doppelte sind Fehler', () => {
    const rows = ['economy.json:x.y=1', 'kaputt', 'economy.json:x.y=2', 'a/b.json:p=1'].map(parseLine);
    const out = checkRows(rows, null);
    expect(out[0].error).toBeUndefined();
    expect(out[1].error).toContain('nicht lesbar');
    expect(out[2].error).toContain('schon');
    expect(out[3].error).toContain('Form');
  });

  it('weist auf ganze Zahlen hin, wenn ein Bereich Dezimalwerte nimmt', () => {
    expect(check('buildings.json:wall.cost.gold=1.5..3')[0].note).toContain('ganze Zahlen');
    expect(check('waves.json:perExtraPlayer=0.1..0.5')[0].note).toBeUndefined();
  });
});

describe('Bereiche (AC-03)', () => {
  it('ein Bereich macht aus dem Auftrag eine Abstimmung (Modus Suche, Budget)', () => {
    const rows = rowsFromText('economy.json:purse.startGold=20\nbuildings.json:wall.cost.gold=1..4');
    const form = { ...EMPTY_FORM, tune: { ...EMPTY_FORM.tune, budget: '20' } };
    const { spec, error } = buildSpec(form, rows, false);
    expect(error).toBeUndefined();
    expect(spec).toEqual({ vary: ['economy.json:purse.startGold=20'], tune: { mode: 'search', params: ['buildings.json:wall.cost.gold=1..4'], budget: 20 } });
  });

  it('Sweep nimmt Schritte, nicht Budget', () => {
    const rows = rowsFromText('buildings.json:wall.cost.gold=1..4');
    const form = { ...EMPTY_FORM, tune: { mode: 'sweep' as const, params: '', steps: '3', budget: '20' } };
    expect(buildSpec(form, rows, false).spec?.tune).toEqual({ mode: 'sweep', params: ['buildings.json:wall.cost.gold=1..4'], steps: 3 });
  });

  it('drei Bereiche sperren: die dritte Zeile ist falsch', () => {
    const out = check('economy.json:purse.startGold=1..30', 'buildings.json:wall.cost.gold=1..4', 'buildings.json:tower.cost.gold=2..9');
    expect(out.map((c) => c.error)).toEqual([undefined, undefined, 'Höchstens zwei Bereiche (Grenze der Abstimmung).']);
    expect(countKinds(rowsFromText('a.json:b=1..2\nc.json:d=3')).bereich).toBe(1);
  });

  it('„je Variante“ gilt nur für feste Varianten ohne Bereich', () => {
    const fest = rowsFromText('economy.json:purse.startGold=20\nbuildings.json:wall.cost.gold=3');
    expect(buildSpec(EMPTY_FORM, fest, true).spec?.perVariant).toBe(true);
    expect(buildSpec(EMPTY_FORM, [...fest, parseLine('waves.json:perExtraPlayer=0.1..0.5')], true).spec?.perVariant).toBeUndefined();
    expect(buildSpec(EMPTY_FORM, [], true).spec?.perVariant).toBeUndefined();
  });
});

describe('Text und Tabelle (AC-04)', () => {
  const TEXT = 'economy.json:purse.startGold=120\nbuildings.json:wall.cost.gold=2..6\nwaves.json:perExtraPlayer=0.5';

  it('Text → Tabelle → Text ist verlustfrei, auch mit unlesbaren Zeilen', () => {
    expect(rowsToText(rowsFromText(TEXT))).toBe(TEXT);
    const messy = 'economy.json:purse.startGold=\nkaputt\n  waves.json:x=1..z ';
    expect(rowsToText(rowsFromText(messy))).toBe('economy.json:purse.startGold=\nkaputt\nwaves.json:x=1..z');
    expect(rowsFromText(TEXT).map((r) => r.kind)).toEqual(['fest', 'bereich', 'fest']);
  });

  it('Zeilen → Formular → Zeilen liefert dieselben Zeilen', () => {
    const rows = rowsFromText(TEXT);
    const form = formWithRows(EMPTY_FORM, rows);
    expect(form.vary).toBe('economy.json:purse.startGold=120\nwaves.json:perExtraPlayer=0.5');
    expect(rowsToText(rowsFromForm(form))).toBe('economy.json:purse.startGold=120\nwaves.json:perExtraPlayer=0.5\nbuildings.json:wall.cost.gold=2..6');
  });

  it('erzeugt für alle Beispiele denselben Auftrag wie das alte Formular', () => {
    for (const e of EXAMPLES) {
      const form = formFromExample(e, EMPTY_FORM);
      expect(buildSpec(form, rowsFromForm(form), false), e.name).toEqual(specFromForm(form));
    }
  });

  it('Eingaben aus dem Formular: Seeds und Seed-Liste schließen sich weiter aus', () => {
    const form = { ...EMPTY_FORM, seeds: '4', seedList: 'a,b' };
    expect(buildSpec(form, [], false).error).toBe('Seeds oder Seed-Liste, nicht beides');
  });

  it('Voreinstellungen ohne neuere Felder laden mit Vorgaben', () => {
    const old = { seeds: '5', seedList: '', depth: '', players: '', bot: '', grade: '', days: '', vary: 'economy.json:purse.startGold=9', label: 'alt', tune: { mode: '', params: '', steps: '', budget: '' } };
    const form = normalizeForm(old as never);
    expect(form).toMatchObject({ world: '', helps: '', baseline: '', seeds: '5' });
    expect(rowsFromForm(form).map((r) => r.value)).toEqual(['9']);
    expect(normalizeForm(formFromSpec({ seeds: 3, world: 'standard' }))).toMatchObject({ seeds: '3', world: 'standard' });
  });
});

describe('Zusammenfassung und Dauer', () => {
  const run = (id: string, min: number, spec: object) => ({ id, state: 'fertig', started: '2026-10-29T10:00:00Z', ended: new Date(Date.parse('2026-10-29T10:00:00Z') + min * 60000).toISOString(), spec }) as never;

  it('Faustregel nur für eine Welt, sonst unbekannt', () => {
    const world = { ...EMPTY_FORM, world: 'standard', seeds: '2', days: '40' };
    expect(durationText([], world, workOf(world, { fest: 0, bereich: 0 }, false))).toBe('etwa 2 min bis 8 min (Faustregel)');
    expect(durationText([], EMPTY_FORM, workOf(EMPTY_FORM, { fest: 0, bereich: 0 }, false))).toBe('Dauer unbekannt');
  });

  it('nimmt frühere Läufe gleicher Welt und Tage', () => {
    const form = { ...EMPTY_FORM, world: 'standard', seeds: '4', days: '40' };
    const runs = [run('a', 6, { world: 'standard', days: 40, seeds: 2 }), run('b', 9, { world: 'standard', days: 10, seeds: 1 })];
    expect(minutesPerSeed(runs, form)).toBe(3);
    expect(durationText(runs, form, workOf(form, { fest: 0, bereich: 0 }, false))).toBe('etwa 12 min (nach früheren Läufen)');
  });

  it('Umfang: Suche bis Budget, Sweep Raster, je Variante n + 1', () => {
    expect(workOf(EMPTY_FORM, { fest: 0, bereich: 1 }, false)).toEqual({ count: 40, text: 'bis zu 40 Auswertungen' });
    const sweep = { ...EMPTY_FORM, tune: { mode: 'sweep' as const, params: '', steps: '3', budget: '' } };
    expect(workOf(sweep, { fest: 0, bereich: 2 }, false).count).toBe(9);
    expect(workOf(EMPTY_FORM, { fest: 3, bereich: 0 }, true)).toEqual({ count: 4, text: '4 Läufe' });
    expect(seedCount({ ...EMPTY_FORM, seedList: 'a, b,c' })).toBe(3);
    expect(formatMinutes(80)).toBe('1 h 20 min');
    expect(summaryText([], { ...EMPTY_FORM, seeds: '8', players: '1', days: '40' }, { count: 1, text: '' })).toContain('8 Seeds × 1 Spieler × 40 Tage');
  });
});
