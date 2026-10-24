/**
 * UI-Klick-Töne der Menü-Szenen (B-575): Auswahl wechseln und Bestätigen. Reine Client-Sache, kein Server-Ereignis (Q08):
 * die Szene meldet, was der Spieler getan hat, der Ton kommt aus den Effekt-Slots des Registers. Die B-Taste bleibt unbelegt.
 */
import { audioCore } from './audioCore';

export type UiSound = 'select' | 'confirm';

/** Effekt-Slots im Register `data/assets.json` */
export const UI_SLOTS: Readonly<Record<UiSound, string>> = { select: 'sfx:uiSelect', confirm: 'sfx:uiConfirm' };

/** Rein: Eingabe eines Frames → Ton. Bestätigen (Enter, A, Tippen) schlägt Auswahl (Richtung, Seite); nichts getan → `null`. */
export function uiSoundFor(input: { dir?: number; side?: number; confirm?: boolean }): UiSound | null {
  if (input.confirm) return 'confirm';
  return input.dir || input.side ? 'select' : null;
}

/** Spielt den Ton; die Eingabe entsperrt zugleich den Audio-Kontext (die Menüs laufen vor dem Spiel). Wirft nie. */
export function playUi(sound: UiSound | null): void {
  if (!sound) return;
  const core = audioCore();
  void core.onInput();
  core.playSlot(UI_SLOTS[sound]);
}
