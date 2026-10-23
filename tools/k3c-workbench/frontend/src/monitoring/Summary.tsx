import type { ResourceView, ResTrend } from '../api';
import { formatBytes, formatNumber } from '../lib/format';

/** Wachstum je Stunde, ab dem ein Trend als Warnung zählt (wie growthWarn in internal/resmon/report.go). */
const WARN_BYTES_PER_HOUR = 200 * 1024 * 1024;

const trendText = (t: ResTrend) =>
  `${t.perHour >= 0 ? '+' : '−'}${t.bytes ? formatBytes(Math.round(Math.abs(t.perHour))) : formatNumber(Math.abs(t.perHour), 1)}/h`;

/** Kennzahlen der letzten Probe mit Trend, die größten Kindprozesse und abgelegte Profile. */
export function Summary({ view }: { view: ResourceView }) {
  const last = view.points[view.points.length - 1];
  if (!last) return null;
  const trend = (name: string) => view.trends.find((t) => t.name === name);
  const cards: { name: string; value: string; trend?: ResTrend }[] = [
    { name: 'Live-Heap', value: formatBytes(last.heapLive), trend: trend('Live-Heap') },
    { name: 'Privat', value: formatBytes(last.private), trend: trend('Privat') },
    { name: 'RSS', value: formatBytes(last.rss) },
    { name: 'Kinder-RSS', value: `${formatBytes(last.childrenRss)} (${last.children})`, trend: trend('Kinder-RSS') },
    { name: 'Goroutinen', value: formatNumber(last.goroutines), trend: trend('Goroutinen') },
    { name: 'Handles', value: formatNumber(last.handles) },
  ];
  return (
    <section className="mon-summary" aria-label="Kennzahlen">
      {cards.map((c) => (
        <div key={c.name} className="mon-stat">
          <span className="mon-stat-name">{c.name}</span>
          <b>{c.value}</b>
          {c.trend && (
            <span className={c.trend.bytes && c.trend.perHour >= WARN_BYTES_PER_HOUR ? 'mon-trend mon-trend-warn' : 'mon-trend'}>
              {trendText(c.trend)} über {c.trend.windowMin} min
            </span>
          )}
        </div>
      ))}
      {view.children.length > 0 && (
        <div className="mon-stat mon-stat-wide">
          <span className="mon-stat-name">Größte Kindprozesse</span>
          <span>{view.children.map((c) => `${c.name} (${c.pid}) ${formatBytes(c.rss)}`).join(' · ')}</span>
        </div>
      )}
      {view.profiles.length > 0 && (
        <div className="mon-stat mon-stat-wide">
          <span className="mon-stat-name">Profile (go tool pprof)</span>
          <span className="mon-paths">{view.profiles.join(' · ')}</span>
        </div>
      )}
    </section>
  );
}
