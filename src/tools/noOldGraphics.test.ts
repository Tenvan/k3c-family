import { describe, expect, it } from 'vitest';

// CU1.3/AC-05: Die Werkzeugseiten greifen nicht mehr auf alte Pixel-Grafik, Atlas oder Register zu.
const files = import.meta.glob('./*.ts', { query: '?raw', import: 'default', eager: true }) as Record<string, string>;
const VERBOTEN = [/data\/assets\.json/, /data\/sprites\.json/, /grafik\/index\.json/, /grafik-db32/, /public\/sprites/, /['"`/]atlas\//, /(grafik|sprites)\/CREDITS/];

describe('Werkzeugseiten ohne alte Grafik (CU1.3)', () => {
  it('kein Modul unter src/tools nennt Register, alte Packs, Atlas oder Pipeline', () => {
    const treffer = Object.entries(files)
      .filter(([name]) => !name.endsWith('noOldGraphics.test.ts'))
      .flatMap(([name, text]) => VERBOTEN.filter((re) => re.test(text)).map((re) => `${name}: ${re}`));
    expect(treffer).toEqual([]);
  });
});
