// PNG lesen ohne weitere Abhängigkeit: resvg rastert die Datei als <image> in Originalgröße und liefert RGBA.
import { Resvg } from '@resvg/resvg-js';

export type Pixel = { breite: number; hoehe: number; rgba: Uint8Array };

/** Größe aus dem IHDR-Block. */
export function groesse(png: Buffer): { breite: number; hoehe: number } {
  return { breite: png.readUInt32BE(16), hoehe: png.readUInt32BE(20) };
}

export function liesPng(png: Buffer): Pixel {
  const { breite, hoehe } = groesse(png);
  const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${breite}" height="${hoehe}" viewBox="0 0 ${breite} ${hoehe}">` +
    `<image href="data:image/png;base64,${png.toString('base64')}" width="${breite}" height="${hoehe}"/></svg>`;
  return { breite, hoehe, rgba: new Uint8Array(new Resvg(svg, { font: { loadSystemFonts: false } }).render().pixels) };
}

/** Pixel von b, die in a fehlen oder in einem Kanal um mehr als 2 abweichen (Raster von b). */
export function abweichend(a: Pixel, b: Pixel): (x: number, y: number) => boolean {
  return (x, y) => {
    if (x >= a.breite || y >= a.hoehe) return true;
    const i = (y * b.breite + x) * 4, j = (y * a.breite + x) * 4;
    for (let k = 0; k < 4; k++) if (Math.abs(a.rgba[j + k] - b.rgba[i + k]) > 2) return true;
    return false;
  };
}
