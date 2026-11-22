import { useState } from 'react';
import { Button, Flex, Text } from '@webkit/atoms';
import { ListFilter } from '@webkit/molecules';
import type { ReportRating } from '../api';
import { loadJSON, saveJSON } from '../lib/prefs';
import { EntryRow } from './EntryRow';
import { groupByDay, LIST_CONFIG, OLDER_COLLAPSE, type ListEntry } from './entries';
import { HelpButton } from './HelpButton';
import { SelectionBar } from './SelectionBar';

const PREFS = { loadJSON, saveJSON };

const HELP = `**Öffnen:** Klick auf den Namen zeigt den Eintrag rechts. Laufende und wartende Läufe stehen oben; ein fertiger Lauf wird zu seinem Bericht.

**Suchen und filtern:** Die Suche trifft Label, Bot, Welt und Varianten; die Chips (Status, Art) bleiben gemerkt, die Suche nicht.

**Auswählen:** Mit dem Kästchen wählst du Berichte. Die Leiste unten bietet **Vergleichen** (2 bis 4 Berichte, der zuerst gewählte ist **A**), **Bewerten** (gut, schlecht, unklar, Referenz; steht in \`reports/ratings.json\`) und **Löschen** (eine Rückfrage für alle, samt Markdown).`;

interface RowHandlers {
  selected: string;
  checked: string[];
  onSelect: (id: string) => void;
  onCheck: (id: string, on: boolean) => void;
}

interface Props extends RowHandlers {
  entries: ListEntry[];
  onClear: () => void;
  onReload: () => void;
  onCompare: (paths: string[]) => void;
  onRate: (paths: string[], rating: ReportRating) => void;
  onDelete: (paths: string[]) => void;
}

/** Gruppen Läuft, Heute, Gestern, Älter; „Älter“ ist ab 30 Einträgen eingeklappt, bis jemand es öffnet. */
function DayGroups({ items, ...row }: RowHandlers & { items: ListEntry[] }) {
  const [older, setOlder] = useState<boolean | null>(null);
  const now = new Date();
  return (
    <>
      {groupByDay(items, now).map((g) => {
        const fold = g.name === 'Älter';
        const open = !fold || (older ?? g.items.length < OLDER_COLLAPSE);
        return (
          <section key={g.name} className="bal-group" aria-label={g.name}>
            {fold ? (
              <Button size="1" variant="ghost" color="gray" className="bal-ghead" aria-expanded={open} onClick={() => setOlder(!open)}>{open ? '▾' : '▸'} {g.name} ({g.items.length})</Button>
            ) : (
              <Text size="1" weight="bold" color="gray" className="bal-ghead">{g.name} ({g.items.length})</Text>
            )}
            {open && g.items.map((e) => (
              <EntryRow key={e.id} entry={e} now={now} selected={e.id === row.selected} checked={row.checked.includes(e.id)} onSelect={row.onSelect} onCheck={row.onCheck} />
            ))}
          </section>
        );
      })}
    </>
  );
}

/** Eine Liste für Läufe und Berichte (B-660): Suche und Chips (ListFilter), Gruppen nach Tag, Leiste für die Auswahl. */
export function EntryList({ entries, selected, checked, onSelect, onCheck, onClear, onReload, onCompare, onRate, onDelete }: Props) {
  return (
    <>
      <Flex gap="2" align="center" wrap="wrap">
        <Text weight="bold">Läufe und Berichte</Text>
        <HelpButton title="Läufe und Berichte" source={HELP} />
        <Button size="1" variant="ghost" onClick={onReload}>Neu laden</Button>
      </Flex>
      <ListFilter
        {...LIST_CONFIG} items={entries} itemKey={(e) => e.id} storageKey="balancing.list" prefs={PREFS}
        emptyHint="Noch keine Läufe und Berichte (Berichte liegen unter reports/)."
        renderList={(items) => <DayGroups items={items} selected={selected} checked={checked} onSelect={onSelect} onCheck={onCheck} />}
      />
      <SelectionBar checked={checked} entries={entries} onCompare={onCompare} onRate={onRate} onDelete={onDelete} onClear={onClear} />
    </>
  );
}
