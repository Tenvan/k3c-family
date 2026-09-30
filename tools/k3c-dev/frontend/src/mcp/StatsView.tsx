import { SegmentedControl } from '@radix-ui/themes';
import { useState } from 'react';
import type { McpUsage } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { Kpi } from './ServerCards';
import { kpis, ruleLine, SCOPES, type ScopeKey } from './stats';
import { StatsSide } from './StatsSide';
import { StatsTable } from './StatsTable';

/** Unteransicht `Statistik` (B-065): Kacheln, Tool-Tabelle und Seitenspalte für Sitzung oder Gesamtzeit. */
export function StatsView({ usage }: { usage: McpUsage }) {
  const [scopeKey, setScopeKey] = useState<ScopeKey>(() => loadPref('statScope', SCOPES, 'session'));
  const now = useNow(30_000);
  const scope = usage[scopeKey];
  return (
    <div className="mcp-stats">
      <SegmentedControl.Root size="1" value={scopeKey} className="mcp-views"
        onValueChange={(v) => { setScopeKey(v as ScopeKey); savePref('statScope', v); }}>
        <SegmentedControl.Item value="session">Sitzung</SegmentedControl.Item>
        <SegmentedControl.Item value="allTime">All time</SegmentedControl.Item>
      </SegmentedControl.Root>
      <section className="mcp-card">
        <dl className="mcp-kpis mcp-kpis-10">
          {kpis(scope, now).map((k) => <Kpi key={k.label} label={k.label} value={k.value} bad={k.bad} />)}
        </dl>
        <p className="logtab-foot">{ruleLine(usage.rules, scope.since, now)}</p>
      </section>
      <div className="mcp-stats-body">
        <StatsTable scope={scope} />
        <StatsSide scope={scope} now={now} />
      </div>
    </div>
  );
}
