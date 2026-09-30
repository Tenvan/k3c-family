import { Fragment, useState } from 'react';
import type { ToolUsage, UsageScope } from '../api';
import { formatDuration, formatNumber, formatPercent } from '../lib/format';
import { loadText, savePref } from '../lib/prefs';
import { share } from './overview';
import { DEFAULT_SORT, nextSort, rateText, SORT_KEYS, sortTools, type Sort, type SortKey } from './stats';

const COLUMNS: [SortKey, string][] = [
  ['name', 'Tool'], ['calls', 'Aufrufe'], ['share', 'Anteil'], ['errors', 'Fehler'], ['rate', 'Quote'], ['avgMs', 'Ø'],
  ['p50Ms', 'p50'], ['p95Ms', 'p95'], ['maxMs', 'Max'], ['sumMs', 'Σ Zeit'], ['outliers', 'Ausreißer'],
];

function loadSort(): Sort {
  const [key, dir] = loadText('statSort', '').split(':');
  return SORT_KEYS.includes(key as SortKey) ? { key: key as SortKey, desc: dir !== 'asc' } : DEFAULT_SORT;
}

/** Tool-Tabelle, sortierbar per Klick auf den Spaltenkopf; eine Zeile klappt Argumente und Fehler auf. */
export function StatsTable({ scope }: { scope: UsageScope }) {
  const [sort, setSort] = useState<Sort>(loadSort);
  const [open, setOpen] = useState('');
  const click = (key: SortKey) => {
    const next = nextSort(sort, key);
    setSort(next);
    savePref('statSort', `${next.key}:${next.desc ? 'desc' : 'asc'}`);
  };
  if (scope.tools.length === 0) return <section className="mcp-card"><p className="mcp-empty">Noch keine Aufrufe.</p></section>;
  return (
    <section className="mcp-card mcp-stats-table">
      <table>
        <thead>
          <tr>
            {COLUMNS.map(([key, label]) => (
              <th key={key} onClick={() => click(key)} aria-sort={sort.key === key ? (sort.desc ? 'descending' : 'ascending') : undefined}>
                {label}{sort.key === key ? (sort.desc ? ' ↓' : ' ↑') : ''}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {sortTools(scope.tools, sort).map((t) => (
            <Fragment key={t.name}>
              <ToolRow t={t} total={scope.calls} onClick={() => setOpen(open === t.name ? '' : t.name)} />
              {open === t.name && <ToolDetail t={t} />}
            </Fragment>
          ))}
        </tbody>
      </table>
    </section>
  );
}

function ToolRow({ t, total, onClick }: { t: ToolUsage; total: number; onClick: () => void }) {
  const pct = share(t.calls, total);
  return (
    <tr className="mcp-stat-row" onClick={onClick}>
      <td className="mcp-log-tool">{t.name}</td>
      <td>{formatNumber(t.calls)}</td>
      <td><span className="mcp-share mcp-share-inline"><span style={{ width: `${pct}%` }} /></span> {formatPercent(pct)}</td>
      <td className={t.errors > 0 ? 'svc-error-count' : undefined}>{formatNumber(t.errors)}</td>
      <td>{rateText(t.errors, t.calls)}</td>
      <td>{formatDuration(t.avgMs)}</td>
      <td>{formatDuration(t.p50Ms)}</td>
      <td>{formatDuration(t.p95Ms)}</td>
      <td>{formatDuration(t.maxMs)}</td>
      <td>{formatDuration(t.sumMs)}</td>
      <td className={t.outliers > 0 ? 'svc-error-count' : undefined}>{formatNumber(t.outliers)}</td>
    </tr>
  );
}

function ToolDetail({ t }: { t: ToolUsage }) {
  return (
    <tr className="logrow-detail">
      <td colSpan={COLUMNS.length}>
        <div className="mcp-stat-detail">
          <div>
            <div className="svc-label">HÄUFIGSTE ARGUMENTE · LAUFZEIT</div>
            {t.args.map((a) => (
              <div key={a.value} className="mcp-stat-item">
                <span>{formatNumber(a.count)}×</span> <code>{a.value}</code>
                <span className="mcp-dim"> p95 {formatDuration(a.p95Ms ?? 0)} · max {formatDuration(a.maxMs ?? 0)}</span>
              </div>
            ))}
          </div>
          <div>
            <div className="svc-label">HÄUFIGSTE FEHLER</div>
            {t.topErrors.length === 0 && <div className="mcp-dim">Keine Fehler</div>}
            {t.topErrors.map((e) => (
              <div key={e.value} className="mcp-stat-item"><span>{formatNumber(e.count)}×</span> <span className="svc-error-count">{e.value}</span></div>
            ))}
          </div>
        </div>
      </td>
    </tr>
  );
}
