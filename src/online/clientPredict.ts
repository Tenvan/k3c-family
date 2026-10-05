import { MONARCH } from '../model/data';
import type { SlotInput, SlotSeat, WorldState } from './clientProtocol';
import type { TimedState } from './clientTimeline';

/**
 * Anzeige-Vorhersage der lokalen Monarchen (B-039, B-277): nur die x-Position. Je Frame läuft der Monarch mit der
 * lokalen Laufrichtung × beobachteter Geschwindigkeit und wird um `PULL` zu einem Ziel gezogen: neuestes Server-x plus
 * die eigene Bewegung, die der Server noch nicht kennen kann (Schritte der letzten Latenz). Großer Abstand übernimmt
 * das Ziel. Keine Spiel-Logik: Rand, Tod, Bauen usw. korrigiert der Server.
 */

/** Anteil je 60-FPS-Frame, um den die Vorhersage zum Ziel gezogen wird (angenommen; bei anderer Bildrate gleich schnell je Sekunde) */
export const PULL = 0.12;
/** Ab diesem Abstand zum Ziel wird sofort übernommen (Teleport, Respawn; angenommen) */
export const SNAP_UNITS = 3;
/** Bei hoher Geschwindigkeit (Zeitraffer) liegt die Sprung-Schwelle bei so viel Laufzeit (s, angenommen) */
const SNAP_SECONDS = 0.4;
/** Obergrenze des Vorlaufs vor dem Server-x, in ms Laufzeit (angenommen) */
export const MAX_LEAD_MS = 150;
/** Geschwindigkeit, bis eine Bewegung beobachtet ist: `base.speed` aus `data/monarch.json` (Units/s) */
export const FALLBACK_SPEED = MONARCH.base.speed;
/** Glättung der beobachteten Geschwindigkeit je Zustand (angenommen) */
const SPEED_SMOOTHING = 0.3;
/** Langsamer zählt als Stehen und ändert die Geschwindigkeit nicht (Units/s, angenommen) */
const MIN_SPEED = 0.5;

const snapUnits = (speed: number) => Math.max(SNAP_UNITS, speed * SNAP_SECONDS);

interface Track {
  serverX: number;
  tick: number;
  receivedAt: number;
  speed: number;
  /** Tot (`respawnIn` > 0) oder Raum angehalten: keine eigene Bewegung */
  blocked: boolean;
  x: number | null;
  /** Vorhergesagte Schritte (Uhrzeit, Units), solange der Server sie noch nicht zeigen kann */
  steps: { at: number; dx: number }[];
}

export class Predictor {
  private tracks = new Map<number, Track>();
  private lastNow: number | null = null;

  constructor(private readonly tickMs: number) {}

  /** Je empfangenem Frame: neuestes Server-x und beobachtete Geschwindigkeit der lokalen Monarchen. */
  observe(frame: TimedState, you: readonly SlotSeat[]): void {
    for (const { monarch } of you) {
      const p = frame.state.players.find((q) => q.index === monarch);
      if (!p) continue;
      const blocked = p.respawnIn > 0 || frame.state.devPaused === true;
      const t = this.tracks.get(monarch);
      if (!t) {
        this.tracks.set(monarch, { serverX: p.x, tick: frame.tick, receivedAt: frame.receivedAt, speed: FALLBACK_SPEED, blocked, x: null, steps: [] });
        continue;
      }
      const dx = Math.abs(p.x - t.serverX);
      const v = (dx * 1000) / (Math.max(1, frame.tick - t.tick) * this.tickMs);
      if (v > MIN_SPEED && dx < snapUnits(t.speed)) t.speed += (v - t.speed) * SPEED_SMOOTHING;
      Object.assign(t, { serverX: p.x, tick: frame.tick, receivedAt: frame.receivedAt, blocked });
    }
  }

  /**
   * Gezeichneter Zustand mit vorhergesagtem x der lokalen Monarchen; alles andere unverändert.
   * `latencyMs`: gemessene Latenz Eingabe → Zustand (Mittel), null = noch keine Messung (dann ein Tick).
   */
  draw(state: WorldState, now: number, inputs: readonly SlotInput[], you: readonly SlotSeat[], latencyMs: number | null = null): WorldState {
    const dt = this.lastNow === null ? 0 : (now - this.lastNow) / 1000;
    this.lastNow = now;
    const leadMs = latencyMs === null ? this.tickMs : Math.min(MAX_LEAD_MS, Math.max(0, latencyMs));
    const xs = new Map<number, number>();
    for (const { slot, monarch } of you) {
      const t = this.tracks.get(monarch);
      if (!t) continue;
      const dir = t.blocked ? 0 : Math.sign(inputs.find((i) => i.slot === slot)?.moveX ?? 0);
      const step = dir * t.speed * dt;
      t.steps.push({ at: now, dx: step });
      // ohne neue Zustände (Verbindung weg) läuft der Vorlauf höchstens MAX_LEAD_MS weiter
      const since = Math.max(t.receivedAt, now - MAX_LEAD_MS) - leadMs;
      while (t.steps.length > 0 && t.steps[0]!.at <= since) t.steps.shift();
      const target = t.serverX + t.steps.reduce((a, s) => a + s.dx, 0);
      let x = (t.x ?? t.serverX) + step;
      x += (target - x) * (1 - (1 - PULL) ** (dt * 60));
      t.x = Math.abs(target - x) > snapUnits(t.speed) ? target : x;
      xs.set(monarch, t.x);
    }
    if (xs.size === 0) return state;
    return { ...state, players: state.players.map((p) => (xs.has(p.index) ? { ...p, x: xs.get(p.index)! } : p)) };
  }
}
