import { useRef, type PointerEvent as ReactPointerEvent } from 'react';
import { clampSplit, splitShare } from '../lib/split';

interface Props {
  value: number;
  onChange: (v: number) => void;
  onReset: () => void;
  horizontal?: boolean;
  /** Zusätzliche Klasse für das Layout der Seite; das Aussehen kommt aus theme.css (`wk-split`). */
  className?: string;
}

/** Splitter: Ziehen setzt den Anteil der linken Spalte (20–80 %), Doppelklick setzt zurück. Mit `horizontal` ein
 *  waagerechter Balken, dessen Wert der Anteil des unteren Bereichs ist. */
export function Splitter({ value, onChange, onReset, horizontal = false, className }: Props) {
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
  const cls = ['wk-split', horizontal ? 'wk-split-h' : 'wk-split-v', className].filter(Boolean).join(' ');
  return (
    <div ref={ref} className={cls} role="separator" aria-orientation={horizontal ? 'horizontal' : 'vertical'} aria-valuenow={value}
      aria-valuemin={20} aria-valuemax={80} title="Ziehen ändert die Größe, Doppelklick setzt zurück" onPointerDown={drag} onDoubleClick={onReset} />
  );
}
