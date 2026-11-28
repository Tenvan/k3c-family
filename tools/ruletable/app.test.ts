// Lauf des Regeltabellen-Generators (aus main_test.go), auf Kopien unter .work/tmp.

import { mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { afterEach, describe, expect, it } from 'vitest';
import { sandbox, type Sandbox } from '../checkdiff/testrepo';
import { main, run } from './app';

const REPO = fileURLToPath(new URL('../../', import.meta.url));
const DOC = 'docs/rules/materialien-gebaeude.md';

const boxes: Sandbox[] = [];
afterEach(() => {
  for (const box of boxes.splice(0)) box.remove();
});

function copy(change: (doc: string) => string = (doc) => doc): string {
  const box = sandbox('ruletable');
  boxes.push(box);
  for (const rel of ['data/hub.json', DOC]) {
    const text = readFileSync(path.join(REPO, rel), 'utf8');
    mkdirSync(path.dirname(path.join(box.root, rel)), { recursive: true });
    writeFileSync(path.join(box.root, rel), rel === DOC ? change(text) : text);
  }
  return box.root;
}

describe('run', () => {
  // WZ15.2: Der Task ändert die Regeln auf dem heutigen Stand nicht, geprüft auf einer Kopie.
  it('ist auf einer Kopie des heutigen Stands stabil', () => {
    expect(run(copy())).toBe(false);
  });

  // WZ15.2: Weicht der Block ab, schreibt der Lauf ihn atomar zurück und lässt keine temporäre Datei liegen.
  it('schreibt einen abweichenden Block zurück', () => {
    const dir = copy((doc) => doc.replace('Werkstatt −16', 'Werkstatt −17'));
    expect(run(dir)).toBe(true);
    expect(run(dir)).toBe(false);
    expect(readdirSync(path.join(dir, 'docs', 'rules')).filter((f) => f.endsWith('.tmp'))).toEqual([]);
    expect(readFileSync(path.join(dir, DOC), 'utf8')).toBe(readFileSync(path.join(REPO, DOC), 'utf8'));
  });
});

describe('main', () => {
  const call = (args: string[]) => {
    let out = '';
    let err = '';
    const code = main(args, { out: (t) => void (out += t), err: (t) => void (err += t) });
    return { code, out, err };
  };

  it('meldet Änderung, Stand und Fehler', () => {
    const dir = copy((doc) => doc.replace('Werkstatt −16', 'Werkstatt −17'));
    expect(call(['-root', dir]).out).toBe('✅ regeltabelle: Block aktualisiert\n');
    expect(call([`-root=${dir}`]).out).toBe('✅ regeltabelle: Block ist aktuell\n');
    writeFileSync(path.join(dir, DOC), 'ohne Marken');
    const res = call(['-root', dir]);
    expect(res.code).toBe(1);
    expect(res.err).toContain('❌ regeltabelle: ');
    expect(res.err).toContain('müssen je genau einmal vorkommen');
    expect(call(['-x']).code).toBe(2);
    expect(call(['-h']).code).toBe(0);
  });
});
