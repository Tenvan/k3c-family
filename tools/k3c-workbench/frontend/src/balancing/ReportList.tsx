import { Button, StatusBadge, Text } from '@webkit/atoms';
import type { BalanceReportInfo } from '../api';

interface Props {
  reports: BalanceReportInfo[];
  selected: string;
  base: string;
  onSelect: (path: string) => void;
  onBase: (path: string) => void;
  onReload: () => void;
}

/** Berichtsliste (neueste zuerst): Öffnen als B, „Als Basis“ setzt A für den Vergleich. */
export function ReportList({ reports, selected, base, onSelect, onBase, onReload }: Props) {
  return (
    <section className="bal-card" aria-label="Berichte">
      <div className="bal-run-head">
        <Text weight="bold">Berichte</Text>
        <Button size="1" variant="ghost" onClick={onReload}>Neu laden</Button>
      </div>
      {reports.length === 0 && <Text color="gray" size="2">Keine Berichte unter reports/.</Text>}
      {reports.map((r) => (
        <div key={r.path} className="bal-report-row" data-on={r.path === selected ? 'true' : undefined}>
          <Button size="1" variant="ghost" onClick={() => onSelect(r.path)}>{r.run || r.path.split('/').pop()}</Button>
          <StatusBadge tone={r.pass ? 'ok' : 'error'}>{r.pass ? 'Pass' : 'Fail'}</StatusBadge>
          {r.kind === 'tuning' && <StatusBadge tone="info">Abstimmung</StatusBadge>}
          <Text size="1" color="gray">{r.kind === 'tuning' ? '' : `${r.seeds} Seeds · `}{new Date(r.modified).toLocaleString('de-DE')}{r.varianten?.length ? ` · ${r.varianten.length} Var.` : ''}</Text>
          {r.kind !== 'tuning' && (
            <Button size="1" variant={r.path === base ? 'solid' : 'soft'} onClick={() => onBase(r.path === base ? '' : r.path)}>
              {r.path === base ? 'Basis A' : 'Als Basis'}
            </Button>
          )}
        </div>
      ))}
    </section>
  );
}
