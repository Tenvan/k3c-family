import { describe, expect, it } from 'vitest';

/**
 * B-768/AC-04 (BD11.3): Kein Modul außer `runConfig.ts` liest die Abfrage der Seite (`location.search`, auch `window.location.search`
 * und `new URL(location…)`). Start-Parameter kommen aus `runSearch()`/`runParam()` (Menü vor URL vor Standard). Ausgenommen:
 * `src/landing/` (die Landingpage entfällt mit BD11.6) und Kommentare.
 */
const sources = import.meta.glob(['../**/*.ts', '!../**/*.test.ts', '!./runConfig.ts', '!../landing/**'], { query: '?raw', import: 'default', eager: true }) as Record<string, string>;

const READS_LOCATION = /\b(?:window\.|document\.)?location\.(?:search|href|hash)\b|new URL\((?:window\.)?location/;
const withoutComments = (text: string): string =>
  text
    .split('\n')
    .map((line) => line.replace(/\/\/.*$/, '').replace(/^\s*\*.*$/, '').replace(/\/\*.*?\*\//g, ''))
    .join('\n');

/** Stellen, die die Adresse der Seite aus anderem Grund brauchen: Log-Eintrag, Kachel-Ziele der Werkzeugseite, Gamepad-Bericht */
const ALLOWED = ['./clientLog.ts', '../tools/dev.ts', '../tools/gamepadTest.ts'];

describe('Start-Parameter nur über RunConfig (B-768/AC-04)', () => {
  it('findet die Quellen', () => {
    expect(Object.keys(sources).length).toBeGreaterThan(100);
  });

  it('kein Modul liest location.search, location.href oder location.hash außer den erlaubten', () => {
    const offenders = Object.entries(sources)
      .filter(([file]) => !ALLOWED.includes(file))
      .filter(([, text]) => READS_LOCATION.test(withoutComments(text)))
      .map(([file]) => file);
    expect(offenders).toEqual([]);
  });

  it('erkennt Lesezugriffe, nicht aber Kommentare', () => {
    expect(READS_LOCATION.test(withoutComments('const q = location.search;'))).toBe(true);
    expect(READS_LOCATION.test(withoutComments('const q = window.location.search;'))).toBe(true);
    expect(READS_LOCATION.test(withoutComments('// liest location.search nicht'))).toBe(false);
    expect(READS_LOCATION.test(withoutComments(' * `search` ist `location.search`'))).toBe(false);
  });
});
