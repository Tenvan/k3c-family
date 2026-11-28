// Paketprüfung (aus pruefen_test.go): Pakete als Dateiliste im Speicher; nur der Lauf auf der Platte schreibt
// unter .work/tmp.

import { mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { describe, expect, it } from 'vitest';
import { sandbox } from '../checkdiff/testrepo';
import { main } from './app';
import { pruefe, type Entry } from './pruefen';

const MIB = 1 << 20;
const LIZENZEN = 'Kopf\n\n==== a 1.0.0 (MIT) ====\ntext';

// Ein vollständiges Paket: Datei → Größe.
function fullPackage(): Map<string, number> {
  const files = [
    'k3c-game.exe', 'k3c-devwin.exe', 'README.txt', 'LICENSE', 'LICENSE-ASSETS', 'LIZENZEN.txt', 'data/biomes.json',
    'data/worlds/standard.json', 'public/grafik/figuren/index.paper.json', 'public/audio/README.md',
    'public/schrift/LICENSE', 'public/rig/rig.json',
  ];
  return new Map(files.map((f) => [f, 3]));
}

/** Dateiliste samt der Ordner, wie sie `listEntries` liefert. */
function entries(files: Map<string, number>): Entry[] {
  const dirs = new Set<string>();
  for (const file of files.keys()) {
    const parts = file.split('/');
    for (let i = 1; i < parts.length; i++) dirs.add(parts.slice(0, i).join('/'));
  }
  return [
    ...[...dirs].map((p) => ({ path: p, dir: true, size: 0 })),
    ...[...files].map(([p, size]) => ({ path: p, dir: false, size })),
  ];
}

const check = (files: Map<string, number>, maxExeBytes = 40 * MIB, lokal = false, lizenzen = LIZENZEN) =>
  pruefe(entries(files), { maxExeBytes, lokal, lizenzen: () => lizenzen });

describe('pruefe', () => {
  it('nimmt ein vollständiges Paket an', () => {
    expect(check(fullPackage())).toEqual({ mangel: [], n: 12 });
  });

  it('meldet fehlende Pflichtdateien und -ordner', () => {
    const p = fullPackage();
    for (const name of ['LIZENZEN.txt', 'public/audio/README.md', 'data/worlds/standard.json']) p.delete(name);
    const { mangel } = check(p);
    expect(mangel).toContain('Pflichtdatei fehlt: LIZENZEN.txt');
    expect(mangel).toContain('Pflichtordner leer oder fehlt: public/audio (erwartet *)');
    expect(mangel).toContain('Pflichtordner leer oder fehlt: data/worlds (erwartet *.json)');
  });

  it('meldet verbotene Dateien und Ordner', () => {
    const p = fullPackage();
    for (const name of ['data/x.go', 'assets.txt', 'saves/s1.json', 'data/README.md', 'LIZENZEN.txt.tmp', 'public/rig/go.mod']) {
      p.set(name, 1);
    }
    // Reihenfolge wie WalkDir; der Inhalt von saves/ wird nicht gezählt
    expect(check(p)).toEqual({
      mangel: [
        'verbotene Datei: LIZENZEN.txt.tmp (Endung .tmp)',
        'verbotene Datei: assets.txt (Entwickler-Datei)',
        'verbotene Datei: data/README.md (Doku unter data/)',
        'verbotene Datei: data/x.go (Endung .go)',
        'verbotene Datei: public/rig/go.mod (Entwickler-Datei)',
        'verbotener Ordner: saves',
      ],
      n: 17,
    });
  });

  it('meldet zu große EXE und Lizenzdatei ohne Crate', () => {
    const p = fullPackage().set('k3c-game.exe', 3 * MIB);
    const { mangel } = check(p, 2 * MIB, false, 'Kopf ohne Crate-Abschnitt');
    expect(mangel).toContain('k3c-game.exe zu groß: 3.0 MiB, erlaubt 2.0 MiB');
    expect(mangel).toContain('LIZENZEN.txt enthält keinen Crate-Abschnitt');
  });

  it('prüft die Fenster-EXE', () => {
    const p = fullPackage().set('k3c-devwin.exe', 6 * MIB);
    expect(check(p).mangel).toContain('k3c-devwin.exe zu groß: 6.0 MiB, erlaubt 5.0 MiB');
    p.delete('k3c-devwin.exe');
    expect(check(p).mangel).toContain('Pflichtdatei fehlt: k3c-devwin.exe');
  });

  // Im Entwicklungsordner (-lokal) sind Spielstände, Berichte und die Aufzeichnung nach dem Spielen kein Mangel.
  it('duldet mit -lokal die Laufzeit-Ordner', () => {
    const p = fullPackage();
    for (const name of ['saves/s1.json', 'reports/r.json', 'trace/messung-nacht.jsonl.gz']) p.set(name, 1);
    expect(check(p, 40 * MIB, true).mangel).toEqual([]);
    expect(check(p).mangel).toContain('verbotener Ordner: saves');
  });
});

describe('main pruefen', () => {
  const run = (args: string[]) => {
    let out = '';
    let err = '';
    const code = main(args, { out: (t) => void (out += t.toString()), err: (t) => void (err += t) });
    return { code, out, err };
  };

  it('meldet einen fehlenden Ordner', () => {
    expect(run(['pruefen', 'gibt-es-nicht'])).toEqual({
      code: 1,
      out: '',
      err: '❌ rustpaket: paketordner gibt-es-nicht fehlt (erst task rust:game:build)\n',
    });
  });

  it('liest einen Paketordner von der Platte', () => {
    const box = sandbox('rustpaket');
    try {
      const dir = path.join(box.root, 'paket');
      for (const [file] of fullPackage()) {
        mkdirSync(path.dirname(path.join(dir, file)), { recursive: true });
        writeFileSync(path.join(dir, file), file === 'LIZENZEN.txt' ? LIZENZEN : 'x');
      }
      expect(run(['pruefen', dir])).toEqual({ code: 0, out: `✅ Paket ${dir} vollständig (12 Dateien)\n`, err: '' });
      mkdirSync(path.join(dir, 'reports')); // auch ein leerer verbotener Ordner zählt
      expect(run(['pruefen', dir]).out).toBe('❌ verbotener Ordner: reports\n');
      expect(run(['pruefen', '-lokal', dir]).code).toBe(0);
    } finally {
      box.remove();
    }
  });

  it('lehnt falsche Aufrufe ab', () => {
    expect(run([]).code).toBe(2);
    expect(run(['anders']).code).toBe(2);
    expect(run(['pruefen']).err).toContain('genau ein Paketordner erwartet');
    expect(run(['pruefen', '-x', '.']).err).toContain('flag provided but not defined: -x');
    expect(run(['pruefen', '-max-exe-mib', 'viel', '.']).code).toBe(1);
    expect(run(['lizenzen'])).toEqual({ code: 1, out: '', err: '❌ rustpaket: -exe fehlt\n' });
  });
});
