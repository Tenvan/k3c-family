import { useEffect, useMemo, useState } from 'react';
import { backend, type AssetSlot } from '../api';
import { loadPref, savePref } from '../lib/prefs';
import { DEFAULT_CONTROL, type Anim, type Control } from '../playback/figure';
import { actionOf, skillPlay, skillRows, type ActionId } from './actions';
import { DEFAULT_MOUNT, PLAYERS, monarchItems, monarchSlotId, moveOf, mountSlotId, slotOptions, type Choice, type MoveId, type SlotOption } from './monarchLogic';

/** Modus der Monarch-Seite (B-671): eine Bewegung, eine Aktion oder ein Skill */
export type Mode = MoveId | 'action' | 'skill';

/** Modus als Segment-Knöpfe; Aktion und Skill öffnen ihre Auswahl */
export const MODES: readonly { value: Mode; label: string }[] = [
  { value: 'stand', label: 'Stehen' }, { value: 'walk', label: 'Laufen' }, { value: 'sprint', label: 'Sprinten' },
  { value: 'action', label: 'Aktion …' }, { value: 'skill', label: 'Skill …' },
];

const isMove = (m: Mode): m is MoveId => m === 'stand' || m === 'walk' || m === 'sprint';

/** Gemerkter Zustand des Skills-Panels (offen/zu), Schlüssel `k3c-studio.monarch.skills` */
export const PANEL_PREF = 'monarch.skills';
const PANEL_STATES = ['open', 'closed'] as const;

/** Skill, den „Skill …“ ohne frühere Wahl zeigt */
const DEFAULT_SKILL = skillRows().find((r) => r.id === 'fireball')?.id ?? skillRows()[0]?.id ?? '';

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

/** Skills-Panel offen oder zu, gemerkt per localStorage (B-671); standardmäßig zu. */
export function useSkillPanel(): [boolean, (open: boolean) => void] {
  const [open, setOpen] = useState(() => loadPref(PANEL_PREF, PANEL_STATES, 'closed') === 'open');
  return [open, (o: boolean) => { setOpen(o); savePref(PANEL_PREF, o ? 'open' : 'closed'); }];
}

/** Grafik-Auswahl aus den Slots: je Spieler und für das Reittier eine Präferenz als Vorschau */
function useChoice(mount: string) {
  const slots = useSlots();
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
  return {
    choice, riderOptions, mountOptions, riders, mountChoice,
    pickRider: (index: number, option: number) => setRiderPick({ ...riderPick, [index]: option }),
    pickMount: (option: number) => setMountPick({ ...mountPick, [mount]: option }),
  };
}

/** Auswahl der Seite: Reittier, Modus (Bewegung, Aktion, Skill), Animation, Wiedergabe und die Grafik-Auswahl. Alles, was
 *  die Bühne zeigt, steht in `items`; sie ändern sich nur, wenn sich eine Eingabe ändert. */
export function useMonarch() {
  const [mount, setMount] = useState(DEFAULT_MOUNT);
  const [mode, setMode] = useState<Mode>('stand');
  const [anim, setAnim] = useState<Anim>('idle');
  const [actionId, setActionId] = useState<ActionId>('swing');
  const [skillId, setSkillId] = useState(DEFAULT_SKILL);
  const [control, setControl] = useState<Control>(DEFAULT_CONTROL);
  const c = useChoice(mount);
  const action = mode === 'action' ? actionOf(actionId) : undefined;
  const skill = useMemo(() => (mode === 'skill' ? skillPlay(skillId) : undefined), [mode, skillId]);
  const key = JSON.stringify([mount, mode, anim, c.choice, actionId, skillId]);
  const items = useMemo(() => {
    const figures = monarchItems(mount, moveOf(isMove(mode) ? mode : 'stand'), skill ? 'idle' : anim, c.choice, action);
    return skill ? figures.map((f) => ({ ...f, timed: skill.timed })) : figures;
  }, [key]); // eslint-disable-line react-hooks/exhaustive-deps
  return {
    ...c, mount, setMount, mode, setMode, anim, setAnim, actionId, setActionId, action, skillId, skill, control, setControl, items,
    /** Skill abspielen (▶ im Panel): Modus springt auf „Skill“ */
    playSkill: (id: string) => { setSkillId(id); setMode('skill'); },
    setSkillId,
  };
}

export type Monarch = ReturnType<typeof useMonarch>;
