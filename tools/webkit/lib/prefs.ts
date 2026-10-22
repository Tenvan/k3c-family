// Gemerkte Einstellungen der Oberfläche (Reiter, Farbmodus) in localStorage, Schlüssel `<app>.<seite>.<was>`
// (Workbench-Spec › Gemerkter Zustand). Der Zugriff kann werfen (gesperrte Website-Daten); dann gilt die Vorgabe und
// nichts wird gemerkt. Freitext-Filter werden nicht gemerkt. Jede App bindet die Funktionen mit ihrem Präfix
// (`k3c-workbench.`, `k3c-studio.`) in ihrem `src/lib/prefs.ts`.

export function createPrefs(prefix: string) {
  function loadPref<T extends string>(key: string, allowed: readonly T[], fallback: T): T {
    try {
      const v = localStorage.getItem(prefix + key);
      return allowed.includes(v as T) ? (v as T) : fallback;
    } catch {
      return fallback;
    }
  }

  /** Freier Text, z. B. die gewählte Quelle; der Aufrufer prüft ihn selbst. */
  function loadText(key: string, fallback: string): string {
    try {
      return localStorage.getItem(prefix + key) ?? fallback;
    } catch {
      return fallback;
    }
  }

  function savePref(key: string, value: string): void {
    try {
      localStorage.setItem(prefix + key, value);
    } catch {
      // nicht merkbar, siehe oben
    }
  }

  /**
   * JSON-Wert mit Prüfung (Workbench-Spec › Gemerkter Zustand): ein unlesbarer oder veralteter Wert gilt als nicht
   * gemerkt, statt die Seite zu stören. Schlüssel `<seite>.<was>`, z. B. `tasks.favorites`.
   */
  function loadJSON<T>(key: string, valid: (v: unknown) => v is T, fallback: T): T {
    try {
      const v: unknown = JSON.parse(localStorage.getItem(prefix + key) ?? 'null');
      return valid(v) ? v : fallback;
    } catch {
      return fallback;
    }
  }

  function saveJSON(key: string, value: unknown): void {
    savePref(key, JSON.stringify(value));
  }

  return { loadPref, loadText, savePref, loadJSON, saveJSON };
}

/** Prüfung für loadJSON: eine Liste von Texten. */
export const isStringList = (v: unknown): v is string[] => Array.isArray(v) && v.every((x) => typeof x === 'string');
