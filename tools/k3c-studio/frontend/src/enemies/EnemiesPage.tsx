import { SelectField, Text, plainOptions, type SelectOption } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { useEffect, useMemo, useState } from 'react';
import { backend, type AssetSlot } from '../api';
import { DEFAULT_CONTROL, resolveAnim, type Anim, type Control } from '../playback/figure';
import { FigureStage } from '../playback/FigureStage';
import { GapBadge, PlaybackControls } from '../playback/PlaybackControls';
import { slotOptions } from '../monarch/monarchLogic';
import type { FigureInfo } from '../stage/PreviewStage';
import { useBossDisplay } from './BossDisplay';
import { bossCards } from './bossLogic';
import { ENEMY_ACTIONS, ENEMY_ANIMS, cardOf, enemyItems, groups, type EnemyActionId, type EnemyCard, type Group } from './enemyLogic';

/** Gruppe der Bosse: je Tiefe einer (5 Minibosse und der Endboss), Daten aus `data/bosses.json` */
const BOSS_GROUP_ID = 'bosse';

const ACTION_OPTIONS: SelectOption<EnemyActionId | ''>[] = [{ value: '', label: 'Keine' }, ...ENEMY_ACTIONS.map((a) => ({ value: a.id, label: a.label }))];

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

function CardRow({ card, slots, pick, onPick, gap }: { card: EnemyCard; slots: AssetSlot[]; pick: number; onPick: (n: number) => void; gap: FigureInfo['gap'] | undefined }) {
  const options = slotOptions(slots, card.boss ? 'boss' : card.id);
  const d = card.def;
  return (
    <li className="wd-card" data-enemy={card.id}>
      <Text size="2" weight="bold">{d?.name ?? card.id}</Text>
      <Text size="1" color="gray">{d ? (card.boss ? `${card.id} · Tiefe ${d.depth} · Tempo ${d.speed} · Reichweite ${d.range}` : `${card.id} · ${d.tier} · Tiefe ${d.depth} · ${d.hp} HP · ${d.damage} Schaden · Tempo ${d.speed} · Reichweite ${d.range}`) : card.id}</Text>
      <span className="wd-traits">
        {card.traits.map((t) => <span key={t.trait} className="wd-trait" data-trait={t.trait}>{t.trait}{t.gap && <GapBadge gap={t.gap} />}</span>)}
      </span>
      {card.extras.map((x) => <span key={x.field} className="wd-trait" data-extra={x.field}>{x.field} {x.value}{x.gap && <GapBadge gap={x.gap} />}</span>)}
      {card.boss && (
        <span className="wd-boss">
          <Text size="1">{card.boss.kind === 'end' ? 'Endboss' : 'Miniboss'}{card.boss.ability ? ` · ${card.boss.ability}` : ''}</Text>
          {card.boss.phases.map((p) => <Text key={p} size="1" color="gray">{p}</Text>)}
          {card.boss.displays.map((g) => <GapBadge key={g.reason} gap={g} />)}
        </span>
      )}
      {card.gaps.map((g) => <GapBadge key={g.reason} gap={g} />)}
      <GapBadge gap={gap} />
      {options.length > 0 && (
        <SelectField aria-label={`Grafik ${card.id}`} value={String(pick)} onChange={(v) => onPick(Number(v))}
          options={options.map((o) => ({ value: String(o.index), label: `Präferenz ${o.index + 1}: ${o.sheet}` }))} />
      )}
    </li>
  );
}

/** Gegner-Seite des Asset-Studios (B-464): alle Gegnertypen nach Biom und Tiefe (Portal vor Nacht, dazu „ohne Biom“) mit
 *  den Animationen des Spiels. Karten mit Merkmalen und Lücken, Getroffen und Sterben mit den Effekten des Spiels,
 *  Monarch-Vergleich auf derselben Bodenlinie, Grafik-Auswahl je Gegner-ID als Vorschau (das Register bleibt unverändert). */
export function EnemiesPage() {
  const slots = useSlots();
  const all = useMemo<Group[]>(() => [...groups(), { id: BOSS_GROUP_ID, title: 'Bosse (je Tiefe)', depth: null, ids: [] }], []);
  const [groupId, setGroupId] = useState(all[0]!.id);
  const [anim, setAnim] = useState<Anim>('idle');
  const [actionId, setActionId] = useState<EnemyActionId | ''>('');
  const [withMonarch, setWithMonarch] = useState(false);
  const [control, setControl] = useState<Control>(DEFAULT_CONTROL);
  const [infos, setInfos] = useState<FigureInfo[]>([]);
  const [picks, setPicks] = useState<Record<string, number>>({});
  const group = all.find((g) => g.id === groupId) ?? all[0]!;
  const cards = useMemo(() => (group.id === BOSS_GROUP_ID ? bossCards() : group.ids.map(cardOf)), [group]);
  const sheets: Record<string, string | undefined> = {};
  for (const c of cards) {
    const options = slotOptions(slots, c.boss ? 'boss' : c.id);
    const chosen = options.find((o) => o.index === (picks[c.id] ?? 0)) ?? options[0];
    sheets[c.id] = chosen && !chosen.gap ? chosen.sheet : undefined;
  }
  const action = ENEMY_ACTIONS.find((a) => a.id === actionId);
  const key = JSON.stringify([group.id, anim, actionId, withMonarch, sheets]);
  const items = useMemo(() => enemyItems(cards, { anim, ...(action ? { action } : {}), withMonarch, sheets }), [key]); // eslint-disable-line react-hooks/exhaustive-deps
  const offset = withMonarch ? 1 : 0; // die Antworten der Bühne zählen den Monarch-Vergleich mit
  // Boss-Anzeigen des Spiels (B-469) nur in der Gruppe der Bosse; die Mitte des Warnkreises ist die Lage des Boss-Bildes
  const boss = useBossDisplay(group.id === BOSS_GROUP_ID, (id) => items.find((i) => i.id === `enemy-${id}`)?.x ?? 0);

  return (
    <div className="wd-page wd-gegner">
      <div className="wd-bar">
        <SelectField aria-label="Gruppe" value={group.id} options={all.map((g) => ({ value: g.id, label: g.title }))} onChange={setGroupId} />
        <SelectField aria-label="Animation" value={anim} options={plainOptions(ENEMY_ANIMS)} onChange={setAnim} />
        <SelectField aria-label="Aktion" value={actionId} options={ACTION_OPTIONS} onChange={setActionId} />
        <CheckField label="Monarch-Vergleich" checked={withMonarch} onCheckedChange={(c) => setWithMonarch(c === true)} aria-label="Monarch-Vergleich" />
        {action?.gaps.map((g) => <GapBadge key={g.reason} gap={g} />)}
      </div>
      <PlaybackControls control={control} onChange={setControl} frames={infos[offset]?.frames ?? 1} />
      {boss.bar}
      <FigureStage items={items} control={control} kampf={boss.world} onInfo={setInfos} />
      <ul className="wd-cards" aria-label="Gegner">
        {cards.map((c, i) => (
          <CardRow key={c.id} card={c} slots={slots} pick={picks[c.id] ?? 0} onPick={(n) => setPicks({ ...picks, [c.id]: n })}
            gap={infos[i + offset]?.gap ?? (infos[i + offset] ? undefined : resolveAnim(c.spec.sheet, anim).gap)} />
        ))}
      </ul>
    </div>
  );
}
