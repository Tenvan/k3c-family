import { useState } from 'react';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { SegmentedControl, Text } from '@webkit/atoms';
import { NoticeCard } from '@webkit/molecules';
import type { ResGauge, ResourceView } from '../api';
import { formatBytes, formatNumber } from '../lib/format';
import { loadPref, savePref } from '../lib/prefs';
import { deltas, type Line } from './chart';
import { OpsTable } from './OpsTable';
import { SeriesChart } from './SeriesChart';
import { Summary } from './Summary';
import { useResources } from './useResources';

/** Zeiträume in Minuten; gemerkt wird der Name. */
const WINDOWS = { '15m': 15, '1h': 60, '6h': 360, '24h': 1440 } as const;
type WindowKey = keyof typeof WINDOWS;
const KEYS = Object.keys(WINDOWS) as WindowKey[];

const count = (v: number) => formatNumber(Math.round(v));

/** Reiter `Monitoring`: Speicher, Kindprozesse, Goroutinen, Puffer und Vorgänge der Workbench über die Zeit. Die Seite
 *  bleibt eingehängt, fragt aber nur ab und rechnet nur, solange sie `active` ist. */
export function MonitoringPage({ active }: { active: boolean }) {
  const [key, setKey] = useState<WindowKey>(() => loadPref('monitoring.window', KEYS, '1h'));
  useTicketPage('monitoring', key, active);
  const { view, error } = useResources(WINDOWS[key], active);
  const choose = (v: string) => {
    const next = KEYS.find((k) => k === v) ?? '1h';
    setKey(next);
    savePref('monitoring.window', next);
  };
  return (
    <div className="mon-page">
      <div className="mon-bar">
        <SegmentedControl.Root value={key} onValueChange={choose}>
          {KEYS.map((k) => <SegmentedControl.Item key={k} value={k}>{k}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
        {view?.enabled && <Text color="gray" size="1">Probe alle {view.intervalS} s · {view.points.length} Punkte (Maximum je Zeitfenster)</Text>}
      </div>
      {!view && !error && <Text color="gray">Lade Messwerte …</Text>}
      {error && <NoticeCard title="Messwerte nicht geladen" tone="error">{error}</NoticeCard>}
      {view && !view.enabled && <NoticeCard title="Kein Messgerät" tone="warn">Diese Workbench-Version liefert keine Ressourcen-Messung.</NoticeCard>}
      {view?.enabled && active && <Charts view={view} />}
    </div>
  );
}

function Charts({ view }: { view: ResourceView }) {
  const p = view.points;
  const times = p.map((x) => x.t);
  const col = (f: (x: (typeof p)[number]) => number) => p.map(f);
  const gauge = (g: ResGauge, i: number): Line => ({ label: g.name, values: p.map((x) => x.gaugeValues[i] ?? 0), cls: `mon-c${i % 4}` });
  const byteGauges = view.gauges.flatMap((g, i) => (g.bytes ? [gauge(g, i)] : []));
  const countGauges = view.gauges.flatMap((g, i) => (g.bytes ? [] : [gauge(g, i)]));
  return (
    <>
      <Summary view={view} />
      <div className="mon-grid">
        <SeriesChart title="Speicher der Workbench" times={times} format={formatBytes} lines={[
          { label: 'Live-Heap', values: col((x) => x.heapLive), cls: 'mon-c0' },
          { label: 'Go gesamt', values: col((x) => x.total), cls: 'mon-c1' },
          { label: 'Privat', values: col((x) => x.private), cls: 'mon-c2' },
          { label: 'RSS', values: col((x) => x.rss), cls: 'mon-c3' },
        ]} />
        <SeriesChart title="Kindprozesse (RSS)" times={times} format={formatBytes} area lines={[
          { label: 'Summe der Kinder', values: col((x) => x.childrenRss), cls: 'mon-c1' },
        ]} />
        <SeriesChart title="Goroutinen" times={times} format={count} lines={[{ label: 'Goroutinen', values: col((x) => x.goroutines), cls: 'mon-c0' }]} />
        <SeriesChart title="Handles und Kinder" times={times} format={count} lines={[
          { label: 'Handles', values: col((x) => x.handles), cls: 'mon-c2' },
          { label: 'Kindprozesse', values: col((x) => x.children), cls: 'mon-c3' },
        ]} />
        {byteGauges.length > 0 && <SeriesChart title="Puffer (Bytes)" times={times} format={formatBytes} lines={byteGauges} />}
        {countGauges.length > 0 && <SeriesChart title="Puffer (Anzahl)" times={times} format={count} lines={countGauges} />}
        <SeriesChart title="GC-Läufe je Punkt" times={times} format={count} area lines={[{ label: 'GC-Läufe', values: deltas(col((x) => x.gcCycles)), cls: 'mon-c1' }]} />
      </div>
      <OpsTable ops={view.ops} />
    </>
  );
}
