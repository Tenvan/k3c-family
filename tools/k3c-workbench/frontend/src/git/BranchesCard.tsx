import { useMemo, useState } from 'react';
import { ActionButton, Button, Checkbox, Code, Flex, SegmentedControl, StatusBadge, Text, type Tone } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { Table } from '@webkit/organisms';
import type { GitBranch, GitBranchState } from '../api';
import { CopyPrompt } from '../planning/PromptParts';
import type { ConfirmRequest } from './Confirm';
import { ago } from './format';
import { cleanupMarkedPrompt, cleanupStalePrompt, staleBranches } from './prompts';

const FILTERS = { alle: 'Alle', veraltet: 'Veraltet', ungemergt: 'Ungemergt', remote: 'Nur remote' } as const;
type Filter = keyof typeof FILTERS;

const RANK: Record<GitBranchState, number> = { aktuell: 0, basis: 1, worktree: 2, ungemergt: 3, squash: 4, gemergt: 5 };

/** Ton und Text des Zustands; „ungemergt“ ohne Prüfung ist nur vermutet („ungeprüft“). */
export function stateBadge(b: GitBranch): [Tone, string] {
  switch (b.state) {
    case 'aktuell': return ['info', 'aktuell'];
    case 'worktree': return ['info', 'in Worktree'];
    case 'basis': return ['neutral', 'Basis'];
    case 'gemergt': return ['ok', 'gemergt'];
    case 'squash': return ['ok', b.pr > 0 ? `per PR #${b.pr} gemergt` : 'per Squash gemergt'];
    default: return b.checked ? ['warn', 'ungemergt'] : ['neutral', 'ungeprüft'];
  }
}

/** Markieren lässt sich jeder lokale Branch außer Basis, aktuellem und Worktree; löschen nur die gemergten davon. */
export const markable = (b: GitBranch) => !b.remote && b.state !== 'basis' && b.state !== 'aktuell' && b.state !== 'worktree';

/** Lokale vor entfernten, dann Zustand, dann neueste zuerst. */
export function sortBranches(list: GitBranch[]): GitBranch[] {
  return [...list].sort((a, b) => Number(a.remote) - Number(b.remote) || RANK[a.state] - RANK[b.state] || b.date - a.date);
}

export function filterBranches(list: GitBranch[], f: Filter): GitBranch[] {
  if (f === 'veraltet') return staleBranches(list);
  if (f === 'ungemergt') return list.filter((b) => b.state === 'ungemergt');
  if (f === 'remote') return list.filter((b) => b.remote);
  return list;
}

interface Props {
  branches: GitBranch[];
  remove: (names: string[], remote: boolean) => Promise<void>;
  /** Gründliche Prüfung auf Squash-Merges; leer = alle ungeprüften. */
  check: (names: string[]) => Promise<void>;
  confirm: (r: ConfirmRequest) => void;
}

/** Vierter Schritt des Ablaufs: lokale und entfernte Branches. Gemergte markierte löscht die Seite selbst, für alle
 *  markierten gibt es den Prompt an Claude. */
export function BranchesCard({ branches, remove, check, confirm }: Props) {
  const [filter, setFilter] = useState<Filter>('alle');
  const [marked, setMarked] = useState<Set<string>>(new Set());
  const [remote, setRemote] = useState(false);
  const shown = useMemo(() => sortBranches(filterBranches(branches, filter)), [branches, filter]);
  const picked = branches.filter((b) => markable(b) && marked.has(b.name));
  const toggle = (name: string, on: boolean) => setMarked((m) => {
    const next = new Set(m);
    if (on) next.add(name);
    else next.delete(name);
    return next;
  });
  const del = () => {
    const go = picked.filter((b) => b.deletable).map((b) => b.name);
    const stay = picked.filter((b) => !b.deletable).map((b) => b.name);
    confirm({
      title: `${go.length} Branches löschen?`, action: 'Löschen', danger: true,
      text: `${go.join(', ')} ${remote ? 'lokal und auf origin' : 'lokal'}; develop enthält sie, die Workbench prüft das noch einmal.`
        + (stay.length > 0 ? ` Bleiben stehen (ungemergt oder ungeprüft): ${stay.join(', ')}.` : ''),
      run: () => void remove(go, remote).then(() => setMarked(new Set())),
    });
  };
  const unchecked = branches.filter((b) => !b.remote && b.state === 'ungemergt' && !b.checked).length;
  return (
    <section className="git-card" aria-label="Branches">
      <header className="git-head">
        <Text weight="bold">Branches <Text color="gray" size="1">({branches.length})</Text></Text>
        <Flex gap="3" align="center" wrap="wrap">
          <SegmentedControl.Root size="1" value={filter} onValueChange={(v) => setFilter(v as Filter)}>
            {Object.entries(FILTERS).map(([k, label]) => <SegmentedControl.Item key={k} value={k}>{label}</SegmentedControl.Item>)}
          </SegmentedControl.Root>
          <ActionButton size="1" variant="soft" disabled={unchecked === 0} onClick={() => check([])}
            title="Sucht gemergte PRs und Squash-Merges; dauert je Branch einen Moment">Ungeprüfte prüfen ({unchecked})</ActionButton>
          <CheckField size="1" label="auch auf origin" checked={remote} onCheckedChange={(v) => setRemote(v === true)} />
          <Button size="1" color="red" variant="soft" disabled={!picked.some((b) => b.deletable)} onClick={del}>
            Gemergte löschen ({picked.filter((b) => b.deletable).length})
          </Button>
          <Text size="1" color="gray">Prompt:</Text>
          {picked.length > 0 && <CopyPrompt prompt={cleanupMarkedPrompt(picked.map((b) => b.name))} what={`${picked.length} markierte aufräumen`} />}
          <CopyPrompt prompt={cleanupStalePrompt(branches)} what="Veraltete aufräumen" />
        </Flex>
      </header>
      <BranchTable list={shown} marked={marked} toggle={toggle} check={check} />
      {shown.length === 0 && <Text size="2" color="gray">Keine Branches in dieser Auswahl.</Text>}
    </section>
  );
}

interface TableProps {
  list: GitBranch[];
  marked: Set<string>;
  toggle: (n: string, on: boolean) => void;
  check: (names: string[]) => Promise<void>;
}

function BranchTable({ list, marked, toggle, check }: TableProps) {
  return (
    <Table.Root size="1" className="git-table">
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeaderCell width="28px" />
          <Table.ColumnHeaderCell>Branch</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>Remote</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>zu develop</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>Zustand</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>Commit</Table.ColumnHeaderCell>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {list.map((b) => <BranchRow key={`${b.remote}:${b.name}`} b={b} marked={marked.has(b.name)} toggle={toggle} check={check} />)}
      </Table.Body>
    </Table.Root>
  );
}

function BranchRow({ b, marked, toggle, check }: { b: GitBranch; marked: boolean } & Omit<TableProps, 'list' | 'marked'>) {
  const [tone, label] = stateBadge(b);
  const track = b.track === 'voraus' ? `+${b.ahead}` : b.track === 'zurück' ? `−${b.behind}` : b.track === 'auseinander' ? `+${b.ahead} / −${b.behind}` : '';
  return (
    <Table.Row>
      <Table.Cell>
        {markable(b) && <Checkbox size="1" aria-label={`${b.name} markieren`} checked={marked} onCheckedChange={(v) => toggle(b.name, v === true)} />}
      </Table.Cell>
      <Table.Cell><Code size="1" variant="ghost">{b.remote ? `origin/${b.name}` : b.name}</Code></Table.Cell>
      <Table.Cell><Text size="1" color={b.track === 'gone' ? 'red' : 'gray'}>{b.track}{track && ` ${track}`}</Text></Table.Cell>
      <Table.Cell><Text size="1" color="gray">+{b.baseAhead} / −{b.baseBehind}</Text></Table.Cell>
      <Table.Cell title={b.reason}>
        <Flex gap="2" align="center">
          <StatusBadge tone={tone}>{label}</StatusBadge>
          {!b.remote && b.state === 'ungemergt' && !b.checked && (
            <ActionButton size="1" variant="ghost" onClick={() => check([b.name])}>Prüfen</ActionButton>
          )}
        </Flex>
      </Table.Cell>
      <Table.Cell><Text size="1" color="gray" title={b.subject} className="git-ellipsis">{ago(b.date * 1000)} · {b.subject}</Text></Table.Cell>
    </Table.Row>
  );
}
