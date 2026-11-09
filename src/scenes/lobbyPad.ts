/**
 * Pad-Tasten der Lobby (Spielmenü): A bestätigt, X und Y fragen nach dem Löschen eines Spielstands (oder dem Schließen eines Raums),
 * D-Pad oder linker Stick wählen; B bleibt frei (Edge-Zurück). Rein, ohne Phaser: liest nur Tasten und Stick.
 */

/** Was von einem Pad gelesen wird (Teil von `Phaser.Input.Gamepad.Gamepad`) */
export interface LobbyPad {
  buttons: ReadonlyArray<{ pressed?: boolean } | undefined>;
  leftStick: { x: number; y: number };
}

export type LobbyKey = 'a' | 'x' | 'up' | 'down' | 'left' | 'right';

const PAD_A = 0;
const PAD_X = 2;
/** Y löscht ebenso (RX1: X kam auf der Xbox nicht an; beide Tasten sind in der Lobby frei) */
const PAD_Y = 3;
const PAD_UP = 12;
const PAD_DOWN = 13;
const PAD_LEFT = 14;
const PAD_RIGHT = 15;
const STICK = 0.5;

/** Gerade gehaltene Lobby-Tasten aller Pads: 'a', 'x' (X oder Y), 'up', 'down', 'left', 'right' */
export function heldPadKeys(pads: readonly (LobbyPad | null | undefined)[]): Set<LobbyKey> {
  const held = new Set<LobbyKey>();
  for (const pad of pads) {
    if (!pad) continue;
    const pressed = (i: number): boolean => pad.buttons[i]?.pressed === true;
    if (pressed(PAD_A)) held.add('a');
    if (pressed(PAD_X) || pressed(PAD_Y)) held.add('x');
    if (pressed(PAD_UP) || pad.leftStick.y < -STICK) held.add('up');
    if (pressed(PAD_DOWN) || pad.leftStick.y > STICK) held.add('down');
    if (pressed(PAD_LEFT) || pad.leftStick.x < -STICK) held.add('left');
    if (pressed(PAD_RIGHT) || pad.leftStick.x > STICK) held.add('right');
  }
  return held;
}
