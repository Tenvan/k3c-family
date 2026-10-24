import { Button, StatusBadge, Text } from '@webkit/atoms';
import type { BalanceSummary } from '../api';
import { compareSummaries, type CompareRow } from './compare';
import { comparisonMarkdown, downloadText, num, signed } from './export';

function Rows({ rows }: { rows: CompareRow[] }) {
  return (
    <table className="bal-table">
      <thead><tr><th>Wert</th><th>A</th><th>B</th><th>Differenz</th><th>Status</th></tr></thead>
      <tbody>
        {rows.map((r) => (
          <tr key={r.key} data-tipped={r.tipped ? 'true' : undefined}>
            <td>{r.label}</td><td>{num(r.a)}</td><td>{num(r.b)}</td>
            <td data-sign={r.delta === null || r.delta === 0 ? '0' : r.delta > 0 ? '+' : '-'}>{signed(r.delta)}</td>
            <td>{r.statusA && r.statusB ? `${r.statusA} → ${r.statusB}` : ''}</td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

interface Props {
  nameA: string;
  nameB: string;
  a: BalanceSummary;
  b: BalanceSummary;
}

/** Zwei Berichte nebeneinander: A (Basis) und B mit Differenz B − A, wie balance.Compare in Go. */
export function CompareView({ nameA, nameB, a, b }: Props) {
  const c = compareSummaries(a, b);
  const tipped = c.targets.filter((r) => r.tipped).length;
  return (
    <article className="bal-report" aria-label="Vergleich">
      <div className="bal-run-head">
        <Text weight="bold">Vergleich</Text>
        <StatusBadge tone={c.passA ? 'ok' : 'error'}>A {c.passA ? 'Pass' : 'Fail'}</StatusBadge>
        <StatusBadge tone={c.passB ? 'ok' : 'error'}>B {c.passB ? 'Pass' : 'Fail'}</StatusBadge>
        {tipped > 0 && <StatusBadge tone="warn">{tipped} Ziel(e) gekippt</StatusBadge>}
      </div>
      <Text size="1" color="gray">A (Basis): {nameA} · B: {nameB}</Text>
      <div className="bal-run-actions">
        <Button size="1" variant="soft" onClick={() => downloadText('vergleich.md', comparisonMarkdown(c, nameA, nameB), 'text/markdown')}>Export Markdown</Button>
      </div>
      <Text weight="medium">Ziele</Text>
      <Rows rows={c.targets} />
      {c.figures.length > 0 && <><Text weight="medium">Kennzahlen</Text><Rows rows={c.figures} /></>}
    </article>
  );
}
