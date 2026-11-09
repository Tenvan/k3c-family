import { describe, expect, it } from 'vitest';
import { FrameAverage, antialiasOff, canvasWarning, gfxEnabled, readRenderInfo, renderInfoLines, rendererRequest, textureMiB, type RenderInfo, type RenderSource } from './renderInfo';

const WEBGL = 2;
const info = (over: Partial<RenderInfo> = {}): RenderInfo => ({ kind: 'WebGL2', gpu: 'ANGLE (AMD)', maxTexture: 16384, width: 1920, height: 1080, devicePixelRatio: 1.5, viewport: '1280×720', textures: 3, textureMiB: 12.5, request: 'auto', ...over });

describe('URL-Schalter (AC-02, AC-04)', () => {
  it('renderer: webgl, canvas, sonst auto', () => {
    expect(rendererRequest('?renderer=webgl')).toBe('webgl');
    expect(rendererRequest('?renderer=canvas')).toBe('canvas');
    expect(rendererRequest('?renderer=x')).toBe('auto');
    expect(rendererRequest('')).toBe('auto');
  });
  it('gfx und aa', () => {
    expect(gfxEnabled('?gfx=1')).toBe(true);
    expect(gfxEnabled('?gfx=0')).toBe(false);
    expect(gfxEnabled('')).toBe(true);
    expect(antialiasOff('?aa=0')).toBe(true);
    expect(antialiasOff('')).toBe(false);
  });
});

describe('Texturspeicher (AC-01)', () => {
  it('Breite × Höhe × 4 je Quellbild in MiB', () => {
    expect(textureMiB([{ source: [{ width: 1024, height: 1024 }] }, { source: [{ width: 512, height: 512 }, { width: 512, height: 512 }] }])).toBeCloseTo(6, 5);
    expect(textureMiB([])).toBe(0);
  });
});

describe('readRenderInfo (AC-01)', () => {
  const gl = (ext: boolean) => ({ MAX_TEXTURE_SIZE: 1, getParameter: (p: unknown) => (p === 1 ? 8192 : 'Mali-G'), getExtension: () => (ext ? { UNMASKED_RENDERER_WEBGL: 2 } : null) }) as unknown as WebGLRenderingContext;
  const src = (over: Partial<RenderSource>): RenderSource => ({ renderer: { type: WEBGL, gl: gl(true), width: 1920, height: 1080 }, webglType: WEBGL, textureList: [], devicePixelRatio: 2, viewport: '1x1', request: 'auto', ...over });

  it('WebGL: GPU und Texturgröße', () => {
    const r = readRenderInfo(src({}));
    expect(r).toMatchObject({ kind: 'WebGL', maxTexture: 8192, gpu: 'Mali-G', width: 1920 });
  });
  it('ohne Debug-Erweiterung: GPU unbekannt', () => {
    expect(readRenderInfo(src({ renderer: { type: WEBGL, gl: gl(false) } })).gpu).toBeNull();
  });
  it('Canvas: kein GL, keine Texturgröße', () => {
    expect(readRenderInfo(src({ renderer: { type: 1, width: 1920, height: 1080 } }))).toMatchObject({ kind: 'Canvas', gpu: null, maxTexture: null });
  });
});

describe('Anzeige (AC-01, AC-03)', () => {
  it('WebGL: keine Warnung, alle Werte', () => {
    expect(canvasWarning(info())).toBeNull();
    expect(renderInfoLines(info(), 59.7, 16.64)).toEqual([
      'Renderer WebGL2',
      'GPU ANGLE (AMD) · max. Textur 16384 px',
      '60 FPS · Ø Frame 16.6 ms',
      'Texturen 3 · ca. 12.5 MiB',
      'Puffer 1920×1080 · Fenster 1280×720 · DPR 1.50',
    ]);
  });
  it('Canvas: Warnzeile zuerst, keine GPU-Zeile', () => {
    const lines = renderInfoLines(info({ kind: 'Canvas', gpu: null, maxTexture: null, request: 'canvas' }), 20, 50);
    expect(lines[0]).toBe('CANVAS-FALLBACK (langsam)');
    expect(lines[1]).toBe('Renderer Canvas (erzwungen: canvas)');
    expect(lines.some((l) => l.startsWith('GPU'))).toBe(false);
  });
  it('fehlende Messwerte als Strich', () => {
    expect(renderInfoLines(info(), null, null)[2]).toBe('– FPS · Ø Frame – ms');
  });
});

describe('FrameAverage', () => {
  it('Mittel der letzten Frames, Ausreißer fallen heraus', () => {
    const a = new FrameAverage(2);
    expect(a.mean).toBeNull();
    [10, 20, 30].forEach((v) => a.add(v));
    expect(a.mean).toBe(25);
    a.add(NaN);
    expect(a.mean).toBe(25);
  });
});
