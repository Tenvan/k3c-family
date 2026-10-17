import { useRef, useState } from 'react';
import { loadText, savePref } from '../lib/prefs';

// Splitter, Kopie aus k3c-dev (tasks/TaskBars.tsx, B-403/B-445); ein gemeinsames UI-Paket wäre ein eigenes Ticket.

/** Anteil in Prozent, den der Zeiger im Kasten einnimmt: senkrechte Spalte von links, waagerechter Balken von unten. */
export function splitShare(horizontal: boolean, box: { left: number; bottom: number; width: number; height: number }, x: number, y: number): number {
  return horizontal ? ((box.bottom - y) / box.height) * 100 : ((x - box.left) / box.width) * 100;
}

/** Klemmt den Anteil auf 20–80 % und rundet auf ganze Prozent. */
export function clampSplit(share: number): number {
  return Math.round(Math.min(80, Math.max(20, share)));
}

/** Gemerkter Anteil als Text: 20–80 %, unlesbar oder leer gilt die Vorgabe. */
export function parseSplit(raw: string, dflt: number): number {
  const n = Number(raw);
  return raw.trim() !== '' && Number.isFinite(n) ? clampSplit(n) : dflt;
}

/** Gemerkter Anteil eines Splitters unter `k3c-studio.<key>`; liefert Wert, Setzen und Zurücksetzen auf die Vorgabe. */
export function useSplit(key: string, dflt: number): [number, (v: number) => void, () => void] {
  const [split, setState] = useState(() => parseSplit(loadText(key, ''), dflt));
  const set = (v: number) => {
    const n = clampSplit(v);
    setState(n);
    savePref(key, String(n));
  };
  return [split, set, () => set(dflt)];
}

/** Splitter: Ziehen setzt den Anteil der linken Spalte (20–80 %), Doppelklick setzt zurück. Mit `horizontal` ein
 *  waagerechter Balken, dessen Wert der Anteil des unteren Bereichs ist. */
export function Splitter({ value, onChange, onReset, horizontal = false, className = 'pl-split-v' }:
  { value: number; onChange: (v: number) => void; onReset: () => void; horizontal?: boolean; className?: string }) {
  const ref = useRef<HTMLDivElement>(null);
  const drag = (e: React.PointerEvent) => {
    const box = ref.current?.parentElement?.getBoundingClientRect();
    if (!box) return;
    e.preventDefault();
    const move = (ev: PointerEvent) => onChange(clampSplit(splitShare(horizontal, box, ev.clientX, ev.clientY)));
    const up = () => {
      window.removeEventListener('pointermove', move);
      window.removeEventListener('pointerup', up);
    };
    window.addEventListener('pointermove', move);
    window.addEventListener('pointerup', up);
  };
  return (
    <div ref={ref} className={className} role="separator" aria-orientation={horizontal ? 'horizontal' : 'vertical'} aria-valuenow={value}
      aria-valuemin={20} aria-valuemax={80} title="Ziehen ändert die Größe, Doppelklick setzt zurück" onPointerDown={drag} onDoubleClick={onReset} />
  );
}
