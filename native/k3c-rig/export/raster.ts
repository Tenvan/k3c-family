// Rastert ein Stück SVG (Lagen eines Teils) mit den Filtern der Stil-Datei und schneidet auf die deckenden Pixel zu.
// Zwei Durchgänge: erst der ganze Bereich (Hülle suchen), dann nur der Ausschnitt auf demselben Pixelraster.
import { Resvg } from '@resvg/resvg-js';
import type { Punkt, Stil } from '../../../tools/paper/schema/types';
import { defs } from '../../../tools/paper/render/svg';

export type Bereich = [number, number, number, number];
export type Roh = { png: Buffer; versatz: Punkt; groesse: [number, number] };

/** Rastern einer Figur: Filter wie im Sheet (`blattDichte`), Pixel in `dichte` (Welt-Einheiten), Teile in `massstab`. */
export type Kontext = { stil: Stil; papierId: string; blattDichte: number; dichte: number; massstab: number; bereich: Bereich };

const PAPIER = '<filter id="paper" x="-5%" y="-5%" width="110%" height="110%">';
const RAND = 2;

/** Filter von tools/paper; der Papier-Filter gilt für das ganze Bild `vb` statt für die Hülle des Teils (sonst schneidet er
 * Schatten kleiner Teile ab). Die Struktur hängt am Ort in Einheiten, nicht am Bildausschnitt. */
function filterDefs(k: Kontext, [x, y, b, h]: Bereich): string {
  const d = defs(k.stil, k.papierId, k.blattDichte);
  if (!d.includes(PAPIER)) throw new Error('Papier-Filter von tools/paper hat ein anderes Format als erwartet');
  return d.replace(PAPIER, `<filter id="paper" filterUnits="userSpaceOnUse" x="${x}" y="${y}" width="${b}" height="${h}">`);
}

const svg = (k: Kontext, inhalt: string, vb: Bereich, w: number, h: number) =>
  `<svg xmlns="http://www.w3.org/2000/svg" width="${w}" height="${h}" viewBox="${vb.join(' ')}">${filterDefs(k, vb)}<g filter="url(#paper)">${inhalt}</g></svg>`;

const rendere = (text: string) => new Resvg(text, { font: { loadSystemFonts: false } }).render();

/** Hülle aller Pixel mit Deckkraft > 0: [x0, y0, x1, y1] (x1, y1 exklusiv) oder undefined. */
function huelle(rgba: Uint8Array, w: number, h: number): Bereich | undefined {
  let x0 = w, y0 = h, x1 = -1, y1 = -1;
  for (let y = 0; y < h; y++)
    for (let x = 0; x < w; x++) {
      if (rgba[(y * w + x) * 4 + 3] === 0) continue;
      if (x < x0) x0 = x;
      if (x > x1) x1 = x;
      if (y < y0) y0 = y;
      if (y > y1) y1 = y;
    }
  return x1 < 0 ? undefined : [x0, y0, x1 + 1, y1 + 1];
}

export const runde = (v: number) => Math.round(v * 1e4) / 1e4;

/** `inhalt` in Einheiten der Figur; Ergebnis: PNG, linke obere Ecke in Einheiten der Figur, Größe in Pixeln. */
export function rastere(k: Kontext, inhalt: string, name: string): Roh {
  const [bx, by, bb, bh] = k.bereich, D = k.dichte, W = Math.ceil(bb * D), H = Math.ceil(bh * D);
  const welt = k.massstab === 1 ? inhalt : `<g transform="scale(${k.massstab})">${inhalt}</g>`;
  const box = huelle(new Uint8Array(rendere(svg(k, welt, [bx, by, W / D, H / D], W, H)).pixels), W, H);
  if (!box) throw new Error(`${name}: leeres Bild`);
  if (box[0] === 0 || box[1] === 0 || box[2] === W || box[3] === H) throw new Error(`${name}: Bereich zu klein, das Teil stößt an den Rand`);
  const x0 = Math.max(0, box[0] - RAND), y0 = Math.max(0, box[1] - RAND);
  const w = Math.min(W, box[2] + RAND) - x0, h = Math.min(H, box[3] + RAND) - y0;
  const aus = rendere(svg(k, welt, [bx + x0 / D, by + y0 / D, w / D, h / D], w, h));
  return { png: Buffer.from(aus.asPng()), versatz: [runde((bx + x0 / D) / k.massstab), runde((by + y0 / D) / k.massstab)], groesse: [w, h] };
}
