import { Button } from '@radix-ui/themes';
import { MAX_SCALE, MIN_SCALE, ZOOM_PERCENTS, scaleOfPercent, type JumpPoint, type Views } from './viewState';

const ZOOM_STEP = 1.25;
const percentOf = (scale: number) => Math.round((scale / scaleOfPercent(100)) * 100);

/** Leiste der Ansicht (B-477/AC-01 bis AC-03, B-485/AC-05): Zoom beider Vorschauen (Knöpfe, Regler stufenlos, feste Stufen;
 *  Regler und Stufen zeigen die Chunk-Vorschau und setzen beide gleich), Sprungpunkte und die eine Positions-Leiste. Sie
 *  führt die Weltposition der Bildmitte im Bereich `range`. Hat die Live-Vorschau einen eigenen Zoom (Alt + Rad), steht er daneben. */
export function ViewBar({ views, range, jumps, onScale, onZoom, onCenter }: {
  views: Views; range: [number, number]; jumps: JumpPoint[];
  onScale: (scale: number) => void; onZoom: (factor: number) => void; onCenter: (x: number) => void;
}) {
  const scale = views.chunk;
  const fixed = ZOOM_PERCENTS.includes(percentOf(scale));
  const [lo, hi] = range;
  return (
    <div className="wd-viewbar">
      <div className="wd-bar">
        <Button variant="soft" aria-label="Herauszoomen" onClick={() => onZoom(1 / ZOOM_STEP)}>−</Button>
        <input type="range" aria-label="Zoom" min={MIN_SCALE} max={MAX_SCALE} step={0.5} value={scale} onChange={(e) => onScale(Number(e.target.value))} />
        <Button variant="soft" aria-label="Hineinzoomen" onClick={() => onZoom(ZOOM_STEP)}>+</Button>
        <select className="wd-select" aria-label="Zoomstufe" value={fixed ? percentOf(scale) : ''} onChange={(e) => onScale(scaleOfPercent(Number(e.target.value)))}>
          {!fixed && <option value="">{percentOf(scale)} %</option>}
          {ZOOM_PERCENTS.map((p) => <option key={p} value={p}>{p} %</option>)}
        </select>
        {views.live !== scale && <span className="wd-zoom-live" aria-label="Zoom Live-Vorschau">Live {percentOf(views.live)} %</span>}
        <span className="wd-jumps" role="group" aria-label="Sprungpunkte">
          {jumps.map((j) => (
            <Button key={j.id} variant="outline" size="1" disabled={j.x === null} onClick={() => j.x !== null && onCenter(j.x)}>{j.label}</Button>
          ))}
        </span>
      </div>
      <input className="wd-position" type="range" aria-label="Position" min={lo} max={hi} step="any"
        value={Math.min(hi, Math.max(lo, views.center))} disabled={hi - lo < 1} onChange={(e) => onCenter(Number(e.target.value))} />
    </div>
  );
}
