import { describe, expect, it } from 'vitest';
import { exampleTuning } from '../api/mockBalance';
import { EMPTY_FORM, formFromSpec, isPresetList, specFromForm } from './spec';
import { bestIndex, paramError, scoreBars, scoreSeries, tuneFromForm, type TuneForm } from './tune';

const form = (t: Partial<TuneForm>): TuneForm => ({ mode: 'sweep', params: 'buildings.json:wall.cost=1..5', steps: '', budget: '', ...t });

describe('Abstimm-Auftrag aus dem Formular', () => {
  it('ohne Modus kein Auftrag und kein Fehler', () => {
    expect(tuneFromForm(form({ mode: '' }))).toEqual({});
  });

  it('baut Auftrag, lässt leere Schritte und Budget weg', () => {
    expect(tuneFromForm(form({})).tune).toEqual({ mode: 'sweep', params: ['buildings.json:wall.cost=1..5'] });
    const t = tuneFromForm(form({ mode: 'search', params: 'a.json:x=0..1\n b.json:y[2]=-3..4.5 ', steps: '7', budget: '12' })).tune;
    expect(t).toEqual({ mode: 'search', params: ['a.json:x=0..1', 'b.json:y[2]=-3..4.5'], steps: 7, budget: 12 });
  });

  it('prüft wie Go', () => {
    expect(tuneFromForm(form({ params: '' })).error).toMatch(/1 bis 2 Parameter/);
    expect(tuneFromForm(form({ params: 'a.json:x=0..1\nb.json:y=0..1\nc.json:z=0..1' })).error).toMatch(/1 bis 2 Parameter/);
    expect(paramError('a.json:x=1')).toMatch(/min < max/);
    expect(paramError('a.json:x=5..1')).toMatch(/min < max/);
    expect(paramError('a.json:x=1..1')).toMatch(/min < max/);
    expect(paramError('../a.json:x=1..2')).toBeDefined();
    expect(paramError('a.json:x..y=1..2')).toBeDefined();
    expect(paramError('dir/a.json:x=1..2')).toBeDefined();
    expect(paramError('a.json:x=1..2')).toBeUndefined();
    expect(tuneFromForm(form({ steps: '1' })).error).toMatch(/Schritte/);
    expect(tuneFromForm(form({ steps: '22' })).error).toMatch(/Schritte/);
    expect(tuneFromForm(form({ budget: '0' })).error).toMatch(/Budget/);
    expect(tuneFromForm(form({ budget: '1001' })).error).toMatch(/Budget/);
  });

  it('wandert über specFromForm und zurück', () => {
    const { spec } = specFromForm({ ...EMPTY_FORM, tune: form({ steps: '4' }) });
    expect(spec?.tune).toEqual({ mode: 'sweep', params: ['buildings.json:wall.cost=1..5'], steps: 4 });
    expect(formFromSpec(spec).tune).toEqual(form({ steps: '4' }));
    expect(specFromForm({ ...EMPTY_FORM, tune: form({ params: 'x' }) }).error).toMatch(/Parameter/);
    expect(isPresetList([{ name: 'a', form: { ...EMPTY_FORM } }])).toBe(true);
    expect(isPresetList([{ name: 'a', form: { ...EMPTY_FORM, tune: undefined } }])).toBe(false);
  });
});

describe('Abstimm-Ergebnis aufbereiten', () => {
  it('eine Linie Score über Wert, sortiert', () => {
    const t = exampleTuning(1);
    t.points.reverse();
    const [s] = scoreSeries(t);
    expect(s.points.map((p) => p.x)).toEqual([0.2, 0.4, 0.6, 0.8, 1]);
    expect(Math.min(...s.points.map((p) => p.y))).toBe(0);
    expect(scoreSeries(exampleTuning(2))).toEqual([]);
  });

  it('Balken je Punkt bei zwei Parametern, bester Punkt gefunden', () => {
    const t = exampleTuning(2);
    const bars = scoreBars(t);
    expect(bars).toHaveLength(t.points.length);
    expect(bars.find((b) => b.tone === 'ok')?.label).toBe('0.6 / 0.5');
    expect(bestIndex(t)).toBe(t.points.findIndex((p) => p.pass));
  });
});
