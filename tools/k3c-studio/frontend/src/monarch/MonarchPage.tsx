import { Text } from '@radix-ui/themes';
import { useEffect, useMemo, useState } from 'react';
import { backend, type AssetSlot } from '../api';
import { DEFAULT_CONTROL, type Anim, type Control } from '../abspiel/figure';
import { FigureStage } from '../abspiel/FigureStage';
import { GapBadge, PlaybackControls } from '../abspiel/PlaybackControls';
import type { FigureInfo } from '../stage/PreviewStage';
import { ACTIONS, actionOf, type ActionId } from './actions';
import { SkillList } from './SkillList';
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

function Bar({ m, gap }: { m: Monarch; gap: FigureInfo['gap'] | undefined }) {
  return (
    <div className="wd-bar">
      <select className="wd-select" aria-label="Reittier" value={m.mount} onChange={(e) => m.setMount(e.target.value)}>
        {MOUNT_KEYS.map((k) => <option key={k} value={k}>{k}</option>)}
      </select>
      <select className="wd-select" aria-label="Aktion" value={m.actionId} onChange={(e) => m.setActionId(e.target.value as ActionId | '')}>
        <option value="">Keine (Bewegung)</option>
        {ACTIONS.map((a) => <option key={a.id} value={a.id}>{a.label}</option>)}
      </select>
      <select className="wd-select" aria-label="Bewegung" disabled={m.actionId !== ''} value={m.move} onChange={(e) => m.setMove(e.target.value as MoveId)}>
        {MOVES.map((o) => <option key={o.id} value={o.id}>{o.label}</option>)}
      </select>
      <select className="wd-select" aria-label="Animation" disabled={m.actionId !== ''} value={m.anim} onChange={(e) => m.setAnim(e.target.value as Anim)}>
        {MONARCH_ANIMS.map((a) => <option key={a} value={a}>{a}</option>)}
      </select>
      {m.mountOptions.length > 0 && (
        <select className="wd-select" aria-label="Grafik Reittier" value={m.mountChoice?.index ?? 0} onChange={(e) => m.pickMount(Number(e.target.value))}>
          {m.mountOptions.map((o) => <option key={o.index} value={o.index}>{`Präferenz ${o.index + 1}: ${o.sheet}`}</option>)}
        </select>
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
            <select className="wd-select" aria-label={`Grafik Monarch ${p.index + 1}`} value={m.riders[p.index]?.index ?? 0} onChange={(e) => m.pickRider(p.index, Number(e.target.value))}>
              {m.riderOptions[p.index]!.map((o) => <option key={o.index} value={o.index}>{`Präferenz ${o.index + 1}: ${o.sheet}`}</option>)}
            </select>
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
  return (
    <div className="wd-page wd-monarch">
      <Bar m={m} gap={infos[0]?.gap} />
      <PlaybackControls control={m.control} onChange={m.setControl} frames={infos[0]?.frames ?? 1} />
      {missing.length > 0 && <Text as="p" size="2" color="red" role="status">Grafiken nicht geladen: {missing.join(', ')}</Text>}
      <FigureStage items={m.items} control={m.control} onInfo={setInfos} onLoadError={setMissing} />
      <Colors m={m} />
      <SkillList />
    </div>
  );
}
