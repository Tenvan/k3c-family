import { Button, SelectField, Slider } from '@webkit/atoms';
import { RangeSlider, type Span } from './RangeSlider';
import { MAX_SCALE, MIN_SCALE, ZOOM_PERCENTS, scaleOfPercent, type JumpPoint, type Views } from './viewState';

const ZOOM_STEP = 1.25;
const percentOf = (scale: number) => Math.round((scale / scaleOfPercent(100)) * 100);

/** Zoom und Sprungpunkte der Werkzeugleiste über der Live-Vorschau (B-477/AC-01 bis AC-03, B-485/AC-05, B-665): Zoom beider
 *  Vorschauen (Knöpfe, Regler stufenlos, feste Stufen; Regler und Stufen zeigen die Chunk-Vorschau und setzen beide gleich),
 *  Sprungpunkte im Menü „Springe zu“. Hat die Live-Vorschau einen eigenen Zoom (Alt + Rad), steht er daneben. */
export function ViewBar({ views, jumps, onScale, onZoom, onCenter }: {
  views: Views; jumps: JumpPoint[]; onScale: (scale: number) => void; onZoom: (factor: number) => void; onCenter: (x: number) => void;
}) {
  const scale = views.chunk;
  const fixed = ZOOM_PERCENTS.includes(percentOf(scale));
  return (
    <div className="wd-bar wd-viewbar">
      <Button variant="soft" aria-label="Herauszoomen" onClick={() => onZoom(1 / ZOOM_STEP)}>−</Button>
      <Slider className="wd-zoom" aria-label="Zoom" min={MIN_SCALE} max={MAX_SCALE} step={0.5} value={[scale]} onValueChange={([v]) => v !== undefined && onScale(v)} />
      <Button variant="soft" aria-label="Hineinzoomen" onClick={() => onZoom(ZOOM_STEP)}>+</Button>
      <SelectField aria-label="Zoomstufe" value={fixed ? String(percentOf(scale)) : ''} onChange={(v) => v !== '' && onScale(scaleOfPercent(Number(v)))}
        options={[...(fixed ? [] : [{ value: '', label: `${percentOf(scale)} %` }]), ...ZOOM_PERCENTS.map((p) => ({ value: String(p), label: `${p} %` }))]} />
      {views.live !== scale && <span className="wd-zoom-live" aria-label="Zoom Live-Vorschau">Live {percentOf(views.live)} %</span>}
      <details className="wd-menu wd-jumpmenu">
        <summary title="Zu einem Punkt des Levels springen">Springe zu ▾</summary>
        <span className="wd-menu-body wd-jumps" role="group" aria-label="Sprungpunkte">
          {jumps.map((j) => (
            <Button key={j.id} variant="outline" size="1" disabled={j.x === null} onClick={() => j.x !== null && onCenter(j.x)}>{j.label}</Button>
          ))}
        </span>
      </details>
    </div>
  );
}

/** Der eine Bereichs-Schieber für die Weltposition der Bildmitte im Bereich `range`, unter der Chunk-Vorschau. Hat die
 *  Live-Vorschau einen eigenen Zoom (Alt + Rad), erscheint ihr Bereich als zweiter, dünner Griff. */
export function PositionBar({ views, spans, widthUnits, range, onCenter }: {
  views: Views; spans: { chunk: Span; live: Span }; widthUnits: number; range: [number, number]; onCenter: (x: number) => void;
}) {
  return (
    <RangeSlider label="Position" widthUnits={widthUnits} center={views.center} range={range} main={spans.chunk}
      second={views.live !== views.chunk ? spans.live : undefined} onCenter={onCenter} />
  );
}
