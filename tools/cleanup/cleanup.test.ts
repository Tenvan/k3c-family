// Branches und Worktrees (aus cleanup_test.go) gegen ein Wegwerf-Repo mit eigenem origin; nie gegen das echte Repo.

import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { commit, sandbox, type Sandbox } from '../checkdiff/testrepo';
import { main } from './app';
import { run } from './cleanup';
import { gitWith, type PrMerged } from './git';

const boxes: Sandbox[] = [];
afterEach(() => {
  for (const box of boxes.splice(0)) box.remove();
});

/** Branches ohne Upstream, per cherry-pick in main, und Branches, die nur hier stehen. */
function branches(box: Sandbox, dir: string): void {
  for (const n of ['sicher', 'sprint/xx1', 'wt-sauber', 'wt-schmutzig', 'aktuell']) {
    box.git(dir, 'checkout', '-q', '-b', n, 'main');
    commit(box, dir, `f-${n.replaceAll('/', '-')}`);
    box.git(dir, 'checkout', '-q', 'main');
    box.git(dir, 'cherry-pick', n);
  }
  box.git(dir, 'push', '-q', 'origin', 'main');
  for (const n of ['ungemergt', 'weg-gemergt', 'weg-offen']) {
    box.git(dir, 'checkout', '-q', '-b', n, 'main');
    commit(box, dir, `f-${n}`);
  }
  box.git(dir, 'checkout', '-q', 'main');
  for (const n of ['weg-gemergt', 'weg-offen']) {
    box.git(dir, 'push', '-q', '-u', 'origin', n);
    box.git(dir, 'push', '-q', 'origin', '--delete', n);
  }
  box.git(dir, 'fetch', '-q', '--prune');
}

/**
 * Ein Repo mit origin und je einem Branch pro Fall; der Checkout steht auf `aktuell`. `sicher` steckt per cherry-pick in
 * main, `ungemergt` fehlt dort, `weg-*` haben keinen Upstream mehr, `sprint/xx1` gehört zu einem aktiven Sprint.
 */
function fixture() {
  const box = sandbox('cleanup');
  boxes.push(box);
  const origin = path.join(box.root, 'origin.git');
  const dir = path.join(box.root, 'work');
  box.git(box.root, 'init', '-q', '--bare', '-b', 'main', origin);
  box.git(box.root, 'clone', '-q', origin, dir);
  box.git(dir, 'checkout', '-q', '-b', 'main');
  commit(box, dir, 'basis');
  branches(box, dir);
  mkdirSync(path.join(dir, 'docs', 'sprints', 'aktiv', 'XX1-test'), { recursive: true });
  const wt = (name: string) => path.join(box.root, name);
  const f = { box, dir, clean: wt('wt-sauber'), dirty: wt('wt-schmutzig'), detached: wt('wt-detached'), old: wt('wt-detached-alt') };
  box.git(dir, 'worktree', 'add', '-q', f.clean, 'wt-sauber');
  box.git(dir, 'worktree', 'add', '-q', f.dirty, 'wt-schmutzig');
  writeFileSync(path.join(f.dirty, 'lose.txt'), 'x');
  box.git(dir, 'worktree', 'add', '-q', '--detach', f.detached, 'main');
  box.git(dir, 'worktree', 'add', '-q', '--detach', f.old, 'ungemergt');
  box.git(dir, 'checkout', '-q', 'aktuell');
  // Sicherung: der Lauf trifft nur das Wegwerf-Repo.
  expect(path.resolve(box.git(dir, 'rev-parse', '--show-toplevel'))).toBe(dir);
  return f;
}

type Fixture = ReturnType<typeof fixture>;

const stub = (...merged: string[]): PrMerged => (branch) => merged.includes(branch);

function clean(f: Fixture, dry: boolean, prMerged: PrMerged): string {
  let out = '';
  run({ root: f.dir, dry, git: gitWith(f.box.env), prMerged, out: (text) => void (out += text) });
  return out;
}

const has = (f: Fixture, name: string): boolean => f.box.git(f.dir, 'branch', '--list', name) !== '';

const state = (f: Fixture): string =>
  [f.box.git(f.dir, 'for-each-ref'), f.box.git(f.dir, 'worktree', 'list', '--porcelain')].join('\n');

describe('run', { timeout: 60_000 }, () => {
  it('löscht nur sichere Fälle', () => {
    const f = fixture();
    const origin = f.box.git(f.dir, 'ls-remote', 'origin');
    const out = clean(f, false, stub('weg-gemergt'));
    for (const n of ['sicher', 'weg-gemergt', 'wt-sauber']) expect(has(f, n), `${n}\n${out}`).toBe(false);
    for (const n of ['main', 'aktuell', 'sprint/xx1', 'ungemergt', 'weg-offen', 'wt-schmutzig']) {
      expect(has(f, n), `${n}\n${out}`).toBe(true);
    }
    expect(existsSync(f.clean)).toBe(false);
    expect(existsSync(path.join(f.dirty, 'lose.txt'))).toBe(true);
    expect(existsSync(f.detached)).toBe(false);
    expect(existsSync(f.old)).toBe(true);
    expect(f.box.git(f.dir, 'ls-remote', 'origin')).toBe(origin);
    expect(out).toMatch(/\naufraeumen: 4 gelöscht, \d+ bleiben\n/);
  });

  it('--dry ändert nichts', () => {
    const f = fixture();
    mkdirSync(path.join(f.dir, 'bin'));
    writeFileSync(path.join(f.dir, 'bin', 'k3c-server.exe'), 'x');
    const before = state(f);
    const out = clean(f, true, stub('weg-gemergt'));
    expect(state(f)).toBe(before);
    expect(existsSync(path.join(f.dir, 'bin', 'k3c-server.exe'))).toBe(true);
    for (const want of [
      'würde löschen  Branch sicher – ',
      'würde löschen  Branch weg-gemergt – Upstream weg, Pull Request gemergt',
      'bleibt   Branch ungemergt – nicht gemergt, 1 Commits',
      'bleibt   Branch sprint/xx1 – Branch eines aktiven Sprints',
      'hat Änderungen oder ignorierte Dateien',
      'aufraeumen: 4 würden gelöscht, ',
      '🧹 bin/k3c-server.exe (0.0 MiB) würde gelöscht (--dry)',
    ]) {
      expect(out).toContain(want);
    }
  });

  it('ohne gh nur die per cherry sicheren', () => {
    const f = fixture();
    const out = clean(f, false, () => {
      throw new Error('gh fehlt');
    });
    expect(has(f, 'sicher')).toBe(false);
    for (const n of ['weg-gemergt', 'weg-offen']) expect(has(f, n)).toBe(true);
    expect(out).toContain('PR-Status unbekannt: gh fehlt');
  });

  it('räumt aus einem Worktree das bin/ der Repo-Wurzel und lässt den eigenen stehen', () => {
    const f = fixture();
    for (const root of [f.dir, f.dirty]) {
      mkdirSync(path.join(root, 'bin'));
      writeFileSync(path.join(root, 'bin', 'k3c-sim.exe'), 'x');
    }
    clean({ ...f, dir: f.dirty }, false, stub());
    expect(existsSync(path.join(f.dir, 'bin', 'k3c-sim.exe'))).toBe(false);
    expect(existsSync(path.join(f.dirty, 'bin', 'k3c-sim.exe'))).toBe(true);
    expect(has(f, 'wt-schmutzig')).toBe(true);
  });

  it('ein Branch, dessen Name mit Strich beginnt, bleibt', () => {
    const f = fixture();
    f.box.git(f.dir, 'update-ref', 'refs/heads/-v', 'ungemergt');
    const out = clean(f, false, stub('-v'));
    expect(out).toContain('bleibt   Branch -v – Name beginnt mit einem Strich');
    expect(f.box.git(f.dir, 'for-each-ref', 'refs/heads/-v')).not.toBe('');
  });
});

describe('main', { timeout: 60_000 }, () => {
  function call(argv: string[], env: NodeJS.ProcessEnv): { code: number; out: string; err: string } {
    let out = '';
    let err = '';
    const code = main(argv, env, { out: (t) => void (out += t), err: (t) => void (err += t) });
    return { code, out, err };
  }

  it('--dry über die Flags ändert nichts', () => {
    const f = fixture();
    const before = state(f);
    const got = call(['-root', f.dir, '--dry'], f.box.env);
    expect(got.code).toBe(0);
    expect(got.out).toContain('würde löschen  Branch sicher');
    expect(state(f)).toBe(before);
  });

  it('lose Argumente und unbekannte Flags brechen ab, bevor etwas geschieht', () => {
    const f = fixture();
    const before = state(f);
    for (const argv of [['-root', f.dir, 'dry'], ['-root', f.dir, '--dry-run'], ['-root', f.dir, '-dry=ja']]) {
      expect(call(argv, f.box.env).code, argv.join(' ')).toBe(2);
    }
    expect(state(f)).toBe(before);
  });

  it('kein Repo: Meldung und Exit 1, ohne in das umgebende Repo zu greifen', () => {
    const box = sandbox('cleanup-leer');
    boxes.push(box);
    const got = call(['-root', box.root], box.env);
    expect(got).toEqual({ code: 1, out: '', err: 'aufraeumen: weder origin/main noch main gefunden\n' });
  });
});
