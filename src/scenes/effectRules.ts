import type { SlotSeat } from '../online/clientProtocol';
import type { GameEvent } from '../model/types';
import { FLASH_MIN_GAP_MS, RUMBLE } from './effects';
import type { Cell } from './layout';

/** Blitz erlaubt? `lastFlashAt` ist die Zeit des letzten ausgeführten Blitzes (ms) oder `null`. */
export function flashAllowed(now: number, lastFlashAt: number | null): boolean {
  return lastFlashAt === null || now - lastFlashAt >= FLASH_MIN_GAP_MS;
}

/** Effekt ausführen? Blitz-Effekte entfallen mit Schalter „aus“ und im Sperrfenster. */
export function runEffect(isFlash: boolean, settings: { flash: boolean }, now: number, lastFlashAt: number | null): boolean {
  return !isFlash || (settings.flash && flashAllowed(now, lastFlashAt));
}

/** Sitz (Platz in der nach Slot sortierten Liste) des getroffenen lokalen Spielers; nur `hit` mit `target: 'player'`. */
export function hurtSeat(e: GameEvent, seats: readonly SlotSeat[]): number | null {
  if (e.type !== 'hit' || e.target !== 'player') return null;
  const i = [...seats].sort((a, b) => a.slot - b.slot).findIndex((s) => s.monarch === e.id);
  return i < 0 ? null : i;
}

/** Zelle (= Kamera-Index) des getroffenen lokalen Spielers; Spieler eines anderen Geräts, Gegner und Gebäude → `null`. */
export function shakeCell(e: GameEvent, seats: readonly SlotSeat[], cells: readonly Cell[]): number | null {
  const seat = hurtSeat(e, seats);
  if (seat === null) return null;
  const i = cells.findIndex((c) => c.kind === 'player' && c.seat === seat);
  return i < 0 ? null : i;
}

type Actuator = { playEffect?: (type: string, params: object) => Promise<unknown> };

/** Kurze Vibration; fehlender `vibrationActuator` oder abgelehntes Promise → nichts, `onFail` höchstens einmal je Aufruf. */
export function rumblePad(pad: unknown, onFail?: () => void): void {
  try {
    const actuator = (pad as { vibrationActuator?: Actuator } | null)?.vibrationActuator;
    if (!actuator?.playEffect) return onFail?.();
    void Promise.resolve(actuator.playEffect('dual-rumble', RUMBLE)).catch(() => onFail?.());
  } catch {
    onFail?.();
  }
}
