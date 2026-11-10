// B-773/AC-01, AC-02: Die Textur-Testseite füllt genau einen Screen. Ein Layout-Motor fehlt in Vitest (happy-dom), deshalb prüft der Test den
// Vertrag im Quelltext; die gerenderte Messung (1920x1080, 1280x720, 800x600) steht im Ergebnis der Session LG1.4.
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const html = readFileSync(resolve(__dirname, '../texturetest.html'), 'utf8');
const script = readFileSync(resolve(__dirname, '../src/tools/textureTest.ts'), 'utf8');
const css = html.slice(html.indexOf('<style>'), html.indexOf('</style>'));
/** CSS ohne die Regeln für sehr kleine Fenster (dort ist Scrollen erlaubt) */
const mainCss = css.split('@media (max-width: 639px)')[0]!;

describe('Textur-Testseite: ein Screen (B-773)', () => {
  it('Seite hat Viewport-Höhe und scrollt nicht, nur die Textfelder scrollen intern', () => {
    expect(mainCss).toMatch(/body \{ overflow: hidden;/);
    expect(mainCss).toMatch(/main \{[^}]*height: 100dvh/);
    expect(mainCss).toMatch(/\.cols \{[^}]*min-height: 0/);
    expect(mainCss).toMatch(/pre \{[^}]*overflow: auto/);
  });

  it('unter 640x480 darf die Seite scrollen, nichts wird abgeschnitten', () => {
    expect(css).toMatch(/@media \(max-width: 639px\), \(max-height: 479px\) \{ body \{ overflow: auto;/);
  });

  it('der Rahmen bleibt: installPageChrome(), ca. 70 px frei oben, Skala nach Viewport', () => {
    expect(script).toMatch(/^installPageChrome\(\);$/m);
    expect(mainCss).toMatch(/padding: max\(4\.75rem, 56px\)/);
    expect(mainCss).toMatch(/html \{ font-size: clamp\(/);
  });

  it('alle Blöcke der Diagnose sind vorhanden', () => {
    for (const id of ['sizes', 'btn-start', 'btn-send', 'state', 'sent', 'diagResult', 'diag', 'btn-diag', 'result', 'out', 'device', 'stage']) {
      expect(html).toContain(`id="${id}"`);
    }
  });
});
