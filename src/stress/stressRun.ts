/**
 * Messlauf der Stress-Szene (B-765): sammelt je Stufe Frame-Zeiten, schreibt eine Log-Zeile je Stufe, Ereigniszeilen und eine
 * maschinenlesbare Endzeile (Dichte-Grenze). Phaser-frei; Uhr, Log und Renderer-Werte kommen vom Aufrufer.
 * Die Zeiten laufen auf der Spielzeit (Summe der Frame-Zeiten, je höchstens 1 s): Eine Pause des Tabs verschiebt die Rampe nicht.
 * Die ersten `WARMUP_MS` jeder Stufe zählen nicht in die Messung (Ladespitzen nach dem Anlegen neuer Figuren).
 */
import { Histogram } from '../online/clientPerfReport';
import type { RenderInfo } from '../core/renderInfo';
import type { LogFn } from '../core/renderLog';

export const WARMUP_MS = 2000;
/** Ab dieser Rate (Ø FPS) hält eine Stufe */
export const OK_FPS = 30;
const SLOW_MS = 33.4;
const DROP_MS = 50;
const MAX_FRAME_MS = 1000;

export interface RunMeta {
  runId: string;
  scenario: string;
  players: number;
  seed: number;
  /** Hash der Szenario-Datei */
  fileHash: string;
  params: string;
  version: string;
  userAgent: string;
  stages: number;
  everyMs: number;
}

export interface StageSummary {
  stage: number;
  entities: number;
  sheets: number;
  /** Figuren im Bild je Spieler-Viewport */
  inView: number[];
  durationMs: number;
  frames: number;
  warmupFrames: number;
  fpsMean: number;
  fpsMin: number;
  fpsP5: number;
  frameMeanMs: number;
  frameP50: number;
  frameP95: number;
  frameP99: number;
  frameMax: number;
  over50: number;
  over33: number;
  textures: number;
  textureMiB: number;
  heapMiB: number | null;
}

export type EndReason = 'done' | 'contextLost' | 'aborted';

export interface RunResult {
  reason: EndReason;
  stages: StageSummary[];
  /** Letzte Stufe einer ununterbrochenen Reihe ab Stufe 1 mit Ø FPS ≥ `OK_FPS`; `null` = schon Stufe 1 zu langsam */
  densityLimit: { stage: number; entities: number } | null;
}

export interface RunEnv {
  log: LogFn;
  info(): RenderInfo;
  heapMiB(): number | null;
}

const r1 = (v: number): number => Math.round(v * 10) / 10;

/** Größte Stufe, bis zu der alle Stufen ≥ `OK_FPS` Ø FPS hielten */
export function densityLimit(stages: readonly StageSummary[]): RunResult['densityLimit'] {
  let last: StageSummary | null = null;
  for (const s of stages) {
    if (s.fpsMean < OK_FPS) break;
    last = s;
  }
  return last ? { stage: last.stage, entities: last.entities } : null;
}

export class StressRun {
  private readonly stages: StageSummary[] = [];
  private clock = 0;
  private hist = new Histogram();
  private sumMs = 0;
  private frames = 0;
  private warm = 0;
  private slow = 0;
  private drops = 0;
  private maxMs = 0;
  private open: { stage: number; entities: number; sheets: number } | null = null;
  private ended = false;

  constructor(
    private readonly meta: RunMeta,
    private readonly env: RunEnv,
  ) {}

  /** Startzeile mit allen Angaben, die einen Lauf eindeutig machen */
  begin(): void {
    const m = this.meta;
    this.env.log('info', `🚀 Stress ${m.scenario}: ${m.players} Spieler, Seed ${m.seed}, ${m.stages} Stufen à ${m.everyMs / 1000} s [${m.runId}]`, {
      runId: m.runId,
      scenario: m.scenario,
      players: m.players,
      seed: m.seed,
      stages: m.stages,
      everyMs: m.everyMs,
      fileHash: m.fileHash,
      params: m.params,
      version: m.version,
      ua: m.userAgent,
    });
  }

  /** Beginnt Stufe `stage` (ab 1) mit der Zahl der Figuren und verschiedenen Typen */
  startStage(stage: number, entities: number, sheets: number): void {
    this.open = { stage, entities, sheets };
    this.clock = 0;
    this.hist = new Histogram();
    this.sumMs = this.frames = this.warm = this.slow = this.drops = this.maxMs = 0;
  }

  /** Spielzeit der laufenden Stufe in ms */
  get stageClockMs(): number {
    return this.clock;
  }

  /** Ein Frame; `true`, wenn die Stufe fertig ist (`everyMs` Spielzeit) */
  frame(deltaMs: number): boolean {
    if (!this.open || this.ended || !Number.isFinite(deltaMs) || deltaMs < 0) return false;
    this.clock += Math.min(deltaMs, MAX_FRAME_MS);
    if (this.clock <= WARMUP_MS) {
      this.warm++;
      return false;
    }
    this.frames++;
    this.sumMs += deltaMs;
    this.hist.add(deltaMs);
    this.maxMs = Math.max(this.maxMs, deltaMs);
    if (deltaMs > SLOW_MS) this.slow++;
    if (deltaMs > DROP_MS) this.drops++;
    return this.clock >= this.meta.everyMs;
  }

  /** Beendet die Stufe, schreibt die Zeile und liefert die Zusammenfassung */
  endStage(inView: number[]): StageSummary {
    const o = this.open!;
    const q = this.hist.quantiles();
    const info = this.env.info();
    const mean = this.frames > 0 ? this.sumMs / this.frames : 0;
    const s: StageSummary = {
      stage: o.stage,
      entities: o.entities,
      sheets: o.sheets,
      inView,
      durationMs: Math.round(this.sumMs),
      frames: this.frames,
      warmupFrames: this.warm,
      fpsMean: this.sumMs > 0 ? r1((this.frames * 1000) / this.sumMs) : 0,
      fpsMin: this.maxMs > 0 ? r1(1000 / this.maxMs) : 0,
      fpsP5: q && q.p95 > 0 ? r1(1000 / q.p95) : 0,
      frameMeanMs: r1(mean),
      frameP50: q?.p50 ?? 0,
      frameP95: q?.p95 ?? 0,
      frameP99: q?.p99 ?? 0,
      frameMax: Math.round(this.maxMs),
      over50: this.drops,
      over33: this.slow,
      textures: info.textures,
      textureMiB: r1(info.textureMiB),
      heapMiB: this.env.heapMiB() === null ? null : r1(this.env.heapMiB()!),
    };
    this.stages.push(s);
    this.open = null;
    this.env.log(s.fpsMean >= OK_FPS ? 'info' : 'warn', `💓 Stufe ${s.stage}/${this.meta.stages}: ${s.entities} Figuren, ${s.sheets} Typen, FPS Ø ${s.fpsMean} (min ${s.fpsMin}), p95 ${s.frameP95} ms, ${s.textureMiB} MiB [${this.meta.runId}]`, {
      runId: this.meta.runId,
      ...s,
    });
    return s;
  }

  /** Ereignis mit Stufe und Spielzeit (Kontextverlust, Pause, Abbruch, Ladefehler) */
  event(text: string, detail: Record<string, unknown> = {}): void {
    this.env.log('warn', `🚦 ${text} (Stufe ${this.open?.stage ?? '–'}, ${Math.round(this.clock)} ms) [${this.meta.runId}]`, { runId: this.meta.runId, stage: this.open?.stage ?? null, clockMs: Math.round(this.clock), ...detail });
  }

  /** Endzeile: Dichte-Grenze und kompakte Tabelle je Stufe; mit einer offenen Stufe wird sie nicht mitgezählt */
  finish(reason: EndReason): RunResult {
    if (this.ended) return this.result(reason);
    this.ended = true;
    const res = this.result(reason);
    const lim = res.densityLimit;
    const why = { done: 'Ziel erreicht', contextLost: 'Kontextverlust', aborted: 'abgebrochen' }[reason];
    this.env.log(reason === 'done' ? 'info' : 'error', `✅ Ergebnis ${this.meta.scenario}: Dichte-Grenze ${lim ? `Stufe ${lim.stage} (${lim.entities} Figuren)` : 'keine (Stufe 1 unter ' + OK_FPS + ' FPS)'}, Ende: ${why} [${this.meta.runId}]`, {
      runId: this.meta.runId,
      reason,
      limit: lim,
      stages: res.stages.map((s) => ({ n: s.stage, e: s.entities, fps: s.fpsMean, min: s.fpsMin, p95: s.frameP95, d50: s.over50, mib: s.textureMiB })),
    });
    return res;
  }

  private result(reason: EndReason): RunResult {
    return { reason, stages: [...this.stages], densityLimit: densityLimit(this.stages) };
  }
}
