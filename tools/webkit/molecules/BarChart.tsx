import { extent, formatTick, niceExtent, niceTicks, scaleLinear } from '../lib/chartScale';
import { ChartBandShape } from './ChartBandShape';
import { AXIS_TEXT, type ChartBand, type ChartTone, GRID, TONE_FILL } from './chartTheme';

export interface BarChartProps {
  bars: { label: string; value: number; tone?: ChartTone }[];
  band?: ChartBand;
  unit?: string;
}

const W = 600;
const H = 220;
const M = { l: 48, r: 12, t: 12, b: 34 };

/** Balkendiagramm als reines SVG; Ton färbt den Balken, optionaler Zielkorridor als grünes Band. */
export function BarChart({ bars, band, unit = '' }: BarChartProps) {
  const bandVals = [band?.lower, band?.upper].filter((v): v is number => v !== undefined);
  const y = niceExtent(extent([0, ...bars.map((b) => b.value), ...bandVals], 0.08));
  const sy = scaleLinear(y, [H - M.b, M.t]);
  const slot = (W - M.l - M.r) / Math.max(bars.length, 1);
  const bw = Math.min(slot * 0.6, 56);
  const label = `Balken${unit ? ` in ${unit}` : ''}: ${bars.map((b) => `${b.label} ${formatTick(b.value)}`).join(', ') || 'keine Daten'}`;
  return (
    <svg viewBox={`0 0 ${W} ${H}`} width="100%" role="img" aria-label={label} className="wk-chart">
      {niceTicks(y.min, y.max).map((t) => (
        <g key={t}>
          <line x1={M.l} x2={W - M.r} y1={sy(t)} y2={sy(t)} stroke={GRID} />
          <text x={M.l - 6} y={sy(t)} textAnchor="end" dominantBaseline="middle" fontSize="11" fill={AXIS_TEXT}>{formatTick(t)}</text>
        </g>
      ))}
      {unit && <text x={M.l - 6} y={8} textAnchor="end" fontSize="11" fill={AXIS_TEXT}>{unit}</text>}
      {band && <ChartBandShape band={band} sy={sy} left={M.l} right={W - M.r} top={M.t} bottom={H - M.b} />}
      {bars.map((b, i) => {
        const x = M.l + slot * i + (slot - bw) / 2;
        const y0 = sy(0);
        const y1 = sy(b.value);
        return (
          <g key={`${b.label}${i}`}>
            <rect x={x} y={Math.min(y0, y1)} width={bw} height={Math.abs(y0 - y1)} fill={TONE_FILL[b.tone ?? 'ok']} rx="2" data-tone={b.tone ?? 'ok'} />
            <text x={x + bw / 2} y={H - M.b + 14} textAnchor="middle" fontSize="11" fill={AXIS_TEXT}>{b.label}</text>
          </g>
        );
      })}
    </svg>
  );
}
