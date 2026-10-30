import { useEffect, useMemo, useState } from 'react';
import { backend, type AssetSlot } from '../api';
import { slotOptions } from '../monarch/monarchLogic';
import { DEFAULT_CONTROL, type Anim, type Control, type Gap } from '../playback/figure';
import type { FigureInfo } from '../stage/PreviewStage';
import { abilityEntries, abilityPlay, type AbilityEntry } from './abilityLogic';
import { bossCards } from './bossLogic';
import { ENEMY_ACTIONS, SPACING, cardOf, enemyItems, groups, type EnemyCard, type EnemyMode, type Group } from './enemyLogic';

/** Gruppe der Bosse: je Tiefe einer (5 Minibosse und der Endboss), Daten aus `data/bosses.json` */
export const BOSS_GROUP_ID = 'bosse';

/** Slot-Register des Studios (nur lesen): Zuordnungen der Slots je Gegner-ID als Vorschau-Auswahl */
function useSlots(): AssetSlot[] {
  const [slots, setSlots] = useState<AssetSlot[]>([]);
  useEffect(() => {
    let current = true;
    void backend.assetsList().then((v) => { if (current && !v.error) setSlots(v.slots); });
    return () => { current = false; };
  }, []);
  return slots;
}

/** Slot einer Karte: Bosse teilen sich den Slot `boss` */
export const slotIdOf = (c: EnemyCard): string => (c.boss ? 'boss' : c.id);

/** Lücken einer Karte: Merkmale, Zusatzfelder, Daten und Grafik, Anzeigen der Bosse und die der gezeichneten Figur */
export function cardGaps(c: EnemyCard, figure: Gap | null | undefined): Gap[] {
  const all = [...c.traits.map((t) => t.gap), ...c.extras.map((x) => x.gap), ...c.gaps, ...(c.boss?.displays ?? []), figure];
  return all.filter((g): g is Gap => Boolean(g));
}

/** Gruppe, Modus, Auswahl und Wiedergabe der Gegner-Seite (B-464, B-672); `items` ändern sich nur bei geänderter Eingabe. */
export function useEnemies() {
  const slots = useSlots();
  const all = useMemo<Group[]>(() => [...groups(), { id: BOSS_GROUP_ID, title: 'Bosse (je Tiefe)', depth: null, ids: [] }], []);
  const [groupId, setGroupId] = useState(all[0]!.id);
  const [mode, setMode] = useState<EnemyMode>('idle');
  const [abilityKey, setAbilityKey] = useState('');
  const [withMonarch, setWithMonarch] = useState(false);
  const [control, setControl] = useState<Control>(DEFAULT_CONTROL);
  const [infos, setInfos] = useState<FigureInfo[]>([]);
  const [picks, setPicks] = useState<Record<string, number>>({});
  const [selectedId, setSelectedId] = useState<string | null>(null);
  const group = all.find((g) => g.id === groupId) ?? all[0]!;
  const isBoss = group.id === BOSS_GROUP_ID;
  const cards = useMemo(() => (isBoss ? bossCards() : group.ids.map(cardOf)), [group, isBoss]);
  const entries = useMemo(() => (isBoss ? abilityEntries() : []), [isBoss]);
  const entry: AbilityEntry | undefined = mode === 'ability' ? entries.find((e) => e.key === abilityKey) ?? entries[0] : undefined;
  const sheets: Record<string, string | undefined> = {};
  for (const c of cards) {
    const options = slotOptions(slots, slotIdOf(c));
    const chosen = options.find((o) => o.index === (picks[c.id] ?? 0)) ?? options[0];
    sheets[c.id] = chosen && !chosen.gap ? chosen.sheet : undefined;
  }
  const action = ENEMY_ACTIONS.find((a) => a.id === mode);
  const anim: Anim = mode === 'run' || mode === 'attack' ? mode : 'idle';
  const play = useMemo(() => {
    const index = entry ? cards.findIndex((c) => c.id === entry.bossId) : -1;
    return entry && index >= 0 ? abilityPlay(entry.ability, { x: index * SPACING, range: cards[index]!.def?.range ?? 0 }, withMonarch ? -1 : 1) : undefined;
  }, [entry, cards, withMonarch]);
  const key = JSON.stringify([group.id, mode, entry?.key, withMonarch, sheets]);
  const items = useMemo(() => enemyItems(cards, {
    anim, withMonarch, sheets, attack: mode === 'attack',
    ...(action ? { action } : {}), ...(entry && play ? { ability: { bossId: entry.bossId, play } } : {}),
  }), [key]); // eslint-disable-line react-hooks/exhaustive-deps
  const selected = cards.find((c) => c.id === selectedId) ?? cards[0];
  const infoOf = (id: string): FigureInfo | undefined => infos[items.findIndex((i) => i.id === `enemy-${id}`)];
  return {
    all, group, isBoss, mode,
    /** Gruppe wechseln; die Fähigkeit gibt es nur bei den Bossen */
    setGroupId: (id: string) => { setGroupId(id); if (mode === 'ability' && id !== BOSS_GROUP_ID) setMode('idle'); }, setMode, entries, entry, setAbilityKey, play, withMonarch, setWithMonarch, control, setControl, setInfos, infoOf,
    slots, picks, pick: (id: string, n: number) => setPicks({ ...picks, [id]: n }), cards, items, action, selected, setSelectedId,
    /** ▶ einer Phase: Modus „Fähigkeit“ mit diesem Eintrag, der Boss wird gewählt */
    playAbility: (e: AbilityEntry) => { setAbilityKey(e.key); setMode('ability'); setSelectedId(e.bossId); },
  };
}

export type Enemies = ReturnType<typeof useEnemies>;
