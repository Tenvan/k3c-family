import { Checkbox, Text } from '@radix-ui/themes';
import { useState } from 'react';
import buildings from '@data/buildings.json';
import type { SiteLook } from '../api';
import { buildingGroups, type BuildingToggle } from './stageLogic';

/** Baustufen einer Art (data/buildings.json › levels); 0 = die Art hat keine Stufen */
const levelCount = (kind: string): number => (buildings as Record<string, { levels?: unknown[] }>)[kind]?.levels?.length ?? 0;

const HP_STEPS = [100, 75, 50, 25, 10] as const;

/** Zustand eines Platzes für die Vorschau: Baustufe (Auto = wie die Hub-Stufe sie ausbaut), Trefferpunkte (Zerstörungsstufe)
 *  oder nicht gebaut. */
function SiteLookControls({ site, look, onLook }: { site: BuildingToggle; look: SiteLook; onLook: (look: SiteLook) => void }) {
  const levels = levelCount(site.kind);
  const state = look.unbuilt ? 'unbuilt' : String(look.hpPercent ?? 0);
  return (
    <span className="wd-look">
      {levels > 0 && (
        <select className="wd-select" aria-label={`Baustufe ${site.key}`} value={look.level ?? 0} disabled={look.unbuilt} onChange={(e) => onLook({ ...look, level: Number(e.target.value) || undefined })}>
          <option value={0}>Stufe Auto</option>
          {Array.from({ length: levels }, (_, i) => <option key={i} value={i + 1}>{`Stufe ${i + 1}`}</option>)}
        </select>
      )}
      <select className="wd-select" aria-label={`Zustand ${site.key}`} value={state}
        onChange={(e) => { const v = e.target.value; onLook(v === 'unbuilt' ? { unbuilt: true } : { level: look.level, hpPercent: Number(v) || undefined }); }}>
        <option value={0}>Zustand Auto</option>
        {HP_STEPS.map((p) => <option key={p} value={p}>{`${p} % Trefferpunkte`}</option>)}
        <option value="unbuilt">Nicht gebaut</option>
      </select>
    </span>
  );
}

/** Gebäude-Filter der Live-Vorschau (B-477/AC-05): nach Art gruppiert, je Gruppe ein Schalter „alle ein/aus“ (gemischt, wenn
 *  nur ein Teil an ist) und eine aufklappbare Liste der einzelnen Plätze. `off` sind die abgewählten Schlüssel. */
export function BuildingFilter({ toggles, off, onSet, looks, onLook }: {
  toggles: readonly BuildingToggle[]; off: ReadonlySet<string>; onSet: (keys: string[], on: boolean) => void;
  /** Zustand je Platz (Schlüssel kind@x); mit `onLook` zeigt jeder Platz Baustufe und Zustand zur Auswahl */
  looks?: Readonly<Record<string, SiteLook>>; onLook?: (key: string, look: SiteLook) => void;
}) {
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
                  {onLook && !off.has(b.key) && <SiteLookControls site={b} look={looks?.[b.key] ?? {}} onLook={(look) => onLook(b.key, look)} />}
                </li>
              ))}
            </ul>
          )}
        </li>
      ))}
    </ul>
  );
}
