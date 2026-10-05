/**
 * Glyphen (S6.2, B-149): Aktion und Gerät → Glyph-Schlüssel und Beschriftung, ohne Phaser, damit es getestet werden kann.
 * Die Tasten kommen aus der Belegung (`src/input/slotBindings.ts`), hier steht nur, welche Taste ein Bild bekommt.
 * Schema: `pad:<Name aus PAD>`, `key:<Name aus KeyCodes>`, `touch:<Bildschirmtaste>`; `null` = Text-Rückfall.
 * B und View bekommen nie ein Bild (Edge-Zurück bzw. reserviert für View + Menu).
 */
import { t } from '../core/texts';
import { KEY_ACTIONS, PAD, PAD_ACTIONS, slotBindings, type Action, type Device, type SlotAction } from '../input/slotBindings';

export interface Glyph {
  /** Bild-Schlüssel, `null` = nur Text zeigen */
  key: string | null;
  /** Beschriftung im Bild bzw. Text-Rückfall */
  label: string;
}

/** Füllung und Schrift je Glyph-Art (Xbox-Farben für A/X/Y), Kontrast ≥ 4,5:1 (Test) */
export const GLYPH_COLORS = {
  A: { fill: 0x107c10, ink: 0xffffff },
  X: { fill: 0x0e6ac7, ink: 0xffffff },
  Y: { fill: 0xffb900, ink: 0x000000 },
  pad: { fill: 0x2b2b2b, ink: 0xffffff },
  key: { fill: 0xf2f2f2, ink: 0x111111 },
  touch: { fill: 0x1e1e1e, ink: 0xffffff },
} as const;

/** Glyph-Höhe zur Schriftgröße der Zeile: 28 px → 36 px, 24 px → 31 px (Mindestgröße Q03: 28 bzw. 24) */
export const glyphPx = (fontPx: number): number => Math.ceil(fontPx * 1.3);

/** Controller-Tasten mit Bild (B-149 › Anforderungen) */
export const PAD_GLYPHS = ['A', 'X', 'Y', 'LB', 'RB', 'LT', 'RT', 'UP', 'DOWN', 'MENU'] as const;
const PAD_NAME = Object.fromEntries(Object.entries(PAD).map(([name, i]) => [i, name])) as Record<number, string>;
/** Touch: Bildschirmtasten außerhalb der Slot-Leiste (`touchInput.ts`); Pausieren hat keine */
const TOUCH_KEYS: Partial<Record<Action, { key: string; label: string }>> = { confirm: { key: 'confirm', label: '🪙' }, fullscreen: { key: 'fullscreen', label: '⛶' } };

const isSlot = (action: string): action is SlotAction => slotBindings('pad').some((b) => b.action === action);
const slotLabel = (device: Device, action: SlotAction): string => slotBindings(device).find((b) => b.action === action)!.label;
const fallback = (label: string): Glyph => ({ key: null, label });

function padGlyph(action: Action): Glyph {
  const name = PAD_NAME[PAD_ACTIONS[action]] ?? action;
  const label = isSlot(action) ? slotLabel('pad', action) : name;
  return (PAD_GLYPHS as readonly string[]).includes(name) ? { key: `pad:${name}`, label } : fallback(label);
}

function keyGlyph(action: Action): Glyph {
  const name = KEY_ACTIONS[action];
  return { key: `key:${name}`, label: name === 'SPACE' ? t('hint.space') : name === 'ESC' ? 'Esc' : name };
}

function touchGlyph(action: Action): Glyph {
  const k = isSlot(action) ? { key: action, label: slotLabel('touch', action) } : TOUCH_KEYS[action];
  return k ? { key: `touch:${k.key}`, label: k.label } : fallback(action);
}

/** Aktion + Gerät → Glyph; unbekannte Aktion oder unbekanntes Gerät → Text-Rückfall mit dem Aktionsnamen */
export function glyphOf(action: string, device: string): Glyph {
  if (!Object.hasOwn(PAD_ACTIONS, action)) return fallback(action);
  const a = action as Action;
  if (device === 'pad') return padGlyph(a);
  if (device === 'keyboard') return keyGlyph(a);
  return device === 'touch' ? touchGlyph(a) : fallback(action);
}
