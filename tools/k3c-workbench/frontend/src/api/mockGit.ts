import type { GitBackend, GitBranch, GitPR, GitView } from './typesGit';

const DAY = 86_400;

/** Branches des Mocks: aktueller Feature-Branch, Basis, gemergte, ungemergte, Worktree und nur entfernt. */
function mockBranches(): GitBranch[] {
  const now = Math.floor(Date.now() / 1000);
  const branch = (b: Partial<GitBranch> & Pick<GitBranch, 'name' | 'state'>): GitBranch => ({
    remote: false, upstream: `origin/${b.name}`, sha: '0123456789abcdef', track: 'synchron', ahead: 0, behind: 0, baseAhead: 0, baseBehind: 0,
    date: now - DAY, subject: '', deletable: b.state === 'gemergt' || b.state === 'squash', reason: '', ...b,
  });
  return [
    branch({ name: 'feat/git-seite', state: 'aktuell', track: 'voraus', ahead: 1, baseAhead: 1, subject: 'feat(dev): Git-Seite', date: now - 600, reason: 'ausgecheckt' }),
    branch({ name: 'develop', state: 'basis', subject: 'Neun Sprints (#451)', reason: 'develop und main bleiben' }),
    branch({ name: 'main', state: 'basis', track: 'zurück', behind: 275, baseBehind: 275, subject: 'M8 (#106)', date: now - 40 * DAY, reason: 'develop und main bleiben' }),
    branch({ name: 'sprint/bd2', state: 'squash', track: 'gone', baseAhead: 4, baseBehind: 61, subject: 'docs(bd2): Review', date: now - 12 * DAY }),
    branch({ name: 'sprint/gr8', state: 'squash', track: 'gone', baseAhead: 3, baseBehind: 80, subject: 'docs(cli): Review GR8', date: now - 15 * DAY }),
    branch({ name: 'k4-weiter-arbeit', state: 'gemergt', track: 'gone', baseBehind: 120, subject: 'feat(srv): Raum tauscht die Insel', date: now - 20 * DAY }),
    branch({ name: 'backup/wz19-alt', state: 'ungemergt', track: 'auseinander', ahead: 18, behind: 26, baseAhead: 18, baseBehind: 90, subject: 'docs(dev): WZ19.1', date: now - 25 * DAY, reason: 'nicht in develop' }),
    branch({ name: 'sprint/gr9', state: 'worktree', track: 'gone', baseAhead: 2, subject: 'docs(cli): GR10', date: now - 9 * DAY, reason: 'in einem Worktree ausgecheckt' }),
    branch({ name: 'sprint/k5', state: 'ungemergt', remote: true, upstream: '', track: 'nur remote', baseAhead: 4, baseBehind: 38, subject: 'feat(cli): Debug-Overlay (K5.3)', date: now - 3 * DAY, reason: 'nur remote' }),
  ];
}

/** Erfundener Stand für `?mock=1` und Tests: ein Feature-Branch mit ungepushtem Commit, gemergte und offene Branches,
 *  zwei PRs. Aktionen ändern den Stand so, dass die Seite sichtbar reagiert. */
export function mockGit(): GitBackend {
  let branches = mockBranches();
  let stashes = 0;
  let prs: GitPR[] = [
    { number: 452, title: 'K5: Debug-Overlay nur mit ?dev=1', url: 'https://github.com/tenvan/k3c-family/pull/452', head: 'sprint/k5', author: 'tenvan', updated: new Date(Date.now() - 3_600_000).toISOString(), state: 'offen', ci: 'grün', merge: 'konfliktfrei' },
    { number: 447, title: 'W11: Bauplätze mit Platz-Stufe', url: 'https://github.com/tenvan/k3c-family/pull/447', head: 'sprint/w11', author: 'tenvan', updated: new Date(Date.now() - 2 * 86_400_000).toISOString(), state: 'offen', ci: 'rot', merge: 'Konflikt' },
  ];
  const view = (): GitView => ({
    status: {
      branch: 'feat/git-seite', upstream: 'origin/feat/git-seite', upstreamGone: false, ahead: 1, behind: 0, baseAhead: 1,
      baseBehind: 0, mainBehind: 275, fetchedAt: Date.now() - 120_000,
      changes: [{ path: 'tools/k3c-workbench/uiapi.go', code: 'M', staged: true }, { path: 'tools/k3c-workbench/app_git.go', code: 'M', staged: false },
        { path: 'docs/notiz.md', code: '?', staged: false }],
      stashes: Array.from({ length: stashes }, (_, i) => ({ ref: `stash@{${i}}`, age: 'vor 1 Minute', subject: 'On feat/git-seite: workbench' })),
      worktrees: [{ path: 'C:/WORKSPACE/k3c/.claude/worktrees/gra-6d4c38', branch: '', head: '4348d4fc' }],
      advice: ['3 geänderte Dateien committen oder stashen', 'Push: 1 Commits nicht gepusht', 'main liegt 275 hinter origin/main (Fast-Forward macht Ralf)'],
    },
    branches,
    files: {
      base: 'origin/develop',
      files: [
        { path: 'tools/k3c-workbench/internal/gitops/status.go', code: 'A', added: 214, deleted: 0, binary: false },
        { path: 'tools/k3c-workbench/uiapi.go', code: 'M', added: 1, deleted: 0, binary: false },
        { path: 'docs/backlog/README.md', code: 'M', added: 1, deleted: 0, binary: false },
      ],
    },
  });
  return {
    gitOverview: async () => view(),
    gitAction: async (action) => {
      if (action === 'stash') stashes += 1;
      if (action === 'stash-pop') {
        if (stashes === 0) throw new Error('git stash: No stash entries found.');
        stashes -= 1;
      }
      return `Mock: ${action} fertig`;
    },
    gitDeleteBranches: async (names) => {
      const bad = names.filter((n) => !branches.find((b) => b.name === n)?.deletable);
      if (bad.length > 0) throw new Error(`nicht gelöscht: ${bad.join(', ')}`);
      branches = branches.filter((b) => !names.includes(b.name));
      return `${names.length} Branches gelöscht`;
    },
    gitPRs: async () => ({ prs, fetched: new Date().toISOString() }),
    gitPRCreate: async (title) => {
      if (!title.trim()) throw new Error('Titel fehlt');
      return 'https://github.com/tenvan/k3c-family/pull/460';
    },
    gitPRAction: async (number, action) => {
      prs = prs.filter((p) => p.number !== number);
      return `PR #${number} ${action === 'close' ? 'geschlossen' : 'gemergt'}`;
    },
  };
}
