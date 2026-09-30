// Gemerkte Einstellungen der Oberfläche (Reiter, Farbmodus) in localStorage. Der Zugriff kann werfen (gesperrte
// Website-Daten); dann gilt die Vorgabe und nichts wird gemerkt.

const PREFIX = 'k3c-dev:';

export function loadPref<T extends string>(key: string, allowed: readonly T[], fallback: T): T {
  try {
    const v = localStorage.getItem(PREFIX + key);
    return allowed.includes(v as T) ? (v as T) : fallback;
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
