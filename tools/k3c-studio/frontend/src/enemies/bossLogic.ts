import bossesJson from '@data/bosses.json';
import { gap, type Gap } from '../playback/figure';
import { cardOf, FALLBACK_ID, type BossInfo, type EnemyCard } from './enemyLogic';

// Bosse der Gegner-Seite (B-464, WZ11.2): 5 Minibosse (je Tiefe 0–4) und der Endboss `crystalHeartWarden` aus
// `data/bosses.json`. Boss-IDs haben keinen Eintrag in `data/sprites.json`; das Spiel zeichnet sie mit dem Rückfall
// `goblin`. Warnkreis, Boss-Leiste, Phasen-Text und Event-Banner zeichnet das Spiel erst nach K5.1 (B-132): Das Studio
// zeigt sie als Lücke, die Anbindung ist B-469. Es zeichnet nichts nach.

interface Ability {
  type: string;
  enemy?: string;
  count?: number;
  radius?: number;
  intervalSeconds?: number;
  damagePerSecond?: number;
  durationSeconds?: number;
}
interface Phase { belowHpPercent?: number; ability?: Ability; tempo?: number }
interface BossData {
  id: string;
  name: string;
  kind: 'mini' | 'end';
  depth: number;
  base: string;
  hpFactor: number;
  damageFactor: number;
  speed: number;
  range: number;
  ability?: Ability;
  phases?: Phase[];
}

const DATA = bossesJson as unknown as { bosses: BossData[]; endBosses: BossData[] };

/** Alle Bosse nach Tiefe (bei gleicher Tiefe Minibosse vor dem Endboss) */
export const BOSS_DATA: readonly BossData[] = [...DATA.bosses, ...DATA.endBosses].sort((a, b) => a.depth - b.depth || (a.kind === 'mini' ? -1 : 1));

/** Text der Lücke für eine Anzeige, die das Spiel noch nicht zeichnet */
export const DISPLAY_GAP = 'Anzeige im Spiel fehlt (B-132, Anbindung B-469)';

export type DisplayKind = 'warnkreis' | 'leiste' | 'phasen' | 'banner';
const LABELS: Record<DisplayKind, string> = { warnkreis: 'Warnkreis', leiste: 'Boss-Leiste', phasen: 'Phasen-Text', banner: 'Event-Banner' };

const abilityText = (a: Ability): string => {
  if (a.type === 'summon') return `ruft ${a.count}× ${a.enemy} alle ${a.intervalSeconds} s`;
  if (a.type === 'aoe') return `Flächenschlag, Radius ${a.radius}, alle ${a.intervalSeconds} s`;
  return `${a.type}${a.radius ? `, Radius ${a.radius}` : ''}${a.intervalSeconds ? `, alle ${a.intervalSeconds} s` : ''}`;
};

/** Texte der Phasen des Endbosses; eine Phase ohne eigene Fähigkeit nennt die der vorigen Phase. */
export function phaseTexts(phases: readonly Phase[] = []): string[] {
  let last: Ability | undefined;
  return phases.map((p, i) => {
    if (p.ability) last = p.ability;
    const from = p.belowHpPercent === undefined ? 'ab Beginn' : `unter ${p.belowHpPercent} % HP`;
    const own = p.ability ? abilityText(p.ability) : last ? `${abilityText(last)} (Fähigkeit der vorigen Phase)` : 'ohne Fähigkeit';
    const tempo = p.tempo ? `, Tempo ×${p.tempo}` : '';
    return `Phase ${i + 1} (${from}): ${own}${tempo}`;
  });
}

/** Hat der Boss einen Flächenschlag (`aoe`) in der Fähigkeit oder einer Phase? Nur dann gehört ein Warnkreis dazu. */
const hasAoe = (b: BossData): boolean => b.ability?.type === 'aoe' || (b.phases ?? []).some((p) => p.ability?.type === 'aoe');

/** Anzeigen, die zu diesem Boss gehören und dem Spiel noch fehlen: Boss-Leiste und Event-Banner bei jedem Boss, Warnkreis
 *  nur bei Flächenschlag, Phasen-Text nur beim Endboss mit Phasen. */
export function displaysOf(b: BossData): DisplayKind[] {
  const out: DisplayKind[] = [];
  if (hasAoe(b)) out.push('warnkreis');
  out.push('leiste');
  if (b.kind === 'end' && (b.phases?.length ?? 0) > 0) out.push('phasen');
  out.push('banner');
  return out;
}

export const displayGap = (kind: DisplayKind): Gap => gap('display', `Lücke: ${LABELS[kind]}: ${DISPLAY_GAP}`);

/** Karte eines Bosses: Grafik wie im Spiel (Rückfall `goblin`, Lücke „keine eigene Boss-Grafik“), Fähigkeit, Phasen und die
 *  Lücken der Anzeigen. */
export function bossCard(b: BossData): EnemyCard {
  const base = cardOf(FALLBACK_ID);
  const info: BossInfo = {
    kind: b.kind, ability: b.ability ? abilityText(b.ability) : '', phases: phaseTexts(b.phases), displays: displaysOf(b).map(displayGap),
  };
  return {
    id: b.id, def: { name: b.name, depth: b.depth, tier: b.kind === 'end' ? 'Endboss' : 'Miniboss', hp: 0, damage: 0, speed: b.speed, range: b.range, attacksPerSecond: 0, traits: [] },
    spec: base.spec, traits: [], extras: [], gaps: [gap('sheet', 'Lücke: keine eigene Boss-Grafik')], boss: info,
  };
}

/** Alle Bosse als Karten, je Tiefe geordnet (heute 6) */
export const bossCards = (): EnemyCard[] => BOSS_DATA.map(bossCard);
