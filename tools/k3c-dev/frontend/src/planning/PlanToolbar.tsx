import { Badge, Button, DropdownMenu, Text, TextField } from '@radix-ui/themes';
import type { PlanningData } from '../api';
import { AREA_LABEL, filterChips, filterGroups, isDirty, JOIN_HINT, resetFilter, toggleValue, type FilterArea, type FilterGroup, type GroupKey } from './filterGroups';
import type { PlanFilter } from './planning';
import { RemovableChip } from './Pill';

const AREAS: FilterArea[] = ['common', 'projects', 'backlog'];

interface Props {
  data: PlanningData;
  filter: PlanFilter;
  setFilter: (f: PlanFilter) => void;
  /** Gezeigte Projekte und Tickets. */
  hits: number;
}

/** Knopf `Gruppe ▾` mit Zähler; die Checkliste bleibt beim Anhaken offen, Esc schließt. */
function GroupMenu({ group, onToggle }: { group: FilterGroup; onToggle: (key: GroupKey, id: string) => void }) {
  const n = group.selected.length;
  return (
    <DropdownMenu.Root>
      <DropdownMenu.Trigger>
        <Button aria-label={`${group.label}, ${n} angehakt`} color="gray" size="1" title={`${group.label}: ${JOIN_HINT[group.join]}`}
          variant={n ? 'soft' : 'outline'}>
          {group.label}
          {n > 0 && <Badge radius="full" size="1">{n}</Badge>}
          <DropdownMenu.TriggerIcon />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content size="1">
        <DropdownMenu.Label>{JOIN_HINT[group.join]}</DropdownMenu.Label>
        {group.options.map((o) => (
          <DropdownMenu.CheckboxItem key={o.id} checked={group.selected.includes(o.id)} onCheckedChange={() => onToggle(group.key, o.id)}
            onSelect={(e) => e.preventDefault()}>
            {o.label}
            {o.count !== undefined && <span className="pl-tb-count">{o.count}</span>}
          </DropdownMenu.CheckboxItem>
        ))}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  );
}

/**
 * Filterleiste über beiden Spalten, gegliedert nach Wirkung (B-439): Gemeinsam (Suche, Prio, Umgebung, Projekt) wirkt
 * auf die Sprints der Projekte und den Backlog, Projekte (Zustand, Sprints, Domänen) nur auf die Projekte, Backlog
 * (Einplanung) nur auf die Tickets. Angehaktes steht als entfernbarer Chip darunter, rechts die Trefferzahl.
 */
export function PlanToolbar({ data, filter, setFilter, hits }: Props) {
  const groups = filterGroups(filter, data);
  const toggle = (key: GroupKey, id: string) => setFilter(toggleValue(filter, key, id));
  return (
    <div className="pl-toolbar">
      {AREAS.map((area) => (
        <div key={area} role="group" aria-label={AREA_LABEL[area]} className="pl-tb-area">
          <Text size="1" className="pl-tb-label">{AREA_LABEL[area]}</Text>
          {area === 'common' && (
            <TextField.Root size="1" className="pl-tb-search" aria-label="Planung durchsuchen" placeholder="Suche: Nr., Titel, Ticket …"
              value={filter.q} onChange={(e) => setFilter({ ...filter, q: e.target.value })} />
          )}
          {groups.filter((g) => g.area === area && g.options.length > 0).map((g) => <GroupMenu key={g.key} group={g} onToggle={toggle} />)}
        </div>
      ))}
      <div role="group" aria-label="Angehakte Filter" className="pl-tb-chips">
        {filterChips(groups).map((c) => (
          <RemovableChip key={`${c.group}:${c.id}`} group={c.groupLabel} label={c.label} onRemove={() => toggle(c.group, c.id)} />
        ))}
        {isDirty(filter) && (
          <button type="button" className="pl-textlink" title="Suche leeren und alle Häkchen entfernen" onClick={() => setFilter(resetFilter(filter))}>
            alle zurücksetzen
          </button>
        )}
        <span className="pl-tb-hits">{hits} Treffer</span>
      </div>
    </div>
  );
}
