// Die zwei Programme des Aufräumens: git und gh, beide als Argumentliste ohne Shell und mit absolutem Pfad.

import { spawnSync } from 'node:child_process';
import { notFound, resolveExe } from '../checkdiff/exe';

/** Ruft git in `dir` auf und liefert die Standardausgabe; wirft mit der Fehlerausgabe. */
export type Git = (dir: string, ...args: string[]) => string;

/** Fragt, ob zu einem Branch ein gemergter Pull Request existiert; wirft, wenn das niemand beantworten kann. */
export type PrMerged = (branch: string) => boolean;

export function gitWith(env: NodeJS.ProcessEnv = process.env): Git {
  return (dir, ...args) => {
    const fail = (reason: string) => new Error(`git ${args.join(' ')}: ${reason}`);
    const git = resolveExe('git', env);
    if (!git) throw fail(notFound('git'));
    const res = spawnSync(git, args, {
      cwd: dir,
      env,
      encoding: 'utf8',
      maxBuffer: 256 * 1024 * 1024,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    if (res.error) throw fail(res.error.message);
    if (res.status !== 0) throw fail(`exit status ${res.status ?? -1}: ${res.stderr.trim()}`);
    return res.stdout.replace(/[\r\n]+$/, '');
  };
}

/** ghMerged fragt per gh. Ohne gh, Netz oder Anmeldung wirft es; das Werkzeug listet den Branch dann nur auf. */
export function ghMerged(root: string, env: NodeJS.ProcessEnv = process.env): PrMerged {
  return (branch) => {
    const gh = resolveExe('gh', env);
    if (!gh) throw new Error('gh fehlt');
    const args = ['pr', 'list', '--state', 'merged', '--head', branch, '--json', 'number', '--jq', 'length'];
    const res = spawnSync(gh, args, {
      cwd: root,
      env,
      encoding: 'utf8',
      timeout: 30_000,
      stdio: ['ignore', 'pipe', 'pipe'],
      windowsHide: true,
    });
    if (res.error || res.status !== 0) throw new Error('gh antwortet nicht (Netz oder Anmeldung)');
    const answer = res.stdout.trim();
    if (!/^\d+$/.test(answer)) throw new Error('gh-Antwort nicht lesbar');
    return Number(answer) > 0;
  };
}
