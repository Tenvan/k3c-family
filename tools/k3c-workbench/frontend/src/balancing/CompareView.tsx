import { useState } from 'react';
import { Button, Code, Heading, StatusBadge, Text } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import type { BalanceReportInfo, BalanceSummary } from '../api';
import { compareSummaries, flips, variantTexts, type CompareCell, type CompareRow, type Comparison } from './compare';
import { comparisonMarkdown, downloadText, num, signed } from './export';

/** Ein Bericht im Vergleich; die erste Spalte ist A (Basis). */
export interface CompareItem {
  path: string;
  name: string;
  info?: BalanceReportInfo;
  summary: BalanceSummary;
}

type Filter = 'changes' | 'tipped' | 'targets' | 'figures';
const LETTERS = ['A', 'B', 'C', 'D'];

function Cell({ cell, index }: { cell: CompareCell; index: number }) {
  const sign = cell.delta === null || cell.delta === 0 ? '0' : cell.delta > 0 ? '+' : '-';
  return (
    <td data-tipped={cell.tipped ? 'true' : undefined}>
      <span className="bal-v">{num(cell.value)}</span>
      {index > 0 && cell.delta !== null && <span className="bal-d" data-sign={sign}>{cell.delta === 0 ? '±0' : signed(cell.delta)}</span>}
      {cell.tipped && <span className="bal-flip">{cell.status}</span>}
    </td>
  );
}

function Rows({ rows, columns }: { rows: CompareRow[]; columns: number }) {
  if (rows.length === 0) return <Text size="2" color="gray">Keine Einträge für diesen Filter.</Text>;
  return (
    <div className="bal-card bal-tbl">
      <table className="bal-table" aria-label="Vergleich">
        <thead><tr><th>Wert</th><th>Korridor</th>{LETTERS.slice(0, columns).map((l) => <th key={l}>{l}</th>)}</tr></thead>
        <tbody>
          {rows.map((r) => (
            <tr key={r.key} data-tipped={r.tipped ? 'true' : undefined}>
              <td>{r.label}</td><td>{r.corridor ?? ''}</td>
              {r.cells.map((c, i) => <Cell key={LETTERS[i]} cell={c} index={i} />)}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}

function Column({ item, index, base, c, onMakeBase, onOpen }: { item: CompareItem; index: number; base: CompareItem; c: Comparison; onMakeBase: (p: string) => void; onOpen: (p: string) => void }) {
  const vars = variantTexts(base.summary.varianten ?? [], item.summary.varianten ?? []);
  return (
    <div className="bal-col" data-column={LETTERS[index]}>
      <Text size="1" weight="bold" color={index === 0 ? 'gray' : 'blue'}>{LETTERS[index]}{index === 0 ? ' · Basis' : ''}</Text>
      <Button size="1" variant="ghost" color="gray" highContrast className="bal-item-name" onClick={() => onOpen(item.path)}>{item.name}</Button>
      <div className="bal-run-head">
        <StatusBadge tone={c.passes[index] ? 'ok' : 'error'}>{c.passes[index] ? 'Pass' : 'Fail'}</StatusBadge>
        <Text size="1" color="gray">{item.info && item.info.targetsTotal > 0 ? `${item.info.targetsPass}/${item.info.targetsTotal} Ziele` : `${item.summary.seeds} Seeds`}</Text>
      </div>
      <Code size="1" variant="ghost">{vars.length ? vars.join(' · ') : 'keine Varianten'}</Code>
      {index > 0 && <Button size="1" variant="ghost" onClick={() => onMakeBase(item.path)}>Als A setzen</Button>}
    </div>
  );
}

/** Satz zum Ergebnis: je Spalte gekippte Ziele gegenüber A. */
export function verdict(c: Comparison, names: string[]): string {
  const parts = names.slice(1).map((n, i) => {
    const { toPass, toFail } = flips(c, i + 1);
    return `${LETTERS[i + 1]} (${n}) kippt ${toPass} Ziele auf Pass und ${toFail} auf Fail`;
  });
  return parts.join('; ') + '.';
}

interface Props {
  items: CompareItem[];
  onMakeBase: (path: string) => void;
  onOpen: (path: string) => void;
}

/** 2 bis 4 Berichte nebeneinander (B-660): A ist die Basis (tauschbar), je Ziel und Kennzahl Wert und Abstand zu A. */
export function CompareView({ items, onMakeBase, onOpen }: Props) {
  const [filter, setFilter] = useState<Filter>('changes');
  const c = compareSummaries(items.map((i) => i.summary));
  const all = [...c.targets, ...c.figures];
  const shown = { changes: all.filter((r) => r.changed), tipped: c.targets.filter((r) => r.tipped), targets: c.targets, figures: c.figures }[filter];
  return (
    <article className="bal-report" aria-label="Vergleich">
      <Heading as="h2" size="4" className="bal-dtitle">Vergleich</Heading>
      <Text size="2">{verdict(c, items.map((i) => i.name))}</Text>
      <div className="bal-cols" style={{ gridTemplateColumns: `repeat(${items.length}, minmax(0, 1fr))` }}>
        {items.map((item, i) => <Column key={item.path} item={item} index={i} base={items[0]} c={c} onMakeBase={onMakeBase} onOpen={onOpen} />)}
      </div>
      <div className="bal-run-head">
        <ChipSwitch label="Vergleich filtern" value={filter} onChange={setFilter} options={[
          { value: 'changes', label: 'Nur Änderungen', count: all.filter((r) => r.changed).length },
          { value: 'tipped', label: 'Gekippt', count: c.targets.filter((r) => r.tipped).length },
          { value: 'targets', label: 'Alle Ziele', count: c.targets.length },
          { value: 'figures', label: 'Kennzahlen', count: c.figures.length },
        ]} />
        <Button size="1" variant="soft" onClick={() => downloadText('vergleich.md', comparisonMarkdown(c, items.map((i) => i.name)), 'text/markdown')}>Export Markdown</Button>
      </div>
      <Rows rows={shown} columns={items.length} />
    </article>
  );
}
