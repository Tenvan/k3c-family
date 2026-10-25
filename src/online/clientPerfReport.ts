/**
 * Performance-Modus (B-334, PM1.2): Mit `?perf=1` sammelt der Client Frame-Zeiten, Zustands-Abstände, Puffer und Latenz,
 * hält Einbrüche mit Begleitwerten fest und schickt am Ende einen Bericht über `POST /api/report` (Server legt ihn
 * unverändert als `reports/*.json` ab, `engine/store/reports.go`). Phaser-frei; ohne `?perf=1` entsteht nichts davon.
 */
import type { PerfValues } from './clientPerf';

/**
 * Schwellen eines Einbruchs. `data/` kennt keine Client-Schwellen (die JSONs dort sind Spiel-Balancing für Client und
 * Server), deshalb Konstanten: Ein Frame über 50 ms (unter 20 FPS) ruckelt sichtbar; eine Zustands-Lücke über 250 ms
 * ist rund 7 Ticks bei 30 Hz und deutlich länger als der Puffer (`MAX_DELAY_MS` 150), die Zeitleiste läuft dann leer.
 */
export const FRAME_DROP_MS = 50;
export const STATE_GAP_MS = 250;
/** Obergrenzen, damit der Bericht klein bleibt (`keepalive` beim Verlassen erlaubt nur 64 KiB) */
export const MAX_DROPS = 100;
export const MAX_SERIES = 180;
/** Verteilungen in 1-ms-Klassen bis hierher; längere Werte zählen in der letzten Klasse (fester Speicher) */
export const HIST_MAX_MS = 2000;
/** Abstand der Verlaufs-Punkte (wie die Diagnose-Zeile) */
export const SERIES_EVERY_MS = 5000;

/** Performance-Modus nur mit `?perf=1` (Freigabe PM1). `search` ist `location.search`. */
export const perfEnabled = (search: string): boolean => new URLSearchParams(search).get('perf') === '1';

export interface Quantiles {
  count: number;
  mean: number;
  p50: number;
  p95: number;
  p99: number;
  max: number;
}

/** Verteilung von Millisekunden mit Perzentilen nach der Rang-Methode (wie `LatencyMeter`). */
export class Histogram {
  private readonly counts = new Uint32Array(HIST_MAX_MS + 1);
  private n = 0;
  private sum = 0;
  private top = 0;

  add(ms: number): void {
    const v = Math.max(0, ms);
    this.counts[Math.min(HIST_MAX_MS, Math.round(v))]!++;
    this.n++;
    this.sum += v;
    this.top = Math.max(this.top, v);
  }

  /** Kleinste Klasse, bis zu der mindestens `p` aller Werte liegen. */
  percentile(p: number): number {
    const target = Math.max(1, Math.ceil(this.n * p));
    let seen = 0;
    for (let i = 0; i < this.counts.length; i++) {
      seen += this.counts[i]!;
      if (seen >= target) return i;
    }
    return HIST_MAX_MS;
  }

  quantiles(): Quantiles | null {
    if (this.n === 0) return null;
    const mean = Math.round((this.sum / this.n) * 10) / 10;
    return { count: this.n, mean, p50: this.percentile(0.5), p95: this.percentile(0.95), p99: this.percentile(0.99), max: Math.round(this.top) };
  }
}

export type DropKind = 'frame' | 'state';

/** Ein Einbruch mit Zeitpunkt (ms seit Messbeginn) und den Werten im selben Moment */
export interface Drop extends PerfValues {
  atMs: number;
  kind: DropKind;
  ms: number;
}

/** Einbruch, wenn die Frame-Zeit oder die Zustands-Lücke die Schwelle überschreitet; sonst null. */
export function detectDrop(kind: DropKind, ms: number, atMs: number, v: PerfValues): Drop | null {
  const limit = kind === 'frame' ? FRAME_DROP_MS : STATE_GAP_MS;
  return ms > limit ? { atMs: Math.round(atMs), kind, ms: Math.round(ms), ...v } : null;
}

export interface SeriesPoint extends PerfValues {
  atMs: number;
}

/** Gerät und Lauf, vom Aufrufer gesetzt */
export interface ReportMeta {
  /** `end`: Spiel verlassen oder beendet; `pagehide`: Seite verlassen */
  reason: 'end' | 'pagehide';
  device: string;
  room: string | null;
  players: number;
  cameras: number;
  userAgent: string;
  viewport: string;
  startedAt: string;
}

/** Bericht für `POST /api/report` (`kind: 'perf'` unterscheidet ihn vom Gamepad-Bericht) */
export interface PerfReport extends ReportMeta {
  kind: 'perf';
  version: 1;
  durationMs: number;
  thresholds: { frameMs: number; stateGapMs: number };
  frameMs: Quantiles | null;
  fps: number | null;
  stateMs: Quantiles | null;
  bufferMs: Quantiles | null;
  latencyMs: Quantiles | null;
  drops: Drop[];
  dropsTotal: number;
  series: SeriesPoint[];
}

/** Sammelt einen Messlauf; alle Zeiten in ms auf der Uhr des Aufrufers. */
export class PerfRecorder {
  private readonly frames = new Histogram();
  private readonly states = new Histogram();
  private readonly buffer = new Histogram();
  private readonly latency = new Histogram();
  private readonly drops: Drop[] = [];
  private dropsTotal = 0;
  private readonly series: SeriesPoint[] = [];
  private lastPointAt = -Infinity;
  /** Der erste Frame nach Start oder Pause misst die Pause mit und zählt nicht (keine Schein-Einbrüche). */
  private skip = true;

  constructor(private readonly startMs: number) {}

  pause(): void {
    this.skip = true;
  }

  /** Frame mit Dauer `dtMs`; Rückgabe ist ein neuer Einbruch oder null. */
  frame(now: number, dtMs: number, v: PerfValues): Drop | null {
    if (this.skip) {
      this.skip = false;
      return null;
    }
    this.frames.add(dtMs);
    if (now - this.lastPointAt >= SERIES_EVERY_MS) this.point(now, v);
    return this.note(detectDrop('frame', dtMs, now - this.startMs, v));
  }

  /** Neuer Zustand `gapMs` nach dem vorigen; Puffer und Latenz zählen je Zustand. */
  state(now: number, gapMs: number, v: PerfValues): Drop | null {
    this.states.add(gapMs);
    this.buffer.add(v.bufferMs);
    if (v.latencyMs !== null) this.latency.add(v.latencyMs);
    return this.note(detectDrop('state', gapMs, now - this.startMs, v));
  }

  report(meta: ReportMeta, now: number): PerfReport {
    const frameMs = this.frames.quantiles();
    return {
      kind: 'perf',
      version: 1,
      ...meta,
      durationMs: Math.round(now - this.startMs),
      thresholds: { frameMs: FRAME_DROP_MS, stateGapMs: STATE_GAP_MS },
      frameMs,
      fps: frameMs && frameMs.mean > 0 ? Math.round(1000 / frameMs.mean) : null,
      stateMs: this.states.quantiles(),
      bufferMs: this.buffer.quantiles(),
      latencyMs: this.latency.quantiles(),
      drops: [...this.drops],
      dropsTotal: this.dropsTotal,
      series: [...this.series],
    };
  }

  private point(now: number, v: PerfValues): void {
    this.lastPointAt = now;
    this.series.push({ atMs: Math.round(now - this.startMs), ...v });
    if (this.series.length > MAX_SERIES) this.series.shift(); // langer Lauf: der jüngste Verlauf zählt
  }

  /** Zählt jeden Einbruch, behält die ersten `MAX_DROPS` mit Begleitwerten. */
  private note(d: Drop | null): Drop | null {
    if (!d) return null;
    this.dropsTotal++;
    if (this.drops.length < MAX_DROPS) this.drops.push(d);
    return d;
  }
}

/** Schickt den Bericht; `keepalive` trägt ihn auch beim Verlassen der Seite. Fehler werden verworfen, nie geworfen. */
export async function sendPerfReport(report: PerfReport, fetchFn: typeof fetch = fetch): Promise<boolean> {
  try {
    const res = await fetchFn('/api/report', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(report), keepalive: true });
    return res.ok;
  } catch {
    return false; // Server nicht erreichbar: Bericht verwerfen, das Spiel läuft weiter
  }
}
