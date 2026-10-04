/**
 * Zentrale Spieltexte (S5.3, B-172): `t(key, params?)` liefert den Text in der gewählten Sprache. Deutsch ist Standard und
 * Rückfall (unbekannte Sprache, fehlender Text → Deutsch, der fehlende Text steht einmal im Client-Log).
 * Die Sprache kommt beim ersten Aufruf aus den Einstellungen des Geräts; die Optionen setzen sie mit `setLanguage`.
 */
import { clientLog } from './clientLog';
import { loadSettings, type Language } from './settings';
import { de, type TextKey } from './texts.de';
import { en } from './texts.en';

export type { TextKey } from './texts.de';

const TABLES: Record<Language, Partial<Record<TextKey, string>>> = { de, en };
const reported = new Set<string>();
let language: Language | undefined;

/** Unbekannte Sprache gilt als Deutsch. */
export function setLanguage(lang: string): void {
  language = lang === 'en' ? 'en' : 'de';
}

export function currentLanguage(): Language {
  return (language ??= loadSettings().language);
}

/** Text zum Schlüssel; `{name}` wird durch `params.name` ersetzt, ein unbekannter Platzhalter bleibt stehen. */
export function t(key: TextKey, params?: Record<string, string | number>): string {
  const lang = currentLanguage();
  let text = TABLES[lang][key];
  if (text === undefined) {
    if (!reported.has(`${lang}:${key}`)) {
      reported.add(`${lang}:${key}`);
      clientLog('warn', `Text fehlt: ${lang}/${key}`);
    }
    text = de[key];
  }
  return params ? text.replace(/\{(\w+)\}/g, (m, name: string) => (name in params ? String(params[name]) : m)) : text;
}

/** Name aus den Daten (Gebäude, Rohstoff, Biom): `group.id` aus den Textdateien, sonst der Name der Daten. */
export function nameOf(group: 'site' | 'res' | 'biome', id: string, fallback: string): string {
  const key = `${group}.${id}`;
  return key in de ? t(key as TextKey) : fallback;
}
