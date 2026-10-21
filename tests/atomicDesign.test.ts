// Atomic Design der Wails-Oberflächen (docs/arbeitsweise.md › Oberflächen der Wails-Programme): Ebenen in tools/webkit,
// Importrichtung, Radix nur über webkit, rohe Controls nur in den Atomen. Gilt für jedes Frontend unter tools/*/frontend.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { dirname, join, relative, resolve, sep } from 'node:path';
import { describe, expect, it } from 'vitest';

const ROOT = resolve(__dirname, '..');
const posix = (p: string) => p.split(sep).join('/');
const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');

/** Code-Dateien unter dir (ohne Tests, node_modules, Build-Ausgaben und Wails-Bindungen). */
function codeFiles(dir: string): string[] {
  if (!existsSync(join(ROOT, dir))) return [];
  return readdirSync(join(ROOT, dir)).flatMap((name) => {
    const path = posix(join(dir, name));
    if (['node_modules', 'dist', 'wailsjs'].includes(name)) return [];
    if (statSync(join(ROOT, path)).isDirectory()) return codeFiles(path);
    return /\.tsx?$/.test(name) && !/\.test\.tsx?$/.test(name) ? [path] : [];
  });
}

const WEBKIT = 'tools/webkit';
/** Ebenen von unten nach oben; lib hat keine Oberfläche, testing nur Test-Helfer. */
const LEVELS = ['lib', 'atoms', 'molecules', 'organisms', 'templates'] as const;
const APPS = readdirSync(join(ROOT, 'tools'))
  .map((d) => `tools/${d}/frontend/src`)
  .filter((d) => existsSync(join(ROOT, d)));
const webkitFiles = codeFiles(WEBKIT).filter((f) => !f.startsWith(`${WEBKIT}/testing/`));
const appFiles = APPS.flatMap(codeFiles);

/** Alle Modul-Angaben einer Datei (import/export … from, import '…', import('…')). */
function specifiers(file: string): string[] {
  const src = read(file);
  const re = /(?:from\s+|import\s+|import\(\s*)['"]([^'"]+)['"]/g;
  return [...src.matchAll(re)].map((m) => m[1]!);
}

/** Ziel einer webkit-internen Angabe als Pfad unter tools/webkit, sonst null. */
function webkitTarget(file: string, spec: string): string | null {
  if (spec.startsWith('@webkit/')) return `${WEBKIT}/${spec.slice('@webkit/'.length)}`;
  if (spec.startsWith('.')) return posix(relative(ROOT, resolve(ROOT, dirname(file), spec)));
  return null;
}

const levelOf = (path: string) => LEVELS.indexOf(path.split('/')[2] as (typeof LEVELS)[number]);

describe('Atomic Design: tools/webkit', () => {
  it('besteht nur aus den Ebenen, lib, testing, theme.css und codemap.md', () => {
    const allowed = [...LEVELS, 'testing', 'theme.css', 'codemap.md'];
    // Punkt-Ordner (z. B. .omc) legen Werkzeuge an, sie gehören nicht zum Code.
    expect(readdirSync(join(ROOT, WEBKIT)).filter((n) => !n.startsWith('.') && !allowed.includes(n))).toEqual([]);
  });

  it('jede Oberflächen-Ebene hat ihr index.ts', () => {
    expect(LEVELS.slice(1).filter((l) => !existsSync(join(ROOT, WEBKIT, l, 'index.ts')))).toEqual([]);
  });

  it('importiert nur nach unten (lib ← atoms ← molecules ← organisms ← templates) und nichts außerhalb', () => {
    const bad = webkitFiles.flatMap((f) =>
      specifiers(f).flatMap((s) => {
        const target = webkitTarget(f, s);
        if (target === null) return [];
        if (!target.startsWith(`${WEBKIT}/`)) return [`${f} → ${s} (außerhalb von webkit)`];
        const to = levelOf(target);
        const from = levelOf(f);
        return to < 0 || to > from || (from === 0 && to > 0) ? [`${f} → ${s}`] : [];
      }),
    );
    expect(bad).toEqual([]);
  });

  it('Radix Themes nur in den Oberflächen-Ebenen', () => {
    const bad = webkitFiles.filter((f) => levelOf(f) < 1 && specifiers(f).includes('@radix-ui/themes'));
    expect(bad).toEqual([]);
  });
});

describe('Atomic Design: Frontends der Wails-Programme', () => {
  it('es gibt die drei Frontends', () => {
    expect(APPS.map((a) => a.split('/')[1])).toEqual(expect.arrayContaining(['k3c-dev', 'k3c-starter', 'k3c-studio']));
  });

  it('rohe <button>, <select>, <input>, <textarea> nur in tools/webkit/atoms', () => {
    const raw = /<(button|select|input|textarea)[\s>/]/;
    // Kommentare zählen nicht (z. B. „statt `<select>`“); `://` in URLs bleibt stehen.
    const code = (f: string) => read(f).replace(/\/\*[\s\S]*?\*\/|(^|[^:])\/\/.*$/gm, '$1');
    const bad = [...appFiles, ...webkitFiles].filter((f) => f.endsWith('.tsx') && !f.startsWith(`${WEBKIT}/atoms/`) && raw.test(code(f)));
    expect(bad).toEqual([]);
  });

  it('Radix nur über @webkit (außer styles.css)', () => {
    const radix = (s: string) => s.startsWith('@radix-ui/') && !s.endsWith('.css');
    expect(appFiles.filter((f) => specifiers(f).some(radix))).toEqual([]);
  });

  it('@webkit nur über die Ebenen-Einstiege, lib/* und theme.css', () => {
    const ok = /^@webkit\/((atoms|molecules|organisms|templates)|lib\/[\w-]+|theme\.css)$/;
    const bad = appFiles.flatMap((f) => specifiers(f).filter((s) => s.startsWith('@webkit') && !ok.test(s)).map((s) => `${f} → ${s}`));
    expect(bad).toEqual([]);
  });

  it('src/ui hat nur molecules, organisms, templates (Atome liegen in webkit)', () => {
    const bad = APPS.flatMap((a) => {
      const ui = join(ROOT, a, 'ui');
      if (!existsSync(ui)) return [];
      return readdirSync(ui).filter((n) => !['molecules', 'organisms', 'templates', 'codemap.md'].includes(n)).map((n) => `${a}/ui/${n}`);
    });
    expect(bad).toEqual([]);
  });

  it('src/ui importiert nur nach unten und keine Seiten (Feature-Ordner)', () => {
    const UI = ['molecules', 'organisms', 'templates'];
    const bad = APPS.flatMap((a) =>
      appFiles
        .filter((f) => f.startsWith(`${a}/ui/`))
        .flatMap((f) =>
          specifiers(f)
            .filter((s) => s.startsWith('.'))
            .filter((s) => {
              const [top, level] = posix(relative(join(ROOT, a), resolve(ROOT, dirname(f), s))).split('/');
              if (top !== 'ui') return !['api', 'lib', '..'].includes(top!); // '..': außerhalb von src, z. B. wailsjs
              return UI.indexOf(level!) > UI.indexOf(f.split('/')[5]!);
            })
            .map((s) => `${f} → ${s}`),
        ),
    );
    expect(bad).toEqual([]);
  });
});
