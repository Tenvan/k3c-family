/**
 * Textur-Grenze (B-764): Testseite ohne Spiel. Legt stufenweise WebGL-Texturen an, zeichnet damit und misst FPS, Frame-Zeit und den Punkt, an
 * dem der Kontext verloren geht (Xbox). Das Ergebnis geht als Bericht an den Server (`reports/*.json`). Logik: `textureProbe.ts`.
 */
import { installPageChrome } from '../core/shell';
import { deviceTag } from '../stress/stressReport';
import { applyTexts, t } from './texts';
import {
  FrameStats, SMOOTH_FPS, TEXTURE_SIZES, defaultConfig, glErrorName, stageCount, stageLine, textureMiB, verdict,
  type Loss, type ProbeConfig, type Stage, type TextureSize,
} from './textureProbe';

installPageChrome();
applyTexts();

const el = <T extends HTMLElement>(id: string): T => document.getElementById(id) as T;
const out = el<HTMLPreElement>('out');
const result = el<HTMLParagraphElement>('result');
const state = el<HTMLParagraphElement>('state');
const stage = el<HTMLDivElement>('stage');
const startBtn = el<HTMLButtonElement>('btn-start');
const sendBtn = el<HTMLButtonElement>('btn-send');

let cfg: ProbeConfig = defaultConfig(2048);
let lastReport: object | null = null;

interface Run {
  gl: WebGL2RenderingContext | WebGLRenderingContext;
  canvas: HTMLCanvasElement;
  textures: WebGLTexture[];
  stats: FrameStats;
  stages: Stage[];
  loss: Loss | null;
  data: Uint8Array;
  startedAt: number;
  stageStart: number;
  last: number;
  frame: number;
  raf: number;
  lostAt: number;
  ended: boolean;
}
let run: Run | null = null;

const VERT = 'attribute vec2 p; varying vec2 uv; void main(){ uv = p * 0.5 + 0.5; gl_Position = vec4(p, 0.0, 1.0); }';
const FRAG = 'precision mediump float; varying vec2 uv; uniform sampler2D tex; void main(){ gl_FragColor = texture2D(tex, uv); }';

function compile(gl: WebGLRenderingContext, type: number, source: string): WebGLShader {
  const shader = gl.createShader(type)!;
  gl.shaderSource(shader, source);
  gl.compileShader(shader);
  return shader;
}

function setupGl(gl: WebGLRenderingContext): void {
  const program = gl.createProgram()!;
  gl.attachShader(program, compile(gl, gl.VERTEX_SHADER, VERT));
  gl.attachShader(program, compile(gl, gl.FRAGMENT_SHADER, FRAG));
  gl.linkProgram(program);
  gl.useProgram(program);
  const buffer = gl.createBuffer();
  gl.bindBuffer(gl.ARRAY_BUFFER, buffer);
  gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1, -1, 1, -1, -1, 1, 1, 1]), gl.STATIC_DRAW);
  const loc = gl.getAttribLocation(program, 'p');
  gl.enableVertexAttribArray(loc);
  gl.vertexAttribPointer(loc, 2, gl.FLOAT, false, 0, 0);
}

/** Nicht leere Pixel, damit der Treiber den Speicher wirklich belegt (ein Muster aus Verläufen) */
function pattern(size: number): Uint8Array {
  const data = new Uint8Array(size * size * 4);
  for (let i = 0; i < data.length; i += 4) {
    const x = (i / 4) % size;
    data[i] = x & 255;
    data[i + 1] = (i >> 12) & 255;
    data[i + 2] = 128;
    data[i + 3] = 255;
  }
  return data;
}

function allocate(r: Run): void {
  const gl = r.gl;
  for (let i = 0; i < cfg.perStage; i++) {
    const texture = gl.createTexture()!;
    gl.bindTexture(gl.TEXTURE_2D, texture);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, cfg.size, cfg.size, 0, gl.RGBA, gl.UNSIGNED_BYTE, r.data);
    r.textures.push(texture);
  }
}

/** Erster WebGL-Fehler seit dem letzten Abruf als Name, sonst `null` */
function takeError(gl: WebGLRenderingContext): string | null {
  let first: string | null = null;
  for (let i = 0; i < 8; i++) {
    const code = gl.getError();
    if (code === 0) break;
    first ??= glErrorName(code);
  }
  return first;
}

function usedMiB(r: Run): number {
  return r.textures.length * textureMiB(cfg.size);
}

function render(): void {
  if (!run) return;
  state.textContent = run.ended ? '' : t('tex.running', { mib: Math.round(usedMiB(run)), stage: run.stages.length + 1 });
  out.textContent = run.stages.map(stageLine).join('\n');
}

/** `aborted`: Nutzer hat abgebrochen (kein Ergebnis bis zur Obergrenze) */
function finish(r: Run, aborted = false): void {
  if (r.ended) return;
  r.ended = true;
  cancelAnimationFrame(r.raf);
  const v = verdict(r.stages, r.loss);
  const lines: string[] = [];
  if (r.loss) {
    lines.push(t('tex.lost', { mib: Math.round(r.loss.mib), s: Math.round(r.loss.atMs / 100) / 10 }));
    lines.push(r.loss.restoredAfterMs === null ? t('tex.notRestored') : t('tex.restored', { ms: Math.round(r.loss.restoredAfterMs) }));
  }
  const key = v.kind === 'limit' ? 'tex.done.limit' : v.kind === 'error' ? 'tex.done.error' : aborted ? 'tex.done.aborted' : 'tex.done.none';
  lines.push(t(key, { safe: Math.round(v.safeMiB), loss: Math.round(v.lossMiB ?? 0), budget: v.budgetMiB }));
  lines.push(v.smoothMiB > 0 ? t('tex.smooth', { fps: SMOOTH_FPS, mib: Math.round(v.smoothMiB) }) : '');
  result.textContent = lines.filter(Boolean).join(' ');
  state.textContent = '';
  startBtn.textContent = t('tex.btn.start');
  if (!r.loss) r.gl.getExtension('WEBGL_lose_context')?.loseContext(); // Speicher freigeben
  lastReport = { ...buildReport(r, v), aborted };
  sendBtn.disabled = false;
  void send();
}

function buildReport(r: Run, v: ReturnType<typeof verdict>): object {
  return {
    kind: 'texture-probe', version: 1, device: deviceTag(), userAgent: navigator.userAgent, viewport: `${innerWidth}x${innerHeight}`, devicePixelRatio: devicePixelRatio,
    startedAt: new Date(Date.now() - (performance.now() - r.startedAt)).toISOString(), durationMs: Math.round(performance.now() - r.startedAt),
    config: cfg, info: describe(r.gl), stages: r.stages, loss: r.loss, verdict: v,
  };
}

function endStage(r: Run, now: number): void {
  const mib = usedMiB(r);
  const error = takeError(r.gl);
  r.stages.push({ stage: r.stages.length + 1, mib, textures: r.textures.length, fps: r.stats.fps(), frameP95: r.stats.p95(), error });
  r.stats.reset();
  r.stageStart = now;
  if (error || r.stages.length >= stageCount(cfg)) return finish(r);
  allocate(r);
}

function frame(now: number): void {
  const r = run;
  if (!r || r.ended) return;
  if (!r.loss) {
    r.stats.add(now - r.last);
    r.gl.bindTexture(r.gl.TEXTURE_2D, r.textures[r.frame++ % r.textures.length]!);
    r.gl.drawArrays(r.gl.TRIANGLE_STRIP, 0, 4);
    if (now - r.stageStart >= cfg.stageMs) endStage(r, now);
  }
  r.last = now;
  render();
  r.raf = requestAnimationFrame(frame);
}

function describe(gl: WebGLRenderingContext): Record<string, string | number> {
  const ext = gl.getExtension('WEBGL_debug_renderer_info');
  const mem = (performance as unknown as { memory?: { usedJSHeapSize: number; jsHeapSizeLimit: number } }).memory;
  return {
    version: gl.getParameter(gl.VERSION) as string,
    renderer: ext ? (gl.getParameter(ext.UNMASKED_RENDERER_WEBGL) as string) : (gl.getParameter(gl.RENDERER) as string),
    vendor: ext ? (gl.getParameter(ext.UNMASKED_VENDOR_WEBGL) as string) : (gl.getParameter(gl.VENDOR) as string),
    maxTexture: gl.getParameter(gl.MAX_TEXTURE_SIZE) as number,
    textureUnits: gl.getParameter(gl.MAX_TEXTURE_IMAGE_UNITS) as number,
    ...(mem ? { jsHeapMiB: Math.round(mem.usedJSHeapSize / 1048576), jsHeapLimitMiB: Math.round(mem.jsHeapSizeLimit / 1048576) } : {}),
  };
}

function showDevice(info: Record<string, string | number> | null): void {
  const dl = el<HTMLDListElement>('device');
  const rows: [string, string][] = [
    ['User-Agent', navigator.userAgent],
    ['Viewport', `${innerWidth}×${innerHeight} @${devicePixelRatio}`],
    ...Object.entries(info ?? {}).map(([k, v]): [string, string] => [k, String(v)]),
  ];
  dl.replaceChildren(...rows.flatMap(([k, v]) => [Object.assign(document.createElement('dt'), { textContent: k }), Object.assign(document.createElement('dd'), { textContent: v })]));
}

function start(): void {
  const canvas = document.createElement('canvas');
  canvas.width = 640;
  canvas.height = 360;
  stage.replaceChildren(canvas);
  const gl = (canvas.getContext('webgl2') ?? canvas.getContext('webgl')) as WebGL2RenderingContext | WebGLRenderingContext | null;
  out.textContent = '';
  result.textContent = '';
  if (!gl) {
    result.textContent = t('tex.noGl');
    return showDevice(null);
  }
  setupGl(gl);
  showDevice(describe(gl));
  const now = performance.now();
  const r: Run = { gl, canvas, textures: [], stats: new FrameStats(), stages: [], loss: null, data: pattern(cfg.size), startedAt: now, stageStart: now, last: now, frame: 0, raf: 0, lostAt: 0, ended: false };
  canvas.addEventListener('webglcontextlost', (e) => {
    e.preventDefault(); // Wiederherstellung erlauben
    r.lostAt = performance.now();
    r.loss = { mib: usedMiB(r), atMs: r.lostAt - r.startedAt, restoredAfterMs: null };
    setTimeout(() => finish(r), 6000); // nicht wiederhergestellt
  });
  canvas.addEventListener('webglcontextrestored', () => {
    if (r.loss) r.loss.restoredAfterMs = performance.now() - r.lostAt;
    finish(r);
  });
  run = r;
  startBtn.textContent = t('tex.btn.stop');
  sendBtn.disabled = true;
  allocate(r);
  r.raf = requestAnimationFrame(frame);
}

async function send(): Promise<void> {
  if (!lastReport) return;
  try {
    const res = await fetch('/api/report', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(lastReport) });
    if (!res.ok) throw new Error(`HTTP ${res.status}`);
    el('sent').textContent = t('tex.sent');
  } catch (err) {
    el('sent').textContent = t('tex.sendFailed', { error: String(err) });
  }
}

startBtn.addEventListener('click', () => {
  if (run && !run.ended) return finish(run, true); // abbrechen: mit dem erreichten Stand auswerten
  start();
});
sendBtn.addEventListener('click', () => void send());

const sizes = el<HTMLDivElement>('sizes');
for (const px of TEXTURE_SIZES) {
  const button = document.createElement('button');
  button.type = 'button';
  button.dataset.size = String(px);
  button.textContent = t('tex.btn.size', { px, mib: textureMiB(px) });
  button.setAttribute('aria-pressed', String(px === cfg.size));
  button.addEventListener('click', () => {
    if (run && !run.ended) return; // nicht während eines Laufs
    cfg = defaultConfig(px as TextureSize);
    sizes.querySelectorAll('button').forEach((b) => b.setAttribute('aria-pressed', String(b === button)));
  });
  sizes.append(button);
}
state.textContent = t('tex.idle');
showDevice(null);
