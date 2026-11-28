// Zugriff auf Git im Checkout: Basis und geänderte Dateien. Aufrufe als Argumentliste, ohne Shell.

import { spawnSync } from 'node:child_process';
import { notFound, resolveExe } from './exe';

/** Ruft git auf und liefert eine Zeile je Eintrag; wirft bei Fehlschlag. */
export type GitLines = (args: string[]) => string[];

export function gitLinesIn(cwd = '.', env: NodeJS.ProcessEnv = process.env): GitLines {
  return (args) => {
    const git = resolveExe('git', env);
    if (!git) throw new Error(notFound('git'));
    const res = spawnSync(git, ['-c', 'core.quotePath=false', ...args], {
      cwd,
      env,
      encoding: 'utf8',
      maxBuffer: 256 * 1024 * 1024,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    if (res.error) throw res.error;
    if (res.status !== 0) throw new Error(`exit status ${res.status ?? -1}`);
    return res.stdout.split('\n').map((l) => l.trim()).filter((l) => l !== '');
  };
}

/**
 * changedFiles liefert Basis und geänderte Dateien. Ohne Vorgabe ist die Basis HEAD (der Commit, der gerade entsteht:
 * geänderte und neue Dateien); ist der Checkout sauber, der letzte Commit (HEAD~1).
 */
export function changedFiles(wanted: string, git: GitLines = gitLinesIn()): { base: string; files: string[] } {
  const untracked = git(['ls-files', '--others', '--exclude-standard']);
  let base = wanted;
  if (base === '') {
    base = 'HEAD';
    const dirty = git(['diff', '--name-only', '--no-renames', 'HEAD']);
    if (dirty.length + untracked.length === 0) base = 'HEAD~1';
  }
  // Eine Basis mit Strich läse git als Option (`--output=…` schriebe eine Datei).
  if (base.startsWith('-')) throw new Error(`Basis ${base} beginnt mit einem Strich`);
  try {
    const mergeBase = git(['merge-base', 'HEAD', base]);
    // wie der eigene Stand vom Ziel abweicht, ohne dessen neuere Commits
    if (mergeBase.length === 1) base = mergeBase[0];
  } catch {
    // keine gemeinsame Basis: die Vorgabe selbst vergleichen
  }
  const files = git(['diff', '--name-only', '--no-renames', base]);
  return { base, files: [...files, ...untracked] };
}
