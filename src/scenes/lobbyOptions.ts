/**
 * Anlegen-Dialog der Lobby ohne Phaser (K5.2, B-105): Raum-Optionen Grad, Ziel und Niederlage-Modus mit Standards je Grad
 * (`data/difficulty.json › defaultDefeat`, `rules/stufen.md` § 5). Der Server prüft die Werte beim Anlegen.
 */
import difficulty from '../../data/difficulty.json';
import { DEFAULT_WORLD } from './saveMenu';

export const GRADES = ['dev', 'easy', 'normal', 'hard', 'ultra'] as const;
export const GOALS = ['endboss', 'gold', 'days', 'mineAll', 'buildAll'] as const;
export const DEFEATS = ['resources', 'stage', 'lost'] as const;
export type Grade = (typeof GRADES)[number];
export type Goal = (typeof GOALS)[number];
export type Defeat = (typeof DEFEATS)[number];
export type OptionRow = 'grade' | 'goal' | 'defeat' | 'world';

/** Zeilen des Dialogs von oben nach unten */
export const DIALOG_ROWS = ['grade', 'goal', 'defeat', 'world', 'create', 'cancel'] as const;

/** Raum-Optionen für `create` (Feldnamen wie `docs/protocol.md`) */
export interface RoomOptions {
  grade: Grade;
  goal: Goal;
  defeat: Defeat;
}

/**
 * `defeatSet`: der Spieler hat den Niederlage-Modus selbst gewählt, ein Grad-Wechsel ändert ihn dann nicht mehr.
 * `world`: gewählte Welt-ID des neuen Stands (B-566); sie gehört nicht zu `RoomOptions`, `create` trägt sie als eigenes Feld.
 */
export interface OptionsState extends RoomOptions {
  defeatSet: boolean;
  world: string;
}

export interface DialogState {
  options: OptionsState;
  /** Index in `DIALOG_ROWS` */
  row: number;
}

/** Eingabe eines Bilds: `dir` Zeile hoch/runter, `side` Wert links/rechts (je -1/0/1), `confirm` A/Enter/Klick */
export interface DialogInput {
  dir: number;
  side: number;
  confirm: boolean;
}

/** Grad „Dev“ bietet der Dialog nur mit `?dev=1` an; die Sperre bleibt beim Server (`bad_request` ohne Dev-Mode). */
export function devAllowed(search: string): boolean {
  return new URLSearchParams(search).get('dev') === '1';
}

export function defaultDefeat(grade: Grade): Defeat {
  return difficulty.grades[grade].defaultDefeat as Defeat;
}

export function gradeChoices(dev: boolean): readonly Grade[] {
  return dev ? GRADES : GRADES.filter((g) => g !== 'dev');
}

export function initialOptions(world = DEFAULT_WORLD): OptionsState {
  return { grade: 'normal', goal: 'endboss', defeat: defaultDefeat('normal'), defeatSet: false, world };
}

export function initialDialog(world = DEFAULT_WORLD): DialogState {
  return { options: initialOptions(world), row: 0 };
}

const clamp = (i: number, count: number) => Math.max(0, Math.min(count - 1, i));

/** Wahlmöglichkeiten einer Zeile */
export function choices(row: OptionRow, dev: boolean, worlds: readonly string[] = [DEFAULT_WORLD]): readonly string[] {
  if (row === 'grade') return gradeChoices(dev);
  if (row === 'world') return worlds;
  return row === 'goal' ? GOALS : DEFEATS;
}

/** Setzt eine Option auf den Wert mit Index `index` (begrenzt); ein neuer Grad bringt seinen Standard-Modus mit. */
export function setOption(s: OptionsState, row: OptionRow, index: number, dev: boolean, worlds: readonly string[] = [DEFAULT_WORLD]): OptionsState {
  const list = choices(row, dev, worlds);
  const value = list[clamp(index, list.length)];
  if (row === 'grade') {
    const grade = value as Grade;
    return { ...s, grade, defeat: s.defeatSet ? s.defeat : defaultDefeat(grade) };
  }
  if (row === 'world') return { ...s, world: value! };
  return row === 'goal' ? { ...s, goal: value as Goal } : { ...s, defeat: value as Defeat, defeatSet: true };
}

export function createOptions(s: OptionsState): RoomOptions {
  return { grade: s.grade, goal: s.goal, defeat: s.defeat };
}

/** Ein Bild Eingabe: Zeile wählen, Wert ändern (links/rechts begrenzt, A schaltet im Kreis weiter) oder Aktion auslösen. */
export function dialogStep(d: DialogState, input: DialogInput, dev: boolean, worlds: readonly string[] = [DEFAULT_WORLD]): { dialog: DialogState; action: 'create' | 'cancel' | null } {
  const row = clamp(d.row + input.dir, DIALOG_ROWS.length);
  const name = DIALOG_ROWS[row]!;
  if (name === 'create' || name === 'cancel') return { dialog: { ...d, row }, action: input.confirm ? name : null };
  if (!input.side && !input.confirm) return { dialog: { ...d, row }, action: null };
  const list = choices(name, dev, worlds);
  const at = list.indexOf(d.options[name]);
  let index = at + input.side;
  if (input.confirm) index = (at + 1) % list.length;
  return { dialog: { options: setOption(d.options, name, index, dev, worlds), row }, action: null };
}
