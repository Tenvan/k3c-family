import { Text } from '@webkit/atoms';
import type { WorldDetail } from '../api';
import { useMemo } from 'react';
import { useCatalog } from '../assets/catalog/useCatalog';
import { useWeltSicht } from '../assets/useWeltSicht';
import { worldProgress, worldStats, type OwnAssets } from './statsCount';

// Statistik rechts oben im Kopf des Welten-Editors: Fortschritt und je Zeile vorhanden / nötig, was fehlt, in Warnfarbe.

export function WorldStats({ detail }: { detail: WorldDetail }) {
  const { list } = useCatalog();
  const { sicht } = useWeltSicht(detail.id);
  // eigene Assets: die exklusiv dieser Welt zugeordneten; die Basis-Welt besitzt alle und bekommt keine Zeile
  const own = useMemo<OwnAssets | null>(() => {
    if (!sicht || sicht.welt === sicht.basis) return null;
    const mine = list.filter((a) => sicht.eigene[`${a.gruppe}/${a.id}`] === sicht.welt);
    return { groups: [...new Set(mine.map((a) => a.gruppe))], count: mine.length };
  }, [list, sicht]);
  const lines = worldStats(detail, own);
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
            <Text size="1" color="gray">{l.optional ? l.have : `${l.have}/${l.need}`}</Text>
            {l.missing && <Text size="1" color="orange">fehlt: {l.missing}</Text>}
          </li>
        ))}
      </ul>
    </aside>
  );
}
