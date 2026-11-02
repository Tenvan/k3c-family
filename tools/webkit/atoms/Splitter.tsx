import { useRef, type KeyboardEvent as ReactKeyboardEvent, type PointerEvent as ReactPointerEvent } from 'react';
import { clampSplit, splitShare } from '../lib/split';

interface Props {
  value: number;
  onChange: (v: number) => void;
  onReset: () => void;
  horizontal?: boolean;
  /** Zusätzliche Klasse für das Layout der Seite; das Aussehen kommt aus theme.css (`wk-split`). */
  className?: string;
  /** Name für Screenreader, z. B. „Höhe der Bühne“. */
  label?: string;
}

/** Schrittweite der Pfeiltasten in Prozent. */
export const SPLIT_KEY_STEP = 2;

/** Pfeiltaste → Änderung des Anteils: senkrecht → / ←, waagerecht ↑ / ↓ (der untere Bereich wächst mit ↑); sonst 0. */
export function splitKeyDelta(horizontal: boolean, key: string): number {
  if (horizontal) return key === 'ArrowUp' ? SPLIT_KEY_STEP : key === 'ArrowDown' ? -SPLIT_KEY_STEP : 0;
  return key === 'ArrowRight' ? SPLIT_KEY_STEP : key === 'ArrowLeft' ? -SPLIT_KEY_STEP : 0;
}

/** Splitter: Ziehen setzt den Anteil der linken Spalte (20–80 %), Doppelklick setzt zurück. Mit `horizontal` ein
 *  waagerechter Balken, dessen Wert der Anteil des unteren Bereichs ist. Fokussierbar: Pfeiltasten ändern den Anteil
 *  in Schritten von 2 %, Pos1 setzt zurück. */
export function Splitter({ value, onChange, onReset, horizontal = false, className, label }: Props) {
  const ref = useRef<HTMLDivElement>(null);
  const drag = (e: ReactPointerEvent) => {
    const box = ref.current?.parentElement?.getBoundingClientRect();
    if (!box || e.button !== 0) return;
    e.preventDefault();
    const move = (ev: PointerEvent) => onChange(clampSplit(splitShare(horizontal, box, ev.clientX, ev.clientY)));
    // Abbruch (pointercancel, Loslassen außerhalb des Fensters) beendet das Ziehen wie das Loslassen.
    const up = () => {
      window.removeEventListener('pointermove', move);
      window.removeEventListener('pointerup', up);
      window.removeEventListener('pointercancel', up);
      window.removeEventListener('blur', up);
    };
    window.addEventListener('pointermove', move);
    window.addEventListener('pointerup', up);
    window.addEventListener('pointercancel', up);
    window.addEventListener('blur', up);
  };
  const key = (e: ReactKeyboardEvent) => {
    if (e.key === 'Home') {
      e.preventDefault();
      onReset();
      return;
    }
    const d = splitKeyDelta(horizontal, e.key);
    if (d === 0) return;
    e.preventDefault();
    onChange(clampSplit(value + d));
  };
  const cls = ['wk-split', horizontal ? 'wk-split-h' : 'wk-split-v', className].filter(Boolean).join(' ');
  return (
    <div ref={ref} className={cls} role="separator" tabIndex={0} aria-label={label} aria-orientation={horizontal ? 'horizontal' : 'vertical'} aria-valuenow={value}
      aria-valuemin={20} aria-valuemax={80} title="Ziehen oder Pfeiltasten ändern die Größe, Doppelklick setzt zurück" onPointerDown={drag} onKeyDown={key} onDoubleClick={onReset} />
  );
}
