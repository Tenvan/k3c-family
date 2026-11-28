// Nur für Tests (checkdiff und cleanup): Wegwerf-Ordner unter .work/tmp und ein abgeschottetes git. Die Ordner liegen
// innerhalb des echten Repos, darum begrenzt GIT_CEILING_DIRECTORIES die Suche: ohne eigenes .git findet git nichts.

import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { resolveExe } from './exe';

const BASE = path.resolve(fileURLToPath(new URL('../../.work/tmp', import.meta.url)));

export interface Sandbox {
  root: string;
  env: NodeJS.ProcessEnv;
  /** git in `dir`; wirft bei Fehlschlag oder wenn dir außerhalb des Wegwerf-Ordners liegt */
  git: (dir: string, ...args: string[]) => string;
  remove: () => void;
}

function isolatedEnv(root: string): NodeJS.ProcessEnv {
  const env: NodeJS.ProcessEnv = {};
  for (const [key, value] of Object.entries(process.env)) {
    if (!/^(GIT_|GH_|GITHUB_)/i.test(key)) env[key] = value;
  }
  const config = path.join(root, 'gitconfig');
  writeFileSync(config, '[user]\n\tname = t\n\temail = t@t\n[commit]\n\tgpgsign = false\n[core]\n\tautocrlf = false\n');
  return Object.assign(env, {
    GIT_CONFIG_NOSYSTEM: '1',
    GIT_CONFIG_GLOBAL: config,
    GIT_CEILING_DIRECTORIES: [BASE, root].join(path.delimiter),
    GIT_ALLOW_PROTOCOL: 'file',
    GIT_TERMINAL_PROMPT: '0',
    XDG_CONFIG_HOME: root,
    GH_CONFIG_DIR: path.join(root, 'gh'),
  });
}

export function sandbox(prefix: string): Sandbox {
  mkdirSync(BASE, { recursive: true });
  const root = mkdtempSync(path.join(BASE, `${prefix}-`));
  const env = isolatedEnv(root);
  const git = (dir: string, ...args: string[]): string => {
    if (path.relative(root, dir).startsWith('..')) throw new Error(`git außerhalb des Wegwerf-Ordners: ${dir}`);
    const res = spawnSync(resolveExe('git', env) ?? 'git', args, { cwd: dir, env, encoding: 'utf8', windowsHide: true });
    if (res.status !== 0) throw new Error(`git ${args.join(' ')}: ${res.stderr}${res.stdout}`);
    return res.stdout.trim();
  };
  const remove = () => rmSync(root, { recursive: true, force: true, maxRetries: 10, retryDelay: 100 });
  return { root, env, git, remove };
}

/** commit legt die Datei `file` an und committet sie. */
export function commit(box: Sandbox, dir: string, file: string): void {
  mkdirSync(path.dirname(path.join(dir, file)), { recursive: true });
  writeFileSync(path.join(dir, file), file);
  box.git(dir, 'add', '--', file);
  box.git(dir, 'commit', '-q', '-m', file);
}
