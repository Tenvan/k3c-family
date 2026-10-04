import { Flex, SegmentedControl } from '@radix-ui/themes';
import { useState } from 'react';
import type { UsageBucket } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { LineChart } from './LineChart';
import { header, METRICS, pointsOf, RANGES, RANGE_SPEC, windowOf, type Metric, type Range } from './series';

/** Live-Band (B-065): flacher Verlauf über die volle Breite, gerechnet aus der Minuten-Zeitreihe. Tools je Aufruf
 *  stehen in der Statistik, damit das Aufruf-Log den meisten Platz bekommt. */
export function LiveMonitors({ minutes }: { minutes: UsageBucket[] }) {
  const [metric, setMetric] = useState<Metric>(() => loadPref('liveMetric', METRICS, 'calls'));
  const [range, setRange] = useState<Range>(() => loadPref('liveRange', RANGES, '1h'));
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
          <SegmentedControl.Root size="1" value={metric} onValueChange={(v) => { setMetric(v as Metric); savePref('liveMetric', v); }}>
            <SegmentedControl.Item value="calls">Aufrufe</SegmentedControl.Item>
            <SegmentedControl.Item value="duration">Laufzeit</SegmentedControl.Item>
          </SegmentedControl.Root>
          <SegmentedControl.Root size="1" value={range} onValueChange={(v) => { setRange(v as Range); savePref('liveRange', v); }}>
            {RANGES.map((r) => <SegmentedControl.Item key={r} value={r}>{RANGE_SPEC[r].label}</SegmentedControl.Item>)}
          </SegmentedControl.Root>
        </Flex>
      </Flex>
      <LineChart pts={pts} metric={metric} range={range} w={w} />
    </section>
  );
}
