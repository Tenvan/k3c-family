import { useState } from 'react';
import type { McpCall } from '../api';
import { Flex, IconButton, SegmentedControl, StatusBadge, type Tone } from '@webkit/atoms';
import { formatDuration, formatNumber, formatPercent, formatTime } from '../lib/format';
import { loadPref, savePref } from '../lib/prefs';
import { useNow } from '../lib/useNow';
import { ConsoleView } from '../logs/ConsoleView';
import { CheckRunChart } from './CheckRunChart';
import { byTarget, consoleSource, runsIn, type CheckRun, type TargetStats } from './checkRuns';
import { prettyArgs } from './lanes';
import { RANGES, RANGE_SPEC, windowOf, type Range } from './series';

/** Farben der Ziele (Klassen lane-c*), ohne Rot: Rot steht für Fehlschläge. */
const COLORS = [0, 1, 2, 3, 4, 6, 7];

/** Auswertung der check_run-Läufe (B-876) zwischen Widgets und Aufruf-Log: Kennzahlen je Ziel, Verlauf der Laufzeit,
 *  Klick auf einen Lauf zeigt Log-Eintrag und Konsole. Quelle ist das Aufruf-Log der Sitzung. */
export function CheckRunPanel({ calls }: { calls: McpCall[] }) {
  const [range, setRange] = useState<Range>(() => loadPref('mcp.checkRange', RANGES, '24h'));
  const [focus, setFocus] = useState('');
  const [selected, setSelected] = useState<number | null>(null);
  const now = useNow(30_000);
  const w = windowOf(range, now);
  const runs = runsIn(calls, w);
  const stats = byTarget(runs);
  const names = stats.map((s) => s.target).sort();
  const colorOf = (t: string) => COLORS[Math.max(0, names.indexOf(t)) % COLORS.length];
  const failed = runs.filter((r) => r.state === 'failed').length;
  const run = runs.find((r) => r.call.id === selected) ?? null;
  const latest = run ? runs.filter((r) => r.target === run.target).at(-1) === run : false;
  return (
    <section className="mcp-card mcp-checks">
      <Flex justify="between" align="center" gap="3" wrap="wrap">
        <Flex align="center" gap="3">
          <span className="mcp-card-title">check_run</span>
          <span className="mcp-chart-head">
            {formatNumber(runs.length)} Läufe · {formatNumber(failed)} Fehlschläge · {formatNumber(stats.length)} {stats.length === 1 ? 'Ziel' : 'Ziele'}
          </span>
        </Flex>
        <SegmentedControl.Root size="1" value={range} onValueChange={(v) => { setRange(v as Range); savePref('mcp.checkRange', v); }}>
          {RANGES.map((r) => <SegmentedControl.Item key={r} value={r}>{RANGE_SPEC[r].label}</SegmentedControl.Item>)}
        </SegmentedControl.Root>
      </Flex>
      <div className="mcp-checks-body">
        <TargetTable stats={stats} colorOf={colorOf} focus={focus} onFocus={(t) => setFocus((f) => (f === t ? '' : t))} />
        <CheckRunChart runs={runs} range={range} w={w} colorOf={colorOf} focus={focus} selected={selected}
          onSelect={(id) => setSelected((s) => (s === id ? null : id))} />
      </div>
      {run && <RunDetail run={run} latest={latest} onClose={() => setSelected(null)} />}
      <p className="logtab-foot">Quelle: Aufruf-Log dieser Sitzung (die letzten 200 Aufrufe aller Tools); Klick auf ein Ziel hebt es hervor.</p>
    </section>
  );
}

function TargetTable({ stats, colorOf, focus, onFocus }: {
  stats: TargetStats[]; colorOf: (t: string) => number; focus: string; onFocus: (t: string) => void;
}) {
  if (stats.length === 0) return <p className="mcp-empty">Keine check_run-Läufe im Zeitraum.</p>;
  return (
    <table className="mcp-checks-table">
      <thead>
        <tr><th>Ziel</th><th>Läufe</th><th>Ø</th><th>max</th><th>Fehlerquote</th></tr>
      </thead>
      <tbody>
        {stats.map((s) => (
          <tr key={s.target} className={focus === s.target ? 'is-focus' : undefined} onClick={() => onFocus(s.target)}
            title={`${s.target}: ${s.count} Läufe, ${s.failed} fehlgeschlagen`}>
            <td className="mcp-checks-target"><i className={`key key-run lane-c${colorOf(s.target)}`} />{s.target}</td>
            <td>{formatNumber(s.count)}</td>
            <td>{formatDuration(s.avgMs)}</td>
            <td>{formatDuration(s.maxMs)}</td>
            <td className={s.failed > 0 ? 'mcp-tile-bad' : undefined}>
              <span className="mcp-rate"><span style={{ width: `${s.failRate}%` }} /></span>{formatPercent(s.failRate)}
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

const BADGE: Record<CheckRun['state'], [string, Tone]> = {
  ok: ['OK', 'ok'], failed: ['Fehlschlag', 'error'], pending: ['im Hintergrund', 'info'],
};

/** Ein Lauf: Log-Eintrag (Kopfzeile, Fehler, Argumente) und die Konsole seines Ziels. */
function RunDetail({ run, latest, onClose }: { run: CheckRun; latest: boolean; onClose: () => void }) {
  const [label, tone] = BADGE[run.state];
  const c = run.call;
  return (
    <div className="mcp-run-detail">
      <Flex justify="between" align="center" gap="3">
        <Flex align="center" gap="3" wrap="wrap">
          <StatusBadge tone={tone}>{label}</StatusBadge>
          <strong>{run.target}</strong>
          <span className="mcp-chart-head">{formatTime(new Date(run.at), true)} · {formatDuration(run.ms)} · {c.ref}</span>
        </Flex>
        <IconButton size="1" variant="ghost" aria-label="Lauf schließen" onClick={onClose}>✕</IconButton>
      </Flex>
      <pre className="mcp-run-log">{[c.summary, c.error && c.error !== c.summary ? c.error : '', prettyArgs(c.args)].filter(Boolean).join('\n')}</pre>
      {latest
        ? <div className="mcp-run-console"><ConsoleView name={consoleSource(run.target)} /></div>
        : <span className="mcp-chart-head">Die Konsole {consoleSource(run.target)} hält nur den letzten Lauf dieses Ziels; für diesen Lauf gilt der Log-Eintrag oben.</span>}
    </div>
  );
}
