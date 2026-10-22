import bossesJson from '../../data/bosses.json';
import { UNIT_PX } from '../core/constants';
import { t, type TextKey } from '../core/texts';
import { de } from '../core/texts.de';
import type { Enemy, Merchant, World } from '../model/types';

/**
 * Anzeige von Kampf, Bossen und Events (K5.1, B-132): reine Funktionen ohne Phaser, die aus dem Server-Zustand nur Darstellung
 * ableiten (Anteil, Text, Pixel). Fehlen die Felder (älterer Server), liefern alle `null` bzw. eine leere Liste.
 */

/** Zustand, den die Anzeige liest; alle Felder dürfen fehlen. `merchant.raid` sendet der Server (K4), `types.ts` kennt es noch nicht. */
export type KampfWorld = Partial<Pick<World, 'enemies' | 'cycleSpeed' | 'nightEvents'>> & { merchant?: Merchant & { raid?: boolean } };

export interface BossBarView {
  id: number;
  /** HP / Maximum, 0..1 */
  share: number;
  name: string;
  /** „Phase 2 von 3“, beim Miniboss leer */
  phase: string;
}

export interface WarnCircleView {
  /** Mitte und Radius in Pixeln (Szenenraum) */
  x: number;
  r: number;
  /** 0 = Schlag noch weit, 1 = Schlag jetzt; steigt in den letzten `WARN_WINDOW` Sekunden */
  urgency: number;
}

/** Sekunden vor dem Flächenschlag, in denen der Warnkreis kräftiger wird (reine Darstellung, Abnahme K5.4) */
export const WARN_WINDOW = 3;

/** Phasenzahl je Endboss aus den Daten */
const PHASES = new Map(bossesJson.endBosses.map((b) => [b.id, b.phases.length]));

/** Name aus den Textdateien (`boss.<id>`), sonst die ID */
function bossName(id: string): string {
  const key = `boss.${id}`;
  return key in de ? t(key as TextKey) : id;
}

export function phaseText(e: Pick<Enemy, 'kind' | 'phase'>): string {
  const of = PHASES.get(e.kind);
  return e.phase === undefined || of === undefined ? '' : t('boss.phase', { n: e.phase, of });
}

/** Eine Leiste für alle Spieler: der Endboss, sonst der erste Boss; kein Boss → `null`. */
export function bossBar(w: KampfWorld): BossBarView | null {
  const bosses = (w.enemies ?? []).filter((e) => e.boss);
  const boss = bosses.find((e) => PHASES.has(e.kind)) ?? bosses[0];
  if (!boss) return null;
  const share = boss.maxHp > 0 ? Math.min(1, Math.max(0, boss.hp / boss.maxHp)) : 0;
  return { id: boss.id, share, name: bossName(boss.kind), phase: phaseText(boss) };
}

export function warnCircles(w: KampfWorld): WarnCircleView[] {
  return (w.enemies ?? []).flatMap((e) =>
    e.warn ? [{ x: e.warn.x * UNIT_PX, r: e.warn.r * UNIT_PX, urgency: 1 - Math.min(WARN_WINDOW, Math.max(0, e.warn.in)) / WARN_WINDOW }] : [],
  );
}

/** Restzeit als m:ss */
export function formatTime(seconds: number): string {
  const s = Math.max(0, Math.ceil(seconds));
  return `${Math.floor(s / 60)}:${String(s % 60).padStart(2, '0')}`;
}

function eventName(id: string): string {
  const key = `event.${id}`;
  return key in de ? t(key as TextKey) : id;
}

/** Laufende Events, eine Zeile je Event mit Restzeit der Nacht (Zeitraffer wie die Uhr); Händler-Überfall ohne Restzeit. */
export function eventBanner(w: KampfWorld): string | null {
  const speed = w.cycleSpeed ?? 1;
  const lines = (w.nightEvents ?? []).map((e) => t('event.timed', { name: eventName(e.id), left: formatTime(e.secondsLeft / speed) }));
  if (w.merchant?.raid) lines.push(eventName('merchantRaid'));
  return lines.length > 0 ? lines.join('\n') : null;
}
