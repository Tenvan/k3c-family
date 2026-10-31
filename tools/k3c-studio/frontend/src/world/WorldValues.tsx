import { Button, SelectField, Text, TextField } from '@webkit/atoms';
import { useEffect, useMemo, useState } from 'react';
import type { WorldValuesView } from '../api';
import type { WorldDraftState } from './useWorldDraft';
import { filterRows, parseValue, resetOverride, setOverride, valueRows, valueText, type ValueRow } from './worldOverrides';

/** Eltern-Welt und Kind-Welten: eine Änderung wirkt auf alle Welten, die von dieser erben (welten.md § 1). */
export function WorldKin({ values }: { values: WorldValuesView }) {
  const kids = values.children;
  return (
    <div className="wd-bar wv-kin">
      <Text size="1" color="gray" aria-label="Eltern-Welt">{values.extends ? `erbt von „${values.extends}“` : 'erbt von der Basis (data/)'}</Text>
      <Text size="1" color={kids.length > 0 ? 'orange' : 'gray'} aria-label="Kind-Welten">
        {kids.length > 0 ? `Kind-Welten: ${kids.join(', ')}; Änderungen wirken auch dort` : 'keine Kind-Welten'}
      </Text>
    </div>
  );
}

/** Ein Feld: Basiswert, Wert der Welt (Eingabe, übernommen bei Enter oder Verlassen), Status und Zurücksetzen. */
function ValueLine({ row, onSet, onReset }: { row: ValueRow; onSet: (value: unknown) => void; onReset: () => void }) {
  const shown = valueText(row.override ?? row.base);
  const [text, setText] = useState(shown);
  useEffect(() => setText(shown), [shown]);
  const commit = () => { if (text !== shown) onSet(parseValue(text, row.base)); };
  const own = row.status === 'überschrieben';
  return (
    <li className="wv-row" data-key={row.key} data-status={row.status}>
      <Text size="2" className="wv-path">{row.label}</Text>
      <Text size="1" color="gray" className="wv-base" aria-label={`Basiswert ${row.label}`}>{valueText(row.base)}</Text>
      <TextField.Root size="1" className="wv-input" aria-label={`Wert ${row.label}`} value={text} onChange={(e) => setText(e.target.value)}
        onBlur={commit} onKeyDown={(e) => { if (e.key === 'Enter') commit(); }} />
      <Text size="1" weight={own ? 'bold' : 'regular'} color={own ? 'orange' : 'gray'} className="wv-status" aria-label={`Status ${row.label}`}>{row.status}</Text>
      <Button size="1" variant="soft" disabled={!own} aria-label={`Zurücksetzen ${row.label}`} title="Override entfernen, der Basiswert gilt" onClick={onReset}>↺</Button>
    </li>
  );
}

/** Ansicht „Werte“ des Welten Editors (WZ37.1, B-682): je Feld einer Basis-Datei Basiswert, Wert der Welt und Status
 *  (geerbt/überschrieben), Zurücksetzen je Feld. Änderungen gehen in den Welt-Entwurf (`overrides`, nur Abweichungen);
 *  geprüft wird wie im Server, gespeichert über „Welt speichern“ im Kopf. Die Kind-Welten stehen vor dem Speichern da. */
export function WorldValues({ world }: { world: WorldDraftState }) {
  const { draft, values, check } = world;
  const rows = useMemo(() => valueRows(values?.entries ?? [], draft?.overrides), [values, draft?.overrides]);
  const files = useMemo(() => [...new Set(rows.map((r) => r.file))], [rows]);
  const [picked, setFile] = useState('buildings');
  const [query, setQuery] = useState('');
  const [onlyOwn, setOnlyOwn] = useState(false);
  if (!draft || !values) return null;
  const file = files.includes(picked) ? picked : (files[0] ?? '');
  const shown = filterRows(rows, file, query, onlyOwn);
  const ownIn = (f: string) => rows.filter((r) => r.file === f && r.status === 'überschrieben').length;
  const edit = (overrides: typeof draft.overrides) => world.edit({ ...draft, overrides });
  return (
    <section className="wd-card wv-root" aria-label="Werte der Welt">
      <div className="wd-bar">
        <Text size="2" weight="bold">Werte</Text>
        <WorldKin values={values} />
      </div>
      {values.error && <Text as="p" size="1" color="red" role="alert">{values.error}</Text>}
      {!check.ok && check.field === 'overrides' && <Text as="p" size="1" color="red" role="alert" className="wd-error" data-island={-1} data-field="overrides">{check.error}</Text>}
      <div className="wd-bar">
        <SelectField aria-label="Basis-Datei" value={file} onChange={setFile}
          options={files.map((f) => ({ value: f, label: ownIn(f) > 0 ? `${f} (${ownIn(f)} überschrieben)` : f }))} />
        <TextField.Root aria-label="Feld suchen" placeholder="Feld suchen, z. B. wall.cost" value={query} onChange={(e) => setQuery(e.target.value)} />
        <Button variant={onlyOwn ? 'solid' : 'soft'} aria-pressed={onlyOwn} onClick={() => setOnlyOwn((v) => !v)}>Nur überschriebene</Button>
      </div>
      <ul className="wv-list" aria-label="Felder">
        {shown.map((r) => (
          <ValueLine key={r.key} row={r} onSet={(v) => edit(setOverride(draft.overrides, r.file, r.path, v, r.base))} onReset={() => edit(resetOverride(draft.overrides, r.file, r.path))} />
        ))}
      </ul>
      {shown.length === 0 && <Text size="1" color="gray">Keine Felder.</Text>}
    </section>
  );
}
