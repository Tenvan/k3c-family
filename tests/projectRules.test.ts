// Prüft die Projektregeln aus CLAUDE.md, damit die CI Verstöße früh meldet.
import { existsSync, readdirSync, readFileSync, statSync } from 'node:fs';
import { join, resolve, sep } from 'node:path';
import { describe, expect, it } from 'vitest';

const ROOT = resolve(__dirname, '..');
const read = (file: string) => readFileSync(join(ROOT, file), 'utf8');

/** Alle Dateien unter `dir`, deren Name `keep` erfüllt; fehlt der Ordner, keine. */
function filesIn(dir: string, keep: (name: string) => boolean): string[] {
  if (!existsSync(join(ROOT, dir))) return [];
  return readdirSync(join(ROOT, dir)).flatMap((name) => {
    const path = join(dir, name);
    if (name === 'node_modules') return []; // z. B. tools/k3c-workbench/frontend
    if (statSync(join(ROOT, path)).isDirectory()) return filesIn(path, keep);
    return keep(name) ? [path] : [];
  });
}

// Browser-Client und Spielserver sind entfernt (B-848, B-887, Entscheidung B-850): Das Spiel ist nativ (Rust, rust/crates/k3c-game); das Go-Spiel native/ ist seit RU37 entfernt, der letzte Stand steht im Tag go-referenz-final.
describe('Browser-Client und Spielserver entfernt (B-848, B-887)', () => {
  it('keine Spielseiten, kein src/, keine Vite-Konfiguration des Spiels, kein Go-Spiel native/ (RU37.2)', () => {
    expect(readdirSync(ROOT).filter((f) => f.endsWith('.html'))).toEqual([]);
    for (const gone of ['src', 'vite.config.ts', 'Dockerfile', 'compose.yaml', 'cmd/k3c-server', 'cmd/k3c-load', 'cmd/k3c-tui',
      'engine/net', 'engine/watch', 'tools/k3c-starter', 'native']) {
      expect(existsSync(join(ROOT, gone)), gone).toBe(false);
    }
  });

  it('package.json hängt nicht mehr an Phaser', () => {
    expect(read('package.json')).not.toContain('phaser');
  });

  it('Taskfile.yml kennt keine Web- und Server-Tasks mehr', () => {
    const tasks = [...read('Taskfile.yml').matchAll(/^ {2}([\w:-]+):\s*$/gm)].map((m) => m[1]);
    for (const gone of ['dev', 'preview', 'serve', 'start', 'check:starter']) expect(tasks, gone).not.toContain(gone);
    expect(read('Taskfile.yml')).not.toMatch(/k3c-server|k3c-load|k3c-tui|k3c-starter|vite (build|preview)|bunx vite\b/);
  });
});

describe('Komplexität & Schichten', () => {
  const HARD = 400; // Grenze aus docs/arbeitsweise.md › Komplexitäts-Budget; für TypeScript prüft sie Oxlint, für Go dieser Test
  const posix = (path: string) => path.split(sep).join('/');
  const lines = (file: string) => (read(file).match(/\n/g) ?? []).length; // wie `wc -l`
  const goDirs = ['data', 'engine', 'cmd', 'tools'];
  const goFiles = goDirs.flatMap((d) => filesIn(d, (n) => n.endsWith('.go'))).map(posix);

  it('keine Go-Datei über der harten Grenze, auch keine Test-Datei', () => {
    expect(goFiles.filter((f) => lines(f) > HARD)).toEqual([]);
  });

  // Rust-Workspace (RSU, RU1.3): nur rust/crates; rust/target (Build) und rust/spike (Prototyp) zählen nicht. Die Funktionslänge prüft clippy.
  it('keine Rust-Datei unter rust/crates über der harten Grenze, auch keine Test-Datei', () => {
    const rustFiles = filesIn('rust/crates', (n) => n.endsWith('.rs')).map(posix);
    expect(rustFiles.filter((f) => lines(f) > HARD)).toEqual([]);
  });

  // 16 MB (B-417, Ralf 2026-10-13); die großen Läufe liegen seit GD1 gepackt (B-624, B-620).
  it('Golden-Daten gesamt höchstens 16 MB (B-417, GD1)', () => {
    const bytes = filesIn('testdata/golden', () => true).reduce((sum, f) => sum + statSync(join(ROOT, f)).size, 0);
    expect(bytes).toBeLessThanOrEqual(16 * 1024 * 1024);
  });
});

describe('Regeltabelle Hub-Stufe → Gebäude (WZ15.2, B-462)', () => {
  const doc = read('docs/rules/materialien-gebaeude.md');
  const from = doc.indexOf('## 3. Gebäude je Hub-Stufe');
  const rules = doc.slice(from, doc.indexOf('\n## ', from + 1)); // nur § 3
  const count = (needle: string) => rules.split(needle).length - 1;

  it('§ 3 trägt die Marken des erzeugten Blocks je genau einmal, Start vor Ende', () => {
    expect(count('<!-- rules:hub-levels:start -->')).toBe(1);
    expect(count('<!-- rules:hub-levels:end -->')).toBe(1);
    expect(rules.indexOf('<!-- rules:hub-levels:start -->')).toBeLessThan(rules.indexOf('<!-- rules:hub-levels:end -->'));
  });
});

// Das alte Asset-Studio ist entfallen (Entscheidung 005 › 5, WB1/AC-04): seine Tasks und Ports kommen nicht zurück.
// Das neue k3c-studio (WZ63) hat eigene Ports (MCP 5181, dev 5182) und darf in Taskfile und services.json stehen.
describe('Altes Asset-Studio entfernt (005/AC-10, WB1)', () => {
  const STUDIO = /check:studio|\b(8091|8182|5192)\b/;
  const FILES = ['Taskfile.yml', '.github/workflows/ci.yml', '.claude/launch.json', '.oxlintrc.json',
    'tools/k3c-workbench/services.json'];

  it.each(FILES)('%s nennt weder Studio-Tasks noch Studio-Ports', (file) => {
    const hits = read(file).split(/\r?\n/).filter((line) => STUDIO.test(line));
    expect(hits).toEqual([]);
  });

  it('K3C_STUDIO_LAN kommt im Code nicht mehr vor', () => {
    const code = ['engine', 'cmd', 'tools'].flatMap((d) => filesIn(d, (n) => /\.(ts|tsx|go)$/.test(n)));
    expect(code.filter((f) => read(f).includes('K3C_STUDIO_LAN'))).toEqual([]);
  });
});
