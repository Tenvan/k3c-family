import { formatDuration, formatTime } from '../lib/format';
import { axisOf } from './liveStats';
import { ticks, type Range, type Window } from './series';
import type { CheckRun } from './checkRuns';

// Verlauf der check_run-Laufzeiten (B-876): je Ziel eine dünne Linie in seiner Farbe, jeder Lauf ein Punkt, Fehlschläge
// rot und größer. Wie LineChart gestreckt (preserveAspectRatio none), Punkte über non-scaling-stroke.
const W = 1000;
const H = 100;

interface Props {
  runs: CheckRun[];
  range: Range;
  w: Window;
  colorOf: (target: string) => number;
  focus: string;
  selected: number | null;
  onSelect: (id: number) => void;
}

const STATE_TEXT = { ok: 'OK', failed: 'Fehlschlag', pending: 'läuft im Hintergrund' } as const;

/** Tooltip eines Laufs. */
export function runTip(r: CheckRun): string {
  const exit = r.exit !== null ? ` · exit ${r.exit}` : '';
  return `${formatTime(new Date(r.at), true)} · ${r.target} · ${STATE_TEXT[r.state]}${exit} · ${formatDuration(r.ms)}`;
}

export function CheckRunChart({ runs, range, w, colorOf, focus, selected, onSelect }: Props) {
  const axis = axisOf(Math.max(0, ...runs.map((r) => r.ms)), 'duration');
  const x = (at: number) => ((at - w.from) / (w.to - w.from)) * W;
  const y = (v: number) => H - (H * v) / axis.max;
  const targets = [...new Set(runs.map((r) => r.target))];
  const dim = (t: string) => (focus && focus !== t ? ' run-dim' : '');
  return (
    <div className="mcp-chart">
      <span className="chart-unit">Laufzeit je Lauf</span>
      <div className="chart-plot">
        <div className="chart-yaxis" aria-hidden>
          {axis.ticks.map((t) => <span key={t.v} style={{ bottom: `${(t.v / axis.max) * 100}%` }}>{t.label}</span>)}
        </div>
        <div className="chart-box">
          <svg viewBox={`0 0 ${W} ${H}`} preserveAspectRatio="none" className="mcp-svg run-svg" role="img" aria-label="Laufzeit der Prüfläufe">
            {axis.ticks.map((t) => <line key={t.v} x1={0} x2={W} y1={y(t.v)} y2={y(t.v)} className={t.v === 0 ? 'chart-base' : 'chart-grid'} />)}
            {targets.map((t) => {
              const own = runs.filter((r) => r.target === t && r.state !== 'pending');
              const d = own.map((r, i) => `${i === 0 ? 'M' : 'L'} ${x(r.at).toFixed(1)} ${y(r.ms).toFixed(1)}`).join(' ');
              return <path key={t} d={d} className={`run-line lane-c${colorOf(t)}${dim(t)}`} />;
            })}
            {runs.map((r) => (
              <path key={r.call.id} d={`M ${x(r.at).toFixed(1)} ${y(r.ms).toFixed(1)} h 0`} onClick={() => onSelect(r.call.id)}
                className={`run-dot lane-c${colorOf(r.target)} run-${r.state}${dim(r.target)}${selected === r.call.id ? ' run-sel' : ''}`}>
                <title>{runTip(r)}</title>
              </path>
            ))}
          </svg>
          {runs.length === 0 && <span className="chart-empty">Keine check_run-Läufe im Zeitraum</span>}
        </div>
      </div>
      <div className="chart-ticks">{ticks(range, w).map((t) => <span key={t.at}>{t.label}</span>)}</div>
      <div className="chart-legend">
        <span><i className="key key-run" />Lauf (Farbe = Ziel)</span>
        <span><i className="key key-dot" />Fehlschlag (exit ≠ 0, Zeitlimit, abgelehnt)</span>
        <span><i className="key key-pending" />im Hintergrund, noch ohne Ergebnis</span>
        <span className="chart-legend-time">Klick auf einen Punkt zeigt den Lauf</span>
      </div>
    </div>
  );
}
