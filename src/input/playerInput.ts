import Phaser from 'phaser';

/**
 * Einheitliche Eingabe pro Spieler, egal ob Tastatur oder Gamepad.
 * Spiel-Logik fragt nur Aktionen ab, nie konkrete Tasten.
 *
 * Gamepad-Belegung (Xbox, Browser "standard mapping"), angelehnt an das alte GDD:
 *   Linker Stick / D-Pad links-rechts: laufen   RT: sprinten
 *   A: beitreten, halten = Münzen geben (K2C: eine Taste für alles)   X: interagieren (noch frei)
 *   Y: Bau-Menü   View: Skill-Menü   Menu: Pause   LB/RB/LT + D-Pad hoch: Skills (später)
 *   B wird bewusst NICHT belegt: Edge auf der Xbox nutzt B als "Zurück" (im Gamepad-Test prüfen!).
 */
export type Action = 'confirm' | 'interact' | 'build' | 'skillMenu' | 'pause' | 'fullscreen';

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

// Indizes laut W3C Gamepad "standard mapping"
const PAD = { A: 0, B: 1, X: 2, Y: 3, LB: 4, RB: 5, LT: 6, RT: 7, VIEW: 8, MENU: 9, LS: 10, RS: 11, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 } as const;

const PAD_ACTIONS: Record<Action, number> = {
  confirm: PAD.A,
  interact: PAD.X,
  build: PAD.Y,
  skillMenu: PAD.VIEW,
  pause: PAD.MENU,
  fullscreen: PAD.RS,
};

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
}

export class KeyboardInput implements PlayerInput {
  readonly label = 'Tastatur';
  private keys: Record<'left' | 'right' | 'altLeft' | 'altRight' | 'sprint' | Action, Phaser.Input.Keyboard.Key>;
  private pressed = new Set<Action>();

  constructor(keyboard: Phaser.Input.Keyboard.KeyboardPlugin) {
    const K = Phaser.Input.Keyboard.KeyCodes;
    this.keys = keyboard.addKeys({
      left: K.A,
      right: K.D,
      altLeft: K.LEFT,
      altRight: K.RIGHT,
      sprint: K.SHIFT,
      confirm: K.SPACE,
      interact: K.E,
      build: K.B,
      skillMenu: K.K,
      pause: K.ESC,
      fullscreen: K.F,
    }) as KeyboardInput['keys'];
  }

  update(): void {
    this.pressed.clear();
    for (const action of Object.keys(PAD_ACTIONS) as Action[]) {
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
