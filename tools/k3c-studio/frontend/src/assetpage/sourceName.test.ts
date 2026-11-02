import { describe, expect, it } from 'vitest';
import { sourceName } from './model';

// B-677: Die Facette „Quelle“ zeigt Anbieter, nicht die Download-Adresse jeder Datei (Befund mit echten Daten, 2026-10-30).
describe('Anbieter einer Quelle', () => {
  it('fasst Adressen auf den Anbieter zusammen', () => {
    expect(sourceName('https://kenney.nl/assets/digaudio#highUp')).toBe('Kenney');
    expect(sourceName('https://kenney.nl/assets/digaudio')).toBe('Kenney');
    expect(sourceName('https://freesound.org/people/someone/sounds/123/')).toBe('freesound');
    expect(sourceName('https://opengameart.org/content/x')).toBe('OpenGameArt');
    expect(sourceName('https://someone.itch.io/pack')).toBe('itch.io');
    expect(sourceName('https://commons.wikimedia.org/wiki/File:X.png')).toBe('Wikimedia Commons');
  });

  it('zeigt unbekannte Hosts ohne www und lässt Text ohne Adresse stehen', () => {
    expect(sourceName('https://www.github.com/AntumDeluge/resources')).toBe('github.com');
    expect(sourceName('Luis Zuno')).toBe('Luis Zuno');
    expect(sourceName('selbst erzeugt (kein Credit nötig)')).toBe('eigene');
  });

  it('leer oder kaputt wird zum Strich', () => {
    expect(sourceName('')).toBe('–');
    expect(sourceName(undefined)).toBe('–');
    expect(sourceName('   ')).toBe('–');
    expect(sourceName('https://')).toBe('https://');
  });
});
