// B-752: Reiter und Reittier als getrennte Ebenen; übereinandergelegt ergeben sie den Komplett-Sheet.
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import './register';
import { checkSpec } from '../schema/check';
import type { Spec } from '../schema/types';
import { bildVon } from '../render/bild';
import { type FrameGroesse, rastere, streifenSvg } from '../render/raster';
import { defs } from '../render/svg';
import { ladeStil, ROOT } from '../render/stil';
import { ausgaben } from '../render/stufen';
import { readFileSync } from 'node:fs';

const stil = ladeStil();
const pfad = (gruppe: string, id: string) => join(ROOT, 'public/grafik', gruppe, `${id}.paper.json`);
const lade = (gruppe: string, id: string) => ({ spec: JSON.parse(readFileSync(pfad(gruppe, id), 'utf8')) as Spec, specPfad: pfad(gruppe, id) });
const monarch = lade('figuren', 'monarch'), schimmel = lade('reittiere', 'schimmel');

type Blatt = { g: FrameGroesse; n: number; rgba: Uint8Array; breite: number };
function blatt(a: { spec: Spec; specPfad: string }, key: string): Blatt {
  const aus = ausgaben(a.spec).find((x) => x.schluessel === key)!;
  const bild = bildVon(a.spec, aus, { stil, specPfad: a.specPfad }), g = bild.groesse;
  const r = rastere(streifenSvg(bild.frames.map((f) => f.svg), g, defs(stil, a.spec.id, g.frameH / g.rahmen[3])));
  return { g, n: bild.frames.length, rgba: r.rgba, breite: r.breite };
}
function frame(b: Blatt, n: number): Uint8Array {
  const { frameW: w, frameH: h } = b.g, out = new Uint8Array(w * h * 4);
  for (let y = 0; y < h; y++) out.set(b.rgba.subarray((y * b.breite + n * w) * 4, (y * b.breite + n * w + w) * 4), y * w * 4);
  return out;
}

/** Größter Alpha-Wert auf dem äußersten Pixelrand eines Frames */
function randAlpha(f: Uint8Array, g: FrameGroesse): number {
  const { frameW: w, frameH: h } = g, a = (x: number, y: number) => f[(y * w + x) * 4 + 3]!;
  const rand = [...Array.from({ length: w }, (_, x) => Math.max(a(x, 0), a(x, h - 1))), ...Array.from({ length: h }, (_, y) => Math.max(a(0, y), a(w - 1, y)))];
  return Math.max(...rand);
}

/** Legt eine Ebene (Alpha-Überlagerung) nach ihrem Rahmen auf die Leinwand des Komplett-Sheets */
function auflegen(leinwand: Float32Array, ziel: FrameGroesse, L: Blatt, F: Uint8Array, px: number): void {
  const ox = Math.round((L.g.rahmen[0] - ziel.rahmen[0]) * px), oy = Math.round((L.g.rahmen[1] - ziel.rahmen[1]) * px);
  for (let y = 0; y < L.g.frameH; y++) {
    for (let x = 0; x < L.g.frameW; x++) {
      const X = x + ox, Y = y + oy, s = (y * L.g.frameW + x) * 4, d = (Y * ziel.frameW + X) * 4, a = F[s + 3]! / 255;
      if (!a || X < 0 || Y < 0 || X >= ziel.frameW || Y >= ziel.frameH) continue;
      const da = leinwand[d + 3]! / 255, oa = a + da * (1 - a);
      for (let c = 0; c < 3; c++) leinwand[d + c] = (F[s + c]! * a + leinwand[d + c]! * da * (1 - a)) / oa;
      leinwand[d + 3] = oa * 255;
    }
  }
}

describe('Ebenen von Reiter und Reittier (B-752)', { timeout: 120_000 }, () => {
  it('die Specs mit Ebenen sind gültig', () => {
    for (const a of [monarch, schimmel]) expect(checkSpec(a.spec, stil)).toEqual([]);
    expect(Object.keys(monarch.spec.animationen!).filter((n) => n.startsWith('reiter-'))).toEqual(['reiter-schritt', 'reiter-trab', 'reiter-galopp']);
    expect(Object.keys(schimmel.spec.animationen!).filter((n) => /^(hinten|vorn)-/.test(n)).sort()).toEqual(
      ['hinten-galopp', 'hinten-schritt', 'hinten-trab', 'vorn-galopp', 'vorn-schritt', 'vorn-trab']);
  });

  it('ebene braucht das passende rig und die Pflichtfelder', () => {
    const falsch = structuredClone(schimmel.spec);
    falsch.animationen!['hinten-trab'] = { frames: 8, ebene: 'reiter' };
    const texte = checkSpec(falsch, stil).map((f) => f.text).join(' | ');
    expect(texte).toContain('gilt nur für rig figur');
    expect(texte).toContain('gangart fehlt');
    expect(texte).toContain('reittier fehlt');
  });

  it('Pixel je Einheit sind in allen Ebenen gleich, die Ränder bleiben leer', () => {
    const hin = blatt(schimmel, 'hinten-trab'), vor = blatt(schimmel, 'vorn-trab');
    const reiter = ['schritt', 'trab', 'galopp'].map((g) => blatt(monarch, `reiter-${g}`));
    const ppu = (b: Blatt) => b.g.frameH / b.g.rahmen[3];
    for (const b of [hin, vor, ...reiter]) expect(Math.abs(ppu(b) - ppu(reiter[0]!))).toBeLessThan(0.01);
    expect(hin.g.rahmen).toEqual(vor.g.rahmen);
    for (const b of [hin, vor, ...reiter]) for (let n = 0; n < b.n; n++) expect(randAlpha(frame(b, n), b.g)).toBe(0);
  });

  it.each(['schritt', 'trab', 'galopp'])('hinten + Reiter + vorn ≈ Komplett-Sheet %s-schimmel', (gang) => {
    const komplett = blatt(monarch, `${gang}-schimmel`), hin = blatt(schimmel, `hinten-${gang}`), vor = blatt(schimmel, `vorn-${gang}`), rei = blatt(monarch, `reiter-${gang}`);
    const px = komplett.g.frameH / komplett.g.rahmen[3];
    let sichtbar = 0, abweichend = 0;
    for (let n = 0; n < komplett.n; n++) {
      const C = frame(komplett, n), { frameW: W, frameH: H } = komplett.g, leinwand = new Float32Array(W * H * 4);
      for (const L of [hin, rei, vor]) auflegen(leinwand, komplett.g, L, frame(L, n), px);
      for (let i = 0; i < W * H; i++) {
        const d = i * 4, ca = C[d + 3]!, ka = leinwand[d + 3]!;
        if (ca > 8 || ka > 8) sichtbar++;
        const farbe = Math.max(...[0, 1, 2].map((c) => Math.abs(C[d + c]! - leinwand[d + c]!) * Math.min(ca, ka) / 255));
        if (Math.max(Math.abs(ca - ka), farbe) > 48) abweichend++;
      }
    }
    expect(abweichend / sichtbar).toBeLessThan(0.02); // Kantenglättung und Rundung der Pixel-Versätze
  });
});
