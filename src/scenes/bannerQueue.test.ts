import { describe, expect, it } from 'vitest';
import { BANNER_MAX_CHARS, enqueueBanner, fitBanner } from './bannerQueue';

describe('Welt-Meldungen im Band (B-414)', () => {
  it('kurze Meldungen bleiben unverändert', () => {
    expect(fitBanner('Nacht')).toBe('Nacht');
  });

  it('lange Meldungen werden mit „…“ gekürzt und überschreiten die Länge nie', () => {
    const r = fitBanner('Die Nacht bricht an und die Gegner kommen aus allen Portalen gleichzeitig über die Mauer');
    expect(r.length).toBeLessThanOrEqual(BANNER_MAX_CHARS);
    expect(r.endsWith('…')).toBe(true);
  });

  it('Umbrüche werden zu einem Leerzeichen, die Meldung bleibt einzeilig', () => {
    expect(fitBanner('Welle 2\nbeginnt')).toBe('Welle 2 beginnt');
  });

  it('eine Meldung kommt hinter die wartenden, nichts geht verloren', () => {
    expect(enqueueBanner(['Nacht'], 'Welle 1', 'Welle 2')).toEqual(['Nacht', 'Welle 2']);
  });

  it('dieselbe Meldung steht nur einmal, laufend oder wartend', () => {
    expect(enqueueBanner([], 'Nacht', 'Nacht')).toEqual([]);
    expect(enqueueBanner(['Nacht'], null, 'Nacht')).toEqual(['Nacht']);
  });

  it('eine leere Meldung wird nicht eingereiht', () => {
    expect(enqueueBanner(['Nacht'], null, '   ')).toEqual(['Nacht']);
  });
});
