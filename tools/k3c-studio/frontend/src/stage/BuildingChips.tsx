import { Button } from '@webkit/atoms';
import type { CSSProperties } from 'react';
import { buildingGroups, kindColor, kindLabel, stripLanes, type BuildingToggle } from './stageLogic';

/** Mindestabstand zweier Marken einer Spur als Anteil der Leistenbreite */
const LANE_GAP = 0.012;

/** Gebäude über der Bühne (Gebäude-Planer, B-670): eine Zeile Arten-Chips mit Farbe und Anzahl (Klick blendet die Art aus und
 *  ein, ausgeblendet durchgestrichen) und darunter die Lageleiste: jeder Platz eine Marke in der Farbe seiner Art an seiner
 *  Stelle in der Welt. Ein Klick auf eine Marke wählt den Platz (`onSelect`, mit Sprung); Plätze einer ausgeblendeten Art
 *  sind nicht wählbar. `off` sind die abgewählten Schlüssel. */
export function BuildingChips({ toggles, off, onSet, selected, onSelect }: {
  toggles: readonly BuildingToggle[]; off: ReadonlySet<string>; onSet: (keys: string[], on: boolean) => void;
  selected: readonly string[]; onSelect: (keys: string[], jump: boolean) => void;
}) {
  const groups = buildingGroups(toggles, off);
  return (
    <div className="pl-chipbar" role="group" aria-label="Gebäude">
      <div className="pl-kinds">
        {groups.map((g) => (
          <span key={g.kind} className="pl-kindchip" data-state={g.state} style={{ '--kind': kindColor(g.kind) } as CSSProperties}>
            <Button size="1" variant="soft" color="gray" className="pl-kind" aria-pressed={g.state !== 'none'}
              title={g.state === 'none' ? 'einblenden' : 'ausblenden'} onClick={() => onSet(g.items.map((t) => t.key), g.state !== 'all')}>
              <span className="pl-dot" aria-hidden />
              {g.label} <span className="pl-count">{g.state === 'some' ? `${g.on}/${g.items.length}` : g.items.length}</span>
            </Button>
          </span>
        ))}
      </div>
      <SiteStrip toggles={toggles} off={off} selected={selected} onSelect={onSelect} />
    </div>
  );
}

/** Lageleiste: Marken je Platz von links nach rechts über die Spannweite der Plätze, nahe Plätze in eigenen Spuren. */
function SiteStrip({ toggles, off, selected, onSelect }: {
  toggles: readonly BuildingToggle[]; off: ReadonlySet<string>; selected: readonly string[]; onSelect: (keys: string[], jump: boolean) => void;
}) {
  if (toggles.length === 0) return null;
  const min = toggles[0]!.x;
  const span = Math.max(1, toggles.at(-1)!.x - min);
  const { lanes, count } = stripLanes(toggles.map((t) => t.x), span * LANE_GAP);
  return (
    <ul className="pl-strip" aria-label="Bauplätze nach Lage" style={{ '--lanes': count } as CSSProperties}>
      {toggles.map((b, i) => {
        const on = selected.includes(b.key);
        const style = { '--kind': kindColor(b.kind), '--f': (b.x - min) / span, '--lane': lanes[i] } as CSSProperties;
        return (
          <li key={b.key} style={style}>
            <Button size="1" variant="ghost" className="pl-site" disabled={off.has(b.key)} aria-pressed={on} data-on={on || undefined}
              aria-label={`${kindLabel(b.kind)} bei x ${b.x}`} title={`${kindLabel(b.kind)} · x ${b.x}`} onClick={() => onSelect([b.key], true)} />
          </li>
        );
      })}
      <li className="pl-strip-end" aria-hidden>{min}</li>
      <li className="pl-strip-end pl-strip-max" aria-hidden>{toggles.at(-1)!.x}</li>
    </ul>
  );
}
