import { readFileSync } from 'node:fs';
import { describe, expect, it } from 'vitest';

const css = readFileSync(new URL('./world.css', import.meta.url), 'utf8');

/** B-663/AC-01: Die Seite bricht nach ihrer eigenen Breite um (der Chat belegt rechts einen Teil des Fensters), nie nach dem Fenster. */
describe('world.css', () => {
  it('Spalten von Welten Editor und Monarch umbrechen per Container Query', () => {
    expect(css).not.toMatch(/@media\s*\(max-width[^)]*\)\s*\{[^}]*\.wd-(columns|monarch)/);
    expect(css).toMatch(/@container\s*\(max-width:\s*1100px\)\s*\{\s*\.wd-columns/);
    expect(css).toMatch(/@container\s*\(max-width:\s*900px\)\s*\{\s*\.wd-monarch/);
  });

  it('der Seitencontainer ist ein Inline-Size-Container', () => {
    expect(css).toMatch(/\.wd-root\s*\{[^}]*container-type:\s*inline-size/);
  });

  it('Raster haben keine feste Mindestbreite über der Seitenbreite', () => {
    expect(css).not.toMatch(/\.wd-columns\s*\{[^}]*minmax\(\d+rem/);
    expect(css).not.toMatch(/\.wd-monarch\s*\{[^}]*minmax\(\d+rem,/);
  });
});
