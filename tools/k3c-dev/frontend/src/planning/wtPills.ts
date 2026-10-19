import type { PlanningData, PlanProject, PlanSprint, PlanTicket, WorktreeChange, WorktreeDiff } from '../api';

// Worktree-Pills (B-467, WZ12.2): Die Änderungen der Worktrees (`worktreeCheck`) hängen als Marke am betroffenen Projekt,
// Sprint, an der Session oder am Ticket. Reine Logik; die Anzeige steht in `WtPills.tsx`.

/** Eine Änderung eines Worktrees an einem Eintrag. */
export interface WtMark {
  branch: string;
  kind: WorktreeChange['kind'];
  text: string;
}

/** Änderungen je Eintrag, Schlüssel `Objekt:ID` (z. B. `Session:WZ6.2`). */
export type WtIndex = ReadonlyMap<string, readonly WtMark[]>;

export const EMPTY_INDEX: WtIndex = new Map();

const keyOf = (item: string, id: string) => `${item}:${id}`;

/** Ordnet die Änderungen aller Worktrees den Einträgen zu; ein Worktree mit Fehler trägt nichts bei. */
export function indexChanges(diffs: readonly WorktreeDiff[]): WtIndex {
  const out = new Map<string, WtMark[]>();
  for (const d of diffs) {
    if (d.error) continue;
    for (const c of d.changes) {
      const k = keyOf(c.item, c.id);
      out.set(k, [...(out.get(k) ?? []), { branch: d.branch || d.path, kind: c.kind, text: c.text }]);
    }
  }
  return out;
}

export const marksFor = (index: WtIndex, item: WorktreeChange['item'], id: string): readonly WtMark[] => index.get(keyOf(item, id)) ?? [];

/** Text der Pill: „neu“, wenn der Eintrag nur im Worktree existiert, sonst „Worktree“, bei mehreren die Zahl. */
export function pillLabel(marks: readonly WtMark[]): string {
  const branches = new Set(marks.map((m) => m.branch)).size;
  if (marks.every((m) => m.kind === 'neu')) return branches > 1 ? `neu · ${branches} Worktrees` : 'neu';
  return branches > 1 ? `${branches} Worktrees` : 'Worktree';
}

/** Tooltip: je Änderung eine Zeile `Branch: Änderung`, z. B. „sprint/wz6: Status offen → in Arbeit“. */
export function pillTitle(marks: readonly WtMark[]): string {
  return marks.map((m) => `${m.branch}: ${m.kind === 'neu' ? 'neu' : m.kind === 'entfernt' ? 'entfernt' : m.text}`.trim()).join('\n');
}

const sprintOf = (nr: string) => nr.split('.')[0] ?? '';

type Add = (data: Draft, c: WorktreeChange) => void;
interface Draft { sprints: PlanSprint[]; tickets: PlanTicket[]; projects?: PlanProject[] }

const ADD: Record<WorktreeChange['item'], Add> = {
  Projekt: (d, c) => {
    if (d.projects && !d.projects.some((p) => p.id === c.id)) d.projects.push({ id: c.id, title: c.text, status: 'aktiv', rang: '–', sprints: [] });
  },
  Sprint: (d, c) => {
    if (!d.sprints.some((s) => s.id === c.id)) d.sprints.push({ id: c.id, title: c.text, domain: '', status: 'geplant', reife: '', spec: '', tickets: [], sessions: [], project: c.project || '–' });
    const owner = d.projects?.find((p) => p.id === c.project);
    if (owner && !owner.sprints.includes(c.id)) d.projects = d.projects?.map((p) => (p === owner ? { ...p, sprints: [...p.sprints, c.id] } : p));
  },
  Ticket: (d, c) => {
    if (!d.tickets.some((x) => x.nr === c.id)) d.tickets.push({ nr: c.id, title: c.text, domain: '', typ: '', prio: '', env: '', status: 'offen', sprint: c.sprint || '', spec: '', project: c.project || '–' });
  },
  Session: (d, c) => {
    const sp = d.sprints.find((s) => s.id === sprintOf(c.id));
    if (sp && !sp.sessions.some((x) => x.nr === c.id)) sp.sessions.push({ nr: c.id, typ: '', agent: '', status: 'entwurf', titel: c.text });
  },
};
const ORDER: WorktreeChange['item'][] = ['Projekt', 'Sprint', 'Ticket', 'Session']; // Sprint vor seinen Sessions

/**
 * Ergänzt die Daten der Wurzel um Einträge, die nur in einem Worktree existieren (Änderung `neu`): Ticket in der
 * Ticket-Liste, Session unter ihrem Sprint, Sprint und Projekt in ihren Listen (B-467/AC-04). Sie tragen die Pill „neu“
 * über den Index; die Wurzel-Daten bleiben unverändert.
 */
export function withWorktreeEntries(data: PlanningData, diffs: readonly WorktreeDiff[]): PlanningData {
  const news = diffs.filter((d) => !d.error).flatMap((d) => d.changes.filter((c) => c.kind === 'neu'));
  if (news.length === 0) return data;
  const draft: Draft = { sprints: data.sprints.map((s) => ({ ...s, sessions: [...s.sessions] })), tickets: [...data.tickets] };
  if (data.projects) draft.projects = [...data.projects];
  for (const item of ORDER) news.filter((c) => c.item === item).forEach((c) => ADD[item](draft, c));
  return { ...data, sprints: draft.sprints, tickets: draft.tickets, ...(draft.projects ? { projects: draft.projects } : {}) };
}
