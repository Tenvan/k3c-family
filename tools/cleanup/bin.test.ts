// Alte EXE in bin/ (aus bin_test.go): nur die feste Liste, nur reguläre Dateien, nie über einen Link.

import { existsSync, mkdirSync, readdirSync, symlinkSync, unlinkSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { afterEach, describe, expect, it } from 'vitest';
import { sandbox, type Sandbox } from '../checkdiff/testrepo';
import { binCandidates, cleanBin, type BinOptions } from './bin';
import { mainCheckout } from './cleanup';

const boxes: Sandbox[] = [];
afterEach(() => {
  for (const box of boxes.splice(0)) box.remove();
});

function fixture(): { root: string; bin: string } {
  const box = sandbox('cleanup-bin');
  boxes.push(box);
  const bin = path.join(box.root, 'bin');
  mkdirSync(path.join(bin, 'k3c-tui'), { recursive: true }); // Ordner mit altem Namen
  const files = [
    'k3c-server.exe', 'k3c-load.exe', 'k3c-tui.exe', 'k3c-starter.exe', 'k3c-workbench-web.exe',
    'k3c-workbench-web.exe~', 'k3c-workbench.exe', 'k3c-studio.exe', 'k3c-sim.exe', 'soundatlas.exe', 'aufraeumen.exe',
  ];
  for (const file of files) writeFileSync(path.join(bin, file), 'x');
  writeFileSync(path.join(bin, 'k3c-tui', 'keep.txt'), 'x');
  writeFileSync(path.join(box.root, 'k3c-server.exe'), 'x'); // außerhalb von bin/: nie anfassen
  return { root: box.root, bin };
}

function clean(dir: string, extra: Partial<BinOptions> = {}): string {
  let out = '';
  cleanBin({ dry: false, out: (text) => void (out += text), ...extra }, dir);
  return out;
}

const names = (dir: string): string[] => readdirSync(dir).sort();

/** Legt eine Junction (Windows, ohne Rechte) oder einen Ordner-Link an. */
const link = (target: string, at: string) => symlinkSync(target, at, process.platform === 'win32' ? 'junction' : 'dir');

describe('cleanBin', () => {
  it('löscht genau die alten Programme', () => {
    const { root, bin } = fixture();
    const out = clean(bin);
    expect(names(bin)).toEqual(['k3c-studio.exe', 'k3c-tui', 'k3c-workbench.exe']);
    expect(existsSync(path.join(bin, 'k3c-tui', 'keep.txt'))).toBe(true);
    expect(existsSync(path.join(root, 'k3c-server.exe'))).toBe(true);
    expect(out).toContain('🧹 bin/k3c-server.exe (0.0 MiB) gelöscht\n');
    expect(out).toContain('🚫 bin/k3c-tui übersprungen: keine reguläre Datei\n');
  });

  it('--dry löscht nichts', () => {
    const { bin } = fixture();
    const before = names(bin);
    const out = clean(bin, { dry: true });
    expect(names(bin)).toEqual(before);
    expect(out.split('würde gelöscht').length - 1).toBe(9);
  });

  it('meldet eine gesperrte Datei und macht weiter', () => {
    const { bin } = fixture();
    const remove = (file: string) => {
      if (file.endsWith('k3c-load.exe')) throw new Error('EPERM: gesperrt');
      unlinkSync(file);
    };
    const out = clean(bin, { remove });
    expect(out).toContain('nicht gelöscht, vermutlich in Benutzung oder gesperrt: EPERM');
    expect(existsSync(path.join(bin, 'k3c-load.exe'))).toBe(true);
    expect(existsSync(path.join(bin, 'k3c-starter.exe'))).toBe(false);
  });

  it('fehlendes bin/ ist kein Fehler', () => {
    const { root } = fixture();
    expect(clean(path.join(root, 'gibtsnicht', 'bin'))).toBe('');
  });

  it('lehnt einen Link oder eine Junction als bin/ ab', () => {
    const { root, bin } = fixture();
    const at = path.join(root, 'link');
    link(bin, at);
    const before = names(bin);
    expect(clean(at)).toContain('🚫 bin/ ist kein echter Ordner');
    expect(clean(`${at}${path.sep}..${path.sep}link`)).toContain('kein echter Ordner');
    expect(names(bin)).toEqual(before);
  });

  it('lehnt eine Datei als bin/ ab', () => {
    const { root } = fixture();
    expect(clean(path.join(root, 'k3c-server.exe'))).toContain('kein echter Ordner');
    expect(existsSync(path.join(root, 'k3c-server.exe'))).toBe(true);
  });

  it('folgt keinem Link unter einem alten Namen nach draußen', () => {
    const { root, bin } = fixture();
    const outside = path.join(root, 'draussen');
    mkdirSync(outside);
    writeFileSync(path.join(outside, 'k3c-server.exe'), 'x');
    unlinkSync(path.join(bin, 'k3c-server.exe'));
    link(outside, path.join(bin, 'k3c-server.exe'));
    expect(clean(bin)).toContain('🚫 bin/k3c-server.exe übersprungen: keine reguläre Datei');
    expect(existsSync(path.join(outside, 'k3c-server.exe'))).toBe(true);
    expect(existsSync(path.join(bin, 'k3c-server.exe'))).toBe(true);
  });

  it('die Namensliste ist fest und ohne Pfade oder Muster', () => {
    for (const name of binCandidates()) {
      expect(name).toMatch(/^[a-z0-9-]+(\.exe~?)?$/);
      expect(['k3c-workbench.exe', 'k3c-studio.exe']).not.toContain(name);
    }
    expect(binCandidates()).toEqual(expect.arrayContaining(['aufraeumen.exe', 'regeltabelle.exe', 'soundatlas.exe']));
  });
});

describe('mainCheckout', () => {
  const tree = (dir: string) => ({ path: dir, head: '', branch: '', detached: false });

  it('nimmt die Repo-Wurzel (git nennt sie zuerst), sonst den aktuellen Checkout', () => {
    const worktree = 'C:/repo/.claude/worktrees/x';
    expect(mainCheckout([tree('C:/repo'), tree(worktree)], worktree)).toBe('C:/repo');
    expect(mainCheckout([], 'C:/top')).toBe('C:/top');
    expect(mainCheckout([tree('')], 'C:/top')).toBe('C:/top');
  });
});
