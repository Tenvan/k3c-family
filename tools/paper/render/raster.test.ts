// RD1.1: Frame-Größe, Frame-Streifen als ein SVG, resvg-Raster und Frame-Hashes.
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { describe, expect, it } from 'vitest';
import type { Spec } from '../schema/types';
import { frameGroesse, frameHashes, rastere, streifenSvg } from './raster';

const mauer = JSON.parse(readFileSync(join(import.meta.dirname, '../fixtures/specs/mauer.paper.json'), 'utf8')) as Spec;
const rahmen: [number, number, number, number] = [-10, -20, 20, 20];
const groesse = { rahmen, frameW: 16, frameH: 16 };
const kreis = '<circle cx="0" cy="-10" r="5" fill="#336699"/>';

describe('Raster', () => {
  it('frameGroesse: Breite = round(hoehe_px · b / h)', () => {
    expect(frameGroesse(mauer.ausgabe)).toEqual({ rahmen: [-66, -112, 132, 116], frameW: 146, frameH: 128 });
  });

  it('streifen: N Frames nebeneinander, Bildbreite N · frameW', () => {
    const r = rastere(streifenSvg([kreis, kreis, kreis], groesse, ''));
    expect([r.breite, r.hoehe]).toEqual([48, 16]);
    expect(r.png.subarray(1, 4).toString()).toBe('PNG');
  });

  it('frameHashes: je Frame SHA-256 der RGBA-Pixel des Ausschnitts; Frames laufen nicht ineinander', () => {
    const riesig = '<rect x="-100" y="-100" width="400" height="400" fill="#ff0000"/>';
    const r = rastere(streifenSvg([kreis, riesig, '', kreis], groesse, ''));
    const h = frameHashes(r.rgba, r.breite, 16, 16, 4);
    expect(h).toHaveLength(4);
    expect(h[0]).toMatch(/^[0-9a-f]{64}$/);
    expect(h[3]).toBe(h[0]);
    expect(h[1]).not.toBe(h[0]);
    const leer = frameHashes(rastere(streifenSvg([''], groesse, '')).rgba, 16, 16, 16, 1)[0];
    expect(h[2]).toBe(leer);
  });

  it('zwei Läufe byte-gleich', () => {
    const svg = streifenSvg([kreis, kreis], groesse, '');
    expect(rastere(svg).png.equals(rastere(svg).png)).toBe(true);
  });
});
