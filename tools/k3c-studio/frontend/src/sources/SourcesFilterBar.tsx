import { Button, Flex, IconButton, SegmentedControl, SelectField, Text, TextField, Tip, type SelectOption } from '@webkit/atoms';
import { CheckField, Popover } from '@webkit/molecules';
import { ART_OPTIONS, extraFilters, STATUS_OPTIONS, type ArtFilter, type SourceFilter, type StatusFilter } from './sourcesFilter';

const ALL = '';

interface Props {
  filter: SourceFilter;
  onChange: (patch: Partial<SourceFilter>) => void;
  slots: readonly string[];
  licenses: readonly string[];
  /** Vorrat aufräumen und neu laden */
  busy: boolean;
  onCleanup: () => void;
  onReload: () => void;
}

const options = (values: readonly string[], all: string): SelectOption<string>[] => [{ value: ALL, label: all }, ...values.map((v) => ({ value: v, label: v }))];

/** Weitere Filter hinter „Filter ▾“: Slot, Lizenz und gesperrte Einträge. */
function MoreFilters({ filter, onChange, slots, licenses }: Pick<Props, 'filter' | 'onChange' | 'slots' | 'licenses'>) {
  const n = extraFilters(filter);
  return (
    <Popover.Root>
      <Popover.Trigger>
        <Button size="1" variant={n > 0 ? 'solid' : 'soft'} color={n > 0 ? undefined : 'gray'} aria-label="Weitere Filter">Filter{n > 0 ? ` (${n})` : ''} ▾</Button>
      </Popover.Trigger>
      <Popover.Content size="1" width="280px">
        <Flex direction="column" gap="3">
          <Flex direction="column" gap="1">
            <Text size="1" color="gray">Slot</Text>
            <SelectField size="1" aria-label="Slot" value={filter.slot} options={options(slots, 'alle Slots')} onChange={(slot) => onChange({ slot })} />
          </Flex>
          <Flex direction="column" gap="1">
            <Text size="1" color="gray">Lizenz</Text>
            <SelectField size="1" aria-label="Lizenz" value={filter.lizenz} options={options(licenses, 'alle Lizenzen')} onChange={(lizenz) => onChange({ lizenz })} />
          </Flex>
          <CheckField size="1" label="gesperrte zeigen" checked={filter.gesperrte} onCheckedChange={(c) => onChange({ gesperrte: c === true })} />
        </Flex>
      </Popover.Content>
    </Popover.Root>
  );
}

/** Eine Filterzeile (B-673): Suche, Art und Status als Segmente, „Filter ▾“, Symbol-Knöpfe Aufräumen und Neu laden. */
export function SourcesFilterBar({ filter, onChange, slots, licenses, busy, onCleanup, onReload }: Props) {
  return (
    <Flex gap="2" align="center" wrap="wrap" data-testid="source-filter">
      <TextField.Root size="1" style={{ minWidth: 200, flex: '1 1 200px' }} placeholder="Suchen: Titel, Slot, Urheber …" aria-label="Suche" value={filter.query} onChange={(e) => onChange({ query: e.target.value })} />
      <SegmentedControl.Root size="1" value={filter.art} onValueChange={(v) => onChange({ art: v as ArtFilter })} aria-label="Art">
        {ART_OPTIONS.map((o) => <SegmentedControl.Item key={o.value} value={o.value}>{o.label}</SegmentedControl.Item>)}
      </SegmentedControl.Root>
      <SegmentedControl.Root size="1" value={filter.status} onValueChange={(v) => onChange({ status: v as StatusFilter })} aria-label="Status">
        {STATUS_OPTIONS.map((o) => <SegmentedControl.Item key={o.value} value={o.value}>{o.label}</SegmentedControl.Item>)}
      </SegmentedControl.Root>
      <MoreFilters filter={filter} onChange={onChange} slots={slots} licenses={licenses} />
      <Tip content="Vorrat aufräumen"><IconButton size="1" variant="soft" color="gray" disabled={busy} aria-label="Vorrat aufräumen" onClick={onCleanup}>🧹</IconButton></Tip>
      <Tip content="Neu laden"><IconButton size="1" variant="soft" color="gray" aria-label="Neu laden" onClick={onReload}>↻</IconButton></Tip>
    </Flex>
  );
}
