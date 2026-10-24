// Seite Git (B-580); Go: internal/gitops, internal/github/prs.go, app_git.go.

/** Datei im Working Tree; code: M, A, D, R, C, U (Konflikt), ? (ungetrackt). */
export interface GitChange {
  path: string;
  code: string;
  staged: boolean;
}

export interface GitStash {
  ref: string;
  age: string;
  subject: string;
}

export interface GitWorktree {
  path: string;
  /** Leer bei detached HEAD. */
  branch: string;
  head: string;
}

export interface GitStatus {
  /** Leer bei detached HEAD. */
  branch: string;
  upstream: string;
  upstreamGone: boolean;
  ahead: number;
  behind: number;
  baseAhead: number;
  baseBehind: number;
  mainBehind: number;
  changes: GitChange[];
  stashes: GitStash[];
  worktrees: GitWorktree[];
  /** Unix-Millisekunden des letzten Fetch, 0 = unbekannt. */
  fetchedAt: number;
  advice: string[];
}

export type GitBranchState = 'aktuell' | 'worktree' | 'basis' | 'gemergt' | 'squash' | 'ungemergt';
export type GitTrack = 'synchron' | 'voraus' | 'zurück' | 'auseinander' | 'gone' | 'fehlt' | 'nur remote';

export interface GitBranch {
  name: string;
  remote: boolean;
  upstream: string;
  track: GitTrack;
  ahead: number;
  behind: number;
  baseAhead: number;
  baseBehind: number;
  state: GitBranchState;
  sha: string;
  /** Unix-Sekunden des letzten Commits. */
  date: number;
  subject: string;
  deletable: boolean;
  /** Warum nicht löschbar. */
  reason: string;
}

export interface GitDiffFile {
  path: string;
  code: string;
  added: number;
  deleted: number;
  binary: boolean;
}

export interface GitView {
  status: GitStatus;
  branches: GitBranch[];
  files: { base: string; files: GitDiffFile[] };
}

export interface GitPR {
  number: number;
  title: string;
  url: string;
  head: string;
  author: string;
  updated: string;
  state: 'offen' | 'Entwurf';
  ci: string;
  merge: string;
}

export interface GitPRList {
  prs: GitPR[];
  /** Hinweis von gh (fehlt, nicht angemeldet); die übrige Seite arbeitet weiter. */
  error?: string;
  fetched?: string;
}

export type GitAction = 'fetch' | 'pull' | 'push' | 'stash' | 'stash-pop';
export type GitPRAction = 'merge' | 'squash' | 'close';

/** Methoden der Seite Git am Backend. Fehler kommen als Ablehnung mit dem Text aus Go. */
export interface GitBackend {
  /** Status, Branches (mit Squash-Erkennung) und Diff des Branches zu origin/develop. */
  gitOverview(): Promise<GitView>;
  /** Nie mit Force; Push lehnt develop und main ab, Pull nur Fast-Forward. Antwort ist ein Satz zum Ergebnis. */
  gitAction(action: GitAction): Promise<string>;
  /** Löscht nur gemergte lokale Branches (Go prüft erneut); remote auch auf origin. */
  gitDeleteBranches(names: string[], remote: boolean): Promise<string>;
  gitPRs(): Promise<GitPRList>;
  /** PR vom aktuellen Branch nach develop; Antwort ist die Adresse. */
  gitPRCreate(title: string, body: string): Promise<string>;
  gitPRAction(number: number, action: GitPRAction): Promise<string>;
}
