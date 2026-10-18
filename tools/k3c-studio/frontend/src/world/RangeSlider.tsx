import { useEffect, useRef, type KeyboardEvent, type PointerEvent } from 'react';

/** Sichtbarer Bereich einer Vorschau: linke Kante und Breite in Units */
export interface Span {
  left: number;
  units: number;
}

/** Lage und Breite eines Griffs in Prozent der Spur; breiter als die Stufe = ganze Spur, nie außerhalb. */
export function thumbBox(span: Span, widthUnits: number): { left: number; width: number } {
  if (widthUnits <= 0) return { left: 0, width: 100 };
  const width = Math.min(100, (span.units / widthUnits) * 100);
  const left = Math.max(0, Math.min(100 - width, (span.left / widthUnits) * 100));
  return { left, width };
}

/** Bereichs-Schieber der Ansicht (B-485/AC-05, WZ17.4): die Spur ist die ganze Stufe, der Griff der sichtbare Bereich der
 *  Chunk-Vorschau (`main`), seine Breite folgt dem Zoom. Ein dünner zweiter Griff zeigt den Bereich der Live-Vorschau
 *  (`second`), wenn ihr Zoom abweicht. Ziehen verschiebt die Mitte, ein Klick auf die Spur springt dorthin, Pfeile
 *  verschieben um ein Zehntel des Sichtbereichs, Pos1 und Ende gehen an die Grenzen `range`. Die Seite begrenzt die Mitte. */
export function RangeSlider({ label, widthUnits, center, range, main, second, onCenter }: {
  label: string; widthUnits: number; center: number; range: [number, number]; main: Span; second?: Span; onCenter: (x: number) => void;
}) {
  const track = useRef<HTMLDivElement>(null);
  const stopDrag = useRef<(() => void) | null>(null);
  useEffect(() => () => stopDrag.current?.(), []); // beim Aushängen kein Ziehen übrig lassen
  const unitsPerPx = () => widthUnits / (track.current?.getBoundingClientRect().width || 1);
  const box = thumbBox(main, widthUnits);
  const live = second && thumbBox(second, widthUnits);

  const jump = (e: PointerEvent<HTMLDivElement>) => {
    if (e.button !== 0) return;
    const rect = track.current?.getBoundingClientRect();
    if (rect && rect.width > 0) onCenter(((e.clientX - rect.left) / rect.width) * widthUnits);
  };
  const drag = (e: PointerEvent<HTMLDivElement>) => {
    e.stopPropagation(); // kein Sprung der Spur
    if (e.button !== 0) return;
    stopDrag.current?.();
    const [startX, startCenter, scale] = [e.clientX, center, unitsPerPx()];
    const move = (m: globalThis.PointerEvent) => onCenter(startCenter + (m.clientX - startX) * scale);
    const ends = ['pointerup', 'pointercancel', 'blur'] as const;
    const up = () => {
      window.removeEventListener('pointermove', move);
      for (const t of ends) window.removeEventListener(t, up);
      stopDrag.current = null;
    };
    window.addEventListener('pointermove', move);
    for (const t of ends) window.addEventListener(t, up);
    stopDrag.current = up;
  };
  const onKey = (e: KeyboardEvent<HTMLDivElement>) => {
    const step = main.units / 10;
    const to = { ArrowLeft: center - step, ArrowRight: center + step, Home: range[0], End: range[1] }[e.key];
    if (to === undefined) return;
    e.preventDefault();
    e.stopPropagation(); // die Pfeiltasten der Seite verschieben sonst ein zweites Mal
    onCenter(to);
  };

  return (
    <div className="wd-range" ref={track} onPointerDown={jump}>
      <div className="wd-range-thumb" role="slider" tabIndex={0} aria-label={label} aria-orientation="horizontal"
        aria-valuemin={Math.round(range[0])} aria-valuemax={Math.round(range[1])} aria-valuenow={Math.round(center)}
        aria-valuetext={`Mitte bei ${Math.round(center)} von ${Math.round(widthUnits)} Units`}
        style={{ left: `${box.left}%`, width: `${box.width}%` }} onPointerDown={drag} onKeyDown={onKey} />
      {live && <div className="wd-range-second" aria-hidden="true" title="Bereich der Live-Vorschau" style={{ left: `${live.left}%`, width: `${live.width}%` }} />}
    </div>
  );
}
