import { Flex, SegmentedControl } from '@radix-ui/themes';
import { useState } from 'react';
import type { UsageBucket } from '../api';
import { formatDuration, formatNumber } from '../lib/format';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { LineChart } from './LineChart';
import {
  bars, breakName, header, METRICS, pointsOf, RANGES, RANGE_SPEC, toolTotals, windowOf, type Metric, type Range,
  type ToolTotal,
} from './series';

const W = 260;
const H = 180;
const BASE = 138; // Grundlinie der Säulen, darunter die Namen

/** Band 2 rechts (B-065): Säulen und Liniendiagramm, gerechnet aus der Minuten-Zeitreihe. */
export function LiveMonitors({ minutes }: { minutes: UsageBucket[] }) {
  const [metric, setMetric] = useState<Metric>(() => loadPref('liveMetric', METRICS, 'calls'));
  const [range, setRange] = useState<Range>(() => loadPref('liveRange', RANGES, '1h'));
  const now = useNow(30_000); // das Fenster wandert mit der Uhr
  const w = windowOf(range, now);
  const pts = pointsOf(minutes, w);
  const top = bars(toolTotals(minutes, w), metric);
  return (
    <section className="mcp-card mcp-live">
      <Flex justify="between" align="center" gap="3" wrap="wrap">
        <span className="mcp-card-title">Live</span>
        <Flex gap="2" wrap="wrap">
          <SegmentedControl.Root size="1" value={metric} onValueChange={(v) => { setMetric(v as Metric); savePref('liveMetric', v); }}>
            <SegmentedControl.Item value="calls">Aufrufe</SegmentedControl.Item>
            <SegmentedControl.Item value="duration">Laufzeit</SegmentedControl.Item>
          </SegmentedControl.Root>
          <SegmentedControl.Root size="1" value={range} onValueChange={(v) => { setRange(v as Range); savePref('liveRange', v); }}>
            {RANGES.map((r) => <SegmentedControl.Item key={r} value={r}>{RANGE_SPEC[r].label}</SegmentedControl.Item>)}
          </SegmentedControl.Root>
        </Flex>
      </Flex>
      <div className="mcp-live-body">
        <div className="mcp-bars">
          <div className="mcp-chart-head">{metric === 'calls' ? 'Top 5 Tools' : 'Langsamste Tools · max'}</div>
          <Bars list={top} metric={metric} />
        </div>
        <div className="mcp-line">
          <div className="mcp-chart-head">{header(pts, metric)}</div>
          <LineChart pts={pts} metric={metric} range={range} w={w} />
        </div>
      </div>
    </section>
  );
}

function Bars({ list, metric }: { list: ToolTotal[]; metric: Metric }) {
  const value = (t: ToolTotal) => (metric === 'calls' ? t.calls : t.maxMs);
  const peak = Math.max(1, ...list.map(value));
  const slot = W / 5;
  if (list.length === 0) return <p className="mcp-empty">Keine Aufrufe im Zeitraum.</p>;
  return (
    <svg viewBox={`0 0 ${W} ${H}`} className="mcp-svg" role="img" aria-label="Säulen je Tool">
      {list.map((t, i) => {
        const h = Math.max(2, ((BASE - 18) * value(t)) / peak);
        const x = i * slot + slot * 0.2;
        const bad = metric === 'duration' && t.outliers > 0;
        return (
          <g key={t.tool}>
            <rect x={x} y={BASE - h} width={slot * 0.6} height={h} rx={2} className={bad ? 'bar bar-bad' : 'bar'} />
            <text x={x + slot * 0.3} y={BASE - h - 4} className="bar-value">
              {metric === 'calls' ? formatNumber(t.calls) : formatDuration(t.maxMs)}
            </text>
            <text x={x + slot * 0.3} y={BASE + 13} className="bar-name">
              {breakName(t.tool).map((part, j) => <tspan key={j} x={x + slot * 0.3} dy={j === 0 ? 0 : 11}>{part}</tspan>)}
            </text>
          </g>
        );
      })}
    </svg>
  );
}
