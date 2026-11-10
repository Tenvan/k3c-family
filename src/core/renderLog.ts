/**
 * Render-Log (B-762, RX1.1): Startzeile, Ladedaten, Kontext-Ereignisse und Leistungsverlauf des Clients im Server-Log
 * (`POST /api/clientlog`). Phaser-frei; die Werte kommen aus `renderInfo.ts` (keine zweite Messung), die Uhr und das
 * Schreiben sind austauschbar, damit Tests ohne Browser laufen. Eine Zeile je Ereignis (Rate-Limit des Servers).
 */
import { clientLog, type LogLevel } from './clientLog';
import type { RenderInfo } from './renderInfo';
import { budgetState, budgetText } from './textureBudget';

/** Abstand der Verlaufs-Zeilen */
export const TREND_EVERY_MS = 30_000;
/** Darunter gilt der Client als zu langsam (Warnzeile) */
export const LOW_FPS = 20;
/** Höchstens eine Warnzeile je Minute */
export const WARN_EVERY_MS = 60_000;

/** Schlüssel in der Phaser-Registry: `{ log: RenderLog, version: string }` */
export const RENDER_LOG_KEY = 'renderLog';

export type ContextEvent = 'lost' | 'restored' | 'fallback';

export type LogFn = (level: LogLevel, msg: string, ctx?: Record<string, unknown>) => void;

export interface RenderLogEnv {
  log: LogFn;
  /** Millisekunden, monoton (performance.now) */
  now(): number;
  /** Aktueller Stand von Renderer und Texturen (aus `readRenderInfo`) */
  info(): RenderInfo;
  /** JS-Heap in MiB, `null` wenn der Browser ihn nicht liefert */
  heapMiB(): number | null;
  /** Texturbudget dieses Geräts in MiB (B-772), `null`/fehlend ohne Budget */
  budgetMiB?(): number | null;
}

const f1 = (v: number): number => Math.round(v * 10) / 10;

/** Startzeile: Renderer, GPU, Texturen, Größen, Browser und Client-Version */
export function startLine(info: RenderInfo, meta: { userAgent: string; version: string; budgetMiB?: number | null }): { msg: string; ctx: Record<string, unknown> } {
  const gpu = info.kind === 'Canvas' ? '' : `, GPU ${info.gpu ?? 'unbekannt'}, maxTex ${info.maxTexture ?? '–'}`;
  const budget = meta.budgetMiB ?? null;
  const budgetPart = budget === null ? '' : `, ${budgetText(info.textureMiB, budget)} [${budgetState(info.textureMiB, budget)}]`;
  const msg = `🚀 Renderer: ${info.kind}${gpu}, ${info.textures} Texturen, ~${f1(info.textureMiB)} MiB${budgetPart}`;
  return {
    msg,
    ctx: {
      // WebGL-Version: Phaser fordert getContext('webgl') an, das ist WebGL 1 (kind 'WebGL'); 2 nur bei einem WebGL2-Kontext (B-772)
      glVersion: info.kind === 'WebGL2' ? 2 : info.kind === 'WebGL' ? 1 : 0,
      budgetMiB: budget,
      budgetState: budget === null ? null : budgetState(info.textureMiB, budget),
      renderer: info.kind,
      requested: info.request,
      gpu: info.gpu,
      maxTexture: info.maxTexture,
      textures: info.textures,
      textureMiB: f1(info.textureMiB),
      buffer: `${info.width}x${info.height}`,
      viewport: info.viewport,
      dpr: info.devicePixelRatio,
      ua: meta.userAgent,
      version: meta.version,
    },
  };
}

/** Sammelt Frames und schreibt Verlauf, Warnung und Kontext-Ereignisse */
export class RenderLog {
  private readonly startedAt: number;
  private windowStart: number;
  private frames = 0;
  private sumMs = 0;
  private minFps = Infinity;
  private trends = 0;
  private lastWarnAt = -Infinity;
  private lastBudgetWarnAt = -Infinity;

  constructor(private readonly env: RenderLogEnv) {
    this.startedAt = env.now();
    this.windowStart = this.startedAt;
  }

  /** Seit Start in Sekunden, eine Nachkommastelle */
  private sinceStart(): number {
    return f1((this.env.now() - this.startedAt) / 1000);
  }

  private state(extra: Record<string, unknown> = {}): Record<string, unknown> {
    const i = this.env.info();
    const budget = this.env.budgetMiB?.() ?? null;
    const budgetPart = budget === null ? {} : { budgetMiB: budget, budgetState: budgetState(i.textureMiB, budget) };
    return { sinceStartS: this.sinceStart(), textures: i.textures, textureMiB: f1(i.textureMiB), renderer: i.kind, ...budgetPart, ...extra };
  }

  /** Einmal nach dem Laden */
  start(meta: { userAgent: string; version: string }): void {
    const { msg, ctx } = startLine(this.env.info(), { ...meta, budgetMiB: this.env.budgetMiB?.() ?? null });
    this.env.log('info', msg, ctx);
  }

  /** Warnzeile, wenn der Texturspeicher das Budget erreicht (höchstens eine je Minute); ohne Budget nichts */
  private checkBudget(): void {
    const budget = this.env.budgetMiB?.() ?? null;
    if (budget === null) return;
    const used = this.env.info().textureMiB;
    const now = this.env.now();
    if (budgetState(used, budget) !== 'over' || now - this.lastBudgetWarnAt < WARN_EVERY_MS) return;
    this.lastBudgetWarnAt = now;
    this.env.log('warn', `🐢 ${budgetText(used, budget)}: Budget überschritten, der WebGL-Kontext ist in Gefahr`, this.state({ budgetExceeded: true }));
  }

  /** Ein Frame: `deltaMs` ist die Frame-Zeit, `fps` die aktuell gemessene Rate */
  frame(deltaMs: number, fps: number): void {
    if (!Number.isFinite(deltaMs) || deltaMs < 0) return;
    const now = this.env.now();
    this.frames++;
    this.sumMs += deltaMs;
    if (fps > 0) this.minFps = Math.min(this.minFps, fps);
    if (fps > 0 && fps < LOW_FPS && now - this.lastWarnAt >= WARN_EVERY_MS) {
      this.lastWarnAt = now;
      this.env.log('warn', `🐢 FPS ${Math.round(fps)} unter ${LOW_FPS}, Texturen ~${f1(this.env.info().textureMiB)} MiB`, this.state({ fps: Math.round(fps) }));
    }
    if (now - this.windowStart >= TREND_EVERY_MS) this.trend(now);
  }

  private trend(now: number): void {
    const seconds = (now - this.windowStart) / 1000;
    const meanFps = this.sumMs > 0 ? (this.frames * 1000) / this.sumMs : 0;
    const meanMs = this.frames > 0 ? this.sumMs / this.frames : 0;
    const heap = this.env.heapMiB();
    this.trends++;
    const minFps = Number.isFinite(this.minFps) ? Math.round(this.minFps) : 0;
    const textureMiB = f1(this.env.info().textureMiB);
    this.env.log('info', `💓 FPS Ø ${Math.round(meanFps)}, min ${minFps}, Frame Ø ${f1(meanMs)} ms, Texturen ~${textureMiB} MiB #${this.trends}`, {
      ...this.state({ fpsMean: Math.round(meanFps), fpsMin: minFps, frameMeanMs: f1(meanMs), frames: this.frames, windowS: f1(seconds), heapMiB: heap === null ? null : f1(heap) }),
    });
    this.windowStart = now;
    this.frames = 0;
    this.sumMs = 0;
    this.minFps = Infinity;
    this.checkBudget();
  }

  /** `webglcontextlost`, `webglcontextrestored` oder Canvas-Fallback, jeweils mit dem Zustand dieses Augenblicks */
  context(event: ContextEvent, fps: number | null): void {
    const words = { lost: '💥 WebGL Context lost', restored: '🩹 WebGL Context restored', fallback: '🚦 Canvas-Fallback' } as const;
    const i = this.env.info();
    this.env.log(event === 'restored' ? 'info' : 'error', `${words[event]} nach ${this.sinceStart().toString().replace('.', ',')} s, ${i.textures} Texturen, ~${f1(i.textureMiB)} MiB`, this.state({ event, fps: fps === null ? null : Math.round(fps) }));
  }

  /** Ladevorgang einer Gruppe oder Stufe: Anzahl, geschätzte Größe, Dauer */
  loaded(group: string, files: number, mib: number, ms: number): void {
    this.env.log('info', `📂 Geladen ${group}: ${files} Dateien, ~${f1(mib)} MiB, ${Math.round(ms)} ms`, this.state({ group, files, loadedMiB: f1(mib), loadMs: Math.round(ms) }));
    this.checkBudget();
  }
}

/** Fehlgeschlagener Ladevorgang: Datei und HTTP-Status (0 = Netz/unbekannt) */
export function logLoadError(file: string, status: number, log: LogFn = clientLog): void {
  log('error', `💥 Laden fehlgeschlagen: ${file} (Status ${status})`, { file, status });
}
