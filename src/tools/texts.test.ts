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
    expect(t('tool.opening', { title: 'Wald' })).toBe('Öffne „Wald“ …');
    setLanguage('en');
    expect(t('tool.opening', { title: 'Wald' })).toBe('Opening “Wald” …');
  });

  it('fällt auf Deutsch zurück, wenn der englische Text fehlt', () => {
    setLanguage('en');
    const saved = en['monitor.tab.overview'];
    delete en['monitor.tab.overview'];
    try {
      expect(t('monitor.tab.overview')).toBe('Übersicht');
    } finally {
      en['monitor.tab.overview'] = saved;
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
    const els = [target({ t: 'tool.opening' }), target({ tAria: 'monitor.tab.history', t: 'monitor.tab.overview' }), target({ tPlaceholder: 'monitor.tab.history' }), target({ t: 'gibt.es.nicht' })];
    applyTexts({ querySelectorAll: () => els });
    expect(els.map((e) => e.textContent)).toEqual(['Opening “{title}” …', 'Overview', 'alt', 'alt']);
    expect(els[1]!.attrs).toEqual({ 'aria-label': 'History' });
    expect(els[2]!.attrs).toEqual({ placeholder: 'History' });
  });
});
