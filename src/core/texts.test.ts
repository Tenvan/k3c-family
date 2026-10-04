import { afterEach, describe, expect, it } from 'vitest';
import { currentLanguage, nameOf, setLanguage, t, type TextKey } from './texts';
import { de } from './texts.de';
import { en } from './texts.en';

const keys = Object.keys(de) as TextKey[];
const placeholders = (s: string) => [...s.matchAll(/\{(\w+)\}/g)].map((m) => m[1]!).sort();

afterEach(() => setLanguage('de'));

describe('Textdateien (AC-01)', () => {
  it.each(keys)('%s hat einen englischen Text mit denselben Platzhaltern', (key) => {
    expect(en[key]?.trim()).toBeTruthy();
    expect(placeholders(en[key])).toEqual(placeholders(de[key]));
  });

  it('die englische Datei hat keine Schlüssel mehr als die deutsche', () => {
    expect(Object.keys(en).sort()).toEqual([...keys].sort());
  });

  it('erkennt einen fehlenden englischen Text (Beleg für die Prüfung oben)', () => {
    const broken: Partial<Record<TextKey, string>> = { ...en };
    delete broken['opt.title'];
    expect(keys.filter((k) => !broken[k]?.trim())).toEqual(['opt.title']);
  });
});

describe('t() (AC-02)', () => {
  it('liefert die gewählte Sprache und ersetzt Platzhalter', () => {
    setLanguage('en');
    expect(t('ev.night', { day: 3 })).toBe('Night 3');
    setLanguage('de');
    expect(t('ev.night', { day: 3 })).toBe('Nacht 3');
    expect(t('ev.chest', { p: 1, gold: 5 })).toBe('P1 findet 5 Gold');
  });

  it('unbekannte Sprache → Deutsch', () => {
    setLanguage('fr');
    expect(currentLanguage()).toBe('de');
    expect(t('opt.title')).toBe('Optionen');
  });

  it('fehlender englischer Text → Deutsch', () => {
    const saved = en['opt.title'];
    delete (en as Partial<typeof en>)['opt.title'];
    setLanguage('en');
    expect(t('opt.title')).toBe('Optionen');
    en['opt.title'] = saved;
    expect(t('opt.title')).toBe('Options');
  });

  it('unbekannter Platzhalter bleibt stehen', () => {
    expect(t('ev.night', {})).toBe('Nacht {day}');
  });

  it('Namen aus den Daten: Schlüssel vorhanden → Text, sonst Name der Daten', () => {
    expect(nameOf('site', 'wall', 'Mauer (Daten)')).toBe('Mauer');
    setLanguage('en');
    expect(nameOf('site', 'wall', 'Mauer (Daten)')).toBe('Wall');
    expect(nameOf('biome', 'unbekannt', 'Daten-Name')).toBe('Daten-Name');
  });
});
