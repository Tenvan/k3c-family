import { Button } from '@webkit/atoms';
import { useState } from 'react';
import { buildingGroups, type BuildingToggle } from './stageLogic';

/** Chip-Leiste der Gebäude über der Bühne (Gebäude-Planer, B-670): je Art ein Chip mit Anzahl (Klick blendet die Art aus
 *  und ein, ausgeblendet durchgestrichen) und ▾ für die Plätze der Art als kleine Chips. Ein Klick auf einen Platz wählt
 *  ihn (`onSelect`, mit Sprung); Plätze einer ausgeblendeten Art sind nicht wählbar. `off` sind die abgewählten Schlüssel. */
export function BuildingChips({ toggles, off, onSet, selected, onSelect }: {
  toggles: readonly BuildingToggle[]; off: ReadonlySet<string>; onSet: (keys: string[], on: boolean) => void;
  selected: readonly string[]; onSelect: (keys: string[], jump: boolean) => void;
}) {
  const [open, setOpen] = useState<ReadonlySet<string>>(new Set());
  const flip = (kind: string) => setOpen((o) => { const next = new Set(o); if (next.has(kind)) next.delete(kind); else next.add(kind); return next; });
  const groups = buildingGroups(toggles, off);
  return (
    <div className="pl-chipbar" role="group" aria-label="Gebäude">
      <div className="pl-kinds">
        {groups.map((g) => (
          <span key={g.kind} className="pl-kindchip" data-state={g.state}>
            <Button size="1" variant="soft" color={g.state === 'none' ? 'gray' : undefined} className="pl-kind" aria-pressed={g.state !== 'none'}
              title={g.state === 'none' ? 'einblenden' : 'ausblenden'} onClick={() => onSet(g.items.map((t) => t.key), g.state !== 'all')}>
              {g.label} {g.items.length}
            </Button>
            <Button size="1" variant="ghost" color="gray" className="pl-open" aria-label={`${g.label}: Plätze`} aria-expanded={open.has(g.kind)} onClick={() => flip(g.kind)}>
              {open.has(g.kind) ? '▴' : '▾'}
            </Button>
          </span>
        ))}
      </div>
      {groups.filter((g) => open.has(g.kind)).map((g) => (
        <ul key={g.kind} className="pl-sites" aria-label={`${g.label}: Bauplätze`}>
          {g.items.map((b) => {
            const on = selected.includes(b.key);
            return (
              <li key={b.key}>
                <Button size="1" variant={on ? 'solid' : 'soft'} color={on ? undefined : 'gray'} className="pl-site" disabled={off.has(b.key)}
                  aria-pressed={on} aria-label={`${g.label} bei x ${b.x}`} onClick={() => onSelect([b.key], true)}>
                  {b.x}
                </Button>
              </li>
            );
          })}
        </ul>
      ))}
    </div>
  );
}
