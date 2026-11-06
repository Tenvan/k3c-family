// RD1.1: 2D-Matrizen und Teile-Baum (Kinder erben die Drehung des Elternteils um dessen Pivot).
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { weltMatrizen } from './baum';
import { anwenden, drehung, EINHEIT, mul, skaliere, verschiebe } from './matrix';

const spec = (teile: Spec['teile']): Spec => ({
  id: 't', rig: 'requisite', version: 1, beschreibung: '', teile, ausgabe: { hoehe_px: 32, rahmen: [0, 0, 1, 1] },
});
const nah = (p: [number, number], q: [number, number]) => {
  expect(p[0]).toBeCloseTo(q[0], 9);
  expect(p[1]).toBeCloseTo(q[1], 9);
};

describe('Matrix', () => {
  it('drehung positiv = im Uhrzeigersinn (y nach unten) um den Pivot', () => {
    nah(anwenden(drehung(90, 0, 0), [1, 0]), [0, 1]);
    nah(anwenden(drehung(90, 10, 0), [20, 0]), [10, 10]);
  });

  it('mul wendet erst die rechte, dann die linke Matrix an', () => {
    const m = mul(verschiebe(5, 0), skaliere(2));
    nah(anwenden(m, [1, 1]), [7, 2]);
  });
});

describe('Teile-Baum', () => {
  it('Kind erbt die Drehung des Elternteils um dessen Pivot', () => {
    const s = spec([
      { id: 'a', pivot: [0, 0], lagen: [] },
      { id: 'b', eltern: 'a', pivot: [10, 0], lagen: [] },
    ]);
    const w = weltMatrizen(s, { a: 90 });
    nah(anwenden(w.get('b')!, [20, 0]), [0, 20]);
  });

  it('Zeichenreihenfolge = Listenreihenfolge, Eltern dürfen später stehen', () => {
    const s = spec([
      { id: 'kind', eltern: 'vater', pivot: [0, 0], lagen: [] },
      { id: 'vater', pivot: [0, 0], lagen: [] },
    ]);
    const w = weltMatrizen(s, { vater: 180 });
    expect([...w.keys()]).toEqual(['kind', 'vater']);
    nah(anwenden(w.get('kind')!, [1, 0]), [-1, 0]);
  });

  it('ohne Winkel ist jede Matrix die Einheit; basis wirkt auf alle Teile', () => {
    const s = spec([{ id: 'a', pivot: [3, 4], lagen: [] }]);
    expect(weltMatrizen(s, {}).get('a')).toEqual(EINHEIT);
    nah(anwenden(weltMatrizen(s, {}, verschiebe(0, 7)).get('a')!, [1, 1]), [1, 8]);
  });
});
