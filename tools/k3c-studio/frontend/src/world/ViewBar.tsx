import { Button } from '@radix-ui/themes';
import { MAX_SCALE, MIN_SCALE, ZOOM_PERCENTS, scaleOfPercent, type JumpPoint, type View } from './viewState';

const ZOOM_STEP = 1.25;
const percentOf = (scale: number) => Math.round((scale / scaleOfPercent(100)) * 100);

/** Leiste der gemeinsamen Ansicht (B-477/AC-01 bis AC-03): Zoom (Knöpfe, Regler stufenlos, feste Stufen), Sprungpunkte und
 *  die eine Positions-Leiste für Chunk-Streifen und Live-Vorschau. */
export function ViewBar({ view, widthUnits, viewUnits, jumps, onScale, onLeft, onJump }: {
  view: View; widthUnits: number; viewUnits: number; jumps: JumpPoint[];
  onScale: (scale: number) => void; onLeft: (left: number) => void; onJump: (x: number) => void;
}) {
  const maxLeft = Math.max(0, Math.floor(widthUnits - viewUnits));
  const fixed = ZOOM_PERCENTS.includes(percentOf(view.scale));
  return (
    <div className="wd-viewbar">
      <div className="wd-bar">
        <Button variant="soft" aria-label="Herauszoomen" onClick={() => onScale(view.scale / ZOOM_STEP)}>−</Button>
        <input type="range" aria-label="Zoom" min={MIN_SCALE} max={MAX_SCALE} step={0.5} value={view.scale} onChange={(e) => onScale(Number(e.target.value))} />
        <Button variant="soft" aria-label="Hineinzoomen" onClick={() => onScale(view.scale * ZOOM_STEP)}>+</Button>
        <select className="wd-select" aria-label="Zoomstufe" value={fixed ? percentOf(view.scale) : ''} onChange={(e) => onScale(scaleOfPercent(Number(e.target.value)))}>
          {!fixed && <option value="">{percentOf(view.scale)} %</option>}
          {ZOOM_PERCENTS.map((p) => <option key={p} value={p}>{p} %</option>)}
        </select>
        <span className="wd-jumps" role="group" aria-label="Sprungpunkte">
          {jumps.map((j) => (
            <Button key={j.id} variant="outline" size="1" disabled={j.x === null} onClick={() => j.x !== null && onJump(j.x)}>{j.label}</Button>
          ))}
        </span>
      </div>
      <input className="wd-position" type="range" aria-label="Position" min={0} max={maxLeft} step={1} value={Math.min(maxLeft, Math.round(view.left))}
        disabled={maxLeft === 0} onChange={(e) => onLeft(Number(e.target.value))} />
    </div>
  );
}
