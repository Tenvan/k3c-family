import Phaser from 'phaser';

import { KEY_ACTIONS, PAD, PAD_ACTIONS, type Action } from './slotBindings';

export type { Action } from './slotBindings';

/**
 * Einheitliche Eingabe pro Spieler, egal ob Tastatur oder Gamepad.
 * Spiel-Logik fragt nur Aktionen ab, nie konkrete Tasten. Belegung: `slotBindings.ts` (Q06).
 *   Linker Stick / D-Pad links-rechts: laufen   RT: sprinten   A: beitreten, halten = Münzen geben
 *   X: Schlag   LB/RB/LT/D-Pad hoch: Skill 1–4   D-Pad runter: Skill-Menü   Menu kurz: Optionen
 *   B wird bewusst NICHT belegt: Edge auf der Xbox nutzt B als "Zurück". View allein ist frei (View + Menu = Landingpage).
 */
export interface PlayerInput {
  readonly label: string;
  /** -1 (links) bis 1 (rechts) */
  moveX(): number;
  sprint(): boolean;
  /** true nur im Frame, in dem die Aktion ausgelöst wurde */
  justPressed(action: Action): boolean;
  /** true, solange die Taste gehalten wird */
  held(action: Action): boolean;
  /** Einmal pro Frame aufrufen, VOR dem Abfragen */
  update(): void;
}

const STICK_DEADZONE = 0.25;

export class GamepadInput implements PlayerInput {
  readonly label: string;
  private previous = new Set<number>();
  private current = new Set<number>();

  constructor(readonly pad: Phaser.Input.Gamepad.Gamepad) {
    this.label = `Controller ${pad.index + 1}`;
  }

  update(): void {
    this.previous = this.current;
    this.current = new Set(this.pad.buttons.flatMap((b, i) => (b.pressed ? [i] : [])));
  }

  moveX(): number {
    if (this.current.has(PAD.LEFT)) return -1;
    if (this.current.has(PAD.RIGHT)) return 1;
    const x = this.pad.leftStick.x;
    return Math.abs(x) < STICK_DEADZONE ? 0 : x;
  }

  sprint(): boolean {
    return this.current.has(PAD.RT);
  }

  justPressed(action: Action): boolean {
    const button = PAD_ACTIONS[action];
    return this.current.has(button) && !this.previous.has(button);
  }

  held(action: Action): boolean {
    return this.current.has(PAD_ACTIONS[action]);
  }

  /** View gehalten: nur für „Menu kurz“ neben View + Menu (`MenuPress`), keine eigene Aktion */
  viewHeld(): boolean {
    return this.current.has(PAD.VIEW);
  }
}

export class KeyboardInput implements PlayerInput {
  readonly label = 'Tastatur';
  private keys: Record<'left' | 'right' | 'altLeft' | 'altRight' | 'sprint' | Action, Phaser.Input.Keyboard.Key>;
  private pressed = new Set<Action>();

  constructor(keyboard: Phaser.Input.Keyboard.KeyboardPlugin) {
    const K = Phaser.Input.Keyboard.KeyCodes;
    const actions = Object.fromEntries(Object.entries(KEY_ACTIONS).map(([action, key]) => [action, K[key as keyof typeof K]]));
    this.keys = keyboard.addKeys({ left: K.A, right: K.D, altLeft: K.LEFT, altRight: K.RIGHT, sprint: K.SHIFT, ...actions }) as KeyboardInput['keys'];
  }

  update(): void {
    this.pressed.clear();
    for (const action of Object.keys(KEY_ACTIONS) as Action[]) {
      if (Phaser.Input.Keyboard.JustDown(this.keys[action])) this.pressed.add(action);
    }
  }

  moveX(): number {
    const left = this.keys.left.isDown || this.keys.altLeft.isDown;
    const right = this.keys.right.isDown || this.keys.altRight.isDown;
    return (right ? 1 : 0) - (left ? 1 : 0);
  }

  sprint(): boolean {
    return this.keys.sprint.isDown;
  }

  justPressed(action: Action): boolean {
    return this.pressed.has(action);
  }

  held(action: Action): boolean {
    return this.keys[action].isDown;
  }
}
