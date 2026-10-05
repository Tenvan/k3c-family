/**
 * Skill-Menü je lokalem Spieler (S3.2, B-124/AC-02), ohne Phaser, damit es getestet werden kann.
 * Der Client entscheidet nichts: „lernbar“ und „Respec“ kommen aus `points` und `actions` des Servers;
 * eine Ablehnung (Tier-Gating) meldet der Server als `bad_request`.
 */
import { nameOf } from '../core/texts';
import { MONARCH } from '../model/data';
import type { Player } from '../model/types';
import { SKILL_ACTIONS, type SlotAction } from '../input/slotBindings';
import type { SlotCommand } from './localSlots';

type SkillData = { line: string; tier: number; kind: string };
const SKILLS = MONARCH.skills as Record<string, SkillData>;

export interface MenuEntry {
  /** Skill-ID oder `respec` (letzter Eintrag, nur wenn der Server Respec anbietet) */
  id: string;
  line: string;
  tier: number;
  passive: boolean;
  learned: boolean;
  learnable: boolean;
}

export interface SlotView {
  /** Skill-ID, `""` = frei */
  skill: string;
  /** Sekunden, 0 = bereit */
  cooldown: number;
}

/** Name aus den Texten, sonst aus der ID („shieldBash“ → „Shield Bash“) */
export const skillName = (id: string): string => nameOf('skill', id, id.replace(/([A-Z])/g, ' $1').replace(/^./, (c) => c.toUpperCase()));

const has = (p: Player, action: string) => p.actions.some((a) => a.action === action);

/** Menüeinträge: alle Skills nach Linie und Tier, gelernte markiert, lernbar nur mit `learn` in `actions`; dazu Respec */
export function menuEntries(p: Player): MenuEntry[] {
  const learned = new Set(p.skills ?? []);
  const canLearn = has(p, 'learn') && p.points > 0;
  const lines = Object.keys(MONARCH.lines);
  const entries = Object.entries(SKILLS)
    .map(([id, s]) => ({ id, line: s.line, tier: s.tier, passive: s.kind === 'passive', learned: learned.has(id), learnable: canLearn && !learned.has(id) }))
    .sort((a, b) => lines.indexOf(a.line) - lines.indexOf(b.line) || a.tier - b.tier);
  if (has(p, 'respec')) entries.push({ id: 'respec', line: '', tier: 0, passive: false, learned: false, learnable: true });
  return entries;
}

/** Skill-Slots 1–4 mit Abklingzeit (fehlende Felder = frei bzw. bereit) */
export function slotViews(p: Player): SlotView[] {
  return SKILL_ACTIONS.map((_, i) => ({ skill: p.slots?.[i] ?? '', cooldown: Math.max(0, p.cooldowns?.[i] ?? 0) }));
}

/** Was das Menü eines Spielers in diesem Bild auslöst */
export type MenuCommand = { t: 'learn'; skill: string } | { t: 'respec' } | null;

export interface MenuInput {
  justPressed(action: 'confirm' | SlotAction): boolean;
  moveX(): number;
}

export interface MenuClient {
  learn(slot: number, skill: string): void;
  respec(slot: number): void;
}

interface MenuState {
  cursor: number;
  /** Stick/D-Pad war im letzten Bild ausgelenkt: ein Schritt je Auslenkung */
  lastDir: number;
}

/** Offene Menüs je lokalem Slot; ein offenes Menü nimmt die Eingabe seines Spielers, die anderen spielen weiter. */
export class SkillMenus {
  private readonly open = new Map<number, MenuState>();

  isOpen(slot: number): boolean {
    return this.open.has(slot);
  }

  cursor(slot: number): number {
    return this.open.get(slot)?.cursor ?? 0;
  }

  /** Skill-Menü-Taste schaltet um; offen: links/rechts wählt, A/Leertaste/Münz-Taste bestätigt. */
  step(slot: number, input: MenuInput, player: Player | undefined): MenuCommand {
    if (input.justPressed('skillMenu')) {
      if (this.open.has(slot)) this.open.delete(slot);
      else this.open.set(slot, { cursor: 0, lastDir: Math.sign(Math.round(input.moveX())) });
      return null;
    }
    const state = this.open.get(slot);
    if (!state || !player) return null;
    const entries = menuEntries(player);
    const dir = Math.sign(Math.round(input.moveX()));
    if (dir !== 0 && dir !== state.lastDir) state.cursor = (state.cursor + dir + entries.length) % entries.length;
    state.lastDir = dir;
    state.cursor = Math.min(state.cursor, entries.length - 1);
    const entry = entries[state.cursor];
    if (!entry || !entry.learnable || !input.justPressed('confirm')) return null;
    return entry.id === 'respec' ? { t: 'respec' } : { t: 'learn', skill: entry.id };
  }

  /**
   * Je bedientem Slot das Menü takten und `learn`/`respec` senden; Slots ohne Eingabe schließen ihr Menü.
   * Rückgabe: die Eingaben, bei offenem Menü steht der Monarch (kein Laufen, Zahlen, Schlag, Skill).
   */
  route(commands: SlotCommand[], bound: readonly (MenuInput | null)[], player: (slot: number) => Player | undefined, client: MenuClient): SlotCommand[] {
    for (const slot of this.open.keys()) if (!bound[slot]) this.open.delete(slot);
    for (const c of commands) {
      const input = bound[c.slot];
      const cmd = input ? this.step(c.slot, input, player(c.slot)) : null;
      if (cmd?.t === 'learn') client.learn(c.slot, cmd.skill);
      else if (cmd?.t === 'respec') client.respec(c.slot);
    }
    return commands.map((c) => (this.open.has(c.slot) ? { slot: c.slot, moveX: 0, sprint: false, pay: false } : c));
  }
}
