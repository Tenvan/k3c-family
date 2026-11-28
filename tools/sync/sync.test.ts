// Kopierwerkzeug (aus main_test.go), auf Wegwerf-Ordnern unter .work/tmp.

import { existsSync, mkdirSync, readFileSync, statSync, utimesSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { sandbox, type Sandbox } from '../checkdiff/testrepo';
import { main, syncPath } from './sync';

const boxes: Sandbox[] = [];
afterEach(() => {
  for (const box of boxes.splice(0)) box.remove();
});

function tempDir(): string {
  const box = sandbox('sync');
  boxes.push(box);
  return box.root;
}

function write(file: string, text: string): void {
  mkdirSync(path.dirname(file), { recursive: true });
  writeFileSync(file, text);
}

function run(args: string[]): { code: number; out: string; err: string } {
  let out = '';
  let err = '';
  const code = main(args, { out: (t) => void (out += t), err: (t) => void (err += t) });
  return { code, out, err };
}

describe('syncPath', () => {
  // Ein Ordner wird ins vorhandene Ziel zusammengeführt; nur Geändertes wird kopiert, Fremdes im Ziel bleibt liegen.
  it('kopiert nur Geändertes', () => {
    const dir = tempDir();
    const src = path.join(dir, 'src');
    const dst = path.join(dir, 'dst');
    const png = path.join(src, 'a', 'idle.png');
    write(png, 'eins');
    write(path.join(src, 'b.json'), 'zwei');
    write(path.join(dst, 'alt.txt'), 'bleibt');
    expect(syncPath(src, dst)).toBe(2);
    expect(syncPath(src, dst)).toBe(0);
    write(png, 'neu!');
    const later = new Date(Date.now() + 60_000);
    utimesSync(png, later, later);
    expect(syncPath(src, dst)).toBe(1);
    expect(readFileSync(path.join(dst, 'a', 'idle.png'), 'utf8')).toBe('neu!');
    expect(existsSync(path.join(dst, 'alt.txt'))).toBe(true);
  });

  // Eine einzelne Datei als Quelle landet unter dem Zielnamen.
  it('nimmt eine einzelne Datei', () => {
    const dir = tempDir();
    const from = path.join(dir, 'x.exe');
    write(from, 'exe');
    expect(syncPath(from, path.join(dir, 'out', 'x.exe'))).toBe(1);
  });

  it('übernimmt die Änderungszeit und kopiert bei abweichender Zeit erneut', () => {
    const dir = tempDir();
    const from = path.join(dir, 'x');
    const to = path.join(dir, 'y');
    write(from, 'x');
    const time = new Date('2020-01-02T03:04:05.123Z');
    utimesSync(from, time, time);
    expect(syncPath(from, to)).toBe(1);
    expect(Math.abs(statSync(to).mtimeMs - statSync(from).mtimeMs)).toBeLessThan(0.01);
    const other = new Date(time.getTime() + 1000);
    utimesSync(to, other, other);
    expect(syncPath(from, to)).toBe(1);
  });
});

describe('main', () => {
  it('zählt und meldet über alle Argumente', () => {
    const dir = tempDir();
    write(path.join(dir, 'a'), 'a');
    write(path.join(dir, 'b'), 'b');
    const res = run([`${path.join(dir, 'a')}=${path.join(dir, 'out', 'a')}`, `${path.join(dir, 'b')}=${path.join(dir, 'out', 'b')}`]);
    expect(res).toEqual({ code: 0, out: '✅ 2 Dateien aktualisiert\n', err: '' });
  });

  it('bricht bei Argument ohne = und fehlender Quelle ab', () => {
    expect(run(['foo'])).toEqual({ code: 1, out: '', err: '❌ argument "foo": erwartet quelle=ziel\n' });
    const dir = tempDir();
    const res = run([`${path.join(dir, 'fehlt')}=${path.join(dir, 'x')}`]);
    expect(res.code).toBe(1);
    expect(res.err).toContain('fehlt');
    expect(existsSync(path.join(dir, 'x'))).toBe(false);
  });
});
