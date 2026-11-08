/**
 * Tabelle „Tastenbelegung“ der Optionen (B-741), ohne Phaser: Zeilen = Aktionen, Spalten = Geräte.
 * Alles kommt aus der echten Belegung (`slotBindings.ts`, `keyboardLayouts.ts`); hier steht nur, wie eine Taste heißt.
 */
import { t } from '../core/texts';
import { KEYBOARD_1, type KeyboardLayout, type KeySpec } from '../input/keyboardLayouts';
import { glyphOf, type Glyph } from './hints/glyphs';
import { PAD, PAD_ACTIONS, SLOT_ACTIONS, slotBindings, type Action } from '../input/slotBindings';

export interface KeyTable {
  head: string[];
  rows: string[][];
  /** Controller-Symbol je Zeile (A, RT, X, LB …), `null` = nur Text (Laufen, Vollbild) */
  padGlyphs: (Glyph | null)[];
}

const NONE = '–';
const PAD_NAME = Object.fromEntries(Object.entries(PAD).map(([name, i]) => [i, name])) as Record<number, string>;
const CODE_NAMES: Record<string, () => string> = { ArrowLeft: () => '←', ArrowRight: () => '→', ControlRight: () => t('keys.ctrlRight') };
const KEYCODE_NAMES: Record<number, string> = { 13: 'Enter', 16: 'Shift', 27: 'Esc' };

/** Name einer Tastatur-Taste: `code` (Strg rechts, Ziffernblock) oder `keyCode` */
export function keyLabel(spec: KeySpec): string {
  if ('code' in spec) return CODE_NAMES[spec.code]?.() ?? spec.code.replace(/^Numpad(\d)$/, 'Num $1');
  return spec.keyCode === 32 ? t('hint.space') : (KEYCODE_NAMES[spec.keyCode] ?? String.fromCharCode(spec.keyCode));
}

const keys = (specs: readonly KeySpec[] | undefined): string => (specs?.length ? specs.map(keyLabel).join(' / ') : NONE);
const padName = (action: Action): string => PAD_NAME[PAD_ACTIONS[action]] ?? NONE;

/** Zeilen in dieser Reihenfolge: Münzen/Bestätigen, Sprinten, Laufen, Schlag, Skills 1–4, Skill-Menü, Optionen, Vollbild */
const ACTION_ROWS: readonly (Action | 'sprint' | 'move')[] = ['confirm', 'sprint', 'move', ...SLOT_ACTIONS, 'pause', 'fullscreen'];

type Row = (typeof ACTION_ROWS)[number];
type Device = 'pad' | 'keyboard' | 'touch';

const isSlotRow = (row: Row): boolean => (SLOT_ACTIONS as readonly string[]).includes(row);
const slotLabel = (device: 'pad' | 'keyboard' | 'touch', row: Row): string => slotBindings(device).find((b) => b.action === row)?.label ?? NONE;

/** Tastatur-Spalte aus einem Layout */
function layoutCell(l: KeyboardLayout, row: Row): string {
  if (row === 'sprint') return keys(l.sprint);
  return row === 'move' ? keys([...l.left, ...l.right]) : keys(l.actions[row]);
}

const PAD_FIXED: Partial<Record<Row, () => string>> = { sprint: () => PAD_NAME[PAD.RT]!, move: () => t('keys.padMove') };
const TOUCH_FIXED: Partial<Record<Row, () => string>> = { sprint: () => '»', move: () => t('keys.touchMove'), confirm: () => '🪙', fullscreen: () => '⛶', pause: () => '☰' };

const CELLS: Record<Device, (row: Row) => string> = {
  pad: (row) => PAD_FIXED[row]?.() ?? (isSlotRow(row) ? slotLabel('pad', row) : padName(row as Action)),
  keyboard: (row) => layoutCell(KEYBOARD_1, row),
  touch: (row) => TOUCH_FIXED[row]?.() ?? (isSlotRow(row) ? slotLabel('touch', row) : NONE),
};

/** Controller-Symbol der Zeile; nur Tasten mit Bild (`PAD_GLYPHS`) */
function padGlyphOf(row: Row): Glyph | null {
  const g = row === 'sprint' ? { key: `pad:${PAD_NAME[PAD.RT]}`, label: PAD_NAME[PAD.RT]! } : row === 'move' ? null : glyphOf(row, 'pad');
  return g?.key ? g : null;
}

/** Tabelle für das Menü: Kopf mit den Geräten, je Aktion eine Zeile mit der Taste je Gerät */
export function keyTable(): KeyTable {
  const devices = ['pad', 'keyboard', 'touch'] as const;
  return {
    head: ['', t('keys.pad'), t('keys.keyboard'), t('keys.touch')],
    padGlyphs: ACTION_ROWS.map(padGlyphOf),
    rows: ACTION_ROWS.map((row) => [t(`keys.row.${row}`), ...devices.map((d) => CELLS[d](row))]),
  };
}
