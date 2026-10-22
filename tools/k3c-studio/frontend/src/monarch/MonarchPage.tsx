import { Button, SelectField, Text, plainOptions, type SelectOption } from '@webkit/atoms';
import { useEffect, useMemo, useRef, useState } from 'react';
import { backend, type AssetSlot } from '../api';
import { DEFAULT_CONTROL, type Anim, type Control } from '../playback/figure';
import { FigureStage } from '../playback/FigureStage';
import { GapBadge, PlaybackControls } from '../playback/PlaybackControls';
import type { FigureInfo } from '../stage/PreviewStage';
import { ACTIONS, actionOf, type ActionId } from './actions';
import { SkillList } from './SkillList';
import { stepZoom, wheelZoom } from './monarchZoom';
import {
  DEFAULT_MOUNT, MONARCH_ANIMS, MOUNT_KEYS, MOVES, PLAYERS, monarchItems, monarchSlotId, moveOf, mountSlotId, slotOptions, type Choice, type MoveId, type SlotOption,
} from './monarchLogic';

/** Slot-Register des Studios (nur lesen): Zuordnungen der Slots `monarch:*` und `mount:*` als Vorschau-Auswahl */
function useSlots(): AssetSlot[] {
  const [slots, setSlots] = useState<AssetSlot[]>([]);
  useEffect(() => {
    let current = true;
    void backend.assetsList().then((v) => { if (current && !v.error) setSlots(v.slots); });
    return () => { current = false; };
  }, []);
  return slots;
}

const pick = (options: SlotOption[], index = 0): SlotOption | undefined => options.find((o) => o.index === index) ?? options[0];

/** Auswahl der Seite: Reittier, Bewegung, Animation, Wiedergabe und die Grafik-Auswahl aus den Slots. Alles, was die Bühne
 *  zeigt, steht in `items`; sie ändern sich nur, wenn sich eine Eingabe ändert. */
function useMonarch() {
  const slots = useSlots();
  const [mount, setMount] = useState(DEFAULT_MOUNT);
  const [move, setMove] = useState<MoveId>('stand');
  const [anim, setAnim] = useState<Anim>('idle');
  const [actionId, setActionId] = useState<ActionId | ''>('');
  const action = actionId ? actionOf(actionId) : undefined;
  const [control, setControl] = useState<Control>(DEFAULT_CONTROL);
  const [riderPick, setRiderPick] = useState<Record<number, number>>({});
  const [mountPick, setMountPick] = useState<Record<string, number>>({});
  const riderOptions = useMemo(() => PLAYERS.map((p) => slotOptions(slots, monarchSlotId(p.index))), [slots]);
  const mountOptions = useMemo(() => slotOptions(slots, mountSlotId(mount)), [slots, mount]);
  const riders = PLAYERS.map((p) => pick(riderOptions[p.index]!, riderPick[p.index]));
  const mountChoice = pick(mountOptions, mountPick[mount]);
  const choice: Choice = {
    riders: riders.map((o) => (o && !o.gap ? o.sheet : undefined)),
    ...(mountChoice && !mountChoice.gap ? { mountSheet: mountChoice.sheet } : {}),
  };
  const key = JSON.stringify([mount, move, anim, choice, actionId]);
  const items = useMemo(() => monarchItems(mount, moveOf(move), anim, choice, action), [key]); // eslint-disable-line react-hooks/exhaustive-deps
  return {
    mount, setMount, move, setMove, anim, setAnim, actionId, setActionId, action, control, setControl, items, riderOptions, mountOptions, riders, mountChoice,
    pickRider: (index: number, option: number) => setRiderPick({ ...riderPick, [index]: option }),
    pickMount: (option: number) => setMountPick({ ...mountPick, [mount]: option }),
  };
}

type Monarch = ReturnType<typeof useMonarch>;

const NO_ACTION: SelectOption<ActionId | ''> = { value: '', label: 'Keine (Bewegung)' };
const ACTION_OPTIONS: SelectOption<ActionId | ''>[] = [NO_ACTION, ...ACTIONS.map((a) => ({ value: a.id, label: a.label }))];
const MOVE_OPTIONS: SelectOption<MoveId>[] = MOVES.map((o) => ({ value: o.id, label: o.label }));

/** Einträge der Grafik-Auswahl aus den Slots: Wert ist der Index der Präferenz. */
const sheetOptions = (options: SlotOption[]): SelectOption<string>[] =>
  options.map((o) => ({ value: String(o.index), label: `Präferenz ${o.index + 1}: ${o.sheet}` }));

function Bar({ m, gap }: { m: Monarch; gap: FigureInfo['gap'] | undefined }) {
  return (
    <div className="wd-bar">
      <SelectField aria-label="Reittier" value={m.mount} options={plainOptions(MOUNT_KEYS)} onChange={m.setMount} />
      <SelectField aria-label="Aktion" value={m.actionId} options={ACTION_OPTIONS} onChange={m.setActionId} />
      <SelectField aria-label="Bewegung" disabled={m.actionId !== ''} value={m.move} options={MOVE_OPTIONS} onChange={m.setMove} />
      <SelectField aria-label="Animation" disabled={m.actionId !== ''} value={m.anim} options={plainOptions(MONARCH_ANIMS)} onChange={m.setAnim} />
      {m.mountOptions.length > 0 && (
        <SelectField aria-label="Grafik Reittier" value={String(m.mountChoice?.index ?? 0)} options={sheetOptions(m.mountOptions)} onChange={(v) => m.pickMount(Number(v))} />
      )}
      {m.mountChoice?.gap && <GapBadge gap={m.mountChoice.gap} />}
      <GapBadge gap={gap} />
      {m.action?.gaps.map((g) => <GapBadge key={g.reason} gap={g} />)}
    </div>
  );
}

function Colors({ m }: { m: Monarch }) {
  return (
    <ul className="wd-colors" aria-label="Spielerfarben">
      {PLAYERS.map((p) => (
        <li key={p.index} data-index={p.index}>
          <span className="wd-badge" style={{ background: p.badge }} title={`Kennfarbe ${p.badge}`} />
          <Text size="2">Spieler {p.index + 1}: {m.riders[p.index]?.sheet ?? p.spec.sheet}{p.spec.tint ? ` · tint ${p.spec.tint}` : ''}</Text>
          {m.riderOptions[p.index]!.length > 0 && (
            <SelectField aria-label={`Grafik Monarch ${p.index + 1}`} value={String(m.riders[p.index]?.index ?? 0)} options={sheetOptions(m.riderOptions[p.index]!)}
              onChange={(v) => m.pickRider(p.index, Number(v))} />
          )}
          {m.riders[p.index]?.gap && <GapBadge gap={m.riders[p.index]!.gap} />}
        </li>
      ))}
    </ul>
  );
}

/** Monarch-Seite des Asset-Studios (B-463): der Monarch immer beritten, mit den Animationen des Spiels. Reiten auf jedem der
 *  13 Reittiere, Laufen und Sprinten beritten, die vier Spielerfarben nebeneinander im selben Takt mit Kennfarbe,
 *  Grafik-Auswahl aus den Slots `monarch:*` und `mount:*` als Vorschau (das Register bleibt unverändert). */
export function MonarchPage() {
  const m = useMonarch();
  const [infos, setInfos] = useState<FigureInfo[]>([]);
  const [missing, setMissing] = useState<string[]>([]);
  const [zoom, setZoom] = useState(1);
  const stageBox = useRef<HTMLDivElement>(null);
  // nativer Listener: React hängt Wheel-Handler passiv an, preventDefault (die Seite soll bei Alt + Rad nicht scrollen) braucht passive: false
  useEffect(() => {
    const el = stageBox.current;
    if (!el) return;
    const onWheel = (e: WheelEvent) => {
      if (!wheelZoom(1, e)) return;
      e.preventDefault();
      setZoom((z) => wheelZoom(z, e) ?? z);
    };
    el.addEventListener('wheel', onWheel, { passive: false });
    return () => el.removeEventListener('wheel', onWheel);
  }, []);
  // Zwei Spalten (B-502): links die Skills, rechts die Monarchen; schmal untereinander (CSS)
  return (
    <div className="wd-page wd-monarch">
      <aside className="wd-monarch-skills"><SkillList /></aside>
      <div className="wd-monarch-main">
        <Bar m={m} gap={infos[0]?.gap} />
        <PlaybackControls control={m.control} onChange={m.setControl} frames={infos[0]?.frames ?? 1} />
        <div className="wd-bar" role="group" aria-label="Zoom">
          <Button variant="soft" aria-label="Verkleinern" onClick={() => setZoom((z) => stepZoom(z, -1))}>−</Button>
          <Text size="2" aria-label="Zoomstufe">{Math.round(zoom * 100)} %</Text>
          <Button variant="soft" aria-label="Vergrößern" onClick={() => setZoom((z) => stepZoom(z, 1))}>+</Button>
          <Button variant="outline" aria-label="Zoom zurücksetzen" disabled={zoom === 1} onClick={() => setZoom(1)}>Spielmaßstab</Button>
          <Text size="1" color="gray">Alt + Mausrad zoomt</Text>
        </div>
        {missing.length > 0 && <Text as="p" size="2" color="red" role="status">Grafiken nicht geladen: {missing.join(', ')}</Text>}
        <div className="wd-monarch-stage" ref={stageBox}>
          <FigureStage items={m.items} control={m.control} zoom={zoom} onInfo={setInfos} onLoadError={setMissing} />
        </div>
        <Colors m={m} />
      </div>
    </div>
  );
}
