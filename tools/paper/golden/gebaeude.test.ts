// MG11.1 (B-721/AC-01): Kein Gebäude ragt aus seinem Rahmen. Oberste Zeile, linke und rechte Spalte jedes Frames
// (jede Stufe) bleiben ohne deckendes Pixel; die unterste Zeile ist der Boden (Ursprung unten mittig, D-MG6-04).
import { readFileSync } from 'node:fs';
import { basename, join } from 'node:path';
import { describe, expect, it } from 'vitest';
import '../rig/register';
import { bildVon } from '../render/bild';
import { specsIn } from '../render/ausgabe';
import { rastere, streifenSvg } from '../render/raster';
import { ausgaben } from '../render/stufen';
import { ladeStil, ROOT } from '../render/stil';
import { defs } from '../render/svg';
import type { Spec } from '../schema/types';

const DECKEND = 200; // Papier ist deckend, sein Schatten (55 %) nicht
const stil = ladeStil();
const ordner = join(ROOT, 'public/grafik/gebaeude-paper');

/** Randpixel mit Alpha ≥ DECKEND je Ausgabe: `<schluessel> oben|links|rechts`. */
function randTreffer(pfad: string): string[] {
  const spec = JSON.parse(readFileSync(pfad, 'utf8')) as Spec;
  const treffer: string[] = [];
  for (const a of ausgaben(spec)) {
    const bild = bildVon(spec, a, { stil, specPfad: pfad }), g = bild.groesse;
    const r = rastere(streifenSvg(bild.frames.map((f) => f.svg), g, defs(stil, spec.id, g.frameH / g.rahmen[3])));
    const alpha = (x: number, y: number) => r.rgba[(y * r.breite + x) * 4 + 3];
    for (let i = 0; i < bild.frames.length; i++) {
      const x0 = i * g.frameW;
      const seiten = {
        oben: Array.from({ length: g.frameW }, (_, x) => alpha(x0 + x, 0)),
        links: Array.from({ length: g.frameH - 1 }, (_, y) => alpha(x0, y)),
        rechts: Array.from({ length: g.frameH - 1 }, (_, y) => alpha(x0 + g.frameW - 1, y)),
      };
      for (const [seite, werte] of Object.entries(seiten)) if (werte.some((v) => v >= DECKEND)) treffer.push(`${a.schluessel} ${seite}`);
    }
  }
  return treffer;
}

describe('Gebäude im Rahmen (B-721/AC-01)', { timeout: 120_000 }, () => {
  it.each(specsIn(ordner).map((p) => [basename(p, '.paper.json'), p]))('%s', (_id, pfad) => {
    expect(randTreffer(pfad)).toEqual([]);
  });
});
