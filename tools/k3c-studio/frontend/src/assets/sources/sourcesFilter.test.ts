import { describe, expect, it } from 'vitest';
import { eintraege, lizenzFamilie } from './sourcesFilter';

// B-729/AC-01: Lizenz-Familie für die Chips; unbekannte Lizenz bleibt eigener Chip.

describe('lizenzFamilie (B-729)', () => {
  it('fasst Versionen zusammen, ohne Angabe „unbekannt“', () => {
    expect(lizenzFamilie('CC BY-NC 4.0')).toBe('CC-BY-NC');
    expect(lizenzFamilie('CC BY 3.0')).toBe('CC-BY');
    expect(lizenzFamilie('CC0')).toBe('CC0');
    expect(lizenzFamilie('OGA-BY v1.0')).toBe('OGA-BY');
    expect(lizenzFamilie('')).toBe('unbekannt');
    expect(lizenzFamilie(undefined)).toBe('unbekannt');
  });

  it('Treffer und Katalog in einer Liste; Treffer ohne Lizenz und Datei nutzen „unbekannt“ und den Link', () => {
    const list = eintraege({
      katalog: [{ datei: 'a.ogg', art: 'sound', pack: 'P', lizenz: 'CC0' }],
      treffer: { 'sfx:x': [{ id: 'h', titel: 'T', link: 'https://x/', quelle: 'Q', lizenzStatus: 'pruefen', erstellt: '' }] },
    });
    expect(list.map((e) => [e.herkunft, e.ort, e.pack, e.lizenz, e.lizenzText])).toEqual([
      ['treffer', 'https://x/', 'Q', 'unbekannt', 'unbekannt'],
      ['katalog', 'a.ogg', 'P', 'CC0', 'CC0'],
    ]);
  });
});
