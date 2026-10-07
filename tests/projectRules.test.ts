// Prüft die Projektregeln aus CLAUDE.md, damit die CI Verstöße früh meldet.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, resolve, sep } from 'node:path';
import { describe, expect, it } from 'vitest';
import { PAGES } from '../src/landing/pages';
import { DEV_TILES } from '../src/tools/devTiles';

const ROOT = resolve(__dirname, '..');
const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');

/** Alle Dateien unter `dir`, deren Name `keep` erfüllt; fehlt der Ordner, keine. */
function filesIn(dir: string, keep: (name: string) => boolean): string[] {
  if (!existsSync(join(ROOT, dir))) return [];
  return readdirSync(join(ROOT, dir)).flatMap((name) => {
    const path = join(dir, name);
    if (name === 'node_modules') return []; // z. B. tools/k3c-dev/frontend
    if (statSync(join(ROOT, path)).isDirectory()) return filesIn(path, keep);
    return keep(name) ? [path] : [];
  });
}
const sourceFiles = (dir: string) => filesIn(dir, (n) => n.endsWith('.ts') && !n.endsWith('.test.ts'));

// dm.html läuft außerhalb der Shell, Aufruf über /dm (B-232) oder die Entwicklerseite (neues Fenster, B-335).
const allHtml = readdirSync(ROOT).filter((f) => f.endsWith('.html'));
const htmlPages = allHtml.filter((f) => f !== 'index.html' && f !== 'dm.html');
// Eine Seite steht auf der Landingpage (Spieler) oder auf der Entwicklerseite (Werkzeuge, B-335).
const hrefs = [...PAGES.map((p) => (typeof p.href === 'function' ? p.href() : p.href)), ...DEV_TILES.map((t) => t.href)].map((h) => h.split('?')[0]);

describe('Seiten & Navigation', () => {
  it.each(htmlPages)('%s ist in src/landing/pages.ts oder src/tools/devTiles.ts eingetragen', (page) => {
    expect(hrefs).toContain(page);
  });

  it('jede Kachel zeigt auf eine vorhandene Seite', () => {
    for (const href of hrefs) expect(allHtml).toContain(href);
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
});

describe('Komplexität & Schichten', () => {
  const HARD = 400; // Grenze aus docs/arbeitsweise.md › Komplexitäts-Budget; für TypeScript prüft sie Oxlint, für Go dieser Test
  const posix = (path: string) => path.split(sep).join('/');
  const lines = (file: string) => (read(file).match(/\n/g) ?? []).length; // wie `wc -l`
  const goDirs = ['data', 'engine', 'cmd', 'tools', 'tests'];
  const goFiles = goDirs.flatMap((d) => filesIn(d, (n) => n.endsWith('.go'))).map(posix);

  it('keine Go-Datei über der harten Grenze, auch keine Test-Datei', () => {
    expect(goFiles.filter((f) => lines(f) > HARD)).toEqual([]);
  });
});
