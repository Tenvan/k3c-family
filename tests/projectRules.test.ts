// Prüft die Projektregeln aus CLAUDE.md, damit die CI Verstöße früh meldet.
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import { PAGES } from '../src/landing/pages';

const ROOT = resolve(__dirname, '..');
const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');

function sourceFiles(dir: string): string[] {
  return readdirSync(join(ROOT, dir)).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(join(ROOT, path)).isDirectory()) return sourceFiles(path);
    return /\.ts$/.test(name) && !name.endsWith('.test.ts') ? [path] : [];
  });
}

const htmlPages = readdirSync(ROOT).filter((f) => f.endsWith('.html') && f !== 'index.html');
const hrefs = PAGES.map((p) => (typeof p.href === 'function' ? p.href() : p.href).split('?')[0]);

describe('Seiten & Navigation', () => {
  it.each(htmlPages)('%s ist in src/landing/pages.ts eingetragen', (page) => {
    expect(hrefs).toContain(page);
  });

  it('jede Kachel zeigt auf eine vorhandene Seite', () => {
    for (const href of hrefs) expect(htmlPages).toContain(href);
  });

  it.each(htmlPages)('%s ruft installPageChrome() auf', (page) => {
    const script = /<script type="module" src="\/([^"]+)"/.exec(read(page))?.[1];
    expect(script, `${page} braucht ein Modul-Script`).toBeDefined();
    expect(read(script!)).toMatch(/^installPageChrome\(/m);
  });
});

describe('Code-Regeln', () => {
  const files = sourceFiles('src');

  it('Vollbild nur über src/core/fullscreen.ts', () => {
    const offenders = files.filter(
      (f) => relative('src/core', f) !== 'fullscreen.ts' && /requestFullscreen\(|scale\.toggleFullscreen/.test(read(f)),
    );
    expect(offenders).toEqual([]);
  });

  it('kein Seitenwechsel per location (nur goHome() / Shell)', () => {
    const offenders = files.filter(
      (f) => !f.startsWith(join('src', 'core')) && !f.startsWith(join('src', 'landing')) && /location\.(href\s*=|assign|replace)/.test(read(f)),
    );
    expect(offenders).toEqual([]);
  });

  it('Spiel-Logik (src/world) ohne Math.random() und ohne Phaser', () => {
    for (const f of files.filter((f) => f.startsWith(join('src', 'world'))).concat('src/core/rng.ts')) {
      const code = read(f).replace(/\/\*[\s\S]*?\*\/|\/\/.*$/gm, '');
      expect(code, f).not.toMatch(/Math\.random\(/);
      expect(code, f).not.toMatch(/from 'phaser'/);
    }
  });
});
