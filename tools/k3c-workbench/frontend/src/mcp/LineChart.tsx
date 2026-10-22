import { avgOf, ticks, type Metric, type Point, type Range, type Window } from './series';

// Das SVG streckt sich auf volle Breite (preserveAspectRatio none); Linien und Punkte behalten dank
// non-scaling-stroke ihre Stärke, Achse und Hinweis stehen als HTML darunter und werden nicht verzerrt.
const W = 1000;
const H = 100;

interface Props {
  pts: Point[];
  metric: Metric;
  range: Range;
  w: Window;
}

/** Liniendiagramm (B-065): Aufrufe als Fläche mit Fehlern gestrichelt, oder Laufzeit Max/Ø mit Ausreißer-Punkten. */
export function LineChart({ pts, metric, range, w }: Props) {
  const x = (i: number) => (pts.length > 1 ? (i * W) / (pts.length - 1) : 0);
  const vals = metric === 'calls' ? pts.map((p) => p.calls) : pts.map((p) => p.maxMs);
  const peak = Math.max(1, ...vals);
  const y = (v: number) => H - (H * 0.92 * v) / peak;
  const line = (values: (number | null)[]) => path(values.map((v, i) => (v === null ? null : [x(i), y(v)])));
  const empty = pts.every((p) => p.calls === 0);
  return (
    <div className="mcp-chart">
      <svg viewBox={`0 0 ${W} ${H}`} preserveAspectRatio="none" className="mcp-svg" role="img" aria-label="Verlauf">
        <line x1={0} x2={W} y1={H} y2={H} className="chart-base" />
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
      </svg>
      {empty && <span className="chart-empty">Keine Aufrufe im Zeitraum</span>}
      <div className="chart-ticks">{ticks(range, w).map((t) => <span key={t.at}>{t.label}</span>)}</div>
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
