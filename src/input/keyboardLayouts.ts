import { KEY_ACTIONS, type Action } from './slotBindings';

/**
 * Zwei Spieler an einer Tastatur (B-316, Beschluss 🧑 2026-10-06), ohne Phaser, damit es getestet werden kann.
 * Eine Taste ist ein `keyCode` (Buchstaben und Sondertasten wie bisher: auf QWERTZ trifft „Z“ die beschriftete Taste)
 * oder ein `KeyboardEvent.code`, wo der Ort zählt (Strg rechts, Ziffernblock; der Ziffernblock wirkt auch ohne NumLock).
 */
export type KeySpec = { keyCode: number } | { code: string };

export interface KeyboardLayout {
  left: readonly KeySpec[];
  right: readonly KeySpec[];
  sprint: readonly KeySpec[];
  actions: Partial<Record<Action, readonly KeySpec[]>>;
}

/** Namen aus `KEY_ACTIONS` (Phasers `KeyCodes`) → keyCode; Buchstaben = Großbuchstabe */
const NAMED: Record<string, number> = { SPACE: 32, ESC: 27, SHIFT: 16 };
const keyCodeOf = (name: string): number => NAMED[name] ?? name.charCodeAt(0);
const key = (keyCode: number): KeySpec => ({ keyCode });
const code = (c: string): KeySpec => ({ code: c });

/** Spieler 1 links: A/D, Shift, Leertaste, E, Q, R, T, Z, K, dazu Esc (Pause) und F (Vollbild) */
export const KEYBOARD_1: KeyboardLayout = {
  left: [key(65)],
  right: [key(68)],
  sprint: [key(16)],
  actions: Object.fromEntries(Object.entries(KEY_ACTIONS).map(([a, name]) => [a, [key(keyCodeOf(name))]])),
};

/**
 * Spieler 2 rechts: Pfeile, Strg rechts, Enter, Ziffernblock 0 = Schlag, 1–4 = Skills, 5 = Skill-Menü.
 * Pfeile nach `code`: ohne NumLock meldet Ziffernblock 4 den keyCode des Pfeils links.
 */
export const KEYBOARD_2: KeyboardLayout = {
  left: [code('ArrowLeft')],
  right: [code('ArrowRight')],
  sprint: [code('ControlRight')],
  actions: {
    confirm: [key(13)],
    attack: [code('Numpad0')],
    skill1: [code('Numpad1')],
    skill2: [code('Numpad2')],
    skill3: [code('Numpad3')],
    skill4: [code('Numpad4')],
    skillMenu: [code('Numpad5')],
  },
};

/** Alle Tasten eines Layouts */
export function layoutKeys(l: KeyboardLayout): KeySpec[] {
  return [...l.left, ...l.right, ...l.sprint, ...Object.values(l.actions).flatMap((k) => k ?? [])];
}

/** Trifft das Tastenereignis die Taste? */
export function keyMatches(spec: KeySpec, e: { keyCode: number; code: string }): boolean {
  return 'code' in spec ? e.code === spec.code : e.keyCode === spec.keyCode;
}
