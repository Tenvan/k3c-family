import { useState } from 'react';
import { StatusBadge, Text } from '@webkit/atoms';
import { ChipSwitch, CorridorBar, NoticeCard } from '@webkit/molecules';
import type { BalanceSummary } from '../api';
import { corridorText, num } from './export';

type Filter = 'alle' | 'verletzt' | 'knapp' | 'ok';

/** Ziele eines Berichts als Tabelle mit Filter Alle, Verletzt, Knapp, Gut. */
export function TargetsTab({ summary }: { summary: BalanceSummary }) {
  const [filter, setFilter] = useState<Filter>('alle');
  const count = (s: string) => summary.targets.filter((t) => t.status === s).length;
  if (summary.targets.length === 0) {
    return <NoticeCard title="Keine Ziele im Bericht" tone="warn">Der Lauf hat keine Ziele bewertet; Korridore fehlen (data/balance-targets.json prüfen).</NoticeCard>;
  }
  const rows = summary.targets.filter((t) => filter === 'alle' || t.status === filter);
  return (
    <>
      <ChipSwitch label="Ziele filtern" value={filter} onChange={setFilter} options={[
        { value: 'alle', label: 'Alle', count: summary.targets.length }, { value: 'verletzt', label: 'Verletzt', count: count('verletzt') },
        { value: 'knapp', label: 'Knapp', count: count('knapp') }, { value: 'ok', label: 'Gut', count: count('ok') },
      ]} />
      <div className="bal-card bal-tbl">
        <table className="bal-table" aria-label="Ziele">
          <thead><tr><th>Ziel</th><th>Wert</th><th>Korridor</th><th className="bal-col-bar">Lage</th><th>Status</th></tr></thead>
          <tbody>
            {rows.map((e) => (
              <tr key={`${e.measure}|${e.name}|${e.players}|${e.bot}|${e.depth}|${e.days}`} data-status={e.status}>
                <td>{e.name}<Text as="div" size="1" color="gray">{e.players} Sp. · {e.bot} · Tiefe {e.depth} · {e.days} T.</Text></td>
                <td><strong>{num(e.value)}</strong></td>
                <td>{corridorText(e)}</td>
                <td><CorridorBar value={e.value} lower={e.lower ?? undefined} upper={e.upper ?? undefined} /></td>
                <td><StatusBadge tone={e.pass ? (e.status === 'knapp' ? 'warn' : 'ok') : 'error'}>{e.status}</StatusBadge></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </>
  );
}
