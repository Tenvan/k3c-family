// Pixel-Helfer der CLI-Attrappe: PNG lesen (über resvg, ohne weitere Abhängigkeit), Pixel vergleichen, Overlay und
// Platzhalter rendern. Deterministisch; die echte Umsetzung liefert RD1 (docs/standards/paper-spec.md › CLI-Vertrag).
import { Resvg } from '@resvg/resvg-js';

export type Bild = { breite: number; hoehe: number; rgba: Uint8Array; png: Buffer };

/** Größe aus dem IHDR-Block einer PNG-Datei. */
export function groesse(png: Buffer): { breite: number; hoehe: number } {
  return { breite: png.readUInt32BE(16), hoehe: png.readUInt32BE(20) };
}

const bildSvg = (png: Buffer, b: number, h: number, extra = '') =>
  `<svg xmlns="http://www.w3.org/2000/svg" width="${b}" height="${h}" viewBox="0 0 ${b} ${h}">` +
  `<image href="data:image/png;base64,${png.toString('base64')}" width="${b}" height="${h}"/>${extra}</svg>`;

const rendere = (svg: string) => new Resvg(svg, { font: { loadSystemFonts: false } }).render();

export function lies(png: Buffer): Bild {
  const { breite, hoehe } = groesse(png);
  return { breite, hoehe, rgba: new Uint8Array(rendere(bildSvg(png, breite, hoehe)).pixels), png };
}

/** Raster aus 8-px-Zellen mit Abweichung (Kanal-Differenz > 2); Anteil der abweichenden Pixel. */
function abweichung(a: Bild, b: Bild): { anteil: number; zellen: Set<number>; spalten: number } {
  const spalten = Math.ceil(b.breite / 8), zellen = new Set<number>();
  let n = 0;
  for (let y = 0; y < b.hoehe; y++)
    for (let x = 0; x < b.breite; x++) {
      const i = (y * b.breite + x) * 4;
      const anders = x >= a.breite || y >= a.hoehe ||
        [0, 1, 2, 3].some((k) => Math.abs(a.rgba[(y * a.breite + x) * 4 + k] - b.rgba[i + k]) > 2);
      if (anders) {
        n++;
        zellen.add(Math.floor(y / 8) * spalten + Math.floor(x / 8));
      }
    }
  return { anteil: n / (b.breite * b.hoehe), zellen, spalten };
}

/** Zusammenhängende Zellen → Rahmen [x, y, b, h] in Pixeln, größte zuerst, höchstens 32. */
function boxen(zellen: Set<number>, spalten: number, b: Bild): number[][] {
  const offen = new Set(zellen), out: number[][] = [];
  for (const start of zellen) {
    if (!offen.delete(start)) continue;
    const stapel = [start];
    let [x0, y0, x1, y1] = [Infinity, Infinity, -1, -1];
    while (stapel.length) {
      const z = stapel.pop()!, zx = z % spalten, zy = Math.floor(z / spalten);
      [x0, y0, x1, y1] = [Math.min(x0, zx), Math.min(y0, zy), Math.max(x1, zx), Math.max(y1, zy)];
      for (const n of [z - 1, z + 1, z - spalten, z + spalten])
        if (offen.has(n) && Math.abs((n % spalten) - zx) <= 1) { offen.delete(n); stapel.push(n); }
    }
    const x = x0 * 8, y = y0 * 8;
    out.push([x, y, Math.min((x1 + 1) * 8, b.breite) - x, Math.min((y1 + 1) * 8, b.hoehe) - y]);
  }
  return out.sort((p, q) => q[2] * q[3] - p[2] * p[3] || p[1] - q[1] || p[0] - q[0]).slice(0, 32);
}

/** Pixel-Diff zweier PNGs; Overlay = b mit rot umrandeten Abweichungen. */
export function pixeldiff(aPng: Buffer, bPng: Buffer): { anteil: number; boxen: number[][]; overlay: Buffer } {
  const a = lies(aPng), b = lies(bPng);
  const d = abweichung(a, b), bx = boxen(d.zellen, d.spalten, b);
  const rahmen = bx.map(([x, y, w, h]) => `<rect x="${x + 0.5}" y="${y + 0.5}" width="${w - 1}" height="${h - 1}" fill="#ff2a2a" fill-opacity="0.18" stroke="#ff2a2a" stroke-width="1"/>`).join('');
  const overlay = Buffer.from(rendere(bildSvg(bPng, b.breite, b.hoehe, rahmen)).asPng());
  return { anteil: Number(d.anteil.toFixed(6)), boxen: bx, overlay };
}

/** Platzhalter-PNG für gültige, aber unbekannte Specs: graues Papier mit Kreuz. */
export function platzhalter(breite: number, hoehe: number): Buffer {
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${breite}" height="${hoehe}"><rect x="4" y="4" width="${breite - 8}" height="${hoehe - 8}" fill="#d9d4cc"/>` +
    `<path d="M8 8L${breite - 8} ${hoehe - 8}M${breite - 8} 8L8 ${hoehe - 8}" stroke="#8a8478" stroke-width="3"/></svg>`;
  return Buffer.from(rendere(svg).asPng());
}
