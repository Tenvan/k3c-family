import { Flex, SegmentedControl } from '@radix-ui/themes';
import { useState } from 'react';
import type { ToolStats, UsageBucket } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { LineChart } from './LineChart';
import { share, sortByCalls } from './overview';
import { header, METRICS, pointsOf, RANGES, RANGE_SPEC, windowOf, type Metric, type Range } from './series';

/** Top-Tools als senkrechte Balken. */
const TOP = 5;

/** Live-Monitore (Workbench-Spec § 3): Top-Tools als Balken, Aufrufe oder Laufzeit über ein mitlaufendes Zeitfenster. */
export function LiveMonitors({ minutes, tools }: { minutes: UsageBucket[]; tools: ToolStats[] }) {
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
        <TopTools tools={tools} />
        <LineChart pts={pts} metric={metric} range={range} w={w} />
      </div>
    </section>
  );
}

function TopTools({ tools }: { tools: ToolStats[] }) {
  const top = sortByCalls(tools).filter((t) => t.calls > 0).slice(0, TOP);
  const max = top[0]?.calls ?? 0;
  if (top.length === 0) return null;
  return (
    <div className="mcp-top" aria-label="Top-Tools nach Aufrufen">
      {top.map((t) => (
        <div key={t.name} className="mcp-top-col" title={`${t.name}: ${t.calls}× · ${t.errors} Fehler`}>
          <div className="mcp-top-bar">
            <span className={t.errors > 0 ? 'mcp-top-fill mcp-top-bad' : 'mcp-top-fill'} style={{ height: `${share(t.calls, max)}%` }} />
          </div>
          <span className="mcp-top-name">{t.name}</span>
        </div>
      ))}
    </div>
  );
}
