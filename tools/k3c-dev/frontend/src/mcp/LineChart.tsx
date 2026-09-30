import { avgOf, ticks, type Metric, type Point, type Range, type Window } from './series';

const W = 600;
const H = 180;
const TOP = 8;
const BASE = 158; // Grundlinie, darunter die Achse

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
  const y = (v: number) => BASE - ((BASE - TOP) * v) / peak;
  const line = (values: (number | null)[]) => path(values.map((v, i) => (v === null ? null : [x(i), y(v)])));
  const empty = pts.every((p) => p.calls === 0);
  return (
    <svg viewBox={`0 0 ${W} ${H}`} className="mcp-svg" role="img" aria-label="Verlauf">
      <line x1={0} x2={W} y1={BASE} y2={BASE} className="chart-base" />
      {metric === 'calls' ? (
        <>
          <path d={`${line(vals)} L ${W} ${BASE} L 0 ${BASE} Z`} className="chart-area" />
          <path d={line(vals)} className="chart-line" />
          <path d={line(pts.map((p) => p.errors))} className="chart-err" />
        </>
      ) : (
        <>
          <path d={line(vals)} className="chart-line" />
          <path d={line(pts.map(avgOf))} className="chart-avg" />
          {pts.map((p, i) => (p.outliers > 0 ? <circle key={i} cx={x(i)} cy={y(p.maxMs)} r={3.5} className="chart-dot" /> : null))}
        </>
      )}
      {ticks(range, w).map((t) => (
        <text key={t.at} x={t.at * W} y={H - 4} className="chart-tick" textAnchor={t.at === 0 ? 'start' : t.at === 1 ? 'end' : 'middle'}>
          {t.label}
        </text>
      ))}
      {empty && <text x={W / 2} y={BASE / 2} className="chart-empty" textAnchor="middle">Keine Aufrufe im Zeitraum</text>}
    </svg>
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
