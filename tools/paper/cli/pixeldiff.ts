// pixeldiff (paper-spec.md › 8): Anteil der Pixel mit Kanal-Differenz > 2, Rahmen zusammenhängender Abweichungen in
// einem 8-px-Raster (höchstens 32, größte zuerst), Overlay = b mit rot markierten Abweichungen.
import { abweichend, liesPng, type Pixel } from '../render/png';
import { rastere } from '../render/raster';

const ZELLE = 8;

function zellen(a: Pixel, b: Pixel): { anteil: number; offen: Set<number>; spalten: number } {
  const spalten = Math.ceil(b.breite / ZELLE), offen = new Set<number>(), anders = abweichend(a, b);
  let n = 0;
  for (let y = 0; y < b.hoehe; y++)
    for (let x = 0; x < b.breite; x++)
      if (anders(x, y)) {
        n++;
        offen.add(Math.floor(y / ZELLE) * spalten + Math.floor(x / ZELLE));
      }
  return { anteil: n / (b.breite * b.hoehe), offen, spalten };
}

/** Zusammenhängende Zellen (4er-Nachbarschaft) → Rahmen [x, y, b, h] in Pixeln. */
function boxen(alle: Set<number>, spalten: number, b: Pixel): number[][] {
  const offen = new Set(alle), out: number[][] = [];
  for (const start of alle) {
    if (!offen.delete(start)) continue;
    const stapel = [start];
    let [x0, y0, x1, y1] = [Infinity, Infinity, -1, -1];
    while (stapel.length) {
      const z = stapel.pop()!, zx = z % spalten, zy = Math.floor(z / spalten);
      [x0, y0, x1, y1] = [Math.min(x0, zx), Math.min(y0, zy), Math.max(x1, zx), Math.max(y1, zy)];
      for (const n of [z - 1, z + 1, z - spalten, z + spalten])
        if (offen.has(n) && Math.abs((n % spalten) - zx) <= 1) {
          offen.delete(n);
          stapel.push(n);
        }
    }
    const x = x0 * ZELLE, y = y0 * ZELLE;
    out.push([x, y, Math.min((x1 + 1) * ZELLE, b.breite) - x, Math.min((y1 + 1) * ZELLE, b.hoehe) - y]);
  }
  return out.sort((p, q) => q[2] * q[3] - p[2] * p[3] || p[1] - q[1] || p[0] - q[0]).slice(0, 32);
}

export function pixeldiff(aPng: Buffer, bPng: Buffer): { anteil: number; boxen: number[][]; overlay: Buffer } {
  const a = liesPng(aPng), b = liesPng(bPng), z = zellen(a, b), bx = boxen(z.offen, z.spalten, b);
  const rahmen = bx.map(([x, y, w, h]) => `<rect x="${x + 0.5}" y="${y + 0.5}" width="${w - 1}" height="${h - 1}" ` +
    `fill="#ff2a2a" fill-opacity="0.18" stroke="#ff2a2a" stroke-width="1"/>`).join('');
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${b.breite}" height="${b.hoehe}" viewBox="0 0 ${b.breite} ${b.hoehe}">` +
    `<image href="data:image/png;base64,${bPng.toString('base64')}" width="${b.breite}" height="${b.hoehe}"/>${rahmen}</svg>`;
  return { anteil: Number(z.anteil.toFixed(6)), boxen: bx, overlay: rastere(svg).png };
}
