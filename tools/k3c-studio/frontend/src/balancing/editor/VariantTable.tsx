import { useState } from 'react';
import { Button, Code, IconButton, Text, TextArea, TextField } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { HelpButton } from '../HelpButton';
import { FIELD_HELP } from '../help';
import type { TuneForm } from '../tune';
import { Choice } from './Choice';
import { TuneBox } from './TuneBox';
import type { RowCheck } from './variantCheck';
import { canPerVariant } from './variantForm';
import { countKinds, deviation, rowsFromText, rowsToText, switchKind, type VariantRow } from './variants';

interface Props {
  rows: VariantRow[];
  /** Prüfung je Zeile (checkRows), gleiche Reihenfolge wie `rows`. */
  checks: RowCheck[];
  onRows: (rows: VariantRow[]) => void;
  tune: TuneForm;
  onTune: (t: TuneForm) => void;
  perVariant: boolean;
  onPerVariant: (on: boolean) => void;
}

const KINDS = [{ value: 'fest', label: 'fest' }, { value: 'bereich', label: 'Bereich' }];

interface RowProps {
  row: VariantRow;
  check: RowCheck;
  onRow: (r: VariantRow) => void;
  onRemove: () => void;
}

function NewValue({ row, onRow }: Pick<RowProps, 'row' | 'onRow'>) {
  if (row.kind === 'fest') {
    return <TextField.Root size="1" className="bal-ed-num" aria-label={`Neuer Wert für ${row.path}`} value={row.value} onChange={(e) => onRow({ ...row, value: e.target.value })} />;
  }
  return (
    <span className="bal-ed-range">
      <TextField.Root size="1" className="bal-ed-num" aria-label={`Von für ${row.path}`} value={row.lo} onChange={(e) => onRow({ ...row, lo: e.target.value })} />
      <Text size="1" color="gray">bis</Text>
      <TextField.Root size="1" className="bal-ed-num" aria-label={`Bis für ${row.path}`} value={row.hi} onChange={(e) => onRow({ ...row, hi: e.target.value })} />
    </span>
  );
}

function Message({ check, row, onRow }: Pick<RowProps, 'check' | 'row' | 'onRow'>) {
  const fix = check.fix;
  return (
    <>
      {check.error && (
        <span className="bal-ed-err" role="alert">
          {check.error}
          {fix && <Button size="1" variant="soft" aria-label={`Vorschlag übernehmen: ${fix.path}`} onClick={() => onRow({ ...row, file: fix.file, path: fix.path })}>Übernehmen</Button>}
        </span>
      )}
      {check.note && <Text size="1" color="amber" as="span">{check.note}</Text>}
    </>
  );
}

function Row({ row, check, onRow, onRemove }: RowProps) {
  const dev = deviation(row, check.base);
  const label = row.raw ?? row.path;
  return (
    <tr data-kind={row.kind} data-bad={check.error ? '' : undefined}>
      <td className="bal-ed-where">
        <Text size="1" color="gray">{row.raw === undefined ? row.file : 'Zeile'}</Text>
        <Code size="1" color={check.error ? 'red' : undefined}>{label}</Code>
        <Message check={check} row={row} onRow={onRow} />
      </td>
      <td className="bal-ed-base">{check.base ?? '–'}</td>
      <td>{row.raw === undefined && <NewValue row={row} onRow={onRow} />}</td>
      <td className="bal-ed-dev" data-tone={dev.tone}>{dev.text}</td>
      <td>{row.raw === undefined && <Choice label={`Art für ${row.path}`} size="1" value={row.kind} options={KINDS} onChange={(k) => onRow(switchKind(row, k as VariantRow['kind'], check.base))} />}</td>
      <td><IconButton size="1" variant="ghost" color="gray" aria-label={`Variante ${label} entfernen`} onClick={onRemove}>×</IconButton></td>
    </tr>
  );
}

function Table({ rows, checks, onRows }: Pick<Props, 'rows' | 'checks' | 'onRows'>) {
  if (rows.length === 0) return <Text size="2" color="gray" as="p">Keine Varianten: Der Lauf spielt die Werte aus data/. Im Daten-Browser „+ fest“ oder „+ Bereich“ wählen.</Text>;
  const set = (id: number) => (r: VariantRow) => onRows(rows.map((x) => (x.id === id ? r : x)));
  return (
    <div className="bal-ed-tablewrap">
      <table className="bal-ed-table" aria-label="Varianten">
        <thead>
          <tr><th>Wert</th><th>Basis</th><th>Neu</th><th>Abweichung</th><th>Art</th><th><span className="bal-ed-sr">Entfernen</span></th></tr>
        </thead>
        <tbody>
          {rows.map((r, i) => <Row key={r.id} row={r} check={checks[i] ?? {}} onRow={set(r.id)} onRemove={() => onRows(rows.filter((x) => x.id !== r.id))} />)}
        </tbody>
      </table>
    </div>
  );
}

function TextMode({ text, onText, checks }: { text: string; onText: (t: string) => void; checks: RowCheck[] }) {
  const bad = checks.map((c, i) => (c.error ? `Zeile ${i + 1}: ${c.error}` : '')).filter(Boolean);
  return (
    <>
      <TextArea aria-label="Varianten als Text" rows={Math.max(4, text.split('\n').length + 1)} placeholder="economy.json:purse.startGold=120" value={text} onChange={(e) => onText(e.target.value)} />
      <Text size="1" color="gray">Eine Variante je Zeile: <Code size="1">datei.json:pfad=wert</Code> (fest) oder <Code size="1">datei.json:pfad=min..max</Code> (Bereich). Einfügen aus der Zwischenablage geht hier.</Text>
      {bad.map((b) => <span key={b} className="bal-ed-err" role="alert">{b}</span>)}
    </>
  );
}

/** Varianten-Tabelle (B-661): Basis, Neu, Abweichung, Art fest/Bereich; „Als Text bearbeiten“ im heutigen Zeilenformat;
 *  ab einem Bereich der Abstimm-Kasten; Option „je Variante“. */
export function VariantTable({ rows, checks, onRows, tune, onTune, perVariant, onPerVariant }: Props) {
  const [typed, setText] = useState<string | null>(null);
  // Ändern sich die Zeilen von außen (Daten-Browser, Zurücksetzen, Ausgangspunkt), gilt der Text der Zeilen, nicht der getippte.
  const text = typed !== null && rowsToText(rowsFromText(typed)) !== rowsToText(rows) ? rowsToText(rows) : typed;
  const { fest, bereich } = countKinds(rows);
  const can = canPerVariant(rows);
  const edit = (t: string) => { setText(t); onRows(rowsFromText(t)); };
  return (
    <div className="bal-ed-variants">
      <div className="bal-ed-vtools">
        <Button size="1" variant="soft" onClick={() => setText(text === null ? rowsToText(rows) : null)}>{text === null ? 'Als Text bearbeiten' : 'Als Tabelle bearbeiten'}</Button>
        <HelpButton title={FIELD_HELP.vary.label} source={FIELD_HELP.vary.help} />
      </div>
      {text === null ? <Table rows={rows} checks={checks} onRows={onRows} /> : <TextMode text={text} onText={edit} checks={checks} />}
      {bereich > 0 && <TuneBox tune={tune} onTune={onTune} ranges={bereich} />}
      <CheckField
        size="2" label={`Jede feste Variante als eigener Lauf gegen die Basis (${fest + 1} Läufe)`}
        checked={perVariant && can} disabled={!can} onCheckedChange={(c) => onPerVariant(c === true)}
      />
      {!can && <Text size="1" color="gray">{bereich > 0 ? 'Nicht mit Abstimmung: Bereiche und „je Variante“ schließen sich aus.' : 'Braucht mindestens eine feste Variante.'}</Text>}
    </div>
  );
}
