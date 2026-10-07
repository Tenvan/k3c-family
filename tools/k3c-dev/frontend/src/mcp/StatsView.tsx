import { SegmentedControl } from '@radix-ui/themes';
import { useState } from 'react';
import type { McpUsage } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { rangeScope } from './rangeScope';
import { RANGES, RANGE_SPEC, type Range } from './series';
import { Kpi } from './ServerCards';
import { kpis, ruleLine, SCOPES } from './stats';
import { StatsSide } from './StatsSide';
import { StatsTable } from './StatsTable';

const CHOICES = [...SCOPES, ...RANGES] as const;
type Choice = (typeof CHOICES)[number];
const isRange = (c: Choice): c is Range => (RANGES as readonly string[]).includes(c);

/** Unteransicht `Statistik` (B-065, B-350): Kacheln, Tool-Tabelle und Seitenspalte für Sitzung, Gesamtzeit oder einen
 *  Zeitraum (15 min bis 7 T, aus der Minuten-Zeitreihe). */
export function StatsView({ usage }: { usage: McpUsage }) {
  const [choice, setChoice] = useState<Choice>(() => loadPref('statScope', CHOICES, 'session'));
  const now = useNow(30_000);
  const names = usage.allTime.tools.map((t) => t.name);
  const scope = isRange(choice) ? rangeScope(usage.minutes, names, choice, now) : usage[choice];
  return (
    <div className="mcp-stats">
      <SegmentedControl.Root size="1" value={choice} className="mcp-views"
        onValueChange={(v) => { setChoice(v as Choice); savePref('statScope', v); }}>
        <SegmentedControl.Item value="session">Sitzung</SegmentedControl.Item>
        <SegmentedControl.Item value="allTime">All time</SegmentedControl.Item>
        {RANGES.map((r) => <SegmentedControl.Item key={r} value={r}>{RANGE_SPEC[r].label}</SegmentedControl.Item>)}
      </SegmentedControl.Root>
      <section className="mcp-card">
        <dl className="mcp-kpis mcp-kpis-10">
          {kpis(scope, now).map((k) => <Kpi key={k.label} label={k.label} value={k.value} bad={k.bad} />)}
        </dl>
        <p className="logtab-foot">
          {isRange(choice)
            ? `letzte ${RANGE_SPEC[choice].label} aus Minutenwerten · Perzentile, Fehler je Tool und Ausreißer-Listen nur in Sitzung und All time`
            : ruleLine(usage.rules, scope.since, now)}
        </p>
      </section>
      <div className="mcp-stats-body">
        <StatsTable scope={scope} />
        {!isRange(choice) && <StatsSide scope={scope} now={now} />}
      </div>
    </div>
  );
}
