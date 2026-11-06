// Frame-Streifen: ein SVG je Datei, je Frame ein verschachteltes <svg> mit viewBox = rahmen (Schatten bleiben im Frame).
// resvg rastert; Frame-Hashes sind SHA-256 der RGBA-Pixel je Ausschnitt (paper-spec.md › 7).
import { createHash } from 'node:crypto';
import { Resvg } from '@resvg/resvg-js';
import { num } from '../schema/hash';
import type { Spec } from '../schema/types';

export type Rahmen = [number, number, number, number];
/** eigen = eigener Rahmen einer Animation (Reiten), steht dann je Animation im Manifest. */
export type FrameGroesse = { rahmen: Rahmen; frameW: number; frameH: number; eigen?: boolean };
export type Raster = { png: Buffer; rgba: Uint8Array; breite: number; hoehe: number };

export function frameGroesse(ausgabe: Spec['ausgabe']): FrameGroesse {
  const [, , b, h] = ausgabe.rahmen;
  return { rahmen: ausgabe.rahmen, frameW: Math.round((ausgabe.hoehe_px * b) / h), frameH: ausgabe.hoehe_px };
}

export function streifenSvg(frames: string[], g: FrameGroesse, defs: string): string {
  const breite = frames.length * g.frameW;
  const vb = g.rahmen.map(num).join(' ');
  const inhalt = frames.map((f, i) =>
    `<svg x="${i * g.frameW}" y="0" width="${g.frameW}" height="${g.frameH}" viewBox="${vb}" preserveAspectRatio="none">${f}</svg>`).join('');
  return `<svg xmlns="http://www.w3.org/2000/svg" width="${breite}" height="${g.frameH}" viewBox="0 0 ${breite} ${g.frameH}">${defs}${inhalt}</svg>`;
}

export function rastere(svg: string): Raster {
  const r = new Resvg(svg, { font: { loadSystemFonts: false } }).render();
  return { png: Buffer.from(r.asPng()), rgba: new Uint8Array(r.pixels), breite: r.width, hoehe: r.height };
}

export function frameHashes(rgba: Uint8Array, breite: number, frameW: number, frameH: number, n: number): string[] {
  return Array.from({ length: n }, (_, i) => {
    const h = createHash('sha256');
    for (let y = 0; y < frameH; y++) {
      const start = (y * breite + i * frameW) * 4;
      h.update(rgba.subarray(start, start + frameW * 4));
    }
    return h.digest('hex');
  });
}
