// Lauf: Zeitmarker (check_run der Workbench liest sie), Bahnen und Programmsuche.

import { writeFileSync } from 'node:fs';
import path from 'node:path';
import { afterAll, describe, expect, it } from 'vitest';
import { resolveExe } from './exe';
import { runStage, runStep } from './run';
import type { Step } from './steps';
import { sandbox } from './testrepo';

const box = sandbox('checkdiff-run');
afterAll(() => box.remove());

const step = (name: string, cmd: string[], lane: Step['lane'] = 'js'): Step => ({ name, lane, stage: 1, dir: box.root, cmd });

function collect(): { text: () => string; write: (d: string | Buffer) => void } {
  const parts: string[] = [];
  return { text: () => parts.join(''), write: (d) => void parts.push(d.toString()) };
}

describe('Zeitmarker', () => {
  it('grüner Schritt: nur Start- und Ende-Zeile, keine Ausgabe', async () => {
    const out = collect();
    expect(await runStep(step('ok', ['git', '--version']), out.write)).toBe(true);
    expect(out.text()).toBe('⏱▶ ok\n⏱■ ok exit=0\n');
  });

  it('roter Schritt: Befehl, Dauer und Ausgabe zwischen den Markern', async () => {
    const out = collect();
    expect(await runStep({ ...step('rot', ['git', 'gibtsnicht']), dir: '.' }, out.write)).toBe(false);
    expect(out.text()).toMatch(/^⏱▶ rot\n❌ rot: git gibtsnicht \(\d+\.\d s\)\n[\s\S]*gibtsnicht[\s\S]*\n⏱■ rot exit=1\n$/);
  });

  it('fehlendes Programm: exit=127', async () => {
    const out = collect();
    expect(await runStep(step('fehlt', ['k3c-gibt-es-nicht']), out.write)).toBe(false);
    expect(out.text()).toContain('k3c-gibt-es-nicht');
    expect(out.text().endsWith('⏱■ fehlt exit=127\n')).toBe(true);
  });

  it('fehlender Arbeitsordner: exit=127', async () => {
    const out = collect();
    expect(await runStep({ ...step('weg', ['git', '--version']), dir: path.join(box.root, 'nix') }, out.write)).toBe(false);
    expect(out.text().endsWith('⏱■ weg exit=127\n')).toBe(true);
  });

  it('Argumente gehen ohne Shell durch', async () => {
    const out = collect();
    await runStep(step('wort', ['git', 'a & echo b > x']), out.write);
    expect(out.text()).toContain("'a & echo b > x' is not a git command");
  });
});

describe('runStage', () => {
  it('eine Bahn bricht beim ersten Fehler ab, die andere läuft weiter', async () => {
    const out = collect();
    const steps = [
      step('a1', ['git', 'gibtsnicht']),
      step('a2', ['git', '--version']),
      step('b1', ['git', '--version'], 'rust'),
      step('b2', ['git', '--version'], 'rust'),
      { ...step('spaeter', ['git', '--version']), stage: 2 as const },
    ];
    expect(await runStage(steps, 1, out.write)).toEqual(['a1']);
    expect(out.text()).not.toContain('a2');
    expect(out.text()).not.toContain('spaeter');
    expect(out.text()).toContain('⏱■ b2 exit=0');
  });
});

describe('resolveExe', () => {
  it('nimmt nur absolute PATH-Einträge und nie den Arbeitsordner', () => {
    const name = process.platform === 'win32' ? 'k3c-planted.exe' : 'k3c-planted';
    writeFileSync(path.join(box.root, name), '', { mode: 0o755 });
    const relative = path.relative(process.cwd(), box.root);
    expect(resolveExe('k3c-planted', { PATH: ['.', relative, ''].join(path.delimiter) })).toBeUndefined();
    expect(resolveExe('k3c-planted', { PATH: box.root })).toBe(path.join(box.root, name));
  });
});
