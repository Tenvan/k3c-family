import { Button, SelectField, Text } from '@webkit/atoms';
import { slotOptions } from '../monarch/monarchLogic';
import type { Gap } from '../playback/figure';
import type { AbilityEntry } from './abilityLogic';
import type { EnemyCard } from './enemyLogic';
import { slotIdOf, type Enemies } from './useEnemies';

const short = (g: Gap): string => g.reason.replace(/^Lücke:\s*/, '');

/** Eine Lücke als kurze Zeile (B-672/AC-02), der Grund steht ganz im Tooltip */
function GapLine({ gap }: { gap: Gap | null | undefined }) {
  if (!gap) return null;
  return <span className="wd-gap-line" data-kind={gap.kind} title={gap.reason}>Lücke: {short(gap)}</span>;
}

/** Phasen bzw. Fähigkeit eines Bosses, je mit ▶ (Modus „Fähigkeit“); eine Phase ohne geltende Fähigkeit hat keinen ▶. */
function BossPhases({ card, entries, onPlay }: { card: EnemyCard; entries: readonly AbilityEntry[]; onPlay: (e: AbilityEntry) => void }) {
  const boss = card.boss!;
  const own = entries.filter((x) => x.bossId === card.id);
  const rows = boss.phases.length > 0 ? boss.phases.map((text, i) => ({ text, entry: own.find((x) => x.phase === i + 1) })) : [{ text: boss.ability || 'ohne Fähigkeit', entry: own[0] }];
  return (
    <>
      <Text size="1" weight="bold">{boss.kind === 'end' ? 'Endboss' : 'Miniboss'} · {boss.phases.length > 0 ? 'Phasen' : 'Fähigkeit'}</Text>
      <ul aria-label="Phasen">
        {rows.map((r) => (
          <li key={r.text} data-phase={r.entry?.phase ?? ''}>
            {r.entry && <Button size="1" variant="ghost" aria-label={`${r.entry.label} abspielen`} onClick={() => onPlay(r.entry!)}>▶</Button>}
            <span>{r.text}</span>
          </li>
        ))}
      </ul>
    </>
  );
}

/** Gewählter Gegner (B-672): Merkmale, Zusatzfelder, Lücken je eine Zeile, Boss-Phasen mit ▶ und die Grafik-Auswahl. */
export function EnemyInspector({ e }: { e: Enemies }) {
  const card = e.selected;
  if (!card) return null;
  const options = slotOptions(e.slots, slotIdOf(card));
  const figureGap = e.infoOf(card.id)?.gap;
  const gaps = [...card.gaps, ...(card.boss?.displays ?? []), ...(figureGap ? [figureGap] : [])];
  return (
    <section className="wd-inspector" aria-label="Inspektor" data-enemy={card.id}>
      <Text size="2" weight="bold">{card.def?.name ?? card.id}</Text>
      <Text size="1" color="gray">{card.id}</Text>
      <ul aria-label="Merkmale">
        {card.traits.map((t) => <li key={t.trait} data-trait={t.trait}><span>{t.trait}</span><GapLine gap={t.gap} /></li>)}
        {card.extras.map((x) => <li key={x.field} data-extra={x.field}><span>{x.field} {x.value}</span><GapLine gap={x.gap} /></li>)}
      </ul>
      {gaps.length > 0 && <ul aria-label="Lücken">{gaps.map((g) => <li key={g.reason}><GapLine gap={g} /></li>)}</ul>}
      {card.boss && <BossPhases card={card} entries={e.entries} onPlay={e.playAbility} />}
      {options.length > 0 && (
        <SelectField size="1" aria-label={`Grafik ${card.id}`} value={String(e.picks[card.id] ?? 0)} onChange={(v) => e.pick(card.id, Number(v))}
          options={options.map((o) => ({ value: String(o.index), label: `Präferenz ${o.index + 1}: ${o.sheet}` }))} />
      )}
    </section>
  );
}
