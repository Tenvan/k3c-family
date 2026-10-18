import { afterEach, describe, expect, it } from 'vitest';
import { setLanguage } from '../core/texts';
import { applyTexts, t, type TextTarget, type ToolTextKey } from './texts';
import { de } from './texts.de';
import { en } from './texts.en';

const keys = Object.keys(de) as ToolTextKey[];
const placeholders = (s: string) => [...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]!).sort();

afterEach(() => setLanguage('de'));

describe('Textdateien der Werkzeug-Seiten (PL2 AC-01)', () => {
  it.each(keys)('%s hat einen englischen Text mit denselben Platzhaltern', (key) => {
    expect(en[key]?.trim()).toBeTruthy();
    expect(placeholders(en[key]!)).toEqual(placeholders(de[key]));
  });

  it('die englische Datei hat keine Schlüssel mehr als die deutsche', () => {
    expect(Object.keys(en).filter((k) => !(k in de))).toEqual([]);
  });
});

describe('t() der Werkzeug-Seiten', () => {
  it('liefert die gewählte Sprache und ersetzt Platzhalter', () => {
    expect(t('test.starting', { title: 'Wald' })).toBe('Starte „Wald“ …');
    setLanguage('en');
    expect(t('test.starting', { title: 'Wald' })).toBe('Starting “Wald” …');
  });

  it('fällt auf Deutsch zurück, wenn der englische Text fehlt', () => {
    setLanguage('en');
    const saved = en['sound.volUp'];
    delete en['sound.volUp'];
    try {
      expect(t('sound.volUp')).toBe('Lauter +');
    } finally {
      en['sound.volUp'] = saved;
    }
  });
});

describe('applyTexts()', () => {
  const target = (dataset: TextTarget['dataset'], text = 'alt'): TextTarget & { attrs: Record<string, string> } => {
    const attrs: Record<string, string> = {};
    return { dataset, textContent: text, attrs, setAttribute: (n, v) => void (attrs[n] = v) };
  };

  it('setzt Inhalt, aria-label und Platzhalter; ein unbekannter Schlüssel lässt den HTML-Text stehen', () => {
    setLanguage('en');
    const els = [target({ t: 'tool.opening' }), target({ tAria: 'sound.volUpAria', t: 'sound.volUp' }), target({ tPlaceholder: 'test.h.players' }), target({ t: 'gibt.es.nicht' })];
    applyTexts({ querySelectorAll: () => els });
    expect(els.map((e) => e.textContent)).toEqual(['Opening “{title}” …', 'Louder +', 'alt', 'alt']);
    expect(els[1]!.attrs).toEqual({ 'aria-label': 'Louder' });
    expect(els[2]!.attrs).toEqual({ placeholder: 'Players' });
  });
});
