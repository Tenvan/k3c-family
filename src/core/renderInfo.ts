/**
 * Technik-Anzeige des Renderers (B-761): WebGL oder Canvas, GPU, Texturgröße, Texturspeicher, FPS, Frametime.
 * Reine Funktionen für Auswahl und Text; `readRenderInfo` liest nur strukturell aus dem Phaser-Spiel, damit Tests ohne Phaser auskommen.
 * Auf der Xbox (Edge) soll sofort sichtbar sein, ob das Spiel im Canvas-Fallback läuft.
 */

/** URL-Wunsch zur Renderer-Wahl: `?renderer=webgl|canvas`, ohne Angabe `auto` (Phaser.AUTO mit Canvas-Rückfall) */
export type RendererRequest = 'auto' | 'webgl' | 'canvas';

export function rendererRequest(search: string): RendererRequest {
  const v = new URLSearchParams(search).get('renderer');
  return v === 'webgl' || v === 'canvas' ? v : 'auto';
}

/** Technik-Anzeige: immer an (RX1-Testphase, gespeicherte Einstellungen älterer Läufe zählen nicht); nur `?gfx=0` schaltet sie ab; `?aa=0` schaltet die Kantenglättung des WebGL-Kontexts ab (Leistungstest) */
export function gfxEnabled(search: string): boolean {
  return new URLSearchParams(search).get('gfx') !== '0';
}
export const antialiasOff = (search: string): boolean => new URLSearchParams(search).get('aa') === '0';

export type RendererKind = 'WebGL2' | 'WebGL' | 'Canvas';

export interface RenderInfo {
  kind: RendererKind;
  /** Wert von `UNMASKED_RENDERER_WEBGL`, `null` ohne Erweiterung oder Canvas */
  gpu: string | null;
  maxTexture: number | null;
  /** Rückpuffer in Pixeln (Spielauflösung) */
  width: number;
  height: number;
  devicePixelRatio: number;
  /** Fenstergröße in CSS-Pixeln */
  viewport: string;
  textures: number;
  /** Geschätzter Texturspeicher in MiB (Breite × Höhe × 4 je Quellbild) */
  textureMiB: number;
  request: RendererRequest;
}

/** Strukturelle Sicht auf das, was gelesen wird (Phaser.Game und WebGL-Kontext) */
export interface RenderSource {
  renderer: { type: number; gl?: WebGLRenderingContext | WebGL2RenderingContext; width?: number; height?: number };
  /** Phaser.WEBGL */
  webglType: number;
  textureList: ReadonlyArray<{ source: ReadonlyArray<{ width: number; height: number }> }>;
  devicePixelRatio: number;
  viewport: string;
  request: RendererRequest;
}

const MIB = 1024 * 1024;

/** Geschätzter Speicher aller Textur-Quellbilder in MiB, ohne die Systemtexturen (`__DEFAULT` u. ä. sind 32×32 und fallen nicht ins Gewicht) */
export function textureMiB(list: RenderSource['textureList']): number {
  let bytes = 0;
  for (const t of list) for (const s of t.source) bytes += s.width * s.height * 4;
  return bytes / MIB;
}

export function readRenderInfo(src: RenderSource): RenderInfo {
  const { renderer } = src;
  const gl = renderer.type === src.webglType ? renderer.gl : undefined;
  let gpu: string | null = null;
  let maxTexture: number | null = null;
  if (gl) {
    maxTexture = Number(gl.getParameter(gl.MAX_TEXTURE_SIZE)) || null;
    const ext = gl.getExtension('WEBGL_debug_renderer_info');
    gpu = ext ? String(gl.getParameter(ext.UNMASKED_RENDERER_WEBGL)) : null;
  }
  const isGl2 = typeof WebGL2RenderingContext !== 'undefined' && gl instanceof WebGL2RenderingContext;
  return {
    kind: gl ? (isGl2 ? 'WebGL2' : 'WebGL') : 'Canvas',
    gpu,
    maxTexture,
    width: renderer.width ?? 0,
    height: renderer.height ?? 0,
    devicePixelRatio: src.devicePixelRatio,
    viewport: src.viewport,
    textures: src.textureList.length,
    textureMiB: textureMiB(src.textureList),
    request: src.request,
  };
}

/** Gleitender Mittelwert der Frametime über die letzten `size` Frames */
export class FrameAverage {
  private readonly buf: number[] = [];
  private next = 0;
  constructor(private readonly size = 60) {}
  add(ms: number): void {
    if (!Number.isFinite(ms) || ms < 0) return;
    this.buf[this.next] = ms;
    this.next = (this.next + 1) % this.size;
  }
  get mean(): number | null {
    return this.buf.length === 0 ? null : this.buf.reduce((a, b) => a + b, 0) / this.buf.length;
  }
}

/** Warnung, wenn der Browser statt WebGL den Canvas-Fallback nutzt; sonst `null` */
export const canvasWarning = (info: RenderInfo): string | null => (info.kind === 'Canvas' ? 'CANVAS-FALLBACK (langsam)' : null);

/** Anzeigezeilen; die erste Zeile ist die Warnung, wenn sie gilt */
export function renderInfoLines(info: RenderInfo, fps: number | null, frameMeanMs: number | null): string[] {
  const fmt = (v: number | null, d = 0) => (v === null || !Number.isFinite(v) ? '–' : v.toFixed(d));
  const lines: string[] = [];
  const warn = canvasWarning(info);
  if (warn) lines.push(warn);
  lines.push(`Renderer ${info.kind}${info.request === 'auto' ? '' : ` (erzwungen: ${info.request})`}`);
  if (info.kind !== 'Canvas') lines.push(`GPU ${info.gpu ?? 'unbekannt'} · max. Textur ${fmt(info.maxTexture)} px`);
  lines.push(`${fmt(fps)} FPS · Ø Frame ${fmt(frameMeanMs, 1)} ms`);
  lines.push(`Texturen ${info.textures} · ca. ${fmt(info.textureMiB, 1)} MiB`);
  lines.push(`Puffer ${info.width}×${info.height} · Fenster ${info.viewport} · DPR ${fmt(info.devicePixelRatio, 2)}`);
  return lines;
}
