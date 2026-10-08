// Gemerkte Einstellungen der Oberfläche (Reiter, Farbmodus) in localStorage, Schlüssel `k3c-dev.<seite>.<was>`
// (Workbench-Spec › Gemerkter Zustand). Der Zugriff kann werfen (gesperrte Website-Daten); dann gilt die Vorgabe und
// nichts wird gemerkt. Freitext-Filter werden nicht gemerkt.

const PREFIX = 'k3c-dev.';

export function loadPref<T extends string>(key: string, allowed: readonly T[], fallback: T): T {
  try {
    const v = localStorage.getItem(PREFIX + key);
    return allowed.includes(v as T) ? (v as T) : fallback;
  } catch {
    return fallback;
  }
}

/** Freier Text, z. B. die gewählte Quelle; der Aufrufer prüft ihn selbst. */
export function loadText(key: string, fallback: string): string {
  try {
    return localStorage.getItem(PREFIX + key) ?? fallback;
  } catch {
    return fallback;
  }
}

export function savePref(key: string, value: string): void {
  try {
    localStorage.setItem(PREFIX + key, value);
  } catch {
    // nicht merkbar, siehe oben
  }
}

/**
 * JSON-Wert mit Prüfung (Workbench-Spec › Gemerkter Zustand): ein unlesbarer oder veralteter Wert gilt als nicht
 * gemerkt, statt die Seite zu stören. Schlüssel `<seite>.<was>`, z. B. `tasks.favorites`.
 */
export function loadJSON<T>(key: string, valid: (v: unknown) => v is T, fallback: T): T {
  try {
    const v: unknown = JSON.parse(localStorage.getItem(PREFIX + key) ?? 'null');
    return valid(v) ? v : fallback;
  } catch {
    return fallback;
  }
}

export function saveJSON(key: string, value: unknown): void {
  savePref(key, JSON.stringify(value));
}

/** Prüfung für loadJSON: eine Liste von Texten. */
export const isStringList = (v: unknown): v is string[] => Array.isArray(v) && v.every((x) => typeof x === 'string');
