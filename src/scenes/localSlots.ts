/**
 * Lokale Spieler eines Geräts (Slots 0–3, Protokoll v2): welche Eingabe steuert welchen Slot, wann `addSlot`, `removeSlot` und `leave`.
 * Ohne Phaser, damit es getestet werden kann (SP08 AC-02, AC-11).
 */
import { botInputs, type BotInput } from '../input/botInput';
import { heldSkill, type SlotAction } from '../input/slotBindings';
import { MAX_LOCAL_PLAYERS } from './layout';

export interface SlotInputDevice {
  justPressed(action: 'confirm'): boolean;
  held(action: 'confirm' | SlotAction): boolean;
  moveX(): number;
  sprint(): boolean;
}

export interface SlotClient {
  addSlot(slot: number): void;
  removeSlot(slot: number): void;
}

export interface SlotCommand {
  slot: number;
  moveX: number;
  sprint: boolean;
  pay: boolean;
  /** Nur gesetzt, wenn gehalten (Protokoll: fehlt = false bzw. 0) */
  attack?: true;
  skill?: number;
}

/** Ein neuer lokaler Spieler muss binnen dieser Zeit vom Server bestätigt werden, sonst wird sein Gerät wieder frei */
export const ADD_SLOT_TIMEOUT_MS = 2000;

export class LocalSlots<I extends SlotInputDevice> {
  /** Eingabe je Slot (null = noch niemand) */
  readonly bound: (I | null)[] = Array.from({ length: MAX_LOCAL_PLAYERS }, () => null);
  /** Slots, die `addSlot` gesendet haben und auf die Bestätigung warten (Slot → Zeitpunkt) */
  private readonly adding = new Map<number, number>();

  /** `mock`: Slots ohne Eingabe, die nur stehen (Testseite); `fixed`: Eingabe i steuert Slot i ohne Beitritt per Taste (Bots, B-353) */
  constructor(
    private readonly mock: readonly number[] = [],
    fixed: readonly I[] = [],
  ) {
    fixed.forEach((input, slot) => (this.bound[slot] = input));
  }

  /** Platz am Server ohne Spieler, der ihn bedient (ein Mock steuert niemand) */
  waiting(seated: readonly number[]): boolean {
    return seated.some((s) => !this.bound[s] && !this.mock.includes(s));
  }

  /** Ein Gerät, das bestätigt (A, Leertaste, Münz-Taste), übernimmt einen freien Platz oder fügt einen Slot hinzu. */
  join(inputs: readonly I[], seated: readonly number[], client: SlotClient, now: number): void {
    for (const slot of seated) this.adding.delete(slot);
    for (const [slot, since] of this.adding) {
      if (now - since <= ADD_SLOT_TIMEOUT_MS) continue;
      this.bound[slot] = null; // abgelehnt (Raum voll, zu viele Spieler)
      this.adding.delete(slot);
    }
    for (const input of inputs) {
      if (this.bound.includes(input) || !input.justPressed('confirm')) continue;
      const open = seated.find((s) => !this.bound[s] && !this.mock.includes(s));
      if (open !== undefined) {
        this.bound[open] = input;
        continue;
      }
      const free = this.bound.findIndex((b, s) => !b && !seated.includes(s) && !this.adding.has(s));
      if (free < 0) continue;
      this.bound[free] = input;
      this.adding.set(free, now);
      client.addSlot(free);
    }
  }

  /** Gerät weg (Controller getrennt): `removeSlot`; war es der letzte Platz, `leave` (nur gemeldet, der Aufrufer verlässt den Raum) statt `removeSlot`. */
  lose(input: I, seated: readonly number[], client: SlotClient): 'leave' | 'removed' | 'none' {
    const slot = this.bound.indexOf(input);
    if (slot < 0) return 'none';
    this.bound[slot] = null;
    if (seated.every((s) => s === slot)) return 'leave';
    client.removeSlot(slot);
    return 'removed';
  }

  /** Eingaben für alle Plätze des Geräts: bedienter Slot mit seiner Eingabe, Mock mit „keine Bewegung“, unbedienter gar nicht. */
  commands(seated: readonly number[]): SlotCommand[] {
    const result: SlotCommand[] = [];
    for (const slot of seated) {
      const input = this.bound[slot];
      if (input) result.push(command(slot, input));
      else if (this.mock.includes(slot)) result.push({ slot, moveX: 0, sprint: false, pay: false });
    }
    return result;
  }
}

let bots: BotInput[] | undefined;

/** Bot-Eingaben der Seite (`?botfeed=…&players=n`, B-353), einmal je Seite; ohne Parameter keine */
export function pageBots(): BotInput[] {
  return (bots ??= botInputs(window.location.search, window.location.hostname));
}

/** Eingabe eines bedienten Slots; `attack` und `skill` nur, wenn gehalten (Schlag, Skill-Slot 1–4) */
function command(slot: number, input: SlotInputDevice): SlotCommand {
  const c: SlotCommand = { slot, moveX: Math.max(-1, Math.min(1, input.moveX())), sprint: input.sprint(), pay: input.held('confirm') };
  if (input.held('attack')) c.attack = true;
  const skill = heldSkill((a) => input.held(a));
  if (skill > 0) c.skill = skill;
  return c;
}
