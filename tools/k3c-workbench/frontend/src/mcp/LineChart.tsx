import type { UsageRules } from '../api';
import { axisOf, outlierNote, pointTip, stepLabel, unitLabel } from './liveStats';
import { avgOf, RANGE_SPEC, ticks, type Metric, type Point, type Range, type Window } from './series';

// Das SVG streckt sich auf volle Breite (preserveAspectRatio none); Linien und Punkte behalten dank
// non-scaling-stroke ihre Stärke. Achsen, Einheit und Legende stehen als HTML daneben und werden nicht verzerrt (B-874).
const W = 1000;
const H = 100;

interface Props {
  pts: Point[];
  metric: Metric;
  range: Range;
  w: Window;
  rules?: UsageRules;
}

/** Liniendiagramm (B-065, B-874): Aufrufe als Fläche mit Fehlern gestrichelt, oder Laufzeit Max/Ø mit Ausreißer-Punkten;
 *  Werteachse mit Einheit, Zeitachse, Legende und Tooltip je Punkt. */
export function LineChart({ pts, metric, range, w, rules }: Props) {
  const x = (i: number) => (pts.length > 1 ? (i * W) / (pts.length - 1) : 0);
  const vals = metric === 'calls' ? pts.map((p) => p.calls) : pts.map((p) => p.maxMs);
  const axis = axisOf(Math.max(0, ...vals), metric);
  const y = (v: number) => H - (H * v) / axis.max;
  const line = (values: (number | null)[]) => path(values.map((v, i) => (v === null ? null : [x(i), y(v)])));
  const empty = pts.every((p) => p.calls === 0);
  const slot = W / Math.max(1, pts.length);
  return (
    <div className="mcp-chart">
      <span className="chart-unit">{unitLabel(range, metric)}</span>
      <div className="chart-plot">
        <div className="chart-yaxis" aria-hidden>
          {axis.ticks.map((t) => <span key={t.v} style={{ bottom: `${(t.v / axis.max) * 100}%` }}>{t.label}</span>)}
        </div>
        <div className="chart-box">
          <svg viewBox={`0 0 ${W} ${H}`} preserveAspectRatio="none" className="mcp-svg" role="img"
            aria-label={`Verlauf ${unitLabel(range, metric)}`}>
            {axis.ticks.map((t) => <line key={t.v} x1={0} x2={W} y1={y(t.v)} y2={y(t.v)} className={t.v === 0 ? 'chart-base' : 'chart-grid'} />)}
            {metric === 'calls' ? (
              <>
                <path d={`${line(vals)} L ${W} ${H} L 0 ${H} Z`} className="chart-area" />
                <path d={line(vals)} className="chart-line" />
                <path d={line(pts.map((p) => p.errors))} className="chart-err" />
              </>
            ) : (
              <>
                <path d={line(vals)} className="chart-line" />
                <path d={line(pts.map(avgOf))} className="chart-avg" />
                {pts.map((p, i) => (p.outliers > 0 ? <path key={i} d={`M ${x(i)} ${y(p.maxMs)} h 0`} className="chart-dot" /> : null))}
              </>
            )}
            {pts.map((p, i) => (
              <rect key={p.ts} x={Math.max(0, x(i) - slot / 2)} width={slot} y={0} height={H} className="chart-hit">
                <title>{pointTip(p, range)}</title>
              </rect>
            ))}
          </svg>
          {empty && <span className="chart-empty">Keine Aufrufe im Zeitraum</span>}
        </div>
      </div>
      <div className="chart-ticks">{ticks(range, w).map((t) => <span key={t.at}>{t.label}</span>)}</div>
      <Legend metric={metric} range={range} rules={rules} />
    </div>
  );
}

function Legend({ metric, range, rules }: { metric: Metric; range: Range; rules?: UsageRules }) {
  const time = `Zeit: ${RANGE_SPEC[range].label}, ein Punkt je ${stepLabel(range)}`;
  return (
    <div className="chart-legend">
      {metric === 'calls' ? (
        <>
          <span><i className="key key-line" />Aufrufe</span>
          <span><i className="key key-err" />Fehler</span>
        </>
      ) : (
        <>
          <span><i className="key key-line" />max</span>
          <span><i className="key key-avg" />Ø</span>
          <span title={outlierNote(rules)} className="chart-legend-help"><i className="key key-dot" />Ausreißer ⓘ</span>
        </>
      )}
      <span className="chart-legend-time">{time}</span>
    </div>
  );
}

/** SVG-Pfad aus Punkten; null unterbricht die Linie (Ø ohne Aufrufe). */
function path(points: ([number, number] | null)[]): string {
  let d = '';
  let open = false;
  for (const p of points) {
    if (!p) {
      open = false;
      continue;
    }
    d += `${open ? 'L' : 'M'} ${p[0].toFixed(1)} ${p[1].toFixed(1)} `;
    open = true;
  }
  return d.trim();
}
