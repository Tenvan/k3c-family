import { Button, Switch, Text, TextField } from '@radix-ui/themes';
import { useRef } from 'react';
import type { TaskCatalog } from '../api';

interface HeadProps {
  query: string;
  onlyKey: boolean;
  pending: number;
  reloading: boolean;
  onQuery: (q: string) => void;
  onOnlyKey: (v: boolean) => void;
  onSave: () => void;
  onReload: () => void;
}

/** Kopf der Tasks-Seite: Filter (Esc leert) · 🔑 nur freigegebene · [n Schalter übernehmen] · Neu laden. */
export function TasksHead(p: HeadProps) {
  return (
    <div className="tk-head">
      <TextField.Root className="tk-filter" size="2" placeholder="Filter: Name oder Beschreibung (Esc leert)" value={p.query}
        onChange={(e) => p.onQuery(e.target.value)} onKeyDown={(e) => e.key === 'Escape' && p.onQuery('')} />
      <Text as="label" size="2" className="tk-key">
        <Switch size="1" checked={p.onlyKey} onCheckedChange={p.onOnlyKey} />
        🔑 nur freigegebene
      </Text>
      {p.pending > 0 && <Button variant="soft" color="amber" onClick={p.onSave}>{p.pending} Schalter übernehmen</Button>}
      <Button variant="soft" loading={p.reloading} onClick={p.onReload}>Neu laden</Button>
    </div>
  );
}

interface FootProps {
  catalog: TaskCatalog;
  shown: number;
  filter: string;
  running: number;
  error: string;
}

/** Fußzeile: n von m Tasks · Filter · k laufen · Zuletzt geladen hh:mm; ein gescheiterter Reload steht hier. */
export function TasksFoot({ catalog, shown, filter, running, error }: FootProps) {
  const parts = [`${shown} von ${catalog.count} Tasks`];
  if (filter) parts.push(`Filter: ${filter}`);
  if (running > 0) parts.push(`${running} ${running === 1 ? 'läuft' : 'laufen'}`);
  if (catalog.loadedAt) {
    parts.push(`Zuletzt geladen ${new Date(catalog.loadedAt).toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' })}`);
  }
  if (error) parts.push(`Neu laden fehlgeschlagen: ${error}`);
  return <Text size="1" color={error ? 'red' : 'gray'} className="tk-foot">{parts.join(' · ')}</Text>;
}

/** Anteil in Prozent, den der Zeiger im Kasten einnimmt: senkrechte Spalte von links, waagerechter Balken von unten. */
export function splitShare(horizontal: boolean, box: { left: number; bottom: number; width: number; height: number }, x: number, y: number): number {
  return horizontal ? ((box.bottom - y) / box.height) * 100 : ((x - box.left) / box.width) * 100;
}

/** Klemmt den Anteil auf 20–80 % und rundet auf ganze Prozent (B-403/AC-06). */
export function clampSplit(share: number): number {
  return Math.round(Math.min(80, Math.max(20, share)));
}

/** Splitter: Ziehen setzt den Anteil der linken Spalte (20–80 %), Doppelklick setzt zurück. Mit `horizontal` ein
 *  waagerechter Balken, dessen Wert der Anteil des unteren Bereichs ist (Planungsseite, B-400). */
export function Splitter({ value, onChange, onReset, horizontal = false, className = 'tk-split' }:
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
