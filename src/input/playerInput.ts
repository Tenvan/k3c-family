import Phaser from 'phaser';

import { KEYBOARD_1, KEYBOARD_2, keyMatches, layoutKeys, type KeyboardLayout, type KeySpec } from './keyboardLayouts';
import { PAD, PAD_ACTIONS, type Action } from './slotBindings';

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

type KeyHit = { keyCode: number; code: string };

/**
 * Ein Spieler an der Tastatur mit seinem Layout (`keyboardLayouts.ts`, B-316). Liest die Tastenereignisse der Szene mit
 * `code`, damit Strg rechts, Enter und Ziffernblock dem richtigen Spieler gehören; Autorepeat löst nichts neu aus.
 */
export class KeyboardInput implements PlayerInput {
  /** gehaltene Tasten: `code` → keyCode */
  private down = new Map<string, number>();
  /** seit dem letzten `update` neu gedrückt (auch kurz getippt und schon wieder losgelassen) */
  private fresh: KeyHit[] = [];
  private pressed = new Set<Action>();

  constructor(
    keyboard: Phaser.Input.Keyboard.KeyboardPlugin,
    private readonly layout: KeyboardLayout = KEYBOARD_1,
    readonly label = 'Tastatur',
  ) {
    keyboard.addCapture(layoutKeys(layout).flatMap((k) => ('keyCode' in k ? [k.keyCode] : []))); // kein Scrollen der Seite
    const id = (e: KeyboardEvent): string => e.code || e.key;
    keyboard.on('keydown', (e: KeyboardEvent) => {
      if (!e.repeat) this.fresh.push({ keyCode: e.keyCode, code: id(e) });
      this.down.set(id(e), e.keyCode);
    });
    keyboard.on('keyup', (e: KeyboardEvent) => this.down.delete(id(e)));
    const clear = (): void => this.down.clear(); // Fenster verliert den Fokus: kein Hängenbleiben
    keyboard.scene.game.events.on(Phaser.Core.Events.BLUR, clear);
    keyboard.scene.events.once(Phaser.Scenes.Events.SHUTDOWN, () => keyboard.scene.game.events.off(Phaser.Core.Events.BLUR, clear));
  }

  update(): void {
    this.pressed.clear();
    for (const [action, specs] of Object.entries(this.layout.actions) as [Action, readonly KeySpec[]][]) {
      if (this.fresh.some((hit) => specs.some((s) => keyMatches(s, hit)))) this.pressed.add(action);
    }
    this.fresh = [];
  }

  private isDown(specs: readonly KeySpec[]): boolean {
    for (const [code, keyCode] of this.down) if (specs.some((s) => keyMatches(s, { keyCode, code }))) return true;
    return false;
  }

  moveX(): number {
    return (this.isDown(this.layout.right) ? 1 : 0) - (this.isDown(this.layout.left) ? 1 : 0);
  }

  sprint(): boolean {
    return this.isDown(this.layout.sprint);
  }

  justPressed(action: Action): boolean {
    return this.pressed.has(action);
  }

  held(action: Action): boolean {
    return this.isDown(this.layout.actions[action] ?? []);
  }
}

/** Zwei Spieler an einer Tastatur (B-316): Spieler 1 links, Spieler 2 rechts (`keyboardLayouts.ts`) */
export function keyboardPlayers(keyboard: Phaser.Input.Keyboard.KeyboardPlugin): [KeyboardInput, KeyboardInput] {
  return [new KeyboardInput(keyboard), new KeyboardInput(keyboard, KEYBOARD_2, 'Tastatur 2')];
}
