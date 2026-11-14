import { useEffect, useMemo, useRef, useState } from 'react';
import { Button, Flex, SelectField, Text, TextField } from '@webkit/atoms';
import type { WorldValuesView } from '../api';
import { filesOf, filterRows, parseValue, valueRows, valueText, type ValueRow } from './worldOverrides';

// Ansicht „Werte“ des Welten-Editors (B-879, Port aus dem Asset-Studio, world/WorldValues.tsx; B-680, B-682): je Feld
// einer Basis-Datei Basiswert, Wert der Welt und Status (geerbt/überschrieben). Eine Eingabe (Enter oder Verlassen)
// schreibt den Override sofort in die Welt-Datei, geprüft wie im Server; ↺ setzt ihn zurück.

/** Eltern-Welt und Kind-Welten: eine Änderung wirkt auf alle Welten, die von dieser erben (welten.md § 1). */
function WorldKin({ values }: { values: WorldValuesView }) {
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

/** Ein Feld: Basiswert, Wert der Welt (Eingabe), Status und Zurücksetzen. */
function ValueLine({ row, onSet }: { row: ValueRow; onSet: (value: unknown) => void }) {
  const shown = valueText(row.override ?? row.base);
  const [text, setText] = useState(shown);
  // zuletzt gesendet: Enter und das folgende Verlassen schicken denselben Wert nur einmal
  const sent = useRef<string | null>(null);
  useEffect(() => { setText(shown); sent.current = null; }, [shown]);
  const commit = () => {
    if (text === shown || text === sent.current) return;
    sent.current = text;
    onSet(parseValue(text, row.base));
  };
  const own = row.status === 'überschrieben';
  return (
    <li className="wv-row" data-key={row.key} data-status={row.status}>
      <Text size="2" className="wv-path">{row.label}</Text>
      <Text size="1" color="gray" aria-label={`Basiswert ${row.label}`}>{valueText(row.base)}</Text>
      <TextField.Root size="1" aria-label={`Wert ${row.label}`} value={text} onChange={(e) => setText(e.target.value)}
        onBlur={commit} onKeyDown={(e) => { if (e.key === 'Enter') commit(); }} />
      <Text size="1" weight={own ? 'bold' : 'regular'} color={own ? 'orange' : 'gray'} aria-label={`Status ${row.label}`}>{row.status}</Text>
      <Button size="1" variant="soft" disabled={!own} aria-label={`Zurücksetzen ${row.label}`} title="Override entfernen, der Basiswert gilt" onClick={() => onSet(null)}>↺</Button>
    </li>
  );
}

interface Props {
  values: WorldValuesView | null;
  onSet: (file: string, path: string[], value: unknown) => void;
}

export function WorldValues({ values, onSet }: Props) {
  const rows = useMemo(() => valueRows(values?.entries ?? []), [values]);
  const files = useMemo(() => filesOf(rows), [rows]);
  const [picked, setFile] = useState('buildings');
  const [query, setQuery] = useState('');
  const [onlyOwn, setOnlyOwn] = useState(false);
  if (!values) return null;
  const file = files.some((f) => f.file === picked) ? picked : (files[0]?.file ?? '');
  const shown = filterRows(rows, file, query, onlyOwn);
  return (
    <section className="wd-card" aria-label="Werte der Welt">
      <Flex gap="3" align="center" wrap="wrap">
        <Text size="2" weight="bold">Werte</Text>
        <WorldKin values={values} />
      </Flex>
      {values.error && <Text as="p" size="1" color="red" role="alert">{values.error}</Text>}
      <Flex gap="2" align="center" wrap="wrap">
        <SelectField size="1" aria-label="Basis-Datei" value={file} onChange={setFile}
          options={files.map((f) => ({ value: f.file, label: f.own > 0 ? `${f.file} (${f.own} überschrieben)` : f.file }))} />
        <TextField.Root size="1" aria-label="Feld suchen" placeholder="Feld suchen, z. B. wall.cost" value={query} onChange={(e) => setQuery(e.target.value)} />
        <Button size="1" variant={onlyOwn ? 'solid' : 'soft'} aria-pressed={onlyOwn} onClick={() => setOnlyOwn((v) => !v)}>Nur überschriebene</Button>
      </Flex>
      <ul className="wv-list" aria-label="Felder">
        {shown.map((r) => <ValueLine key={r.key} row={r} onSet={(v) => onSet(r.file, r.path, v)} />)}
      </ul>
      {shown.length === 0 && <Text size="1" color="gray">Keine Felder.</Text>}
    </section>
  );
}
