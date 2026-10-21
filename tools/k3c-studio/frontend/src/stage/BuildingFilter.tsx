import { Button, Checkbox, SelectField, Text } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { useState } from 'react';
import buildings from '@data/buildings.json';
import type { SiteLook } from '../api';
import { buildingGroups, type BuildingToggle } from './stageLogic';

/** Baustufen einer Art (data/buildings.json › levels); 0 = die Art hat keine Stufen */
const levelCount = (kind: string): number => (buildings as Record<string, { levels?: unknown[] }>)[kind]?.levels?.length ?? 0;

const HP_STEPS = [100, 75, 50, 25, 10] as const;
const STATE_OPTIONS = [
  { value: '0', label: 'Zustand Auto' },
  ...HP_STEPS.map((p) => ({ value: String(p), label: `${p} % Trefferpunkte` })),
  { value: 'unbuilt', label: 'Nicht gebaut' },
];

/** Zustand eines Platzes für die Vorschau: Baustufe (Auto = wie die Hub-Stufe sie ausbaut), Trefferpunkte (Zerstörungsstufe)
 *  oder nicht gebaut. */
function SiteLookControls({ site, look, onLook }: { site: BuildingToggle; look: SiteLook; onLook: (look: SiteLook) => void }) {
  const levels = levelCount(site.kind);
  const state = look.unbuilt ? 'unbuilt' : String(look.hpPercent ?? 0);
  return (
    <span className="wd-look">
      {levels > 0 && (
        <SelectField aria-label={`Baustufe ${site.key}`} value={String(look.level ?? 0)} disabled={look.unbuilt} onChange={(v) => onLook({ ...look, level: Number(v) || undefined })}
          options={[{ value: '0', label: 'Stufe Auto' }, ...Array.from({ length: levels }, (_, i) => ({ value: String(i + 1), label: `Stufe ${i + 1}` }))]} />
      )}
      <SelectField aria-label={`Zustand ${site.key}`} value={state} options={STATE_OPTIONS}
        onChange={(v) => onLook(v === 'unbuilt' ? { unbuilt: true } : { level: look.level, hpPercent: Number(v) || undefined })} />
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
            <Button size="1" variant="ghost" color="gray" highContrast className="wd-group-name" aria-expanded={open.has(g.kind)} onClick={() => flip(g.kind)}>
              <span aria-hidden>{open.has(g.kind) ? '▾' : '▸'}</span> {g.label} <Text size="1" color="gray">{g.on}/{g.items.length}</Text>
            </Button>
          </div>
          {open.has(g.kind) && (
            <ul className="wd-group-items">
              {g.items.map((b) => (
                <li key={b.key}>
                  <CheckField size="1" label={b.x} checked={!off.has(b.key)} onCheckedChange={(c) => onSet([b.key], c === true)} aria-label={b.key} />
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
