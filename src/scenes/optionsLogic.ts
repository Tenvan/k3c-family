/**
 * Logik der Optionen-Szene ohne Phaser (S5.2, B-146, B-135): Eintragsliste, Auswahl, Werte, „Menu kurz“.
 * `OptionsScene` zeichnet nur und gibt Eingaben weiter. Texte stehen vorerst hier, S5.3 zieht sie in die Textdateien um.
 */
import { clampVolume, type Settings } from '../core/settings';
import type { SlotCommand } from './localSlots';

export type OptionId = 'music' | 'sfx' | 'screenshake' | 'flash' | 'colorblind' | 'resume';

/** Feste Reihenfolge der Einträge */
export const OPTION_IDS: readonly OptionId[] = ['music', 'sfx', 'screenshake', 'flash', 'colorblind', 'resume'];

export const OPTIONS_TEXT = {
  title: 'Optionen',
  music: 'Musik',
  sfx: 'Effekte',
  screenshake: 'Wackeln',
  flash: 'Blitze',
  colorblind: 'Farbschwäche-Symbole',
  resume: 'Weiter',
  on: 'an',
  off: 'aus',
  hint: 'Hoch/Runter wählen · Links/Rechts ändern · A / Enter umlegen · Menu / Esc schließen',
  openButton: 'Optionen',
} as const;

/** Pad-Tasten (standard mapping), die die Szene liest. B (Index 1) bleibt frei: Edge-Zurück auf der Xbox. */
export const OPTION_PAD_KEYS = { A: 0, VIEW: 8, MENU: 9, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 } as const;

/** Menu kürzer als das gilt als „kurz“; länger bleibt der Kombi View + Menu (Shell) vorbehalten */
export const MENU_SHORT_MS = 600;
export const VOLUME_STEP = 10;

export type OptionDir = 'left' | 'right' | 'confirm';

/** Auswahl bewegt sich um `dir` (-1/0/1) und springt nicht über die Enden. */
export function moveOption(selected: number, dir: number, count = OPTION_IDS.length): number {
  return Math.max(0, Math.min(count - 1, selected + dir));
}

export interface OptionResult {
  settings: Settings;
  /** „Weiter“ bestätigt: Szene schließen */
  close: boolean;
}

/** Links/Rechts ändert Regler um 10 % (0–100) und kippt Schalter; Bestätigen kippt Schalter oder wählt „Weiter“. */
export function applyOption(settings: Settings, id: OptionId, dir: OptionDir): OptionResult {
  const unchanged = { settings, close: false };
  switch (id) {
    case 'music':
    case 'sfx': {
      if (dir === 'confirm') return unchanged;
      const key = id === 'music' ? 'musicVolume' : 'sfxVolume';
      const value = clampVolume(settings[key] + (dir === 'left' ? -VOLUME_STEP : VOLUME_STEP), settings[key]);
      return { settings: { ...settings, [key]: value }, close: false };
    }
    case 'screenshake':
    case 'flash':
    case 'colorblind': {
      const key = id === 'colorblind' ? 'colorblindSymbols' : id;
      return { settings: { ...settings, [key]: !settings[key] }, close: false };
    }
    case 'resume':
      return { settings, close: dir === 'confirm' };
  }
}

/** Zeilentext, z. B. „Musik  ◀ 70 % ▶“ oder „Blitze  an“. */
export function optionLabel(settings: Settings, id: OptionId): string {
  const t = OPTIONS_TEXT;
  const onOff = (v: boolean) => (v ? t.on : t.off);
  switch (id) {
    case 'music':
      return `${t.music}  ◀ ${settings.musicVolume} % ▶`;
    case 'sfx':
      return `${t.sfx}  ◀ ${settings.sfxVolume} % ▶`;
    case 'screenshake':
      return `${t.screenshake}  ${onOff(settings.screenshake)}`;
    case 'flash':
      return `${t.flash}  ${onOff(settings.flash)}`;
    case 'colorblind':
      return `${t.colorblind}  ${onOff(settings.colorblindSymbols)}`;
    case 'resume':
      return t.resume;
  }
}

/** Tippen: linkes Drittel der Zeile = links, rechtes Drittel = rechts, Mitte = bestätigen. */
export function tapDir(x: number, width: number): OptionDir {
  if (x < width / 3) return 'left';
  return x > (width * 2) / 3 ? 'right' : 'confirm';
}

/**
 * „Menu kurz“: wertet die Taste erst beim Loslassen aus. Öffnet nur, wenn die Haltedauer unter `MENU_SHORT_MS` lag
 * und View in der Zeit nie gehalten wurde (sonst ist es die Kombi „zurück zur Landingpage“).
 */
export class MenuPress {
  private since: number | null = null;
  private viewSeen = false;

  /** Einmal pro Frame; `true` genau im Frame des Loslassens eines kurzen Menu-Drucks. */
  update(menuHeld: boolean, viewHeld: boolean, nowMs: number): boolean {
    if (menuHeld) {
      this.since ??= nowMs;
      this.viewSeen ||= viewHeld;
      return false;
    }
    if (this.since === null) return false;
    const short = nowMs - this.since < MENU_SHORT_MS && !this.viewSeen;
    this.since = null;
    this.viewSeen = false;
    return short;
  }
}

/** Solange die Szene offen ist, stehen alle Monarchen des Geräts: keine Bewegung, kein Sprint, keine Münze. */
export function idleCommands(seated: readonly number[]): SlotCommand[] {
  return seated.map((slot) => ({ slot, moveX: 0, sprint: false, pay: false }));
}
