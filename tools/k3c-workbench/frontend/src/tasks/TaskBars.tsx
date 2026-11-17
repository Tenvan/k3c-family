import type { TaskCatalog } from '../api';
import { Button, Text, TextField } from '@webkit/atoms';

interface HeadProps {
  query: string;
  reloading: boolean;
  onQuery: (q: string) => void;
  onReload: () => void;
}

/** Kopf der Tasks-Seite: Filter (Esc leert) · Neu laden. */
export function TasksHead(p: HeadProps) {
  return (
    <div className="tk-head">
      <TextField.Root className="tk-filter" size="2" placeholder="Filter: Name oder Beschreibung (Esc leert)" value={p.query}
        onChange={(e) => p.onQuery(e.target.value)} onKeyDown={(e) => e.key === 'Escape' && p.onQuery('')} />
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
