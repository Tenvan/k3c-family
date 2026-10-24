import { extent, formatTick, niceExtent, scaleLinear } from '../lib/chartScale';
import { BAND_FILL, BAND_LINE, GRID, TONE_FILL } from './chartTheme';

export interface CorridorBarProps {
  value: number;
  lower?: number;
  upper?: number;
  unit?: string;
}

const W = 320;
const H = 28;
const PAD = 8;

function rangeText(lower?: number, upper?: number): string {
  const parts = [lower === undefined ? '' : `ab ${formatTick(lower)}`, upper === undefined ? '' : `bis ${formatTick(upper)}`];
  return parts.filter(Boolean).join(' ') || 'offen';
}

/** Wert gegen den Zielkorridor: Marke grün im Korridor (Pass), sonst rot (Fail); offene Seiten laufen bis zum Rand. */
export function CorridorBar({ value, lower, upper, unit = '' }: CorridorBarProps) {
  const pass = (lower === undefined || value >= lower) && (upper === undefined || value <= upper);
  const known = [value, lower, upper].filter((v): v is number => v !== undefined);
  const s = scaleLinear(niceExtent(extent(known, 0.15)), [PAD, W - PAD]);
  const x0 = lower === undefined ? PAD : s(lower);
  const x1 = upper === undefined ? W - PAD : s(upper);
  const label = `${formatTick(value)}${unit ? ` ${unit}` : ''}, Korridor ${rangeText(lower, upper)}: ${pass ? 'Pass' : 'Fail'}`;
  return (
    <svg viewBox={`0 0 ${W} ${H}`} width="100%" height={H} role="img" aria-label={label} className="wk-corridor" data-pass={pass}>
      <line x1={PAD} x2={W - PAD} y1={H / 2} y2={H / 2} stroke={GRID} strokeWidth="2" />
      <rect x={x0} y={4} width={Math.max(x1 - x0, 0)} height={H - 8} fill={BAND_FILL} stroke={BAND_LINE} rx="3" />
      <circle cx={s(value)} cy={H / 2} r="6" fill={TONE_FILL[pass ? 'ok' : 'fail']} stroke="var(--color-background)" strokeWidth="2" />
      <title>{label}</title>
    </svg>
  );
}
