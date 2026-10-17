/** Ablehnung von AssetsSet auf veraltetem Stand: Text von assets.ErrStale in Go (B-459 › Ausnahme- und Fehlerfälle). */
export const STALE = 'Register geändert, bitte erneut speichern';

/** Ist die Ablehnung die Schreibregel? Wails liefert den Fehlertext aus Go als String, der Mock einen Error. */
export const isStale = (e: unknown): boolean => (e instanceof Error ? e.message : String(e)).includes(STALE);
