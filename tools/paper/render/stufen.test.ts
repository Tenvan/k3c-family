// RD1.1: Gebäude-Stufen (kumulativ), Varianten und die Liste der Ausgabedateien.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ausgaben, stufenSpec } from './stufen';

const mauer = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/mauer.paper.json'), 'utf8')) as Spec;
const basis: Spec = {
  id: 'turm', rig: 'gebaeude', version: 1, beschreibung: '',
  teile: [{ id: 'korpus', pivot: [0, 0], lagen: [{ art: 'flaeche', form: 'M0 0H1V1Z', material: 'stein' }] }],
  stufen: [
    { zusaetze: [{ id: 'dach', eltern: 'korpus', pivot: [0, 0], lagen: [{ art: 'flaeche', form: 'M0 0H1Z', material: 'holz' }] }] },
    { tausch: { stein: 'gold', holz: 'leder' } },
  ],
  animationen: { stand: { frames: 1 } },
  varianten: { v: { tausch: { stein: 'purpur' } } },
  ausgabe: { hoehe_px: 32, rahmen: [0, 0, 1, 1] },
};

describe('Stufen', () => {
  it('Stufe n wendet stufen[0..n-2] kumulativ an: tausch und zusaetze', () => {
    expect(stufenSpec(basis, 1).teile.map((t) => t.id)).toEqual(['korpus']);
    expect(stufenSpec(basis, 2).teile.map((t) => t.id)).toEqual(['korpus', 'dach']);
    const s3 = stufenSpec(basis, 3);
    expect(s3.teile.map((t) => t.lagen[0].material)).toEqual(['gold', 'leder']);
    expect(basis.teile).toHaveLength(1);
  });

  it('Ausgabedateien: stand.png, stand.s2.png, stand@v.png, stand@v.s2.png', () => {
    expect(ausgaben(basis).map((a) => a.schluessel))
      .toEqual(['stand', 'stand.s2', 'stand.s3', 'stand@v', 'stand@v.s2', 'stand@v.s3']);
    expect(ausgaben(mauer).map((a) => a.schluessel)).toEqual(['stand', 'stand.s2']);
    expect(ausgaben(basis)[4]).toEqual({ schluessel: 'stand@v.s2', animation: 'stand', variante: 'v', stufe: 2 });
  });

  it('Animationen alphabetisch; ohne animationen gilt stand mit einem Frame', () => {
    const s = { ...basis, stufen: undefined, varianten: undefined, animationen: { zz: { frames: 2 }, aa: { frames: 1 } } };
    expect(ausgaben(s).map((a) => a.schluessel)).toEqual(['aa', 'zz']);
    expect(ausgaben({ ...s, animationen: undefined }).map((a) => a.schluessel)).toEqual(['stand']);
  });
});
