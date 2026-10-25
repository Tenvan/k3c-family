/**
 * Szene „Kampf-Anzeigen“ (B-602): Mock-Zustände als reine Daten. Die Darstellung liefern die Funktionen des Spiels
 * (`kampfAnzeige.ts`, `kampfView.ts`); hier entsteht nur der `KampfWorld`-Zustand, den der Server sonst sendet.
 */
import bossesJson from '@data/bosses.json';
import { bossBar, eventBanner, warnCircles, type BossBarView, type KampfWorld, type WarnCircleView } from '@game/scenes/kampfAnzeige';
import { TOP_STRIP } from '@game/scenes/hud/radar';
import { computeLayout, viewZoom } from '@game/scenes/layout';
import { FONTS } from '@game/scenes/render/fontRules';
import type { Enemy } from '@game/model/types';

export { TOP_STRIP };
export const EVENTS = ['fullMoon', 'bloodMoon', 'merchantRaid'] as const;
export type KampfEvent = (typeof EVENTS)[number];
/** Mondanzeige: die Events, die der Mond auslöst (Rest ist der Händler-Überfall) */
export const MOON_EVENTS: readonly KampfEvent[] = ['fullMoon', 'bloodMoon'];

export interface BossInfo {
  id: string;
  kind: 'mini' | 'end';
  /** Standardgegner der Stufe: sein Sprite zeigt die Szene „Bosse“ */
  base: string;
  /** `belowHpPercent` je Phase ab der zweiten; leer = ohne Phasen */
  phases: { belowHpPercent?: number; ability?: string; tempo?: number }[];
  ability?: string;
}

/** Alle Bosse aus `data/bosses.json`: Minibosse zuerst, dann die Endbosse. */
export const BOSSES: readonly BossInfo[] = [
  ...bossesJson.bosses.map((b): BossInfo => ({ id: b.id, kind: 'mini', base: b.base, phases: [], ability: b.ability?.type })),
  ...bossesJson.endBosses.map((b): BossInfo => ({
    id: b.id,
    kind: 'end',
    base: b.base,
    phases: b.phases.map((p) => ({ ...('belowHpPercent' in p && { belowHpPercent: p.belowHpPercent }), ...('ability' in p && { ability: p.ability?.type }), ...('tempo' in p && { tempo: p.tempo }) })),
  })),
];

export interface KampfMock {
  boss: string;
  /** Lebensanteil in Prozent, 0..100 */
  hpPercent: number;
  /** Warnkreis: Radius in Units und Restzeit bis zum Schlag; `null` = kein Schlag */
  warn: { radius: number; seconds: number } | null;
  events: readonly KampfEvent[];
  /** Restzeit der Nacht in Sekunden (Events mit Zeit) */
  secondsLeft: number;
  /** 1 oder 2: bei 2 teilt sich der Bildschirm, die Leiste bleibt einmal */
  spieler: 1 | 2;
}

export const MOCK_START: KampfMock = { boss: 'crystalHeartWarden', hpPercent: 40, warn: { radius: 5, seconds: 1.5 }, events: ['fullMoon'], secondsLeft: 95, spieler: 1 };

/** Phase (ab 1) des Endbosses bei `hpPercent`: jede weitere gilt ab `belowHpPercent`; Miniboss ohne Phasen → `undefined`. */
export function phaseAt(boss: BossInfo, hpPercent: number): number | undefined {
  if (boss.phases.length === 0) return undefined;
  return boss.phases.reduce((n, p, i) => (i > 0 && p.belowHpPercent !== undefined && hpPercent < p.belowHpPercent ? i + 1 : n), 1);
}

/** Zustand wie ihn der Server sendet (Protokoll › Kampf); unbekannter Boss → `null`. */
export function mockWorld(m: KampfMock): KampfWorld | null {
  const boss = BOSSES.find((b) => b.id === m.boss);
  if (!boss) return null;
  const enemy = { id: 1, kind: boss.id, x: BOSS_X, hp: m.hpPercent, maxHp: 100, boss: true, phase: phaseAt(boss, m.hpPercent), ...(m.warn && { warn: { x: BOSS_X, r: m.warn.radius, in: m.warn.seconds } }) } as unknown as Enemy;
  return {
    enemies: [enemy],
    cycleSpeed: 1,
    nightEvents: m.events.filter((e) => e !== 'merchantRaid').map((id) => ({ id, secondsLeft: m.secondsLeft })),
    ...(m.events.includes('merchantRaid') && { merchant: { raid: true } as KampfWorld['merchant'] }),
  };
}

export interface KampfBild {
  boss: BossBarView | null;
  circles: WarnCircleView[];
  banner: string | null;
}

/** Was die Spiel-Funktionen aus dem Mock-Zustand ableiten: Leiste mit Phase, Warnkreise, Banner. */
export function kampfBild(m: KampfMock): KampfBild {
  const w = mockWorld(m);
  return w ? { boss: bossBar(w), circles: warnCircles(w), banner: eventBanner(w) } : { boss: null, circles: [], banner: null };
}

export interface Ablauf {
  mock: KampfMock;
  /** Deckkraft der Leiste (Auftritt) und des Banners (erscheint, verschwindet), 0..1 */
  leiste: number;
  banner: number;
}

/** Dauer des abspielbaren Ablaufs in Sekunden */
export const ABLAUF_SEKUNDEN = 14;
const SCHLAG_TAKT = 4;

/**
 * Ablauf zum Abspielen: Auftritt der Leiste (0–1 s), Leben fällt von 100 auf 10 % (Phasenwechsel an den Schwellen des Bosses),
 * der Warnkreis läuft alle 4 s von 4 auf 0 ab, das Event-Banner steht von 3 bis 11 s (Ein- und Ausblenden je 0,5 s).
 */
export function ablauf(base: KampfMock, t: number): Ablauf {
  const k = Math.min(ABLAUF_SEKUNDEN, Math.max(0, t));
  const fade = (from: number, to: number) => Math.min(1, Math.max(0, Math.min(k - from, to - k) / 0.5));
  return {
    mock: { ...base, hpPercent: Math.round(100 - 90 * (k / ABLAUF_SEKUNDEN)), warn: base.warn && { radius: base.warn.radius, seconds: SCHLAG_TAKT - (k % SCHLAG_TAKT) }, secondsLeft: Math.max(0, base.secondsLeft - k) },
    leiste: Math.min(1, k / 1),
    banner: fade(3, 11),
  };
}

/** Ort des Bosses (und seines Warnkreises) in Units; um ihn zoomt die Welt bei der Zellengröße */
export const BOSS_X = 20;

/** Zellengröße (B-602): Vollbild (1 Spieler), mit Mitspieler eines anderen Geräts (2/3 der Höhe), Streifen bei 2 Spielern (1/2). */
export const ZELLEN = ['voll', 'partner', 'streifen'] as const;
export type Zelle = (typeof ZELLEN)[number];
export const ZELLE_LABEL: Record<Zelle, string> = { voll: 'Zelle: Vollbild', partner: 'Zelle: mit Mitspieler (2/3)', streifen: 'Zelle: Streifen (1/2)' };

/** Zoom der Welt in der Zelle wie im Spiel: Höhe der eigenen Zelle aus `computeLayout`, Zoom aus `viewZoom`; das HUD zoomt nicht. */
export function zellenZoom(z: Zelle): number {
  const cells = z === 'streifen' ? computeLayout(2, false) : computeLayout(1, z === 'partner');
  return viewZoom(cells.find((c) => c.kind === 'player')!.h);
}

/** Kleinste Schrift der Szene (px) aus den Schriftregeln des Spiels; Mindest-Schrift bei 1 Spieler laut `bedienung.md` § 2 */
export const MIN_SCHRIFT_PX = 28;
export const schriftPx = (name: 'bossBar' | 'banner'): number => FONTS[name].px;
