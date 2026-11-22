import { Badge, Flex, Heading, SelectField, Text } from '@webkit/atoms';
import type { NatureValues as Values, WorldLevel } from '../api';

// Natur der Stufe (B-891, docs/rules/welten.md § 2.1): je Eigenschaft und Variante der geltende Wert, ob er aus dem
// Seed kommt oder in der Welt fest steht, und eine Auswahl zum Festsetzen; „aus Seed“ setzt zurück. Geschrieben wird
// sofort in natureByDepth der Insel, geprüft wie im Server; die Chunk-Vorschau zeigt danach das neue Level.

/** Eine Zeile: Schlüssel für das Backend (Eigenschaft oder „gruppe.variante“), Anzeige und Bereich. */
export interface NatureRow {
  key: string;
  label: string;
  hint: string;
  min: number;
  max: number;
  value: number | undefined;
  fixed: number | undefined;
}

const effect = (t: { scales?: string[]; chunks?: string[]; display?: boolean }) =>
  t.display ? 'nur Darstellung' : t.chunks?.length ? `Chunk-Anteil ${t.chunks.join(', ')}` : `Anzahl ${t.scales?.join(', ') ?? ''}`;

/** Zeilen der Stufe: erst die Eigenschaften, dann je Gewichts-Gruppe ihre Varianten (Reihenfolge wie im Biom). */
export function natureRows(level: WorldLevel): NatureRow[] {
  const now: Values = level.nature ?? {};
  const fixed: Values = level.natureFixed ?? {};
  const traits = (level.natureTraits ?? []).map((t) => ({
    key: t.key, label: t.name, hint: `${effect(t)} · neutral ${t.default}`, min: t.min, max: t.max,
    value: now.traits?.[t.key], fixed: fixed.traits?.[t.key],
  }));
  const weights = (level.natureWeights ?? []).flatMap((g) => g.options.map((o) => ({
    key: `${g.key}.${o.key}`, label: `${g.name}: ${o.key}`, hint: `Gewicht je ${g.kind}`, min: o.min, max: o.max,
    value: now.weights?.[g.key]?.[o.key], fixed: fixed.weights?.[g.key]?.[o.key],
  })));
  return [...traits, ...weights];
}

const SEED = 'seed';

function NatureLine({ row, onSet }: { row: NatureRow; onSet: (key: string, value: number | null) => void }) {
  const values = Array.from({ length: row.max - row.min + 1 }, (_, i) => String(row.min + i));
  const options = [{ value: SEED, label: row.fixed === undefined ? `aus Seed (${row.value ?? '–'})` : 'aus Seed' }, ...values.map((v) => ({ value: v, label: v }))];
  const own = row.fixed !== undefined;
  return (
    <li className="wv-row" data-key={row.key} data-status={own ? 'fest' : 'seed'}>
      <Text size="2" className="wv-path">{row.label}</Text>
      <Text size="1" color="gray">{row.hint} · {row.min}–{row.max}</Text>
      <SelectField size="1" aria-label={`Wert ${row.label}`} value={own ? String(row.fixed) : SEED} options={options}
        onChange={(v) => onSet(row.key, v === SEED ? null : Number(v))} />
      <Badge color={own ? 'orange' : 'gray'} aria-label={`Quelle ${row.label}`}>{own ? 'fest' : `Seed: ${row.value ?? '–'}`}</Badge>
    </li>
  );
}

export function NatureValues({ level, onSet }: { level: WorldLevel | null; onSet: (key: string, value: number | null) => void }) {
  if (!level) return null;
  const rows = natureRows(level);
  return (
    <Flex direction="column" gap="2" aria-label="Natur der Stufe">
      <Heading size="3">Natur der Stufe</Heading>
      {rows.length === 0
        ? <Text size="1" color="gray">Das Biom {level.biomeId} hat keine Natur-Eigenschaften.</Text>
        : <ul className="wv-list">{rows.map((r) => <NatureLine key={r.key} row={r} onSet={onSet} />)}</ul>}
    </Flex>
  );
}
