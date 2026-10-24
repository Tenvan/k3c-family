import { extent, formatTick, linePath, niceExtent, niceTicks, scaleLinear } from '../lib/chartScale';
import { ChartBandShape } from './ChartBandShape';
import { AXIS_TEXT, type ChartBand, GRID, SERIES_COLORS } from './chartTheme';

export interface LineSeries {
  name: string;
  points: { x: number; y: number }[];
}

export interface LineChartProps {
  series: LineSeries[];
  band?: ChartBand;
  xLabel: string;
  yLabel: string;
  height?: number;
}

const W = 600;
const M = { l: 48, r: 12, t: 12, b: 40 };

/** Liniendiagramm als reines SVG; mehrere Serien, optionaler Zielkorridor als grünes Band. */
export function LineChart({ series, band, xLabel, yLabel, height = 240 }: LineChartProps) {
  const pts = series.flatMap((s) => s.points);
  const label = `${yLabel} über ${xLabel}: ${series.map((s) => s.name).join(', ') || 'keine Daten'}`;
  const bandVals = [band?.lower, band?.upper].filter((v): v is number => v !== undefined);
  const xs = extent(pts.map((p) => p.x), 0);
  const y = niceExtent(extent([...pts.map((p) => p.y), ...bandVals], 0.08));
  const sx = scaleLinear(xs, [M.l, W - M.r]);
  const sy = scaleLinear(y, [height - M.b, M.t]);
  return (
    <svg viewBox={`0 0 ${W} ${height}`} width="100%" role="img" aria-label={label} className="wk-chart">
      {niceTicks(y.min, y.max).map((t) => (
        <g key={`y${t}`}>
          <line x1={M.l} x2={W - M.r} y1={sy(t)} y2={sy(t)} stroke={GRID} />
          <text x={M.l - 6} y={sy(t)} textAnchor="end" dominantBaseline="middle" fontSize="11" fill={AXIS_TEXT}>{formatTick(t)}</text>
        </g>
      ))}
      {niceTicks(xs.min, xs.max, 6).map((t) => (
        <text key={`x${t}`} x={sx(t)} y={height - M.b + 14} textAnchor="middle" fontSize="11" fill={AXIS_TEXT}>{formatTick(t)}</text>
      ))}
      {band && <ChartBandShape band={band} sy={sy} left={M.l} right={W - M.r} top={M.t} bottom={height - M.b} />}
      {series.map((s, i) => (
        <path key={s.name} d={linePath(s.points.map((p) => ({ x: sx(p.x), y: sy(p.y) })))} fill="none"
          stroke={SERIES_COLORS[i % SERIES_COLORS.length]} strokeWidth="2" strokeLinejoin="round" data-series={s.name} />
      ))}
      <text x={(M.l + W - M.r) / 2} y={height - 4} textAnchor="middle" fontSize="12" fill={AXIS_TEXT}>{xLabel}</text>
      <text transform={`translate(12 ${(M.t + height - M.b) / 2}) rotate(-90)`} textAnchor="middle" fontSize="12" fill={AXIS_TEXT}>{yLabel}</text>
      {series.map((s, i) => (
        <g key={`l${s.name}`} transform={`translate(${M.l + 8 + i * 110} ${M.t + 8})`}>
          <rect width="10" height="10" y="-5" fill={SERIES_COLORS[i % SERIES_COLORS.length]} />
          <text x="14" dominantBaseline="middle" fontSize="11" fill={AXIS_TEXT}>{s.name}</text>
        </g>
      ))}
    </svg>
  );
}
