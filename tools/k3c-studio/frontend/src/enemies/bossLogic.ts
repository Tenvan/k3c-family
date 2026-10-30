import bossesJson from '@data/bosses.json';
import type { Enemy } from '@game/model/types';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import { gap, type Gap } from '../playback/figure';
import { cardOf, FALLBACK_ID, type BossInfo, type EnemyCard } from './enemyLogic';

// Bosse der Gegner-Seite (B-464, WZ11.2): 5 Minibosse (je Tiefe 0–4) und der Endboss `crystalHeartWarden` aus
// `data/bosses.json`. Boss-IDs haben keinen Eintrag in `data/sprites.json`; das Spiel zeichnet sie mit dem Rückfall
// `goblin`. Warnkreis, Boss-Leiste und Phasen-Text zeichnet das Spiel seit K5.1 (B-132) mit `kampfAnzeige.ts` und
// `kampfView.ts`; das Studio füttert sie mit Beispielzuständen (`bossWorld`, B-469) und zeichnet nichts nach. Das
// Event-Banner des Spiels kennt keine Boss-Ereignisse (`bossSpawned` …): dort bleibt die Lücke.

export interface Ability {
  type: string;
  enemy?: string;
  count?: number;
  radius?: number;
  intervalSeconds?: number;
  damagePerSecond?: number;
  durationSeconds?: number;
}
export interface Phase { belowHpPercent?: number; ability?: Ability; tempo?: number }
export interface BossData {
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

/** Anzeigen, die das Spiel (K5.1) für Bosse noch nicht zeichnet: das Event-Banner kennt `bossSpawned`, `bossPhase` und
 *  `bossDefeated` nicht, es zeigt nur Nacht-Events und den Händler-Überfall. */
export const MISSING_DISPLAYS: readonly DisplayKind[] = ['banner'];

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

/** Anzeigen, die zu diesem Boss gehören: Boss-Leiste und Event-Banner bei jedem Boss, Warnkreis nur bei Flächenschlag,
 *  Phasen-Text nur beim Endboss mit Phasen. */
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
    kind: b.kind, ability: b.ability ? abilityText(b.ability) : '', phases: phaseTexts(b.phases),
    displays: displaysOf(b).filter((k) => MISSING_DISPLAYS.includes(k)).map(displayGap),
  };
  return {
    id: b.id, def: { name: b.name, depth: b.depth, tier: b.kind === 'end' ? 'Endboss' : 'Miniboss', hp: 0, damage: 0, speed: b.speed, range: b.range, attacksPerSecond: 0, traits: [] },
    spec: base.spec, traits: [], extras: [], gaps: [gap('sheet', 'Lücke: keine eigene Boss-Grafik')], boss: info,
  };
}

/** Alle Bosse als Karten, je Tiefe geordnet (heute 6) */
export const bossCards = (): EnemyCard[] => BOSS_DATA.map(bossCard);

// ---- Beispielzustände für die Anzeigen des Spiels (B-469) ----------------------------------------------------------

/** Lebenspunkte des Beispiel-Bosses; nur das Verhältnis zählt für die Leiste */
const EXAMPLE_MAX_HP = 1000;
/** Sekunden bis zum Flächenschlag im Beispiel: halbe Dringlichkeit des Warnkreises (Fenster `WARN_WINDOW` = 3 s) */
const EXAMPLE_WARN_IN = 1.5;

/** Anzahl der Phasen (nur der Endboss hat welche) */
export const phaseCount = (b: BossData): number => (b.kind === 'end' ? (b.phases ?? []).length : 0);

/** Fähigkeit, die in der Phase gilt (1-basiert): eine Phase ohne eigene Fähigkeit erbt die der vorigen. Ohne Phase die des Bosses. */
export function abilityIn(b: BossData, phase: number | undefined): Ability | undefined {
  if (phase === undefined) return b.ability;
  let last: Ability | undefined;
  (b.phases ?? []).slice(0, phase).forEach((p) => { last = p.ability ?? last; });
  return last;
}

/** Schwelle der Phase in Prozent HP („unter X %“); die erste Phase hat keine. */
export const phaseThreshold = (b: BossData, phase: number): number | undefined => (b.phases ?? [])[phase - 1]?.belowHpPercent;

/** Beispiel-HP-Anteil einer Phase: knapp unter ihrer Schwelle, die erste Phase mit voller Leiste. */
export function phaseShare(b: BossData, phase: number): number {
  const below = phaseThreshold(b, phase);
  return below === undefined ? 1 : Math.max(0.05, (below - 5) / 100);
}

/** Zustand für `bossBar`, `warnCircles` und `eventBanner` des Spiels: ein Boss mit `share` HP-Anteil, beim Endboss in `phase`
 *  (1-basiert), mit Warnkreis (Mitte `x` in Units) nur, wenn die geltende Fähigkeit ein Flächenschlag ist. */
export function bossWorld(b: BossData, opts: { phase?: number; share: number; x: number }): KampfWorld {
  const phase = b.kind === 'end' ? opts.phase : undefined;
  const ability = abilityIn(b, phase);
  const enemy = {
    id: 1, kind: b.id, x: opts.x, boss: true, hp: Math.round(EXAMPLE_MAX_HP * opts.share), maxHp: EXAMPLE_MAX_HP,
    ...(phase !== undefined ? { phase } : {}),
    ...(ability?.type === 'aoe' && ability.radius ? { warn: { x: opts.x, r: ability.radius, in: EXAMPLE_WARN_IN } } : {}),
  } as unknown as Enemy;
  return { enemies: [enemy] };
}
