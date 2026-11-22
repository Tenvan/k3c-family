import { useMemo, useState } from 'react';
import { Button, Code, Flex, Text, TextField } from '@webkit/atoms';
import { backend, type BalanceDataFileValue } from '../../api';
import { HelpButton } from '../HelpButton';
import type { VariantKind } from './variants';

interface Props {
  /** Alle Werte (balanceDataValuesAll); null = lädt oder nicht ladbar. */
  values: BalanceDataFileValue[] | null;
  /** Fehler beim Laden. */
  error: string;
  /** Schlüssel `datei:pfad` der Werte, die schon in den Varianten stehen. */
  used: ReadonlySet<string>;
  onAdd: (v: BalanceDataFileValue, kind: VariantKind) => void;
}

/** Mehr Treffer zeigt der Browser nicht auf einmal (735 Werte, die Suche grenzt ein). */
export const MAX_HITS = 200;

/** Treffer der Suche: jedes Wort muss in Datei oder Pfad vorkommen (ohne Groß-/Kleinschreibung). */
export function searchValues(values: readonly BalanceDataFileValue[], query: string): BalanceDataFileValue[] {
  const words = query.toLowerCase().split(/\s+/).filter(Boolean);
  return words.length === 0 ? [...values] : values.filter((v) => words.every((w) => `${v.file} ${v.path}`.toLowerCase().includes(w)));
}

/** Beschreibung der Datei: `economy.json` → docs/data/economy.md. */
const docOf = (file: string) => file.replace(/\.json$/, '');

function ValueRow({ v, used, onAdd }: { v: BalanceDataFileValue; used: boolean; onAdd: Props['onAdd'] }) {
  return (
    <div className="bal-ed-val" data-path={v.path}>
      <Code size="1" className="bal-ed-vpath" title={v.path}>{v.path}</Code>
      <Text size="1" color="gray" className="bal-ed-vnum">{v.value}</Text>
      <Flex gap="1" align="center" className="bal-ed-vbtns">
        {used ? <Text size="1" color="green">in den Varianten</Text> : (
          <>
            <Button size="1" variant="soft" aria-label={`${v.path} als feste Variante`} onClick={() => onAdd(v, 'fest')}>+ fest</Button>
            <Button size="1" variant="soft" aria-label={`${v.path} als Bereich`} onClick={() => onAdd(v, 'bereich')}>+ Bereich</Button>
          </>
        )}
      </Flex>
    </div>
  );
}

function groupByFile(hits: readonly BalanceDataFileValue[]): [string, BalanceDataFileValue[]][] {
  const groups = new Map<string, BalanceDataFileValue[]>();
  for (const h of hits) groups.set(h.file, [...(groups.get(h.file) ?? []), h]);
  return [...groups];
}

/** Daten-Browser (B-661): alle Zahlenwerte der variierbaren Dateien, durchsuchbar, je Wert „+ fest“ und „+ Bereich“. */
export function DataBrowser({ values, error, used, onAdd }: Props) {
  const [query, setQuery] = useState('');
  const hits = useMemo(() => (values ? searchValues(values, query) : []), [values, query]);
  const files = useMemo(() => new Set(values?.map((v) => v.file)).size, [values]);
  const shown = hits.slice(0, MAX_HITS);
  return (
    <aside className="bal-ed-browser" aria-label="Daten-Browser">
      <div className="bal-ed-bhead">
        <Flex justify="between" align="baseline" gap="2">
          <Text weight="bold">Daten-Browser</Text>
          {values && <Text size="1" color="gray">{files} Dateien · {values.length} Werte</Text>}
        </Flex>
        <TextField.Root aria-label="Werte in data/ durchsuchen" placeholder="Pfad oder Datei suchen" value={query} onChange={(e) => setQuery(e.target.value)} />
        {values && <Text size="1" color="gray" role="status">{hits.length} Treffer{query.trim() ? ` für „${query.trim()}“` : ''}{hits.length > MAX_HITS ? `, die ersten ${MAX_HITS} – Suche eingrenzen` : ''}</Text>}
      </div>
      <div className="bal-ed-bscroll">
        {error && <p className="svc-error" role="alert">Werte nicht ladbar: {error}. Die Tabelle prüft dann nur das Format.</p>}
        {!values && !error && <Text size="1" color="gray">Lade Werte …</Text>}
        {values && hits.length === 0 && <Text size="1" color="gray" as="p">Kein Wert passt. Nur Dateien, die der Lauf neu lädt, stehen hier.</Text>}
        {groupByFile(shown).map(([file, list]) => (
          <section key={file} className="bal-ed-file" data-file={file} aria-label={file}>
            <Flex justify="between" align="center" className="bal-ed-fhead">
              <Code weight="bold">{file}</Code>
              <HelpButton title={`${file} · Beschreibung`} source={() => backend.balanceDataDoc(docOf(file))} />
            </Flex>
            {list.map((v) => <ValueRow key={v.path} v={v} used={used.has(`${v.file}:${v.path}`)} onAdd={onAdd} />)}
          </section>
        ))}
      </div>
    </aside>
  );
}
