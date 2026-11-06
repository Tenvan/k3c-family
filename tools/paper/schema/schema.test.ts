// AC-03 (PV1.3): Schema und fachliche Regeln gegen die Fixtures; Herkunft 005/AC-01.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import { checkSpec, checkSpecFile, checkStil } from './check';
import { box } from './form';
import type { Spec, Stil } from './types';

const ROOT = join(import.meta.dirname, '../../..');
const SPECS = join(import.meta.dirname, '../fixtures/specs');
const lies = (p: string) => JSON.parse(readFileSync(p, 'utf8'));
const stil = lies(join(ROOT, 'public/grafik/stil.paper.json')) as Stil;
const regeln = (pfad: string) => [...new Set(checkSpecFile(pfad, stil).map((f) => f.regel))];

describe('paper-spec v1', () => {
  it('Stil-Datei ist gültig', () => {
    expect(checkStil(stil)).toEqual([]);
  });

  it.each(['monarch', 'pferd', 'mauer'])('Fixture %s ist gültig', (id) => {
    expect(checkSpecFile(join(SPECS, `${id}.paper.json`), stil)).toEqual([]);
  });

  it.each([
    ['hex-farbe', 'hex-farbe'],
    ['unbekanntes-feld', 'schema'],
    ['fremdes-material', 'material-unbekannt'],
    ['pflicht-teil-fehlt', 'pflicht-teil-fehlt'],
    ['proportion', 'proportion'],
  ])('ungültig/%s liefert genau die Regel %s', (datei, regel) => {
    expect(regeln(join(SPECS, 'ungueltig', `${datei}.paper.json`))).toEqual([regel]);
  });

  it('Fehler nennen Feld als Pfad, Regel und Text', () => {
    const [f] = checkSpecFile(join(SPECS, 'ungueltig', 'hex-farbe.paper.json'), stil);
    expect(f).toEqual({ feld: 'teile[0].lagen[0].material', regel: 'hex-farbe', text: expect.stringContaining('#9a9386') });
  });
});

describe('fachliche Regeln (Beispiele)', () => {
  const basis = (): Spec => lies(join(SPECS, 'mauer.paper.json'));

  it('id-dateiname, eltern-unbekannt, zyklus, teil-unbekannt, ton-ohne-toene', () => {
    const s = basis();
    expect(checkSpec(s, stil, 'andere.paper.json').map((f) => f.regel)).toEqual(['id-dateiname']);
    expect(checkSpec({ ...s, beschreibung: 'wie Ticket #12' }, stil)).toEqual([]);
    s.stufen![0].zusaetze![0].eltern = 'fehlt';
    s.stufen![0].zusaetze![1].eltern = 'zinnen';
    expect(checkSpec(s, stil).map((f) => f.regel)).toEqual(['eltern-unbekannt', 'zyklus']);
    const m: Spec = lies(join(SPECS, 'monarch.paper.json'));
    m.umhang!.anker = 'mantel';
    m.teile[0].lagen.push({ art: 'punkt', mitte: [0, 0], radius: 1, material: 'huf', ton: 'licht' });
    expect(checkSpec(m, stil).map((f) => f.regel).sort()).toEqual(['teil-unbekannt', 'ton-ohne-toene']);
  });

  it('bein nur am vierbeiner, stufen nur am gebaeude', () => {
    const s = { ...basis(), rig: 'requisite' };
    expect(checkSpec(s, stil).map((f) => f.regel)).toContain('schema');
  });

  it('Einzelton und Material dürfen nicht gleich heißen', () => {
    const s = structuredClone(stil);
    s.einzeltoene.stein = '#000000';
    expect(checkStil(s).map((f) => f.regel)).toEqual(['material-doppelt']);
  });

  it('Rahmen eines Pfads mit relativen Befehlen', () => {
    expect(box(['M0 0h10v-5l-2 -1z', 'M1 1C2 2 3 3 4 -9'])).toEqual({ x0: 0, y0: -9, x1: 10, y1: 3 });
  });
});
