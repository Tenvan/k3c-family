import { Button, Checkbox, Text } from '@webkit/atoms';
import { ChipSwitch } from '@webkit/molecules';
import { useState } from 'react';
import buildings from '@data/buildings.json';
import type { SiteLook } from '../api';
import { PRESETS, applyLook, buildingGroups, commonOf, hpOf, presetLooks, type BuildingToggle, type PresetId } from './stageLogic';

/** Baustufen einer Art (data/buildings.json › levels); 0 = die Art hat keine Stufen */
const levelCount = (kind: string): number => (buildings as Record<string, { levels?: unknown[] }>)[kind]?.levels?.length ?? 0;

const HP_STEPS = [100, 75, 50, 25, 10] as const;
const HP_OPTIONS = [...HP_STEPS.map((p) => ({ value: String(p), label: String(p) })), { value: '0', label: '–' }];

const hpText = (hp: number | undefined) => (hp === undefined ? 'gemischt' : hp === 0 ? 'nicht gebaut' : `${hp} %`);
const levelText = (level: number | undefined) => (level === undefined ? 'gemischt' : level === 0 ? 'Stufe Auto' : `Stufe ${level}`);

type Looks = Readonly<Record<string, SiteLook>>;

/** Karte der gewählten Bauplätze: Baustufe und Zustand als Segment-Knöpfe (B-666); mit einer Art „Auf alle <Art> anwenden“.
 *  Auch im Inspektor des Gebäude-Planers (B-670). */
export function SelectionCard({ toggles, selected, looks, onLooks, label }: { toggles: readonly BuildingToggle[]; selected: readonly string[]; looks: Looks; onLooks: (next: Looks) => void; label: (kind: string) => string }) {
  const first = toggles.find((t) => t.key === selected[0])!;
  const sameKind = selected.every((k) => toggles.find((t) => t.key === k)?.kind === first.kind);
  const levels = sameKind ? levelCount(first.kind) : 0;
  const level = commonOf(selected, (k) => looks[k]?.level ?? 0);
  const hp = commonOf(selected, (k) => hpOf(looks[k]));
  const title = selected.length === 1 ? `${label(first.kind)} bei x ${first.x}` : `${selected.length} gewählt${sameKind ? `: ${label(first.kind)}` : ''}`;
  const allOfKind = toggles.filter((t) => t.kind === first.kind).map((t) => t.key);
  const apply = (patch: Partial<SiteLook>) => onLooks(applyLook(looks, selected, patch));
  return (
    <section className="wd-card wd-selected" aria-label="Gewählte Bauplätze">
      <Text size="2" weight="bold">{title}</Text>
      {levels > 0 && (
        <ChipSwitch label="Baustufe" caption="Baustufe" value={level === undefined ? '' : String(level)} onChange={(v) => apply({ level: Number(v) || undefined })}
          options={[{ value: '0', label: 'Auto', disabled: hp === 0 }, ...Array.from({ length: levels }, (_, i) => ({ value: String(i + 1), label: String(i + 1), disabled: hp === 0 }))]} />
      )}
      <ChipSwitch label="Zustand" caption="Zustand %" value={hp === undefined ? '' : String(hp)} options={HP_OPTIONS}
        onChange={(v) => apply(v === '0' ? { unbuilt: true } : { hpPercent: Number(v) === 100 ? undefined : Number(v) })} />
      <Text size="1" color="gray">{levels > 0 ? `${levelText(level)} · ` : ''}{hpText(hp)}</Text>
      {sameKind && selected.length < allOfKind.length && (
        <Button size="1" variant="soft" onClick={() => onLooks(applyLook(looks, allOfKind, { level: looks[selected[0]!]?.level, hpPercent: looks[selected[0]!]?.hpPercent, unbuilt: looks[selected[0]!]?.unbuilt }))}>
          Auf alle {allOfKind.length} {label(first.kind)} anwenden
        </Button>
      )}
    </section>
  );
}

/** Bedienung der Gebäude-Zustände im Reiter Gebäude des Inspektors (B-666): Hinweis „nur Vorschau“, Voreinstellungen für alle
 *  Bauplätze, die gewählten Bauplätze mit Segment-Knöpfen und je Gebäudeart eine Zeile mit Sichtbarkeit, Baustufe, Zustand und
 *  den Plätzen als Chips (Balken = Trefferpunkte, gestrichelt = nicht gebaut). Ein Klick auf einen Chip wählt den Bauplatz und
 *  springt in der Live-Vorschau dorthin, Umschalt + Klick erweitert die Auswahl. `off` sind die abgewählten Schlüssel. */
export function BuildingInspector({ toggles, off, onSet, looks, onLooks, selected, onSelect }: {
  toggles: readonly BuildingToggle[]; off: ReadonlySet<string>; onSet: (keys: string[], on: boolean) => void;
  looks: Looks; onLooks: (next: Looks) => void; selected: readonly string[]; onSelect: (keys: string[], jump: boolean) => void;
}) {
  const [open, setOpen] = useState<ReadonlySet<string>>(new Set());
  const [preset, setPreset] = useState<PresetId | ''>('');
  const groups = buildingGroups(toggles, off);
  const labelOf = (kind: string) => groups.find((g) => g.kind === kind)?.label ?? kind;
  const flip = (kind: string, on?: boolean) => setOpen((o) => { const next = new Set(o); if (on ?? !next.has(kind)) next.add(kind); else next.delete(kind); return next; });
  const reset = () => { setPreset(''); onLooks({}); };
  const change = (next: Looks) => { setPreset(''); onLooks(next); };
  return (
    <div className="wd-buildings-inspector">
      <div className="wd-hint" role="note">
        <Text size="1">Nur Vorschau: Zustände werden nicht gespeichert.</Text>
        <Button size="1" variant="soft" color="gray" onClick={reset}>Zurücksetzen</Button>
      </div>
      <ChipSwitch label="Voreinstellung" caption="Schnell setzen" value={preset} options={PRESETS.map((p) => ({ value: p.id, label: p.label }))}
        onChange={(id) => { setPreset(id); onLooks(presetLooks(id, toggles, levelCount)); }} />
      {selected.length > 0 && <SelectionCard toggles={toggles} selected={selected} looks={looks} onLooks={change} label={labelOf} />}
      <ul className="wd-buildings" aria-label="Gebäude">
        {groups.map((g) => {
          const keys = g.items.map((t) => t.key);
          const level = commonOf(keys, (k) => looks[k]?.level ?? 0);
          const hp = commonOf(keys, (k) => hpOf(looks[k]));
          return (
            <li key={g.kind} className="wd-group" data-state={g.state}>
              <div className="wd-group-head">
                <Checkbox checked={g.state === 'all' ? true : g.state === 'none' ? false : 'indeterminate'} aria-label={`${g.label} alle`} onCheckedChange={() => onSet(keys, g.state !== 'all')} />
                <Button size="1" variant="ghost" color="gray" highContrast className="wd-group-name" aria-label={`${g.label} wählen`} onClick={() => { onSelect(keys.filter((k) => !off.has(k)), false); flip(g.kind, true); }}>
                  {g.label} <Text size="1" color="gray">{g.on}/{g.items.length}</Text>
                </Button>
                <Text size="1" color={level === undefined || hp === undefined ? 'orange' : 'gray'} className="wd-summary">{levelCount(g.kind) > 0 ? `${levelText(level)} · ` : ''}{hpText(hp)}</Text>
                <Button size="1" variant="soft" color="gray" aria-label={`${g.label} aufklappen`} aria-expanded={open.has(g.kind)} onClick={() => flip(g.kind)}>{open.has(g.kind) ? '▾' : '▸'}</Button>
              </div>
              {open.has(g.kind) && (
                <ul className="wd-sites" aria-label={`${g.label}: Bauplätze`}>
                  {g.items.map((b) => {
                    const look = looks[b.key];
                    const value = hpOf(look);
                    const isOff = off.has(b.key);
                    return (
                      <li key={b.key}>
                        <Button size="1" variant={selected.includes(b.key) ? 'solid' : 'soft'} color={selected.includes(b.key) ? undefined : 'gray'} className="wd-site" data-unbuilt={value === 0} disabled={isOff}
                          aria-pressed={selected.includes(b.key)} aria-label={`${g.label} bei x ${b.x}, ${hpText(value)}`}
                          onClick={(e) => onSelect(e.shiftKey ? (selected.includes(b.key) ? selected.filter((k) => k !== b.key) : [...selected, b.key]) : [b.key], true)}>
                          {b.x}<span className="wd-hpbar" aria-hidden style={{ width: `${value}%` }} />
                        </Button>
                      </li>
                    );
                  })}
                </ul>
              )}
            </li>
          );
        })}
      </ul>
    </div>
  );
}
