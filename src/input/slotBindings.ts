/**
 * Tastenbelegung je Gerät (Q06, `docs/rules/monarch.md` § 4), ohne Phaser, damit sie getestet werden kann.
 * Quelle für `PlayerInput`, das Aktionen-Overlay (S3.3) und die Glyphen (S6).
 * B bleibt unbelegt (Edge-Zurück), View und Menu sind nie allein eine Aktion (View + Menu = zurück zur Landingpage),
 * Menu kurz öffnet die Optionen (`pause`), Y ist frei (kein Bau-Menü, Q34).
 */
export type SlotAction = 'attack' | 'skill1' | 'skill2' | 'skill3' | 'skill4' | 'skillMenu';
export type Action = 'confirm' | 'pause' | 'fullscreen' | SlotAction;
export type Device = 'pad' | 'keyboard' | 'touch';

export const SLOT_ACTIONS: readonly SlotAction[] = ['attack', 'skill1', 'skill2', 'skill3', 'skill4', 'skillMenu'];
export const SKILL_ACTIONS = ['skill1', 'skill2', 'skill3', 'skill4'] as const;

// Indizes laut W3C Gamepad "standard mapping"
export const PAD = { A: 0, B: 1, X: 2, Y: 3, LB: 4, RB: 5, LT: 6, RT: 7, VIEW: 8, MENU: 9, LS: 10, RS: 11, UP: 12, DOWN: 13, LEFT: 14, RIGHT: 15 } as const;
type PadButton = keyof typeof PAD;

/** Taste je Aktion: Pad = Name aus `PAD`, Tastatur = Name aus Phasers `KeyCodes`, Touch = Schlüssel der Bildschirmtaste */
const SLOT_KEYS: Record<Device, Record<SlotAction, string>> = {
  pad: { attack: 'X', skill1: 'LB', skill2: 'RB', skill3: 'LT', skill4: 'UP', skillMenu: 'DOWN' },
  keyboard: { attack: 'E', skill1: 'Q', skill2: 'R', skill3: 'T', skill4: 'Z', skillMenu: 'K' },
  touch: { attack: 'attack', skill1: 'skill1', skill2: 'skill2', skill3: 'skill3', skill4: 'skill4', skillMenu: 'skillMenu' },
};

const PAD_LABELS: Partial<Record<PadButton, string>> = { UP: 'D-Pad ↑', DOWN: 'D-Pad ↓' };
const TOUCH_LABELS: Record<SlotAction, string> = { attack: '⚔', skill1: '1', skill2: '2', skill3: '3', skill4: '4', skillMenu: '★' };

export interface Binding {
  action: SlotAction;
  key: string;
  /** Anzeigetext der Taste (bis S6 Glyphen liefert) */
  label: string;
}

function label(device: Device, action: SlotAction, key: string): string {
  if (device === 'touch') return TOUCH_LABELS[action];
  return device === 'pad' ? (PAD_LABELS[key as PadButton] ?? key) : key;
}

/** Belegung von Schlag, Skill-Slot 1–4 und Skill-Menü auf einem Gerät, in der Reihenfolge von `SLOT_ACTIONS` */
export function slotBindings(device: Device): Binding[] {
  return SLOT_ACTIONS.map((action) => {
    const key = SLOT_KEYS[device][action];
    return { action, key, label: label(device, action, key) };
  });
}

/** Alle Controller-Aktionen mit Knopf-Index */
export const PAD_ACTIONS: Record<Action, number> = {
  confirm: PAD.A,
  pause: PAD.MENU,
  fullscreen: PAD.RS,
  ...(Object.fromEntries(slotBindings('pad').map((b) => [b.action, PAD[b.key as PadButton]])) as Record<SlotAction, number>),
};

/** Alle Tastatur-Aktionen mit Namen aus Phasers `KeyCodes` */
export const KEY_ACTIONS: Record<Action, string> = {
  confirm: 'SPACE',
  pause: 'ESC',
  fullscreen: 'F',
  ...(Object.fromEntries(slotBindings('keyboard').map((b) => [b.action, b.key])) as Record<SlotAction, string>),
};

/** Gehaltener Skill-Slot 1–4 fürs Protokoll (`input.p[].skill`), der niedrigste gewinnt; 0 = keiner */
export function heldSkill(held: (action: SlotAction) => boolean): number {
  return SKILL_ACTIONS.findIndex((a) => held(a)) + 1;
}
