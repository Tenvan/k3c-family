import { useState, type CSSProperties } from 'react';
import { useTicketPage } from '@webkit/lib/ticketContext';
import { SegmentedControl, Splitter, Text } from '@webkit/atoms';
import { useSplit } from '@webkit/lib/split';
import { NoticeCard, SplitStack } from '@webkit/molecules';
import { loadPref, prefsStore, savePref } from '../lib/prefs';
import { CallLog } from './CallLog';
import { CheckRunPanel } from './CheckRunPanel';
import { LiveMonitors } from './LiveMonitors';
import { ServerCards } from './ServerCards';
import { StatsView } from './StatsView';
import { ToolTiles } from './ToolTiles';
import { useMcpData } from './useMcpData';

const VIEWS = ['uebersicht', 'statistik'] as const;
type View = (typeof VIEWS)[number];

/** Reiter `MCP` (B-065, Workbench-Spec § 3): Umschalter Widgets | Statistiken (gemerkt); Splitter zwischen Tool-Kacheln
 *  und Live-Monitoren (B-445), waagerechter Splitter zwischen Widgets samt check_run-Auswertung und Aufruf-Log (B-875,
 *  B-876; Anteil des Logs gemerkt unter `mcp.split.log`). */
export function McpPage() {
  const [view, setView] = useState<View>(() => loadPref('mcp.view', VIEWS, 'uebersicht'));
  useTicketPage('mcp', view);
  const data = useMcpData();
  const [split, setSplit, resetSplit] = useSplit(prefsStore, 'mcp.split.band', 60);
  const choose = (v: string) => {
    const next = VIEWS.includes(v as View) ? (v as View) : 'uebersicht';
    setView(next);
    savePref('mcp.view', next);
  };
  const { overview } = data;
  return (
    <div className="mcp-page">
      <SegmentedControl.Root value={view} onValueChange={choose} className="mcp-views">
        <SegmentedControl.Item value="uebersicht">Widgets</SegmentedControl.Item>
        <SegmentedControl.Item value="statistik">Statistiken</SegmentedControl.Item>
      </SegmentedControl.Root>
      {!overview && !data.error && <Text color="gray">Lade MCP-Daten …</Text>}
      {!overview && data.error && <NoticeCard title="MCP-Daten nicht geladen" tone="error">{data.error}</NoticeCard>}
      {overview && view === 'uebersicht' && (
        <>
          <ServerCards overview={overview} error={data.error} reload={data.reload} />
          <SplitStack store={prefsStore} prefKey="mcp.split.log" dflt={45} label="Höhe des Aufruf-Logs" className="mcp-stack"
            top={(
              <>
                <div className="mcp-band2" style={{ '--mcp-band': `${split}%` } as CSSProperties}>
                  <ToolTiles tools={overview.stats.tools} />
                  <Splitter className="mcp-split" value={split} onChange={setSplit} onReset={resetSplit} />
                  <LiveMonitors minutes={data.usage?.minutes ?? []} rules={data.usage?.rules} />
                </div>
                <CheckRunPanel calls={data.calls} />
              </>
            )}
            bottom={<CallLog calls={data.calls} tools={overview.stats.tools.map((t) => t.name)} />} />
        </>
      )}
      {view === 'statistik' && data.usage && <StatsView usage={data.usage} />}
    </div>
  );
}
