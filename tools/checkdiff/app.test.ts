// Aufruf und Git-Teil: Flags, -only, -plan und die Basis gegen ein Wegwerf-Repo.

import { existsSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { afterAll, beforeAll, describe, expect, it, vi } from 'vitest';
import { main } from './app';
import { FlagError, parseFlags } from './flags';
import { changedFiles, gitLinesIn, type GitLines } from './repo';
import { commit, sandbox } from './testrepo';

const box = sandbox('checkdiff-app');
const dir = path.join(box.root, 'work');
const git = gitLinesIn(dir, box.env);

beforeAll(() => {
  box.git(box.root, 'init', '-q', '-b', 'main', dir);
  commit(box, dir, 'basis');
  box.git(dir, 'checkout', '-q', '-b', 'zweig');
  commit(box, dir, 'docs/a.md');
  commit(box, dir, 'rust/crates/k3c-sim/src/lib.rs');
});
afterAll(() => box.remove());

async function run(argv: string[], env: NodeJS.ProcessEnv = {}, lines: GitLines = git): Promise<{ code: number; out: string }> {
  let out = '';
  const code = await main(argv, env, (d) => void (out += d.toString()), lines);
  return { code, out };
}

describe('changedFiles', () => {
  it('sauberer Checkout: der letzte Commit', () => {
    const got = changedFiles('', git);
    expect(got.files).toEqual(['rust/crates/k3c-sim/src/lib.rs']);
    expect(got.base).toBe(box.git(dir, 'rev-parse', 'HEAD~1'));
  });

  it('Vorgabe: ab der gemeinsamen Basis', () => {
    expect(changedFiles('main', git).files).toEqual(['docs/a.md', 'rust/crates/k3c-sim/src/lib.rs']);
  });

  it('geänderte und neue Dateien gegen HEAD', () => {
    writeFileSync(path.join(dir, 'basis'), 'neu');
    writeFileSync(path.join(dir, 'lose ü.txt'), 'x');
    try {
      const got = changedFiles('', git);
      expect(got.files).toEqual(['basis', 'lose ü.txt']);
      expect(got.base).toBe(box.git(dir, 'rev-parse', 'HEAD'));
    } finally {
      box.git(dir, 'checkout', '-q', '--', 'basis');
      box.git(dir, 'clean', '-q', '-f');
    }
  });

  it('eine Basis mit Strich erreicht git nie als Option', () => {
    const target = path.join(box.root, 'geschrieben');
    expect(() => changedFiles(`--output=${target}`, git)).toThrow('beginnt mit einem Strich');
    expect(existsSync(target)).toBe(false);
  });
});

describe('main', () => {
  it('-plan zeigt Basis und Schritte und führt nichts aus', async () => {
    const { code, out } = await run(['-plan', '--base=main']);
    expect(code).toBe(0);
    expect(out).toMatch(/^🚦 check:diff Basis [0-9a-f]{40} · 4 Schritte\n {3}Stufe 1 · regeln: bunx vitest run /);
    expect(out).toContain('   Stufe 2 · rust-test: cargo test -p k3c-sim (in rust)\n');
    expect(out).not.toContain('⏱');
  });

  it('CHECK_BASE gilt, -base geht vor, -only wählt die Bahn', async () => {
    const env = await run(['-plan', '-only', 'js'], { CHECK_BASE: 'main' });
    expect(env.out).toContain('· 1 Schritte\n');
    const flag = await run(['-plan', '-only=rust', '-base', 'HEAD~1'], { CHECK_BASE: 'gibtsnicht' });
    expect(flag.out).toContain('· 3 Schritte\n');
  });

  it.each(['go', 'jss', 'Rust'])('-only %s wird mit den gültigen Werten abgelehnt', async (lane) => {
    const lines = vi.fn(git);
    const { code, out } = await run(['-only', lane], {}, lines);
    expect(code).toBe(1);
    expect(out).toContain('js, rust');
    expect(lines).not.toHaveBeenCalled();
  });

  it('unbekannte Flags brechen mit 2 ab, bevor git läuft', async () => {
    const err = vi.spyOn(process.stderr, 'write').mockReturnValue(true);
    const lines = vi.fn(git);
    try {
      expect((await run(['-gopkgs'], {}, lines)).code).toBe(2);
      expect((await run(['-h'], {}, lines)).code).toBe(0);
      expect(lines).not.toHaveBeenCalled();
    } finally {
      err.mockRestore();
    }
  });

  it('git scheitert: Meldung und Exit 1', async () => {
    const { code, out } = await run(['-base', 'gibtsnicht']);
    expect(code).toBe(1);
    expect(out).toMatch(/^❌ check:diff: git scheitert \(exit status \d+\); Basis mit -base oder CHECK_BASE setzen\n$/);
  });
});

describe('parseFlags', () => {
  const spec = { base: 'string', plan: 'bool' } as const;

  it('liest die Formen von Go', () => {
    expect(parseFlags(['--plan', '-base', '-x'], spec).values).toEqual({ plan: true, base: '-x' });
    expect(parseFlags(['-plan=false', '--base=a=b'], spec).values).toEqual({ plan: false, base: 'a=b' });
    expect(parseFlags(['wort', '-plan'], spec)).toEqual({ values: {}, rest: ['wort', '-plan'] });
    expect(parseFlags(['--', '-plan'], spec).rest).toEqual(['-plan']);
  });

  it.each([[['-nein']], [['-base']], [['-plan=ja']], [['---plan']], [['-constructor']]])('lehnt %j ab', (argv) => {
    expect(() => parseFlags(argv, spec)).toThrow(FlagError);
  });
});
