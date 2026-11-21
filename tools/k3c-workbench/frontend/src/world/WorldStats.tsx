import { Text } from '@webkit/atoms';
import type { WorldDetail } from '../api';
import { worldProgress, worldStats } from './statsCount';

// Statistik rechts oben im Kopf des Welten-Editors: Fortschritt und je Zeile vorhanden / nötig, was fehlt, in Warnfarbe.

export function WorldStats({ detail }: { detail: WorldDetail }) {
  const lines = worldStats(detail);
  const pct = worldProgress(lines);
  return (
    <aside className="wd-stats" aria-label="Stand der Welt">
      <Text as="div" size="2" weight="bold">Stand der Welt · {pct} % fertig</Text>
      <div className="wd-stats-bar" role="progressbar" aria-valuenow={pct} aria-valuemin={0} aria-valuemax={100}><i style={{ width: `${pct}%` }} /></div>
      <ul className="wd-stats-list">
        {lines.map((l) => (
          <li key={l.key} data-stat={l.key} data-ok={String(l.missing === '')}>
            <span aria-hidden="true">{l.missing === '' ? '✅' : '⚠️'}</span>
            <Text size="1">{l.label}</Text>
            <Text size="1" color="gray">{l.have}/{l.need}</Text>
            {l.missing && <Text size="1" color="orange">fehlt: {l.missing}</Text>}
          </li>
        ))}
      </ul>
    </aside>
  );
}
