import { useState } from 'react';
import type { UsageBucket, UsageRules } from '../api';
import { Flex, SegmentedControl } from '@webkit/atoms';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { LineChart } from './LineChart';
import { barLabel, barTip, barValue, toolTotals, type ToolTotal } from './liveStats';
import { share } from './overview';
import { header, METRICS, pointsOf, RANGES, RANGE_SPEC, windowOf, type Metric, type Range } from './series';

/** Tools in der Balkenliste. */
const TOP = 6;

/** Live-Monitore (Workbench-Spec § 3, B-874): Verlauf mit Achsen und Legende, darunter die Tools des Fensters als
 *  waagerechte Balken; Aufrufe oder Laufzeit über ein mitlaufendes Zeitfenster. */
export function LiveMonitors({ minutes, rules }: { minutes: UsageBucket[]; rules?: UsageRules }) {
  const [metric, setMetric] = useState<Metric>(() => loadPref('mcp.liveMetric', METRICS, 'calls'));
  const [range, setRange] = useState<Range>(() => loadPref('mcp.liveRange', RANGES, '1h'));
  const now = useNow(30_000); // das Fenster wandert mit der Uhr
  const w = windowOf(range, now);
  const pts = pointsOf(minutes, w);
  return (
    <section className="mcp-card mcp-live">
      <Flex justify="between" align="center" gap="3" wrap="wrap">
        <Flex align="center" gap="3">
          <span className="mcp-card-title">Live</span>
          <span className="mcp-chart-head">{header(pts, metric)}</span>
        </Flex>
        <Flex gap="2" wrap="wrap">
          <SegmentedControl.Root size="1" value={metric} onValueChange={(v) => { setMetric(v as Metric); savePref('mcp.liveMetric', v); }}>
            <SegmentedControl.Item value="calls">Aufrufe</SegmentedControl.Item>
            <SegmentedControl.Item value="duration">Laufzeit</SegmentedControl.Item>
          </SegmentedControl.Root>
          <SegmentedControl.Root size="1" value={range} onValueChange={(v) => { setRange(v as Range); savePref('mcp.liveRange', v); }}>
            {RANGES.map((r) => <SegmentedControl.Item key={r} value={r}>{RANGE_SPEC[r].label}</SegmentedControl.Item>)}
          </SegmentedControl.Root>
        </Flex>
      </Flex>
      <div className="mcp-live-body">
        <LineChart pts={pts} metric={metric} range={range} w={w} rules={rules} />
        <ToolBars tools={toolTotals(minutes, w)} metric={metric} />
      </div>
    </section>
  );
}

/** Tools des Fensters als waagerechte Balken: Name links, Balken, Wert mit Einheit rechts. */
function ToolBars({ tools, metric }: { tools: ToolTotal[]; metric: Metric }) {
  const top = [...tools].sort((a, b) => barValue(b, metric) - barValue(a, metric)).slice(0, TOP);
  const max = top.length > 0 ? barValue(top[0], metric) : 0;
  if (top.length === 0) return null;
  return (
    <div className="mcp-bars" aria-label={metric === 'calls' ? 'Tools nach Aufrufen im Zeitraum' : 'Tools nach Ø-Laufzeit im Zeitraum'}>
      <span className="mcp-bars-head">{metric === 'calls' ? 'Aufrufe je Tool' : 'Ø-Laufzeit je Tool'}</span>
      {top.map((t) => (
        <div key={t.name} className="mcp-bar-row" title={barTip(t)}>
          <span className="mcp-bar-name">{t.name}</span>
          <span className="mcp-bar-track">
            <span className={t.outliers > 0 && metric === 'duration' ? 'mcp-bar-fill mcp-bar-hot' : 'mcp-bar-fill'}
              style={{ width: `${share(barValue(t, metric), max)}%` }} />
          </span>
          <span className="mcp-bar-value">{barLabel(t, metric)}</span>
        </div>
      ))}
    </div>
  );
}
