import { afterEach, describe, expect, it, vi } from 'vitest';
import { clampVolume, DEFAULT_SETTINGS, loadSettings, normalizeSettings, saveSettings, type Settings } from './settings';

function fakeStorage(initial: Record<string, string> = {}) {
  const data = { ...initial };
  return {
    data,
    getItem: (key: string) => data[key] ?? null,
    setItem: (key: string, value: string) => void (data[key] = value),
  };
}

const throwing = {
  getItem: (): string | null => {
    throw new Error('gesperrt');
  },
  setItem: (): void => {
    throw new Error('voll');
  },
};

const stored = (raw: string) => loadSettings(fakeStorage({ 'k3c-settings': raw }));

afterEach(() => vi.unstubAllGlobals());

describe('settings', () => {
  it('Standard: alles an, Lautstärke 100 %, Deutsch', () => {
    expect(DEFAULT_SETTINGS).toEqual({ musicVolume: 100, sfxVolume: 100, ambientVolume: 100, screenshake: true, flash: true, colorblindSymbols: true, guideHints: true, language: 'de' });
    expect(loadSettings(fakeStorage())).toEqual(DEFAULT_SETTINGS);
  });

  it('begrenzt Lautstärken auf 0–100 und rundet auf ganze Prozent', () => {
    expect(clampVolume(-5, 100)).toBe(0);
    expect(clampVolume(130, 100)).toBe(100);
    expect(clampVolume(42.6, 100)).toBe(43);
    expect(normalizeSettings({ musicVolume: -5, sfxVolume: 130 })).toMatchObject({ musicVolume: 0, sfxVolume: 100 });
  });

  it('NaN, Unendlich und Text → Standard des Feldes', () => {
    expect(clampVolume(Number.NaN, 70)).toBe(70);
    expect(normalizeSettings({ musicVolume: Number.NaN, sfxVolume: '50' })).toMatchObject({ musicVolume: 100, sfxVolume: 100 });
    expect(normalizeSettings({ musicVolume: Infinity })).toMatchObject({ musicVolume: 100 });
  });

  it('kaputtes JSON, null, Array und Nicht-Objekte → Standardwerte', () => {
    for (const raw of ['{kaputt', 'null', '[1,2]', '"text"', '42']) expect(stored(raw)).toEqual(DEFAULT_SETTINGS);
    expect(normalizeSettings(undefined)).toEqual(DEFAULT_SETTINGS);
  });

  it('falsche Feldtypen → Standard, gültige Felder bleiben erhalten', () => {
    const raw = JSON.stringify({ musicVolume: 30, sfxVolume: 'laut', screenshake: false, flash: 'nein', colorblindSymbols: 0, language: 'en' });
    expect(stored(raw)).toEqual({ ...DEFAULT_SETTINGS, musicVolume: 30, screenshake: false, language: 'en' });
  });

  it('unbekannte Sprache → de', () => {
    expect(normalizeSettings({ language: 'fr' }).language).toBe('de');
    expect(normalizeSettings({ language: 'en' }).language).toBe('en');
  });

  it('werfender Speicher: Lesen liefert Standard, Schreiben stürzt nicht ab', () => {
    expect(loadSettings(throwing)).toEqual(DEFAULT_SETTINGS);
    expect(saveSettings(DEFAULT_SETTINGS, throwing)).toBe(false);
  });

  it('ohne localStorage (kein Browser) → Standardwerte, kein Absturz', () => {
    vi.stubGlobal('localStorage', undefined);
    expect(loadSettings()).toEqual(DEFAULT_SETTINGS);
    expect(saveSettings(DEFAULT_SETTINGS)).toBe(false);
  });

  it('ohne Parameter wird localStorage erst beim Aufruf gelesen', () => {
    const storage = fakeStorage();
    vi.stubGlobal('localStorage', storage);
    expect(saveSettings({ ...DEFAULT_SETTINGS, flash: false })).toBe(true);
    expect(loadSettings().flash).toBe(false);
  });

  it('Schreiben, dann neu Lesen liefert dieselben Werte (Neuladen)', () => {
    const storage = fakeStorage();
    const settings: Settings = { musicVolume: 0, sfxVolume: 40, ambientVolume: 25, screenshake: false, flash: false, colorblindSymbols: true, guideHints: false, language: 'en' };
    expect(saveSettings(settings, storage)).toBe(true);
    expect(Object.keys(storage.data)).toEqual(['k3c-settings']);
    expect(loadSettings(storage)).toEqual(settings);
  });

  it('liefert Kopien, Standardwerte bleiben unverändert', () => {
    const s = loadSettings(fakeStorage());
    s.musicVolume = 10;
    expect(DEFAULT_SETTINGS.musicVolume).toBe(100);
  });
});
