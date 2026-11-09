/**
 * WebGL-Diagnose im Browser (RX1): legt für jeden Versuch aus `glProbe.ts` einen eigenen Canvas an, versucht einen Kontext, liest die
 * Werte aus und gibt den Kontext sofort wieder frei. Der Grund eines Fehlschlags kommt aus `webglcontextcreationerror` (Edge nennt ihn
 * nicht immer, dann steht `null`).
 */
import { ATTEMPTS, failed, type Attempt, type AttemptResult, type Env } from './glProbe';

export type Gl = WebGL2RenderingContext | WebGLRenderingContext;

/** Ein Kontext oder der Grund, warum es keinen gab */
export interface Opened {
  gl: Gl | null;
  error: string | null;
}

/** Fordert einen Kontext an; `error` ist der Text aus `webglcontextcreationerror`, `'null'` ohne Grund oder `'exception: …'` */
export function openContext(canvas: HTMLCanvasElement, attempt: Attempt): Opened {
  let reason: string | null = null;
  const onError = (e: Event): void => {
    reason = (e as WebGLContextEvent).statusMessage || 'creation error';
  };
  canvas.addEventListener('webglcontextcreationerror', onError);
  try {
    const gl = canvas.getContext(attempt.kind, attempt.attrs) as Gl | null;
    return gl ? { gl, error: null } : { gl: null, error: reason ?? 'null' };
  } catch (err) {
    return { gl: null, error: `exception: ${String(err)}` };
  } finally {
    canvas.removeEventListener('webglcontextcreationerror', onError);
  }
}

/** Werte eines Kontexts (Grafikkarte ohne `WEBGL_debug_renderer_info` nur als allgemeiner Name) */
export function glValues(gl: Gl): Pick<AttemptResult, 'version' | 'renderer' | 'vendor' | 'maxTexture' | 'textureUnits' | 'extensions' | 'timerQuery'> {
  const ext = gl.getExtension('WEBGL_debug_renderer_info');
  const names = gl.getSupportedExtensions() ?? [];
  return {
    version: String(gl.getParameter(gl.VERSION)),
    renderer: String(gl.getParameter(ext ? ext.UNMASKED_RENDERER_WEBGL : gl.RENDERER)),
    vendor: String(gl.getParameter(ext ? ext.UNMASKED_VENDOR_WEBGL : gl.VENDOR)),
    maxTexture: Number(gl.getParameter(gl.MAX_TEXTURE_SIZE)),
    textureUnits: Number(gl.getParameter(gl.MAX_TEXTURE_IMAGE_UNITS)),
    extensions: names.length,
    timerQuery: names.some((n) => n.startsWith('EXT_disjoint_timer_query')),
  };
}

/** Ein Versuch mit eigenem Canvas; der Kontext wird danach freigegeben, damit die Versuche sich keinen Speicher teilen */
export function runAttempt(attempt: Attempt): AttemptResult {
  const t0 = performance.now();
  const canvas = document.createElement('canvas');
  canvas.width = canvas.height = 64;
  const { gl, error } = openContext(canvas, attempt);
  if (!gl) return failed(attempt.id, error ?? 'null', Math.round(performance.now() - t0));
  const values = glValues(gl);
  gl.getExtension('WEBGL_lose_context')?.loseContext();
  return { id: attempt.id, ok: true, error: null, ...values, ms: Math.round(performance.now() - t0) };
}

export const diagnose = (): AttemptResult[] => ATTEMPTS.map(runAttempt);

/** Umgebung der Seite für den Bericht */
export function gatherEnv(): Env {
  const nav = navigator as Navigator & { deviceMemory?: number; gpu?: unknown };
  const mem = (performance as unknown as { memory?: { usedJSHeapSize: number } }).memory;
  let inPanel = false;
  try {
    inPanel = window.parent !== window;
  } catch {
    inPanel = true;
  }
  return {
    userAgent: navigator.userAgent, viewport: `${innerWidth}x${innerHeight}`, devicePixelRatio, inPanel, hidden: document.hidden,
    hasWebGL1: typeof WebGLRenderingContext !== 'undefined', hasWebGL2: typeof WebGL2RenderingContext !== 'undefined',
    hasOffscreenCanvas: typeof OffscreenCanvas !== 'undefined', hasWebGPU: nav.gpu !== undefined,
    deviceMemoryGiB: nav.deviceMemory ?? null, cores: nav.hardwareConcurrency ?? null, jsHeapMiB: mem ? Math.round(mem.usedJSHeapSize / 1048576) : null,
  };
}
