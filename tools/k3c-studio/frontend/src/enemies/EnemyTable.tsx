import type { EnemyCard } from './enemyLogic';
import { cardGaps, type Enemies } from './useEnemies';

const KIND: Record<string, string> = { mini: 'Mini', end: 'End' };

/** Wert einer Zelle; fehlt der Gegner in den Daten oder hat der Boss den Wert nicht (nur Faktoren), steht „–“. */
const cell = (c: EnemyCard, v: number | string | undefined): string => (c.def && v !== undefined && !(c.boss && v === 0) ? String(v) : '–');

/** Gegner einer Gruppe als Tabelle (B-672): Name, Tier, Tiefe, HP, Schaden, Tempo, Reichweite, Lückenzahl; Bosse mit Art.
 *  Klick auf eine Zeile wählt den Gegner. */
export function EnemyTable({ e }: { e: Enemies }) {
  return (
    <table className="wd-enemy-table" aria-label="Gegner">
      <thead>
        <tr>
          <th>Name</th>{e.isBoss && <th>Art</th>}<th>Tier</th><th>Tiefe</th><th>HP</th><th>Schaden</th><th>Tempo</th><th>Reichweite</th><th>Lücken</th>
        </tr>
      </thead>
      <tbody>
        {e.cards.map((c) => {
          const d = c.def;
          const gaps = cardGaps(c, e.infoOf(c.id)?.gap).length;
          return (
            <tr key={c.id} data-enemy={c.id} aria-selected={c.id === e.selected?.id} onClick={() => e.setSelectedId(c.id)}>
              <td>{d?.name ?? c.id}</td>
              {e.isBoss && <td>{c.boss ? KIND[c.boss.kind] : '–'}</td>}
              <td>{cell(c, d?.tier)}</td><td>{d ? d.depth : '–'}</td><td>{cell(c, d?.hp)}</td><td>{cell(c, d?.damage)}</td>
              <td>{d ? d.speed : '–'}</td><td>{d ? d.range : '–'}</td>
              <td data-gaps={gaps}>{gaps}</td>
            </tr>
          );
        })}
      </tbody>
    </table>
  );
}
