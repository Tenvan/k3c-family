import type { CommitMessage, GitFile } from '../api';
import type { Tone } from '../ui/parts';

// Reine Funktionen der Git-Seite: Nachricht wie gitcommit.Message.Format in Go, Fehlerfeld, Statusfarbe, Auswahl.

export const MAX_SUBJECT = 72;

/** Vorschau der Nachricht: Kopfzeile `typ(scope): Betreff`, bei Rumpf eine Leerzeile dazwischen. */
export function formatMessage(m: CommitMessage): string {
  const scope = m.scope.trim() ? `(${m.scope.trim()})` : '';
  const head = `${m.type}${scope}: ${m.subject.trim()}`;
  return m.body.trim() ? `${head}\n\n${m.body.trim()}` : head;
}

/** Aus `feld: Grund` (Go: gitcommit.FieldError) das Feld und den Grund; sonst nur der Text. */
export function parseFieldError(message: string): { field: string; text: string } {
  const m = /^(type|scope|subject|body): (.*)$/s.exec(message);
  return m ? { field: m[1], text: m[2] } : { field: '', text: message };
}

/** Farbe des Status: neu grün, geändert blau, gelöscht rot, Konflikt/untracked auffällig bzw. neutral. */
export function statusTone(status: string): Tone {
  switch (status) {
    case 'A':
      return 'ok';
    case 'M':
    case 'R':
    case 'C':
    case 'T':
      return 'info';
    case 'D':
    case 'U':
      return 'error';
    default:
      return 'neutral'; // ? untracked
  }
}

export function statusLabel(status: string): string {
  return { A: 'neu', M: 'geändert', D: 'gelöscht', R: 'umbenannt', C: 'kopiert', T: 'Typ', U: 'Konflikt', '?': 'neu (untracked)' }[status] ?? status;
}

/** Verzeichnis und Dateiname getrennt, damit die Liste den Namen hervorheben kann. */
export function splitPath(p: string): { dir: string; name: string } {
  const i = p.lastIndexOf('/');
  return i < 0 ? { dir: '', name: p } : { dir: p.slice(0, i + 1), name: p.slice(i + 1) };
}

/** Auswahl nach einem neuen Stand: nur Pfade, die es in der Liste noch gibt. */
export function pruneSelection(selected: ReadonlySet<string>, files: GitFile[]): Set<string> {
  const alive = new Set(files.map((f) => f.path));
  return new Set([...selected].filter((p) => alive.has(p)));
}
