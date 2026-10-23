import { useState } from 'react';
import { areaPath, linePath, nearestIndex, niceMax, peakOf, timeTicks, H, W, type Line } from './chart';

interface Props {
  title: string;
  times: number[];
  lines: Line[];
  /** Zahl für Achse, Legende und Hinweis (Bytes, Anzahl …). */
  format: (v: number) => string;
  /** Erste Linie als Fläche zeichnen. */
  area?: boolean;
}

/** Liniendiagramm mit Legende (Wert je Linie am Cursor, sonst der letzte), Achsenmaximum und Hinweis unter der Maus.
 *  Das SVG streckt sich auf volle Breite; die Strichstärke bleibt dank non-scaling-stroke, Text steht als HTML daneben. */
export function SeriesChart({ title, times, lines, format, area }: Props) {
  const [hover, setHover] = useState<number | null>(null);
  const max = niceMax(peakOf(lines));
  const n = times.length;
  const at = hover === null ? n - 1 : hover;
  const move = (e: React.MouseEvent<HTMLDivElement>) => {
    const r = e.currentTarget.getBoundingClientRect();
    setHover(nearestIndex((e.clientX - r.left) / Math.max(r.width, 1), n));
  };
  return (
    <section className="mon-card" aria-label={title}>
      <header className="mon-head">
        <h3>{title}</h3>
        <span className="mon-time">{n > 0 ? new Date(times[at]).toLocaleTimeString('de-DE') : '–'}</span>
      </header>
      <div className="mon-legend">
        {lines.map((l) => (
          <span key={l.label} className={`mon-leg ${l.cls}`}>
            <i aria-hidden /> {l.label} <b>{n > 0 ? format(l.values[at] ?? 0) : '–'}</b>
          </span>
        ))}
      </div>
      <div className="mon-plot" onMouseMove={move} onMouseLeave={() => setHover(null)}>
        <span className="mon-max">{format(max)}</span>
        <svg viewBox={`0 0 ${W} ${H}`} preserveAspectRatio="none" role="img" aria-label={title}>
          <line x1={0} x2={W} y1={H} y2={H} className="chart-base" />
          {area && lines[0] && <path d={areaPath(lines[0].values, max)} className={`mon-area ${lines[0].cls}`} />}
          {lines.map((l) => <path key={l.label} d={linePath(l.values, max)} className={`mon-line ${l.cls}`} />)}
          {hover !== null && n > 1 && <line x1={(hover * W) / (n - 1)} x2={(hover * W) / (n - 1)} y1={0} y2={H} className="mon-cursor" />}
        </svg>
        {n === 0 && <span className="chart-empty">Noch keine Proben</span>}
      </div>
      <div className="chart-ticks">{timeTicks(times).map((t) => <span key={t.at}>{t.label}</span>)}</div>
    </section>
  );
}
