import { useMemo, useState } from 'react';
import { Badge, Button, Flex, IconButton, LinkButton, Text, TextField } from '@webkit/atoms';
import { deviation, valueText } from '@webkit/lib/valueEdit';
import { ChipSwitch, ValueField } from '@webkit/molecules';
import type { WorldValuesView } from '../api';
import { ValueTree, type TreePick } from './ValueTree';
import { filterRows, GENERAL, groupRows, valueRows, type RowGroup, type RowMode, type ValueRow } from './worldOverrides';

// Ansicht „Werte“ des Welten-Editors (B-879, B-680, B-682; neu gestaltet in B-1173): je Feld einer Basis-Datei der
// Standardwert, der Wert der Welt, Status und Abweichung. Gruppen nach Thema sind einklappbar, die Kopfzeile bleibt beim
// Scrollen stehen, die Suche trifft Pfad und Basiswert. Eine gültige Eingabe (Enter oder Verlassen) schreibt den Override
// sofort in die Welt-Datei, geprüft wie im Server; ↺ setzt die Zeile zurück.

/** Eltern-Welt und Kind-Welten: eine Änderung wirkt auf alle Welten, die von dieser erben (welten.md § 1). */
export function WorldKin({ values }: { values: WorldValuesView }) {
  const kids = values.children;
  return (
    <Flex gap="3" wrap="wrap">
      <Text size="1" color="gray" aria-label="Eltern-Welt">{values.extends ? `erbt von „${values.extends}“` : 'erbt von der Basis (data/)'}</Text>
      <Text size="1" color={kids.length > 0 ? 'orange' : 'gray'} aria-label="Kind-Welten">
        {kids.length > 0 ? `Kind-Welten: ${kids.join(', ')}; Änderungen wirken auch dort` : 'keine Kind-Welten'}
      </Text>
    </Flex>
  );
}

function ResetIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 15 15" fill="none" aria-hidden="true">
      <path d="M2.5 7.5a5 5 0 1 0 1.6-3.7M2.5 2.5v2.8h2.8" stroke="currentColor" strokeWidth="1.4" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  );
}

/** Ein Feld: Pfad, Standardwert, Eingabe, Status mit Abweichung und Zurücksetzen. */
export function ValueLine({ row, strip, onSet }: { row: ValueRow; strip: number; onSet: (value: unknown) => void }) {
  const own = row.status === 'überschrieben';
  const shown = valueText(own ? row.override : row.base);
  const diff = own ? deviation(row.base, row.override) : '';
  return (
    <div role="row" className="wv-row" data-key={row.key} data-status={row.status}>
      <Text role="cell" size="2" className="wv-path" title={row.label}>{row.path.slice(strip).join('.')}</Text>
      <Text role="cell" size="1" color="gray" className="wv-base" aria-label={`Basiswert ${row.label}`}>{valueText(row.base)}</Text>
      <div role="cell"><ValueField label={`Wert ${row.label}`} shown={shown} base={row.base} onCommit={onSet} /></div>
      <Flex role="cell" direction="column" gap="0">
        <Text size="1" weight={own ? 'bold' : 'regular'} color={own ? 'orange' : 'gray'} aria-label={`Status ${row.label}`}>{row.status}</Text>
        {diff && <Text size="1" color="orange" aria-label={`Abweichung ${row.label}`}>{diff}</Text>}
      </Flex>
      <div role="cell">
        <IconButton size="1" variant="soft" color={own ? 'orange' : 'gray'} disabled={!own} aria-label={`Zurücksetzen ${row.label}`}
          title="Override entfernen, der Basiswert gilt" onClick={() => onSet(null)}><ResetIcon /></IconButton>
      </div>
    </div>
  );
}

interface GroupProps {
  group: RowGroup;
  withFile: boolean;
  open: boolean;
  onToggle: () => void;
  onSet: (row: ValueRow, value: unknown) => void;
}

/** Gruppe mit einklappbarer Überschrift: Thema, Zahl der Felder und der überschriebenen. */
function Group({ group, withFile, open, onToggle, onSet }: GroupProps) {
  const strip = group.group === GENERAL ? 0 : 1;
  return (
    <div role="rowgroup" data-group={group.key.replace('\u0000', ':')}>
      <div role="row"><div role="cell" aria-colspan={5}>
      <Button type="button" variant="ghost" color="gray" className="wv-group" aria-expanded={open} onClick={onToggle}>
        <span className="wv-chev" aria-hidden="true" />
        <Text size="2" weight="bold">{withFile && <Text color="gray" weight="regular">{group.file} › </Text>}{group.group}</Text>
        <Text size="1" color="gray">{group.rows.length} {group.rows.length === 1 ? 'Feld' : 'Felder'}</Text>
        {group.own > 0 && <Badge color="orange" size="1">{group.own} überschrieben</Badge>}
      </Button>
      </div></div>
      {open && group.rows.map((r) => <ValueLine key={r.key} row={r} strip={strip} onSet={(v) => onSet(r, v)} />)}
    </div>
  );
}

export function TableHead() {
  return (
    <div role="rowgroup"><div role="row" className="wv-row wv-head">
      <Text role="columnheader" size="1" weight="medium">Feld</Text>
      <Text role="columnheader" size="1" weight="medium">Standard</Text>
      <Text role="columnheader" size="1" weight="medium">Wert dieser Welt</Text>
      <Text role="columnheader" size="1" weight="medium">Abweichung</Text>
      <Text role="columnheader" size="1" weight="medium"><span className="wv-sr">Zurücksetzen</span></Text>
    </div></div>
  );
}

interface Props {
  values: WorldValuesView | null;
  onSet: (file: string, path: string[], value: unknown) => void;
}

/** Ab so vielen Feldern starten die Gruppen eingeklappt (außer solche mit Überschreibungen). */
const COLLAPSE_FROM = 60;
const ALL_FILES = '';

export function WorldValues({ values, onSet }: Props) {
  const rows = useMemo(() => valueRows(values?.entries ?? []), [values]);
  const [pick, setPick] = useState<TreePick>({ file: '', group: null });
  const [query, setQuery] = useState('');
  const [mode, setMode] = useState<RowMode>('alle');
  const [touched, setTouched] = useState<ReadonlySet<string>>(new Set());
  // von Hand ein- oder ausgeklappt: überschreibt die Vorgabe der Gruppe
  const [toggled, setToggled] = useState<ReadonlyMap<string, boolean>>(new Map());
  if (!values) return null;
  const file = rows.some((r) => r.file === pick.file) ? pick.file : ALL_FILES;
  const only = file === ALL_FILES ? null : pick.group;
  const base = { file: ALL_FILES, query, mode: 'alle' as RowMode, touched };
  const inScope = filterRows(rows, { ...base, query: '' });
  const counts = { alle: inScope.length, ueberschrieben: inScope.filter((r) => r.status === 'überschrieben').length, sitzung: inScope.filter((r) => touched.has(r.key)).length };
  const forTree = filterRows(rows, { ...base, mode });
  const shown = forTree.filter((r) => (file === ALL_FILES || r.file === file) && (only === null || r.group === only));
  const groups = groupRows(shown);
  const isOpen = (g: RowGroup) => toggled.get(g.key) ?? (shown.length <= COLLAPSE_FROM || g.own > 0 || query.trim() !== '');
  const setAll = (open: boolean) => setToggled(new Map(groups.map((g) => [g.key, open])));
  const set = (r: ValueRow, v: unknown) => {
    setTouched((t) => new Set(t).add(r.key));
    onSet(r.file, r.path, v);
  };
  const filtered = query.trim() !== '' || mode !== 'alle';
  return (
    <section className="wd-card" aria-label="Werte der Welt" data-value-table>
      <Flex gap="3" align="baseline" wrap="wrap">
        <Text size="2" weight="bold">Werte</Text>
        <WorldKin values={values} />
      </Flex>
      {values.error && <Text as="p" size="1" color="red" role="alert">{values.error}</Text>}
      <Flex gap="2" align="center" wrap="wrap">
        <TextField.Root size="1" className="wv-search" aria-label="Feld suchen" placeholder="Feld oder Wert suchen, z. B. wall cost" value={query} onChange={(e) => setQuery(e.target.value)} />
        <ChipSwitch label="Zeilen" value={mode} onChange={setMode}
          options={[
            { value: 'alle', label: 'Alle', count: counts.alle },
            { value: 'ueberschrieben', label: 'Überschrieben', count: counts.ueberschrieben },
            { value: 'sitzung', label: 'Diese Sitzung', count: counts.sitzung },
          ]} />
      </Flex>
      <Flex gap="3" align="center" wrap="wrap">
        <Text size="1" color="gray" role="status">{shown.length} von {rows.length} Feldern in {groups.length} {groups.length === 1 ? 'Gruppe' : 'Gruppen'}</Text>
        {groups.length > 1 && <LinkButton size="1" onClick={() => setAll(true)}>alle aufklappen</LinkButton>}
        {groups.length > 1 && <LinkButton size="1" onClick={() => setAll(false)}>alle einklappen</LinkButton>}
        {filtered && <Button size="1" variant="soft" color="gray" onClick={() => { setQuery(''); setMode('alle'); }}>Filter zurücksetzen</Button>}
        <Text size="1" color="gray" className="wv-keys">Eingabe: Enter übernimmt, Esc verwirft, ↑↓ ändern Zahlen (Umschalt ×10), Alt+↑↓ wechselt die Zeile</Text>
      </Flex>
      <div className="vt-layout">
      <ValueTree rows={forTree} pick={{ file, group: only }} onPick={setPick} />
      <div role="table" aria-label="Felder" className="wv-table vt-fields">
        <TableHead />
        {groups.map((g) => (
          <Group key={g.key} group={g} withFile={file === ALL_FILES} open={isOpen(g)} onSet={set}
            onToggle={() => setToggled((m) => new Map(m).set(g.key, !isOpen(g)))} />
        ))}
      </div>
      </div>
      {shown.length === 0 && <Text size="1" color="gray">{filtered ? 'Kein Feld passt zum Filter.' : 'Keine Felder.'}</Text>}
    </section>
  );
}
