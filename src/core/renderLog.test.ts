import { describe, expect, it } from 'vitest';
import { LOW_FPS, RenderLog, TREND_EVERY_MS, WARN_EVERY_MS, logLoadError, startLine, type RenderLogEnv } from './renderLog';
import type { RenderInfo } from './renderInfo';

const webgl: RenderInfo = { kind: 'WebGL2', gpu: 'Mali-450', maxTexture: 4096, width: 1920, height: 1080, devicePixelRatio: 1, viewport: '1280×720', textures: 212, textureMiB: 410.04, request: 'auto' };
const canvas: RenderInfo = { ...webgl, kind: 'Canvas', gpu: null, maxTexture: null };

interface Line { level: string; msg: string; ctx?: Record<string, unknown> }

function setup(info: RenderInfo = webgl) {
  const lines: Line[] = [];
  let t = 1000;
  const env: RenderLogEnv = { log: (level, msg, ctx) => lines.push({ level, msg, ctx }), now: () => t, info: () => info, heapMiB: () => 55.5 };
  return { log: new RenderLog(env), lines, advance: (ms: number) => (t += ms) };
}

describe('Startzeile (AC-01)', () => {
  it('WebGL enthält Renderer, GPU, Texturgröße, Texturen, Speicher, Größen, DPR, Browser und Version', () => {
    const { msg, ctx } = startLine(webgl, { userAgent: 'Edge', version: '1.2.3' });
    expect(msg).toBe('🚀 Renderer: WebGL2, GPU Mali-450, maxTex 4096, 212 Texturen, ~410 MiB');
    expect(ctx).toMatchObject({ renderer: 'WebGL2', gpu: 'Mali-450', maxTexture: 4096, textures: 212, textureMiB: 410, buffer: '1920x1080', viewport: '1280×720', dpr: 1, ua: 'Edge', version: '1.2.3' });
  });

  it('Canvas nennt keine GPU-Werte, aber Texturen und Größen', () => {
    const { msg, ctx } = startLine(canvas, { userAgent: 'Edge', version: '1' });
    expect(msg).toBe('🚀 Renderer: Canvas, 212 Texturen, ~410 MiB');
    expect(ctx).toMatchObject({ renderer: 'Canvas', gpu: null, buffer: '1920x1080', ua: 'Edge' });
  });

  it('ohne WEBGL_debug_renderer_info steht die GPU als unbekannt', () => {
    expect(startLine({ ...webgl, gpu: null }, { userAgent: '', version: '' }).msg).toContain('GPU unbekannt');
  });

  it('start() schreibt genau eine Zeile', () => {
    const { log, lines } = setup();
    log.start({ userAgent: 'Edge', version: '1' });
    expect(lines).toHaveLength(1);
    expect(lines[0]!.level).toBe('info');
  });
});

describe('Kontext-Ereignisse (AC-02)', () => {
  it('Verlust und Wiederherstellung schreiben je eine Zeile mit Zustand', () => {
    const { log, lines, advance } = setup();
    advance(3800);
    log.context('lost', 11.6);
    advance(500);
    log.context('restored', null);
    expect(lines).toHaveLength(2);
    expect(lines[0]).toMatchObject({ level: 'error', msg: '💥 WebGL Context lost nach 3,8 s, 212 Texturen, ~410 MiB' });
    expect(lines[0]!.ctx).toMatchObject({ event: 'lost', fps: 12, sinceStartS: 3.8, textures: 212 });
    expect(lines[1]).toMatchObject({ level: 'info' });
    expect(lines[1]!.msg).toContain('restored');
    expect(lines[1]!.ctx).toMatchObject({ event: 'restored', fps: null });
  });

  it('Canvas-Fallback schreibt eine Zeile', () => {
    const { log, lines } = setup(canvas);
    log.context('fallback', 30);
    expect(lines).toHaveLength(1);
    expect(lines[0]!.msg).toContain('Canvas-Fallback');
  });
});

describe('Leistungsverlauf (AC-03)', () => {
  /** Frames mit 30 FPS (33,3 ms), bis `ms` vergangen sind */
  function run(s: ReturnType<typeof setup>, ms: number, fps = 30) {
    for (let t = 0; t < ms; t += 1000 / fps) {
      s.advance(1000 / fps);
      s.log.frame(1000 / fps, fps);
    }
  }

  it('schreibt höchstens alle 30 s eine Zeile mit FPS Ø/min, Frame Ø, Texturspeicher und Heap', () => {
    const s = setup();
    run(s, TREND_EVERY_MS - 1000);
    expect(s.lines).toHaveLength(0);
    run(s, 2000);
    expect(s.lines).toHaveLength(1);
    expect(s.lines[0]!.msg).toBe('💓 FPS Ø 30, min 30, Frame Ø 33.3 ms, Texturen ~410 MiB #1');
    expect(s.lines[0]!.ctx).toMatchObject({ fpsMean: 30, fpsMin: 30, heapMiB: 55.5, textureMiB: 410 });
    run(s, TREND_EVERY_MS * 2);
    expect(s.lines.filter((l) => l.msg.startsWith('💓'))).toHaveLength(3);
  });

  it('FPS-Warnung sofort, aber höchstens eine je Minute', () => {
    const s = setup();
    s.advance(100);
    s.log.frame(100, LOW_FPS - 5);
    expect(s.lines).toHaveLength(1);
    expect(s.lines[0]).toMatchObject({ level: 'warn' });
    s.advance(100);
    s.log.frame(100, LOW_FPS - 5);
    expect(s.lines).toHaveLength(1);
    s.advance(WARN_EVERY_MS);
    s.log.frame(100, LOW_FPS - 5);
    expect(s.lines.filter((l) => l.level === 'warn')).toHaveLength(2);
  });

  it('FPS ab 20 löst keine Warnung aus', () => {
    const s = setup();
    s.log.frame(40, LOW_FPS);
    expect(s.lines).toHaveLength(0);
  });
});

describe('Ladevorgang (AC-04)', () => {
  it('Fehlerzeile nennt Datei und Status', () => {
    const lines: Line[] = [];
    logLoadError('/grafik/gebaeude/hg-wald-nah-v1-128.png', 404, (level, msg, ctx) => lines.push({ level, msg, ctx }));
    expect(lines).toEqual([{ level: 'error', msg: '💥 Laden fehlgeschlagen: /grafik/gebaeude/hg-wald-nah-v1-128.png (Status 404)', ctx: { file: '/grafik/gebaeude/hg-wald-nah-v1-128.png', status: 404 } }]);
  });

  it('loaded() schreibt Anzahl, Größe und Dauer je Gruppe', () => {
    const { log, lines } = setup();
    log.loaded('gebaeude', 40, 120.26, 2400.4);
    expect(lines).toHaveLength(1);
    expect(lines[0]!.msg).toBe('📂 Geladen gebaeude: 40 Dateien, ~120.3 MiB, 2400 ms');
    expect(lines[0]!.ctx).toMatchObject({ group: 'gebaeude', files: 40, loadMs: 2400 });
  });
});
