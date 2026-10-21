import type { Site } from '../../model/types';

/**
 * Schadens-Darstellung gebauter Gebäude (WZ14.6, B-468, Beschluss WZ14.1): Stufe nach HP-Anteil, bei Ruinen zusätzlich
 * Trümmer. Reine Regeln ohne Phaser; gezeichnet wird in `damageView.ts`.
 */

/** 0 = unbeschädigt, 1 = leicht (≥ 2/3), 2 = mittel (≥ 1/3), 3 = schwer (< 1/3 der HP). */
export type DamageStage = 0 | 1 | 2 | 3;

/** Schadensstufe eines Platzes nach HP-Anteil; nicht gebaut oder ohne MaxHP → 0. */
export function damageStage(s: Pick<Site, 'state' | 'hp' | 'maxHp'>): DamageStage {
  if (s.state !== 'built' || s.maxHp <= 0 || s.hp >= s.maxHp) return 0;
  const share = s.hp / s.maxHp;
  if (share >= 2 / 3) return 1;
  return share >= 1 / 3 ? 2 : 3;
}

/** Trümmerhaufen am Fuß: nur an einer gebauten Ruine unter der Hälfte ihrer HP. */
export const showRubble = (s: Pick<Site, 'state' | 'hp' | 'maxHp' | 'ruin'>): boolean =>
  s.ruin === true && s.state === 'built' && s.maxHp > 0 && s.hp / s.maxHp < 0.5;
