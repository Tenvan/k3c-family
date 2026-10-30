import { Badge, Button, Flex, Text } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import type { SourceProgress } from '../api';
import { ProposalRow } from './ProposalRow';
import type { SlotGroup } from './sourcesFilter';
import { downloadable } from './sourcesLogic';

interface ListProps {
  groups: readonly SlotGroup[];
  collapsed: ReadonlySet<string>;
  offline: boolean;
  picked: ReadonlySet<string>;
  jobs: Map<string, SourceProgress>;
  onFold: (slot: string) => void;
  onToggle: (ids: string[], on: boolean) => void;
  onAdopt: (id: string) => void;
}

/** Nach Slot gruppierte Liste (B-673): Kopf mit Slot, Zahl der Kandidaten und „Lücke“ bzw. „belegt“, darunter die Zeilen. */
export function SourceList({ groups, collapsed, offline, picked, jobs, onFold, onToggle, onAdopt }: ListProps) {
  return (
    <Flex direction="column" gap="2" data-testid="source-groups">
      {groups.map((g) => {
        const open = !collapsed.has(g.slot);
        const ladbar = g.items.filter(downloadable).map((p) => p.id);
        const allOn = ladbar.length > 0 && ladbar.every((id) => picked.has(id));
        return (
          <section key={g.slot} className="src-group" data-slot={g.slot}>
            <Flex gap="2" align="center" className="src-head">
              <Button size="1" variant="ghost" color="gray" aria-expanded={open} aria-label={`${g.slot}: ${open ? 'zuklappen' : 'aufklappen'}`} onClick={() => onFold(g.slot)}>{open ? '▾' : '▸'}</Button>
              <Text weight="bold" size="2">{g.slot}</Text>
              <Text size="1" color="gray">{g.items.length} {g.items.length === 1 ? 'Kandidat' : 'Kandidaten'}</Text>
              <Badge size="1" variant="soft" color={g.gap ? 'amber' : 'green'}>{g.gap ? 'Lücke' : 'belegt'}</Badge>
              {!offline && ladbar.length > 0 && <CheckField size="1" label={`ladbare wählen (${ladbar.length})`} checked={allOn} onCheckedChange={(c) => onToggle(ladbar, c === true)} />}
            </Flex>
            {open && g.items.map((p) => (
              <ProposalRow key={p.id} s={p} offline={offline} checked={picked.has(p.id)} job={jobs.get(p.id)} onCheck={(on) => onToggle([p.id], on)} onAdopt={() => onAdopt(p.id)} />
            ))}
          </section>
        );
      })}
    </Flex>
  );
}

interface EmptyProps {
  /** Quelle ohne Vorschläge (Reiter), "" = keine Quelle gewählt */
  source: string;
  /** die Quelle hat Einträge, der Filter blendet sie aus */
  filtered: boolean;
  offline: boolean;
  onSearch: () => void;
  onResearch: () => void;
  onReset: () => void;
}

/** Hinweis statt leerer Liste: Quelle ohne Vorschläge bietet beide Wege an, ein zu enger Filter lässt sich zurücksetzen. */
export function SourceEmpty({ source, filtered, offline, onSearch, onResearch, onReset }: EmptyProps) {
  if (filtered) {
    return (
      <Flex gap="3" align="center" data-testid="source-empty">
        <Text color="gray">Keine Vorschläge für diesen Filter.</Text>
        <Button size="1" variant="soft" onClick={onReset}>Filter zurücksetzen</Button>
      </Flex>
    );
  }
  return (
    <Flex direction="column" gap="2" p="3" align="start" data-testid="source-empty" style={{ border: '1px dashed var(--gray-7)', borderRadius: 6 }}>
      <Text>{source ? `Noch keine Vorschläge von ${source}.` : 'Noch keine Vorschläge.'}</Text>
      <Flex gap="2" wrap="wrap">
        <Button size="1" onClick={onSearch}>{source ? `Claude in ${source} suchen lassen` : 'Claude suchen lassen'}</Button>
        <Button size="1" variant="soft" disabled={offline} onClick={onResearch}>Aus Recherche übernehmen</Button>
      </Flex>
    </Flex>
  );
}
