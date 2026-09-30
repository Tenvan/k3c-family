import type { SlowCall, UsageScope } from '../api';
import { formatDuration, formatNumber } from '../lib/format';
import { timesP95, whenText } from './stats';

/** Seitenspalte der Statistik: letzte Ausreißer, langsamste Aufrufe, häufigste Fehler aller Tools. */
export function StatsSide({ scope, now }: { scope: UsageScope; now: number }) {
  const outliers = new Set(scope.recentOutliers.map((c) => `${c.at}|${c.tool}`));
  return (
    <aside className="mcp-stats-side">
      <SlowList title="Letzte Ausreißer" list={scope.recentOutliers} isOutlier={() => true} now={now} />
      <SlowList title="Langsamste Aufrufe" list={scope.slowest} isOutlier={(c) => outliers.has(`${c.at}|${c.tool}`)} now={now} />
      <section className="mcp-card">
        <span className="mcp-card-title">Häufigste Fehler (alle Tools)</span>
        {scope.topErrors.length === 0 && <p className="mcp-empty">Keine Fehler.</p>}
        {scope.topErrors.map((e) => (
          <div key={`${e.tool}|${e.value}`} className="mcp-stat-item">
            <span>{formatNumber(e.count)}×</span> <span className="mcp-log-tool">{e.tool}</span>{' '}
            <span className="svc-error-count">{e.value}</span>
          </div>
        ))}
      </section>
    </aside>
  );
}

interface ListProps {
  title: string;
  list: SlowCall[];
  isOutlier: (c: SlowCall) => boolean;
  now: number;
}

function SlowList({ title, list, isOutlier, now }: ListProps) {
  return (
    <section className="mcp-card">
      <span className="mcp-card-title">{title}</span>
      {list.length === 0 && <p className="mcp-empty">Keine.</p>}
      {list.map((c, i) => (
        <div key={`${c.at}|${c.tool}|${i}`} className="mcp-slow">
          <span className={isOutlier(c) ? 'mcp-slow-ms svc-error-count' : 'mcp-slow-ms'}>{formatDuration(c.durationMs)}</span>
          <span className="mcp-log-tool">{c.tool}</span>
          <span className="mcp-dim">{timesP95(c)}{c.ok ? '' : ' · Fehler'} · {whenText(c.at, now)}</span>
          <code className="mcp-slow-args" title={c.args}>{c.args}</code>
        </div>
      ))}
    </section>
  );
}
