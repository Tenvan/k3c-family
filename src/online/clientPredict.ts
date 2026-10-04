import { MONARCH } from '../model/data';
import type { SlotInput, SlotSeat, WorldState } from './clientProtocol';

/**
 * Anzeige-Vorhersage der lokalen Monarchen (B-039, B-277): nur die x-Position. Je Frame läuft der Monarch mit der
 * lokalen Laufrichtung × beobachteter Geschwindigkeit und wird um `PULL` zum neuesten Server-x gezogen; großer Abstand
 * übernimmt den Server-Wert. Keine Spiel-Logik: Rand, Tod, Bauen usw. korrigiert der Server.
 */

/** Anteil je 60-FPS-Frame, um den die Vorhersage zum neuesten Server-x gezogen wird (angenommen; bei anderer Bildrate gleich schnell je Sekunde) */
export const PULL = 0.12;
/** Ab diesem Abstand zum Server-x wird sofort übernommen (Teleport, Respawn; angenommen) */
export const SNAP_UNITS = 3;
/** Geschwindigkeit, bis eine Bewegung beobachtet ist: `base.speed` aus `data/monarch.json` (Units/s) */
export const FALLBACK_SPEED = MONARCH.base.speed;
/** Glättung der beobachteten Geschwindigkeit je Zustand (angenommen) */
const SPEED_SMOOTHING = 0.3;
/** Langsamer zählt als Stehen und ändert die Geschwindigkeit nicht (Units/s, angenommen) */
const MIN_SPEED = 0.5;

interface Track {
  serverX: number;
  tick: number;
  speed: number;
  x: number | null;
}

export class Predictor {
  private tracks = new Map<number, Track>();
  private lastNow: number | null = null;

  constructor(private readonly tickMs: number) {}

  /** Je empfangenem Frame: neuestes Server-x und beobachtete Geschwindigkeit der lokalen Monarchen. */
  observe(frame: { tick: number; state: WorldState }, you: readonly SlotSeat[]): void {
    for (const { monarch } of you) {
      const p = frame.state.players.find((q) => q.index === monarch);
      if (!p) continue;
      const t = this.tracks.get(monarch);
      if (!t) {
        this.tracks.set(monarch, { serverX: p.x, tick: frame.tick, speed: FALLBACK_SPEED, x: null });
        continue;
      }
      const dx = Math.abs(p.x - t.serverX);
      const v = (dx * 1000) / (Math.max(1, frame.tick - t.tick) * this.tickMs);
      if (v > MIN_SPEED && dx < SNAP_UNITS) t.speed += (v - t.speed) * SPEED_SMOOTHING;
      t.serverX = p.x;
      t.tick = frame.tick;
    }
  }

  /** Gezeichneter Zustand mit vorhergesagtem x der lokalen Monarchen; alles andere unverändert. */
  draw(state: WorldState, now: number, inputs: readonly SlotInput[], you: readonly SlotSeat[]): WorldState {
    const dt = this.lastNow === null ? 0 : (now - this.lastNow) / 1000;
    this.lastNow = now;
    const xs = new Map<number, number>();
    for (const { slot, monarch } of you) {
      const t = this.tracks.get(monarch);
      if (!t) continue;
      const dir = Math.sign(inputs.find((i) => i.slot === slot)?.moveX ?? 0);
      let x = (t.x ?? t.serverX) + dir * t.speed * dt;
      x += (t.serverX - x) * (1 - (1 - PULL) ** (dt * 60));
      t.x = Math.abs(t.serverX - x) > SNAP_UNITS ? t.serverX : x;
      xs.set(monarch, t.x);
    }
    if (xs.size === 0) return state;
    return { ...state, players: state.players.map((p) => (xs.has(p.index) ? { ...p, x: xs.get(p.index)! } : p)) };
  }
}
