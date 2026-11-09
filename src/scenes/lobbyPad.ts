/**
 * Pad-Tasten der Lobby (Spielmenü): A bestätigt, X und Y fragen nach dem Löschen eines Spielstands (oder dem Schließen eines Raums),
 * D-Pad oder linker Stick wählen; B bleibt frei (Edge-Zurück). Rein, ohne Phaser: liest nur Tasten und Stick.
 */

/** Was von einem Pad gelesen wird (Teil von `Phaser.Input.Gamepad.Gamepad`) */
export interface LobbyPad {
  buttons: ReadonlyArray<{ pressed?: boolean } | undefined>;
  leftStick: { x: number; y: number };
}

export type LobbyKey = 'a' | 'x' | 'back' | 'up' | 'down' | 'left' | 'right';

const PAD_VIEW = 8;
/** Menu geht eine Ansicht zurück (B bleibt frei); mit View zusammen ist es die Home-Kombi der Shell und zählt nicht */
const PAD_MENU = 9;
const STICK = 0.5;
/** Tasten je Lobby-Eingabe (standard mapping): A bestätigt, X und Y löschen (RX1: X kam auf der Xbox nicht an; beide sind in der Lobby frei), D-Pad wählt */
const BUTTONS: readonly (readonly [LobbyKey, readonly number[]])[] = [['a', [0]], ['x', [2, 3]], ['up', [12]], ['down', [13]], ['left', [14]], ['right', [15]]];
/** Stick-Richtungen: Achse (0 = x, 1 = y), Vorzeichen */
const STICK_DIRS: readonly (readonly [LobbyKey, 0 | 1, 1 | -1])[] = [['up', 1, -1], ['down', 1, 1], ['left', 0, -1], ['right', 0, 1]];

/** Gerade gehaltene Lobby-Tasten aller Pads: 'a', 'x' (X oder Y), 'back' (Menu), 'up', 'down', 'left', 'right' */
export function heldPadKeys(pads: readonly (LobbyPad | null | undefined)[]): Set<LobbyKey> {
  const held = new Set<LobbyKey>();
  for (const pad of pads) {
    if (!pad) continue;
    const pressed = (i: number): boolean => pad.buttons[i]?.pressed === true;
    for (const [key, indices] of BUTTONS) if (indices.some(pressed)) held.add(key);
    if (pressed(PAD_MENU) && !pressed(PAD_VIEW)) held.add('back');
    const axes = [pad.leftStick.x, pad.leftStick.y];
    for (const [key, axis, sign] of STICK_DIRS) if (axes[axis]! * sign > STICK) held.add(key);
  }
  return held;
}
