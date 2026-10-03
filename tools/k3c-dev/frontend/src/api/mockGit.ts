import type { CommitMessage, CommitResult, GitFile, GitView } from './types';

// Erfundener Git-Stand für den Mock. Staging verschiebt Dateien zwischen den Listen, der Commit prüft wie Go
// (gitcommit.Message.Normalize) und legt eine Zeile in `recent` an.

const TYPES = ['feat', 'fix', 'docs', 'refactor', 'test', 'perf', 'chore', 'ci', 'build', 'style'];
const DOMAINS = ['reg', 'sim', 'srv', 'cli', 'plat', 'inf'];

export function mockGit() {
  let staged: GitFile[] = [{ path: 'tools/k3c-dev/internal/taskrun/taskrun.go', status: 'A' }];
  let unstaged: GitFile[] = [
    { path: 'tools/k3c-dev/app.go', status: 'M' },
    { path: 'tools/k3c-dev/frontend/src/App.tsx', status: 'M' },
    { path: 'tools/k3c-dev/frontend/src/tasks/TasksPage.tsx', status: '?' },
    { path: 'tools/k3c-dev/frontend/src/planning/PlanningPage.tsx', status: '?' },
    { path: 'docs/sprints/aktiv/SP11-raspberry-pi/README.md', status: 'M' },
  ];
  let recent = ['e2d97b3 docs(reg): Plan Weiterentwicklung, Fragenkatalog, Tickets B-134 bis B-170', '6188c10 fix(inf): launch.json startet dev und start über task statt npm'];
  const view = (): GitView => ({ branch: 'claude/dev-workbench-migration-07abcf', staged, unstaged, recent, types: TYPES, domains: DOMAINS });
  const move = (paths: string[], from: GitFile[]) => {
    const hit = from.filter((f) => paths.includes(f.path));
    return { hit, rest: from.filter((f) => !paths.includes(f.path)) };
  };
  return {
    git: async () => view(),
    gitStage: async (paths: string[]) => {
      const { hit, rest } = move(paths, unstaged);
      unstaged = rest;
      staged = [...staged, ...hit.map((f) => ({ ...f, status: f.status === '?' ? 'A' : f.status }))];
      return view();
    },
    gitUnstage: async (paths: string[]) => {
      const { hit, rest } = move(paths, staged);
      staged = rest;
      unstaged = [...unstaged, ...hit.map((f) => ({ ...f, status: f.status === 'A' ? '?' : f.status }))];
      return view();
    },
    gitCommit: async (m: CommitMessage): Promise<CommitResult> => {
      if (!TYPES.includes(m.type)) throw new Error(`type: unbekannter Typ, erlaubt: ${TYPES.join(', ')}`);
      if (m.scope && !/^[a-z0-9][a-z0-9-]*$/.test(m.scope)) throw new Error('scope: nur Kleinbuchstaben, Ziffern und Bindestrich');
      if (!m.subject.trim()) throw new Error('subject: darf nicht leer sein');
      if (m.subject.length > 72) throw new Error('subject: höchstens 72 Zeichen');
      if (staged.length === 0) throw new Error('nichts gestaged');
      const hash = Math.random().toString(16).slice(2, 9);
      recent = [`${hash} ${m.type}${m.scope ? `(${m.scope})` : ''}: ${m.subject.trim()}`, ...recent].slice(0, 8);
      staged = [];
      return { hash, view: view() };
    },
  };
}
