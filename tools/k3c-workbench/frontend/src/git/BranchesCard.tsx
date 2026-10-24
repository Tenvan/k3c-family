import { useMemo, useState } from 'react';
import { Button, Checkbox, Code, Flex, SegmentedControl, StatusBadge, Text, type Tone } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { Table } from '@webkit/organisms';
import type { GitBranch, GitBranchState } from '../api';
import { CopyPrompt } from '../planning/PromptParts';
import type { ConfirmRequest } from './Confirm';
import { ago } from './format';
import { cleanupMarkedPrompt, cleanupStalePrompt, staleBranches } from './prompts';

const FILTERS = { alle: 'Alle', veraltet: 'Veraltet', ungemergt: 'Ungemergt', remote: 'Nur remote' } as const;
type Filter = keyof typeof FILTERS;

const STATE: Record<GitBranchState, [Tone, string]> = {
  aktuell: ['info', 'aktuell'], worktree: ['info', 'in Worktree'], basis: ['neutral', 'Basis'],
  gemergt: ['ok', 'gemergt'], squash: ['ok', 'per Squash gemergt'], ungemergt: ['warn', 'ungemergt'],
};

const RANK: Record<GitBranchState, number> = { aktuell: 0, basis: 1, worktree: 2, ungemergt: 3, squash: 4, gemergt: 5 };

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
  confirm: (r: ConfirmRequest) => void;
}

/** Lokale und entfernte Branches; markierte gemergte löscht die Seite selbst, alles andere geht als Prompt an Claude. */
export function BranchesCard({ branches, remove, confirm }: Props) {
  const [filter, setFilter] = useState<Filter>('alle');
  const [marked, setMarked] = useState<Set<string>>(new Set());
  const [remote, setRemote] = useState(false);
  const shown = useMemo(() => sortBranches(filterBranches(branches, filter)), [branches, filter]);
  const names = branches.filter((b) => marked.has(b.name) && b.deletable).map((b) => b.name);
  const toggle = (name: string, on: boolean) => setMarked((m) => {
    const next = new Set(m);
    if (on) next.add(name);
    else next.delete(name);
    return next;
  });
  const del = () => confirm({
    title: `${names.length} Branches löschen?`, action: 'Löschen', danger: true,
    text: `${names.join(', ')} ${remote ? 'lokal und auf origin' : 'lokal'}. develop enthält ihren Inhalt; die Workbench prüft das vor dem Löschen noch einmal.`,
    run: () => void remove(names, remote).then(() => setMarked(new Set())),
  });
  return (
    <section className="git-card" aria-label="Branches">
      <header className="git-head">
        <Text weight="bold">Branches <Text color="gray" size="1">({branches.length})</Text></Text>
        <Flex gap="3" align="center" wrap="wrap">
          <SegmentedControl.Root size="1" value={filter} onValueChange={(v) => setFilter(v as Filter)}>
            {Object.entries(FILTERS).map(([k, label]) => <SegmentedControl.Item key={k} value={k}>{label}</SegmentedControl.Item>)}
          </SegmentedControl.Root>
          <CheckField size="1" label="auch auf origin" checked={remote} onCheckedChange={(v) => setRemote(v === true)} />
          <Button size="1" color="red" variant="soft" disabled={names.length === 0} onClick={del}>Markierte löschen ({names.length})</Button>
          <Text size="1" color="gray">Prompt:</Text>
          {names.length > 0 && <CopyPrompt prompt={cleanupMarkedPrompt(names)} what="Markierte aufräumen" />}
          <CopyPrompt prompt={cleanupStalePrompt(branches)} what="Veraltete aufräumen" />
        </Flex>
      </header>
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
          {shown.map((b) => <BranchRow key={`${b.remote}:${b.name}`} b={b} marked={marked.has(b.name)} toggle={toggle} />)}
        </Table.Body>
      </Table.Root>
      {shown.length === 0 && <Text size="2" color="gray">Keine Branches in dieser Auswahl.</Text>}
    </section>
  );
}

function BranchRow({ b, marked, toggle }: { b: GitBranch; marked: boolean; toggle: (n: string, on: boolean) => void }) {
  const [tone, label] = STATE[b.state];
  const track = b.track === 'voraus' ? `+${b.ahead}` : b.track === 'zurück' ? `−${b.behind}` : b.track === 'auseinander' ? `+${b.ahead} / −${b.behind}` : '';
  return (
    <Table.Row>
      <Table.Cell>
        {b.deletable && <Checkbox size="1" aria-label={`${b.name} markieren`} checked={marked} onCheckedChange={(v) => toggle(b.name, v === true)} />}
      </Table.Cell>
      <Table.Cell><Code size="1" variant="ghost">{b.remote ? `origin/${b.name}` : b.name}</Code></Table.Cell>
      <Table.Cell><Text size="1" color={b.track === 'gone' ? 'red' : 'gray'}>{b.track}{track && ` ${track}`}</Text></Table.Cell>
      <Table.Cell><Text size="1" color="gray">+{b.baseAhead} / −{b.baseBehind}</Text></Table.Cell>
      <Table.Cell title={b.reason}><StatusBadge tone={tone}>{label}</StatusBadge></Table.Cell>
      <Table.Cell><Text size="1" color="gray" title={b.subject} className="git-ellipsis">{ago(b.date * 1000)} · {b.subject}</Text></Table.Cell>
    </Table.Row>
  );
}
