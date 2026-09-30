import { SegmentedControl, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { loadPref, savePref } from '../lib/prefs';
import { NoticeCard } from '../ui/parts';
import { ServerCards } from './ServerCards';
import { ToolTiles } from './ToolTiles';
import { useMcpData } from './useMcpData';

const VIEWS = ['uebersicht', 'statistik'] as const;
type View = (typeof VIEWS)[number];

/** Reiter `MCP` (B-065): Unteransichten Übersicht und Statistik. */
export function McpPage() {
  const [view, setView] = useState<View>(() => loadPref('mcpView', VIEWS, 'uebersicht'));
  const data = useMcpData();
  const choose = (v: string) => {
    const next = VIEWS.includes(v as View) ? (v as View) : 'uebersicht';
    setView(next);
    savePref('mcpView', next);
  };
  const { overview } = data;
  return (
    <div className="mcp-page">
      <SegmentedControl.Root value={view} onValueChange={choose} className="mcp-views">
        <SegmentedControl.Item value="uebersicht">Übersicht</SegmentedControl.Item>
        <SegmentedControl.Item value="statistik">Statistik</SegmentedControl.Item>
      </SegmentedControl.Root>
      {!overview && !data.error && <Text color="gray">Lade MCP-Daten …</Text>}
      {!overview && data.error && <NoticeCard title="MCP-Daten nicht geladen" tone="error">{data.error}</NoticeCard>}
      {overview && view === 'uebersicht' && (
        <>
          <ServerCards overview={overview} error={data.error} reload={data.reload} />
          <div className="mcp-band2">
            <ToolTiles tools={overview.stats.tools} total={overview.stats.totalCalls} />
            <NoticeCard title="Live-Monitore" tone="neutral">Folgen in M5.2.</NoticeCard>
          </div>
          <NoticeCard title="Aufruf-Log" tone="neutral">Folgt in M5.3.</NoticeCard>
        </>
      )}
      {overview && view === 'statistik' && <NoticeCard title="Statistik" tone="neutral">Folgt in M5.4.</NoticeCard>}
    </div>
  );
}
