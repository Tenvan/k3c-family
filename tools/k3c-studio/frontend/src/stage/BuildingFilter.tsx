import { Checkbox, Text } from '@radix-ui/themes';
import { useState } from 'react';
import { buildingGroups, type BuildingToggle } from './stageLogic';

/** Gebäude-Filter der Live-Vorschau (B-477/AC-05): nach Art gruppiert, je Gruppe ein Schalter „alle ein/aus“ (gemischt, wenn
 *  nur ein Teil an ist) und eine aufklappbare Liste der einzelnen Plätze. `off` sind die abgewählten Schlüssel. */
export function BuildingFilter({ toggles, off, onSet }: { toggles: readonly BuildingToggle[]; off: ReadonlySet<string>; onSet: (keys: string[], on: boolean) => void }) {
  const [open, setOpen] = useState<ReadonlySet<string>>(new Set());
  const flip = (kind: string) => setOpen((o) => { const next = new Set(o); if (next.has(kind)) next.delete(kind); else next.add(kind); return next; });
  return (
    <ul className="wd-buildings" aria-label="Gebäude">
      {buildingGroups(toggles, off).map((g) => (
        <li key={g.kind} className="wd-group" data-state={g.state}>
          <div className="wd-group-head">
            <Checkbox checked={g.state === 'all' ? true : g.state === 'none' ? false : 'indeterminate'} aria-label={`${g.label} alle`}
              onCheckedChange={() => onSet(g.items.map((t) => t.key), g.state !== 'all')} />
            <button type="button" className="wd-group-name" aria-expanded={open.has(g.kind)} onClick={() => flip(g.kind)}>
              <span aria-hidden>{open.has(g.kind) ? '▾' : '▸'}</span> {g.label} <Text size="1" color="gray">{g.on}/{g.items.length}</Text>
            </button>
          </div>
          {open.has(g.kind) && (
            <ul className="wd-group-items">
              {g.items.map((b) => (
                <li key={b.key}>
                  <Text as="label" size="1">
                    <Checkbox checked={!off.has(b.key)} onCheckedChange={(c) => onSet([b.key], c === true)} aria-label={b.key} /> {b.x}
                  </Text>
                </li>
              ))}
            </ul>
          )}
        </li>
      ))}
    </ul>
  );
}
