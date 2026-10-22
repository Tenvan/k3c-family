/**
 * Spielstände im Spielmenü (Lobby) ohne Phaser (BD5.2, B-506): Name eines neuen Stands, Welt-Anzeige und die Rückfrage vor
 * dem Löschen. `LobbyScene` zeichnet nur und gibt Eingaben weiter.
 */
import { t } from '../core/texts';
import type { DeleteResult } from '../core/saveStore';

/** Welt-ID ohne Angabe (Stände vor Version 8, `?world=` leer) */
export const DEFAULT_WORLD = 'standard';

/** Präfix neuer Spielstände aus dem Menü (Entscheidung B-534) */
const NEW_PREFIX = 'spiel-';

/** Anzeigename einer Welt-ID: leer = `standard` */
export const worldOf = (id: string | undefined): string => (id ? id : DEFAULT_WORLD);

/** Nächster freier Name `spiel-N` (N ab 1), weder als Spielstand noch als offener Raum vergeben */
export function nextSaveName(taken: Iterable<string>): string {
  const used = new Set(taken);
  let n = 1;
  while (used.has(`${NEW_PREFIX}${n}`)) n++;
  return `${NEW_PREFIX}${n}`;
}

export type DeleteChoice = 'cancel' | 'delete';
/** Reihenfolge der Zeilen der Rückfrage: „Abbrechen“ steht oben und ist vorgewählt */
export const DELETE_CHOICES: readonly DeleteChoice[] = ['cancel', 'delete'];

/**
 * Rückfrage vor dem Löschen genau eines Spielstands. Nur „Löschen“ + Bestätigen liefert ihn; jede andere Bestätigung bricht ab.
 * Erneutes Öffnen (X/Entf) während der Rückfrage bricht ebenfalls ab.
 */
export class DeleteDialog {
  private target: string | null = null;
  private index = 0;

  get open(): boolean {
    return this.target !== null;
  }

  get name(): string | null {
    return this.target;
  }

  get selected(): number {
    return this.index;
  }

  /** Öffnet die Rückfrage für name; ist sie schon offen, schließt sie (Abbrechen) */
  toggle(name: string): void {
    if (this.target !== null) return this.close();
    this.target = name;
    this.index = 0;
  }

  move(dir: number): void {
    if (this.target !== null) this.index = Math.max(0, Math.min(DELETE_CHOICES.length - 1, this.index + dir));
  }

  /** Bestätigt die gewählte Zeile (oder `row` beim Tippen): Name zum Löschen oder null (abgebrochen); schließt die Rückfrage */
  confirm(row = this.index): string | null {
    const name = DELETE_CHOICES[row] === 'delete' ? this.target : null;
    this.close();
    return name;
  }

  close(): void {
    this.target = null;
    this.index = 0;
  }

  /** Zeilentexte der Rückfrage */
  labels(): string[] {
    return DELETE_CHOICES.map((c) => t(c === 'delete' ? 'lobby.deleteYes' : 'lobby.deleteNo'));
  }
}

/** Hinweis nach dem Löschen; nichts bleibt still */
export function deleteNotice(name: string, result: DeleteResult): string {
  if (result === 'ok') return t('lobby.deleted', { name });
  if (result === 'in_use') return t('lobby.deleteInUse', { name });
  if (result === 'not_found') return t('lobby.deleteMissing', { name });
  return t('lobby.deleteFailed', { name });
}
