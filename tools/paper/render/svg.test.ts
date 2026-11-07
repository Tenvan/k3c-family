// RD1.1: Lagen und Teile als SVG mit den Papier-Filtern aus der Stil-Datei.
import { num } from '../schema/hash';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { ladeStil } from './stil';
import { defs, dichteFaktor, lageSvg, seed, teileSvg } from './svg';

const stil = ladeStil();
const spec = (teile: Spec['teile']): Spec => ({
  id: 'probe', rig: 'requisite', version: 1, beschreibung: '', teile, ausgabe: { hoehe_px: 32, rahmen: [0, 0, 1, 1] },
});

describe('Lagen', () => {
  it('flaeche: Pfad mit Füllung und filter ds, auflage mit ds2', () => {
    expect(lageSvg({ art: 'flaeche', form: 'M0 0L1 1Z', material: 'stein' }, stil))
      .toBe('<path d="M0 0L1 1Z" fill="#9a9386" filter="url(#ds)"/>');
    expect(lageSvg({ art: 'flaeche', form: 'M0 0L1 1Z', material: 'stein', ton: 'schatten', auflage: true }, stil))
      .toBe('<path d="M0 0L1 1Z" fill="#6b6559" filter="url(#ds2)"/>');
  });

  it('falz: ohne Füllung, stroke-width = breite, opacity = falz_deckkraft, runde Enden', () => {
    expect(lageSvg({ art: 'falz', form: 'M0 0L4 0', breite: 1.25, material: 'linie' }, stil))
      .toBe('<path d="M0 0L4 0" fill="none" stroke="#2a1d2e" stroke-width="1.25" stroke-linecap="round" opacity="0.25"/>');
  });

  it('punkt: circle ohne Filter', () => {
    expect(lageSvg({ art: 'punkt', mitte: [1.234, -2], radius: 0.5, material: 'auge' }, stil))
      .toBe('<circle cx="1.23" cy="-2" r="0.5" fill="#3b2a20" stroke="none"/>');
  });
});

describe('Teile und Filter', () => {
  it('Zahlen im SVG über num(): höchstens 2 Nachkommastellen, nie -0', () => {
    const s = spec([{ id: 'a', pivot: [0.005, -0.001], lagen: [{ art: 'punkt', mitte: [0, 0], radius: 1, material: 'auge' }] }]);
    const svg = teileSvg(s, stil, { a: 10.123456 });
    expect(svg).toContain('transform="rotate(10.12 0.01 0)"');
    expect(svg).not.toMatch(/-0[ ")]/);
    expect(svg).not.toMatch(/\d\.\d{3}/);
  });

  it('Teile ohne Winkel bekommen kein transform; Kinder tragen die Kette der Eltern', () => {
    const s = spec([
      { id: 'kind', eltern: 'vater', pivot: [1, 1], lagen: [{ art: 'punkt', mitte: [0, 0], radius: 1, material: 'auge' }] },
      { id: 'vater', pivot: [0, 0], lagen: [{ art: 'punkt', mitte: [0, 0], radius: 1, material: 'auge' }] },
    ]);
    const svg = teileSvg(s, stil, { vater: 90, kind: 5 });
    expect(svg.indexOf('rotate(90 0 0) rotate(5 1 1)')).toBeLessThan(svg.indexOf('<g transform="rotate(90 0 0)">'));
    expect(teileSvg(s, stil, {})).not.toContain('transform');
  });

  it('Teile ohne Lagen erzeugen nichts außer dem Einschub', () => {
    const s = spec([{ id: 'leer', pivot: [0, 0], lagen: [] }]);
    expect(teileSvg(s, stil, {})).toBe('');
    expect(teileSvg(s, stil, {}, { einschub: (id) => `<!--${id}-->` })).toBe('<!--leer-->');
  });

  it('Seed der Papierstruktur hängt nur an der Asset-ID', () => {
    expect(seed('mauer')).toBe(seed('mauer'));
    expect(seed('mauer')).not.toBe(seed('monarch'));
    for (const id of ['a', 'mauer', 'monarch', 'pferd']) {
      expect(seed(id)).toBeGreaterThanOrEqual(1);
      expect(seed(id)).toBeLessThanOrEqual(1000);
    }
    expect(defs(stil, 'mauer')).toContain(`seed="${seed('mauer')}"`);
  });

  it('defs: Schatten und Papierstruktur mit Werten der Stil-Datei', () => {
    const d = defs(stil, 'mauer');
    expect(d).toContain('<feDropShadow dx="0.5" dy="0.9" stdDeviation="0.45" flood-color="#1a0f14" flood-opacity="0.55"/>');
    expect(d).toContain('<feDropShadow dx="0.3" dy="0.5" stdDeviation="0.3" flood-color="#1a0f14" flood-opacity="0.35"/>');
    expect(d).toContain('baseFrequency="1.3" numOctaves="2"');
    expect(d).toContain('slope="0.18" intercept="0.86"');
  });

  it('defs: bei der Bezugsdichte (3 px je Einheit) Prototyp-Werte, bei 1,27 px je Einheit umgerechnet (RD2.1)', () => {
    expect(defs(stil, 'mauer', 3)).toBe(defs(stil, 'mauer'));
    const d = defs(stil, 'mauer', 192 / 151), k = 3 / (192 / 151);
    expect(dichteFaktor(stil, 192 / 151)).toBeCloseTo(k, 6);
    expect(d).toContain(`dx="${num(0.5 * k)}" dy="${num(0.9 * k)}" stdDeviation="${num(0.45 * k)}"`);
    expect(d).toContain(`baseFrequency="${num(1.3 / k)}"`);
    expect(d).toContain('flood-opacity="0.55"');
  });

  it('dichteFaktor: nie unter 1 (große Ausgaben) und höchstens 4 (winzige Ausgaben)', () => {
    expect(dichteFaktor(stil, 10)).toBe(1);
    expect(dichteFaktor(stil, 0.1)).toBe(4);
    expect(dichteFaktor(stil, undefined)).toBe(1);
  });
});
