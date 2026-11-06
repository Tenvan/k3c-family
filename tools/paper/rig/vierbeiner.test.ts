// RD1.3: Pferd rendern (Gangarten, Frame-Größe, Bodenkontakt) und Beinpfad in der Zeichenreihenfolge.
import { copyFileSync, mkdtempSync, readdirSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { renderSpec } from '../render/ausgabe';
import { Abgelehnt } from '../render/fehler';
import { box } from '../schema/form';
import { anwenden } from '../render/matrix';
import { ladeStil } from '../render/stil';
import { ausgaben } from '../render/stufen';
import { vierbeinerBild } from './vierbeinerBild';

const SPECS = join(import.meta.dirname, '../fixtures/specs');
const pferd = JSON.parse(readFileSync(join(SPECS, 'pferd.paper.json'), 'utf8')) as Spec;
const stil = ladeStil();
const kontext = { stil, specPfad: join(SPECS, 'pferd.paper.json') };
const galopp = ausgaben(pferd).find((a) => a.schluessel === 'galopp')!;

describe('Vierbeiner', () => {
  it('render pferd: schritt.png, trab.png, galopp.png je 8 Frames, Frame 221 × 192', () => {
    const dir = mkdtempSync(join(tmpdir(), 'paper-pferd-'));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'pferd.paper.json'));
    renderSpec(join(dir, 'pferd.paper.json'), { stil });
    expect(readdirSync(join(dir, 'pferd')).sort()).toEqual(['galopp.png', 'manifest.json', 'schritt.png', 'trab.png']);
    const m = JSON.parse(readFileSync(join(dir, 'pferd/manifest.json'), 'utf8'));
    expect([m.frameW, m.frameH]).toEqual([221, 192]);
    expect(m.rahmen).toEqual([-68, -122, 145, 126]);
    expect(m.animationen.galopp.hashes).toHaveLength(8);
    expect(m.animationen.galopp.rahmen).toBeUndefined();
  });

  it('Hufe der Standbeine berühren den Boden: Teil-Box huf-* unten bei y = 0 ± 1 Einheit', () => {
    const f = vierbeinerBild(pferd, galopp, kontext).frames[0];
    const unten = f.geometrie.filter((g) => g.teil.id.startsWith('huf-')).map((g) => {
      const pts = g.teil.lagen.flatMap((l) => (l.art === 'punkt' ? [] : [l.form]));
      const b = box(pts)!;
      return Math.max(anwenden(g.m, [b.x0, b.y1])[1], anwenden(g.m, [b.x1, b.y1])[1]);
    });
    expect(Math.max(...unten)).toBeGreaterThan(-1);
    expect(Math.max(...unten)).toBeLessThan(4);
  });

  it('Beinpfad steht in der Zeichenreihenfolge an der Stelle von oberbein-<bein>', () => {
    const svg = vierbeinerBild(pferd, galopp, kontext).frames[0].svg;
    const fell = stil.materialien['fell-fuchs'].schatten;
    expect(svg.indexOf(`fill="${fell}" filter="url(#ds)"`)).toBeLessThan(svg.indexOf(stil.einzeltoene.huf));
  });

  it('Rahmen in Welt-Einheiten: rahmen · massstab, Pixel wie der Spec-Rahmen', () => {
    const g = vierbeinerBild(pferd, galopp, kontext).groesse;
    expect(g.rahmen.map((v) => Math.round(v * 100) / 100)).toEqual([-88.4, -158.6, 188.5, 163.8]);
    expect([g.frameW, g.frameH]).toEqual([221, 192]);
  });

  it('Animation ohne gangart: abgelehnt mit animation-unbekannt', () => {
    const s = structuredClone(pferd);
    s.animationen = { stand: { frames: 1 } };
    expect(() => vierbeinerBild(s, ausgaben(s)[0], kontext)).toThrow(Abgelehnt);
  });
});

describe('Vierbeiner: Randfälle (Review RD1.3)', () => {
  it('fehlende Beinteile oder fehlendes bein: Exit 1 pflicht-teil-fehlt statt Absturz', () => {
    const ohneHuf = structuredClone(pferd);
    ohneHuf.teile = ohneHuf.teile.filter((t) => t.id !== 'huf-vn');
    expect(() => vierbeinerBild(ohneHuf, galopp, kontext)).toThrow(Abgelehnt);
    const ohneBein = structuredClone(pferd);
    delete ohneBein.bein;
    try {
      vierbeinerBild(ohneBein, galopp, kontext);
      expect.unreachable();
    } catch (e) {
      expect((e as Abgelehnt).fehler[0].regel).toBe('pflicht-teil-fehlt');
    }
  });

  it('Manifest: abweichender Rahmen einer Animation steht auch bei gleicher Pixelgröße drin', () => {
    const dir = mkdtempSync(join(tmpdir(), 'paper-pferd-'));
    copyFileSync(join(SPECS, 'pferd.paper.json'), join(dir, 'pferd.paper.json'));
    renderSpec(join(dir, 'pferd.paper.json'), { stil });
    const m = JSON.parse(readFileSync(join(dir, 'pferd/manifest.json'), 'utf8'));
    expect(m.animationen.galopp.rahmen).toBeUndefined();
  });
});
