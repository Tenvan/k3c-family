// Prüft die Projektregeln aus CLAUDE.md, damit die CI Verstöße früh meldet.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { dirname, join, relative, resolve, sep } from 'node:path';
import { describe, expect, it } from 'vitest';
import { PAGES } from '../src/landing/pages';

const ROOT = resolve(__dirname, '..');
const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');

/** Alle Dateien unter `dir`, deren Name `keep` erfüllt; fehlt der Ordner, keine. */
function filesIn(dir: string, keep: (name: string) => boolean): string[] {
  if (!existsSync(join(ROOT, dir))) return [];
  return readdirSync(join(ROOT, dir)).flatMap((name) => {
    const path = join(dir, name);
    if (statSync(join(ROOT, path)).isDirectory()) return filesIn(path, keep);
    return keep(name) ? [path] : [];
  });
}
const sourceFiles = (dir: string) => filesIn(dir, (n) => n.endsWith('.ts') && !n.endsWith('.test.ts'));

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

describe('Komplexität & Schichten', () => {
  const TARGET = 300; // Ziel aus docs/arbeitsweise.md › Komplexitäts-Budget, darüber nur mit Baseline-Eintrag
  const HARD = 400; // harte Grenze; für TypeScript prüft sie Oxlint, für Go dieser Test
  const BASELINE = 'tests/complexity-baseline.json';
  const baseline: Record<string, number> = JSON.parse(read(BASELINE));
  const posix = (path: string) => path.split(sep).join('/');
  const lines = (file: string) => (read(file).match(/\n/g) ?? []).length; // wie `wc -l`
  const goDirs = ['data', 'engine', 'cmd'];
  const goFiles = goDirs.flatMap((d) => filesIn(d, (n) => n.endsWith('.go'))).map(posix);
  const codeFiles = sourceFiles('src').map(posix).concat(goFiles.filter((f) => !f.endsWith('_test.go')));

  it.each(codeFiles)('%s liegt im Ziel oder in der Baseline (Ratsche)', (file) => {
    const n = lines(file);
    if (n <= TARGET) return;
    expect(baseline[file], `${file}: ${n} Zeilen > ${TARGET}. Verkleinern; ein neuer Baseline-Eintrag nur mit Zustimmung im Review`)
      .toBeGreaterThanOrEqual(n);
  });

  it.each(Object.keys(baseline))('Baseline-Eintrag %s ist noch nötig', (file) => {
    const needed = existsSync(join(ROOT, file)) && lines(file) > TARGET;
    expect(needed, `Eintrag aus ${BASELINE} entfernen: ${file} liegt im Ziel oder existiert nicht mehr`).toBe(true);
  });

  it('keine Go-Datei über der harten Grenze, auch keine Test-Datei', () => {
    expect(goFiles.filter((f) => lines(f) > HARD)).toEqual([]);
  });

  it('src/world importiert nichts aus scenes/, online/ oder input/', () => {
    const forbidden = ['scenes', 'online', 'input'].map((d) => join(ROOT, 'src', d) + sep);
    const offenders = sourceFiles(join('src', 'world')).flatMap((file) =>
      [...read(file).matchAll(/(?:from|import)\s+'(\.[^']*)'/g)]
        .map((m) => resolve(ROOT, dirname(file), m[1]))
        .filter((target) => forbidden.some((dir) => (target + sep).startsWith(dir)))
        .map((target) => `${posix(file)} → ${posix(relative(ROOT, target))}`),
    );
    expect(offenders).toEqual([]);
  });
});
