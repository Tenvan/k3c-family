/**
 * Optionen je Gerät (B-146, B-172): im `localStorage` des Browsers, nicht auf dem Server.
 * Kaputter, fehlender oder gesperrter Speicher liefert die Standardwerte, nie einen Absturz.
 */

export type Language = 'de' | 'en';

export interface Settings {
  /** Ganze Prozent 0–100. */
  musicVolume: number;
  /** Ganze Prozent 0–100. */
  sfxVolume: number;
  screenshake: boolean;
  flash: boolean;
  colorblindSymbols: boolean;
  language: Language;
}

type SettingsStorage = Pick<Storage, 'getItem' | 'setItem'>;

const KEY = 'k3c-settings';

export const DEFAULT_SETTINGS: Readonly<Settings> = Object.freeze({
  musicVolume: 100,
  sfxVolume: 100,
  screenshake: true,
  flash: true,
  colorblindSymbols: true,
  language: 'de',
});

/** Begrenzt auf ganze Prozent 0–100; keine endliche Zahl → `fallback`. */
export function clampVolume(value: unknown, fallback: number): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) return fallback;
  return Math.min(100, Math.max(0, Math.round(value)));
}

const bool = (value: unknown, fallback: boolean) => (typeof value === 'boolean' ? value : fallback);

/** Rein: macht aus beliebigen Daten gültige Einstellungen, gültige Felder bleiben erhalten. */
export function normalizeSettings(raw: unknown): Settings {
  const r = typeof raw === 'object' && raw !== null && !Array.isArray(raw) ? (raw as Record<string, unknown>) : {};
  const d = DEFAULT_SETTINGS;
  return {
    musicVolume: clampVolume(r.musicVolume, d.musicVolume),
    sfxVolume: clampVolume(r.sfxVolume, d.sfxVolume),
    screenshake: bool(r.screenshake, d.screenshake),
    flash: bool(r.flash, d.flash),
    colorblindSymbols: bool(r.colorblindSymbols, d.colorblindSymbols),
    language: r.language === 'en' ? 'en' : 'de',
  };
}

/** `storage` fehlt → `localStorage`, erst hier gelesen, weil schon der Zugriff werfen kann. */
export function loadSettings(storage?: SettingsStorage): Settings {
  try {
    const s = storage ?? globalThis.localStorage;
    return normalizeSettings(JSON.parse(s.getItem(KEY) ?? 'null'));
  } catch {
    return normalizeSettings(null);
  }
}

/** Wirft nie; `false`, wenn der Speicher gesperrt oder voll ist. */
export function saveSettings(settings: Settings, storage?: SettingsStorage): boolean {
  try {
    const s = storage ?? globalThis.localStorage;
    s.setItem(KEY, JSON.stringify(normalizeSettings(settings)));
    return true;
  } catch {
    return false;
  }
}
