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

/** Senkrechter Splitter: Ziehen setzt den Anteil der linken Spalte (20–80 %), Doppelklick setzt zurück. */
export function Splitter({ value, onChange, onReset }: { value: number; onChange: (v: number) => void; onReset: () => void }) {
  const ref = useRef<HTMLDivElement>(null);
  const drag = (e: React.PointerEvent) => {
    const box = ref.current?.parentElement?.getBoundingClientRect();
    if (!box) return;
    e.preventDefault();
    const move = (ev: PointerEvent) => onChange(Math.round(Math.min(80, Math.max(20, ((ev.clientX - box.left) / box.width) * 100))));
    const up = () => {
      window.removeEventListener('pointermove', move);
      window.removeEventListener('pointerup', up);
    };
    window.addEventListener('pointermove', move);
    window.addEventListener('pointerup', up);
  };
  return (
    <div ref={ref} className="tk-split" role="separator" aria-orientation="vertical" aria-valuenow={value} aria-valuemin={20}
      aria-valuemax={80} title="Ziehen ändert die Breite, Doppelklick setzt zurück" onPointerDown={drag} onDoubleClick={onReset} />
  );
}
