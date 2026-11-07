// RD1.4: frame, diff und pixeldiff des echten Renderers im Detail (Vertrag paper-spec.md › 8).
import { copyFileSync, mkdtempSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { run } from '../bin/paper';
import { groesse } from '../render/png';
import { rastere } from '../render/raster';
import { diffSpecs } from './diff';
import { pixeldiff } from './pixeldiff';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const lies = (id: string) => JSON.parse(readFileSync(join(SPECS, `${id}.paper.json`), 'utf8')) as Spec;
const tmp = () => mkdtempSync(join(tmpdir(), 'paper-cli-'));
const json = (args: string[]) => JSON.parse(run(args).stdout);

describe('frame', { timeout: 60_000 }, () => {
  it('frame monarch idle 0: PNG frameW × frameH, gleich dem Ausschnitt 0 von idle.png', () => {
    const dir = tmp();
    copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer.paper.json'));
    const r = json(['frame', join(dir, 'mauer.paper.json'), '--animation', 'stand', '--frame', '0', '--stufe', '2', '--out', join(dir, 'f.png')]);
    expect(r.ok).toBe(true);
    expect(groesse(readFileSync(join(dir, 'f.png')))).toEqual({ breite: 146, hoehe: 128 });
    run(['render', join(dir, 'mauer.paper.json')]);
    expect(readFileSync(join(dir, 'f.png')).equals(readFileSync(join(dir, 'mauer/stand.s2.png')))).toBe(true);
  });

  it('teile: Box der transformierten Lagen in Pixeln des Frames; Teile ohne Lagen fehlen', () => {
    const dir = tmp();
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'pferd.paper.json'));
    const r = json(['frame', join(dir, 'pferd.paper.json'), '--animation', 'galopp', '--frame', '3', '--out', join(dir, 'f.png')]);
    expect(r.teile['oberbein-vn']).toBeUndefined();
    const [x, y, b, h] = r.teile.hals;
    expect(x).toBeGreaterThan(100);
    expect(y).toBeGreaterThanOrEqual(0);
    expect(b).toBeGreaterThan(10);
    expect(h).toBeGreaterThan(10);
  });

  it('--variante und --stufe wirken wie im Ausgabe-Layout; Frame außerhalb → Exit 1', () => {
    const dir = tmp();
    copyFileSync(join(SPECS, 'monarch.paper.json'), join(dir, 'monarch.paper.json'));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'pferd.paper.json'));
    const a = join(dir, 'a.png'), b = join(dir, 'b.png');
    run(['frame', join(dir, 'monarch.paper.json'), '--animation', 'idle', '--frame', '0', '--out', a]);
    run(['frame', join(dir, 'monarch.paper.json'), '--animation', 'idle', '--frame', '0', '--variante', 'spieler2', '--out', b]);
    expect(readFileSync(a).equals(readFileSync(b))).toBe(false);
    const r = run(['frame', join(dir, 'monarch.paper.json'), '--animation', 'idle', '--frame', '8', '--out', a]);
    expect(r.code).toBe(1);
    expect(run(['frame', join(dir, 'monarch.paper.json'), '--animation', 'idle', '--frame', '0', '--variante', 'gibts', '--out', a]).code).toBe(1);
  }, 30_000);
});

describe('diff', { timeout: 60_000 }, () => {
  it('Teil umsortiert: kein Feld, kein Teil', () => {
    const a = lies('monarch'), b = structuredClone(a);
    b.teile.reverse();
    expect(diffSpecs(a, b)).toEqual({ felder: [], teile: [] });
  });

  it('Teil neu und gelöscht: in teile', () => {
    const a = lies('monarch'), b = structuredClone(a);
    b.teile = b.teile.filter((t) => t.id !== 'schwert');
    b.teile.push({ id: 'schild', eltern: 'unterarm-v', pivot: [0, 0], lagen: [] });
    expect(diffSpecs(a, b).teile.sort()).toEqual(['schild', 'schwert']);
  });

  it('$schema zählt nicht', () => {
    const a = lies('mauer'), b = { ...structuredClone(a), $schema: 'x' } as Spec;
    expect(diffSpecs(a, b)).toEqual({ felder: [], teile: [] });
  });
});

describe('pixeldiff', { timeout: 60_000 }, () => {
  const bild = (extra: string, b = 40, h = 30) => rastere(`<svg xmlns="http://www.w3.org/2000/svg" width="${b}" height="${h}">` +
    `<rect width="${b}" height="${h}" fill="#888"/>${extra}</svg>`).png;

  it('gleiche Bilder: anteil 0, boxen []', () => {
    expect(pixeldiff(bild(''), bild(''))).toMatchObject({ anteil: 0, boxen: [] });
  });

  it('höchstens 32 Boxen, größte zuerst', () => {
    const viele = Array.from({ length: 50 }, (_, i) => `<rect x="${(i % 10) * 16}" y="${Math.floor(i / 10) * 16}" width="${1 + (i % 3)}" height="1" fill="#f00"/>`).join('');
    const d = pixeldiff(bild('', 160, 80), bild(viele, 160, 80));
    expect(d.boxen).toHaveLength(32);
    const flaechen = d.boxen.map(([, , b, h]) => b * h);
    expect(flaechen).toEqual([...flaechen].sort((x, y) => y - x));
  });

  it('unterschiedliche Größe: fehlende Pixel zählen als Abweichung', () => {
    const d = pixeldiff(bild('', 20, 30), bild('', 40, 30));
    expect(d.anteil).toBe(0.5);
  });

  it('Overlay ist b in Originalgröße', () => {
    expect(groesse(pixeldiff(bild(''), bild('<rect width="5" height="5" fill="#f00"/>')).overlay)).toEqual({ breite: 40, hoehe: 30 });
  });
});

describe('frame: Optionswerte (Review RD1.4)', { timeout: 60_000 }, () => {
  it('--frame oder --stufe keine ganze Zahl ≥ 0: Aufruffehler Exit 2, stdout leer', () => {
    const dir = tmp();
    copyFileSync(join(SPECS, 'mauer.paper.json'), join(dir, 'mauer.paper.json'));
    const basis = ['frame', join(dir, 'mauer.paper.json'), '--animation', 'stand', '--out', join(dir, 'f.png')];
    for (const [k, v] of [['frame', 'abc'], ['frame', '1.5'], ['frame', ''], ['frame', '-1'], ['stufe', 'zwei']])
      expect(run([...basis, '--frame', '0', `--${k}`, v]), `${k}=${v}`).toMatchObject({ code: 2, stdout: '' });
  });
});
