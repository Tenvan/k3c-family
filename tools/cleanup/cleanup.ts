// Löscht sichere lokale Branches und Worktrees und listet den Rest mit Grund (B-302). Auf origin löscht es nie etwas.
// Danach räumt es in bin/ der Repo-Wurzel die EXE entfernter Programme (bin.ts, B-1007).

import { readdirSync } from 'node:fs';
import path from 'node:path';
import { cleanBin } from './bin';
import type { Git, PrMerged } from './git';

/** Die Umgebung des Laufs; git und prMerged sind austauschbar (Tests, kein gh). */
export interface Options {
  root: string;
  dry: boolean;
  git: Git;
  prMerged: PrMerged;
  out: (text: string) => void;
}

interface Branch {
  name: string;
  /** hat einen Upstream (auch wenn er weg ist) */
  upstream: boolean;
  /** Upstream ist auf origin gelöscht */
  gone: boolean;
  /** im aktuellen Checkout ausgecheckt */
  current: boolean;
  worktree: string;
}

export interface Worktree {
  path: string;
  head: string;
  /** leer bei detached */
  branch: string;
  detached: boolean;
}

/** Die Entscheidung zu einem Branch oder Worktree. */
interface Action {
  subject: string;
  remove: boolean;
  reason: string;
  /** Worktree, der vor dem Branch entfernt wird */
  wtPath?: string;
  /** Branch, der gelöscht wird */
  brName?: string;
}

const message = (e: unknown): string => (e instanceof Error ? e.message : String(e));

/** mainCheckout ist die Repo-Wurzel (git nennt sie zuerst), sonst der aktuelle Checkout. */
export function mainCheckout(trees: Worktree[], top: string): string {
  return trees.length > 0 && trees[0].path !== '' ? trees[0].path : top;
}

/** origin/main, sonst main: der Stand, in dem gemergte Arbeit stecken muss. */
function baseRef(o: Options): string {
  for (const ref of ['origin/main', 'main']) {
    try {
      o.git(o.root, 'rev-parse', '--verify', '--quiet', ref);
      return ref;
    } catch {
      // nächste Kandidatin
    }
  }
  throw new Error('weder origin/main noch main gefunden');
}

function listBranches(o: Options): Branch[] {
  const format = '--format=%(refname:short)%09%(upstream)%09%(upstream:track)%09%(HEAD)%09%(worktreepath)';
  const out = o.git(o.root, 'for-each-ref', format, 'refs/heads');
  return out
    .split('\n')
    .map((line) => line.replace(/\r+$/, '').split('\t'))
    .filter((f) => f.length >= 5)
    .map((f) => ({ name: f[0], upstream: f[1] !== '', gone: f[2].includes('gone'), current: f[3] === '*', worktree: f[4] }));
}

function listWorktrees(o: Options): Worktree[] {
  const res: Worktree[] = [];
  for (const raw of o.git(o.root, 'worktree', 'list', '--porcelain').split('\n')) {
    const line = raw.replace(/\r+$/, '');
    if (line.startsWith('worktree ')) {
      res.push({ path: line.slice('worktree '.length), head: '', branch: '', detached: false });
      continue;
    }
    const last = res.at(-1);
    if (!last) continue;
    if (line.startsWith('HEAD ')) last.head = line.slice('HEAD '.length);
    else if (line.startsWith('branch refs/heads/')) last.branch = line.slice('branch refs/heads/'.length);
    else if (line === 'detached') last.detached = true;
  }
  return res;
}

/** Branch-Namen aktiver Sprints: `docs/sprints/aktiv/CI1-name` gehört zu `sprint/ci1`. */
function activeSprints(root: string): Set<string> {
  try {
    const entries = readdirSync(path.join(root, 'docs', 'sprints', 'aktiv'), { withFileTypes: true });
    return new Set(entries.filter((e) => e.isDirectory()).map((e) => `sprint/${e.name.split('-')[0].toLowerCase()}`));
  } catch {
    return new Set();
  }
}

/** Zählt die Commits des Branchs, die dem Patch nach nicht in base stecken. */
function cherry(o: Options, base: string, name: string): number {
  return o.git(o.root, 'cherry', base, name).split('\n').filter((line) => line.startsWith('+')).length;
}

/** Entscheidet allein über den Branch; Worktrees prüft judgeBranches danach. */
function judgeBranch(o: Options, base: string, b: Branch, active: Set<string>): { remove: boolean; reason: string } {
  const keep = (reason: string) => ({ remove: false, reason });
  if (b.name === 'main') return keep('geschützt');
  // git läse so einen Namen als Option (`git cherry <base> -v` zählte HEAD statt des Branchs)
  if (b.name.startsWith('-')) return keep('Name beginnt mit einem Strich, von Hand prüfen');
  if (b.current) return keep('im aktuellen Checkout ausgecheckt');
  if (active.has(b.name)) return keep('Branch eines aktiven Sprints');
  if (b.upstream && !b.gone) return keep('Upstream auf origin vorhanden');
  let ahead: number;
  try {
    ahead = cherry(o, base, b.name);
  } catch (e) {
    return keep(`git cherry fehlgeschlagen: ${message(e)}`);
  }
  if (ahead === 0) return { remove: true, reason: `alle Commits stecken in ${base}` };
  if (!b.gone) return keep(`nicht gemergt, ${ahead} Commits nicht in ${base}`);
  try {
    if (o.prMerged(b.name)) return { remove: true, reason: 'Upstream weg, Pull Request gemergt' };
  } catch (e) {
    return keep(`Upstream weg, PR-Status unbekannt: ${message(e)}`);
  }
  return keep(`Upstream weg, aber kein gemergter Pull Request, ${ahead} Commits nicht in ${base}`);
}

/** Vergleich wie filepath.Clean und EqualFold: ohne Schrägstrich am Ende, mit /, ohne Groß/Klein. */
function sameDir(a: string, b: string): boolean {
  const clean = (p: string) => path.normalize(p).replaceAll('\\', '/').replace(/(?<=[^/:])\/+$/, '').toLowerCase();
  return clean(a) === clean(b);
}

const dirty = (o: Options, dir: string): boolean => o.git(dir, 'status', '--porcelain', '--ignored').trim() !== '';

/** Prüft den Worktree eines zu löschenden Branchs: nur ein sauberer, nicht der Hauptcheckout. */
function withWorktree(o: Options, a: Action, b: Branch, trees: Worktree[]): Action {
  const keep = (reason: string): Action => ({ subject: a.subject, remove: false, reason, brName: b.name });
  if (trees.length > 0 && sameDir(b.worktree, trees[0].path)) return keep('im Hauptcheckout ausgecheckt');
  try {
    if (dirty(o, b.worktree)) return keep(`Worktree ${b.worktree} hat Änderungen oder ignorierte Dateien`);
  } catch (e) {
    return keep(`Worktree ${b.worktree} nicht prüfbar: ${message(e)}`);
  }
  return { ...a, wtPath: b.worktree, reason: `${a.reason}; Worktree ${b.worktree} sauber, wird mit entfernt` };
}

function judgeBranches(o: Options, base: string, branches: Branch[], trees: Worktree[], top: string): Action[] {
  const active = activeSprints(o.root);
  return branches.map((b) => {
    const { remove, reason } = judgeBranch(o, base, b, active);
    const a: Action = { subject: `Branch ${b.name}`, remove, reason, brName: b.name };
    return remove && b.worktree !== '' && !sameDir(b.worktree, top) ? withWorktree(o, a, b, trees) : a;
  });
}

/** Warum ein Worktree ohne Branch bleibt; leer, wenn er weg kann. */
function detachedKeepReason(o: Options, base: string, t: Worktree): string {
  try {
    if (dirty(o, t.path)) return 'hat Änderungen oder ignorierte Dateien';
  } catch (e) {
    return `nicht prüfbar: ${message(e)}`;
  }
  try {
    o.git(o.root, 'merge-base', '--is-ancestor', t.head, base);
  } catch {
    return `detached auf einem Stand, der nicht in ${base} steckt`;
  }
  return '';
}

/** Prüft Worktrees ohne Branch: sauber und auf einem Stand, der in base steckt. */
function judgeDetached(o: Options, base: string, trees: Worktree[], top: string): Action[] {
  return trees
    .filter((t, i) => i > 0 && t.detached && !sameDir(t.path, top))
    .map((t) => {
      const subject = `Worktree ${t.path}`;
      const keep = detachedKeepReason(o, base, t);
      if (keep) return { subject, remove: false, reason: keep };
      return { subject, remove: true, reason: `detached auf einem Stand von ${base}, ohne Änderungen`, wtPath: t.path };
    });
}

/** Nie mit --force: git verweigert selbst einen Worktree mit Änderungen. */
function erase(o: Options, a: Action): void {
  if (a.wtPath) o.git(o.root, 'worktree', 'remove', a.wtPath);
  if (a.brName) o.git(o.root, 'branch', '-D', a.brName);
}

function apply(o: Options, actions: Action[]): void {
  let removed = 0;
  let kept = 0;
  for (const a of actions) {
    let verb = 'bleibt   ';
    if (a.remove) verb = o.dry ? 'würde löschen  ' : 'löschen  ';
    if (a.remove && !o.dry) {
      try {
        erase(o, a);
      } catch (e) {
        o.out(`Fehler    ${a.subject}: ${message(e)}\n`);
        kept++;
        continue;
      }
    }
    if (a.remove) removed++;
    else kept++;
    o.out(`${verb}${a.subject} – ${a.reason}\n`);
  }
  o.out(`aufraeumen: ${removed} ${o.dry ? 'würden gelöscht' : 'gelöscht'}, ${kept} bleiben\n`);
}

export function run(o: Options): void {
  const base = baseRef(o);
  const branches = listBranches(o);
  const trees = listWorktrees(o);
  const top = o.git(o.root, 'rev-parse', '--show-toplevel');
  const actions = [...judgeBranches(o, base, branches, trees, top), ...judgeDetached(o, base, trees, top)];
  apply(o, actions);
  cleanBin(o, path.join(mainCheckout(trees, top), 'bin'));
}
