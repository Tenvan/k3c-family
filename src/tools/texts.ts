/**
 * Texte der Werkzeug-Seiten (PL2, B-322): wie `t()` aus `src/core/texts.ts`, mit eigenen Tabellen, damit die Spieltexte klein bleiben.
 * Die Sprache kommt aus den Einstellungen des Geräts (`currentLanguage()`); fehlt ein englischer Text, gilt der deutsche.
 */
import { currentLanguage } from '../core/texts';
import { de, type ToolTextKey } from './texts.de';
import { en } from './texts.en';

export type { ToolTextKey } from './texts.de';

/** Text zum Schlüssel; `{name}` wird durch `params.name` ersetzt, ein unbekannter Platzhalter bleibt stehen. */
export function t(key: ToolTextKey, params?: Record<string, string | number>): string {
  const text = (currentLanguage() === 'en' ? en[key] : undefined) ?? de[key];
  return params ? text.replace(/\{(\w+)\}/g, (m, name: string) => (name in params ? String(params[name]) : m)) : text;
}

/** Das Wenige, was `applyTexts` von einem Element braucht (Tests geben eigene Objekte mit). */
export interface TextTarget {
  dataset: Record<string, string | undefined>;
  textContent: string | null;
  innerHTML?: string;
  setAttribute(name: string, value: string): void;
}

const isKey = (key: string | undefined): key is ToolTextKey => key !== undefined && key in de;

/** Text zu einem zusammengesetzten Schlüssel (`grafik.group.${id}`); ohne Eintrag der Rückfall, z. B. die ID aus den Daten. */
export function textOf(key: string, fallback: string): string {
  return isKey(key) ? t(key) : fallback;
}

/**
 * Setzt die statischen Texte einer Seite: `data-t` den Inhalt, `data-t-html` den Inhalt mit eigenem HTML (nur Texte aus unseren Tabellen),
 * `data-t-aria` das `aria-label`, `data-t-placeholder` den Platzhalter.
 * Ein unbekannter Schlüssel lässt den Text im HTML stehen.
 */
export function applyTexts(root: { querySelectorAll(sel: string): Iterable<TextTarget> } = document as never): void {
  for (const el of root.querySelectorAll('[data-t], [data-t-html], [data-t-aria], [data-t-placeholder]')) {
    const { t: text, tHtml: html, tAria: aria, tPlaceholder: placeholder } = el.dataset;
    if (isKey(text)) el.textContent = t(text);
    if (isKey(html)) el.innerHTML = t(html);
    if (isKey(aria)) el.setAttribute('aria-label', t(aria));
    if (isKey(placeholder)) el.setAttribute('placeholder', t(placeholder));
  }
  if (typeof document !== 'undefined') document.documentElement.lang = currentLanguage();
}
