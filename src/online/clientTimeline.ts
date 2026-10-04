import { interpolate } from './clientInterpolation';
import type { WorldState } from './clientProtocol';

/**
 * Zeitleiste der Darstellung (B-277): puffert empfangene Zustände und zeichnet zur geschätzten Server-Zeit minus einer
 * Verzögerung, die der gemessenen Ankunfts-Schwankung folgt. Läuft der Puffer leer, laufen Figuren kurz weiter.
 * Die Darstellungszeit läuft nur vorwärts und höchstens `RATE_SLACK` schneller oder langsamer als die Uhr.
 */

/** Untergrenze der Verzögerung in Ticks (angenommen) */
const MIN_DELAY_TICKS = 1;
/** Obergrenze der Verzögerung (angenommen) */
export const MAX_DELAY_MS = 150;
/** So lange laufen Figuren ohne neuen Zustand weiter (angenommen) */
export const MAX_EXTRAPOLATE_MS = 100;
/** Glättung der Server-Zeit-Schätzung und der Schwankung je Frame (angenommen) */
const SMOOTHING = 0.1;
/** Verzögerung = Untergrenze + so viele mittlere Abweichungen der Ankunft (angenommen) */
const JITTER_FACTOR = 2;
/** Die Darstellungszeit läuft höchstens 25 % schneller oder langsamer als die Uhr (angenommen) */
const RATE_SLACK = 0.25;
/** Weicht die Darstellungszeit weiter vom Ziel ab (Pause, Wiederverbinden), springt sie (angenommen) */
const SNAP_MS = 250;
/** Größe des Puffers (angenommen, deckt `MAX_DELAY_MS` bei 30 Hz reichlich) */
const MAX_FRAMES = 20;

/** Ein empfangener Zustand: passt zu `Frame` aus `clientConnection.ts`. */
export interface TimedState {
  tick: number;
  receivedAt: number;
  state: WorldState;
}

export class Timeline {
  private frames: TimedState[] = [];
  /** Geschätzte Empfangszeit von Tick 0 (`receivedAt - tick * tickMs`, geglättet) */
  private offset: number | null = null;
  /** Mittlere Abweichung der Ankunft von der Schätzung in ms */
  private jitter = 0;
  /** Gezeichneter Tick (mit Bruchteil) und die Uhrzeit dazu */
  private renderTick: number | null = null;
  private lastNow = 0;

  constructor(private readonly tickMs: number) {}

  /** Aktuelle Verzögerung hinter der geschätzten Server-Zeit in ms (Debug-Overlay). */
  get delayMs(): number {
    return Math.min(MAX_DELAY_MS, Math.max(MIN_DELAY_TICKS * this.tickMs, MIN_DELAY_TICKS * this.tickMs + JITTER_FACTOR * this.jitter));
  }

  push(frame: TimedState): void {
    const o = frame.receivedAt - frame.tick * this.tickMs;
    if (this.offset === null) this.offset = o;
    this.jitter += (Math.abs(o - this.offset) - this.jitter) * SMOOTHING;
    this.offset += (o - this.offset) * SMOOTHING;
    this.frames.push(frame);
    if (this.frames.length > MAX_FRAMES) this.frames.shift();
  }

  /** Gezeichneter Zustand zur Uhrzeit `now`; null, solange kein Zustand da ist. */
  sample(now: number): WorldState | null {
    if (this.offset === null) return null;
    const tick = this.advance(now, (now - this.offset - this.delayMs) / this.tickMs);
    while (this.frames.length > 2 && this.frames[1]!.tick <= tick) this.frames.shift();
    const f = this.frames;
    if (f.length === 1) return f[0]!.state;
    const [a, b] = tick < f[1]!.tick ? [f[0]!, f[1]!] : [f.at(-2)!, f.at(-1)!];
    const t = Math.max(a.tick, Math.min(tick, b.tick + MAX_EXTRAPOLATE_MS / this.tickMs));
    return interpolate(a.state, b.state, (t - a.tick) / (b.tick - a.tick));
  }

  /** Darstellungszeit Richtung Ziel nachführen: nur vorwärts, Tempo begrenzt, großer Abstand springt. */
  private advance(now: number, target: number): number {
    const dt = (now - this.lastNow) / this.tickMs;
    this.lastNow = now;
    const r = this.renderTick;
    if (r === null || Math.abs(target - r) * this.tickMs > SNAP_MS) this.renderTick = target;
    else this.renderTick = Math.min(r + dt * (1 + RATE_SLACK), Math.max(r + dt * (1 - RATE_SLACK), target));
    return this.renderTick;
  }
}
