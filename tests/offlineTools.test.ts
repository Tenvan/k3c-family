import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative, resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

// WZ28.7/AC-07 (B-595/AC-04, B-596/AC-05): Die Oberflächen von Workbench, Studio und Starter samt tools/webkit laden zur
// Laufzeit nichts aus dem Netz – keine Skripte, Stylesheets, Schriften oder Abrufe von fremden Adressen. Erlaubt sind
// Loopback-Adressen und reine Texte (Links in Mock-Daten, SVG-Namensraum).

const ROOT = resolve(__dirname, '..');
const DIRS = ['tools/k3c-workbench/frontend', 'tools/k3c-studio/frontend', 'tools/k3c-starter/frontend', 'tools/webkit'];
const SKIP = new Set(['node_modules', 'dist', 'wailsjs']);

function files(dir: string): string[] {
  if (!existsSync(join(ROOT, dir))) return [];
  return readdirSync(join(ROOT, dir)).flatMap((name) => {
    const path = `${dir}/${name}`;
    if (SKIP.has(name)) return [];
    if (statSync(join(ROOT, path)).isDirectory()) return files(path);
    return /\.(tsx?|css|html)$/.test(name) && !/\.test\.tsx?$/.test(name) ? [path] : [];
  });
}

const URL = String.raw`["'(]\s*(https?:)?//(?!localhost|127\.0\.0\.1|\[::1\])[^"')\s]+`;
/** Stellen, an denen eine Adresse geladen würde: <script src>, <link href>, @import, url(…), fetch, import from. */
const LOADS = [
  new RegExp(String.raw`<script[^>]*\ssrc=` + URL, 'i'),
  new RegExp(String.raw`<link[^>]*\shref=` + URL, 'i'),
  new RegExp(String.raw`@import\s+(url\()?` + URL.slice(6), 'i'),
  new RegExp(String.raw`url\(\s*["']?(https?:)?//(?!localhost|127\.0\.0\.1)`, 'i'),
  new RegExp(String.raw`fetch\(\s*["'\x60](https?:)?//(?!localhost|127\.0\.0\.1)`, 'i'),
  new RegExp(String.raw`(import|from)\s*\(?\s*["']https?://`, 'i'),
];

describe('Offline-Werkzeuge', () => {
  it('laden keine fremden Adressen', () => {
    const all = DIRS.flatMap(files);
    expect(all.length).toBeGreaterThan(50);
    const hits = all.flatMap((f) =>
      readFileSync(join(ROOT, f), 'utf8')
        .split('\n')
        .flatMap((line, i) => (LOADS.some((re) => re.test(line)) ? [`${relative(ROOT, join(ROOT, f))}:${i + 1}: ${line.trim()}`] : [])),
    );
    expect(hits).toEqual([]);
  });

  it('erkennt eine fremde Schrift', () => {
    const lines = ['<link href="https://fonts.googleapis.com/css2?family=Inter" rel="stylesheet">', "@import url('https://cdn.example.org/x.css');", 'fetch("https://api.example.org/x")'];
    for (const l of lines) expect(LOADS.some((re) => re.test(l)), l).toBe(true);
    expect(LOADS.some((re) => re.test('fetch("http://127.0.0.1:8191/api")'))).toBe(false);
  });
});
