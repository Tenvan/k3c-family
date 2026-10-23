import { MONARCH } from '../../model/data';
import { EFFECT_CONFIG, LINE_COLORS, type SkillLook, type SkillMotion } from './effects';

/**
 * Eigener Effekt je Skill (B-578, Default B-561 Variante b): Farbe aus der Palette DB32, Größe, Dauer, Partikelzahl und
 * Bewegung. Ein Skill ohne Eintrag zeigt den Ring seiner Linie (`LINE_COLORS`, SK2.3). Nur Optik, keine Regel.
 */

type SkillInfo = { line: string; kind: string };
const SKILLS = MONARCH.skills as Record<string, SkillInfo>;

const look = (color: number, radius: number, durationMs: number, particles: number, motion: SkillMotion): SkillLook => ({ color, radius, durationMs, particles, motion });

export const SKILL_LOOKS: Record<string, SkillLook> = {
  // Tank: Stahl, Erde, Signalfarben
  taunt: look(0xdf7126, 40, 700, 6, 'wave'),
  shieldBash: look(0xfbf236, 24, 400, 5, 'burst'),
  ironWall: look(0x9badb7, 44, 900, 6, 'rise'),
  lastStand: look(0xac3232, 46, 900, 8, 'rise'),
  armorAura: look(0x639bff, 36, 600, 4, 'wave'),
  thickSkin: look(0x8f563b, 30, 500, 4, 'burst'),
  regeneration: look(0x6abe30, 28, 700, 5, 'rise'),
  guardian: look(0x5b6ee1, 40, 700, 6, 'wave'),
  fortified: look(0x847e87, 32, 500, 4, 'fall'),
  // Zauberer: Feuer, Eis, Blitz, Arkan
  fireball: look(0xdf7126, 34, 450, 8, 'burst'),
  iceWall: look(0x5fcde4, 38, 700, 6, 'wave'),
  lightningStorm: look(0xfbf236, 36, 500, 6, 'fall'),
  meteor: look(0xac3232, 54, 800, 10, 'fall'),
  arcanePower: look(0x76428a, 32, 600, 6, 'rise'),
  spellEcho: look(0xd77bba, 30, 600, 6, 'burst'),
  frostArmor: look(0xcbdbfc, 32, 600, 5, 'fall'),
  elementalMastery: look(0x306082, 34, 600, 6, 'burst'),
  // Heiler: Grün, Gold, Licht
  heal: look(0x99e550, 30, 600, 5, 'rise'),
  groupHeal: look(0x6abe30, 44, 700, 8, 'wave'),
  divineShield: look(0xeec39a, 42, 800, 6, 'rise'),
  resurrection: look(0xffffff, 50, 900, 8, 'rise'),
  healingAura: look(0x37946e, 36, 700, 5, 'wave'),
  blessings: look(0xd9a066, 32, 700, 6, 'fall'),
  holyGround: look(0x8f974a, 40, 700, 6, 'wave'),
  // Dieb (B-577): kurzer Stich, Rauchwolke
  backstab: look(0xd95763, 22, 300, 4, 'burst'),
  smokeBomb: look(0x696a6a, 48, 900, 8, 'rise'),
};

/** Effekt eines Skills: eigener Look, sonst der Ring seiner Linie; unbekannter Skill oder Linie ohne Farbe ergibt `undefined`. */
export function skillLook(id: string): SkillLook | undefined {
  const own = SKILL_LOOKS[id];
  if (own) return own;
  const color = LINE_COLORS[SKILLS[id]?.line ?? ''];
  if (color === undefined) return undefined;
  const { radius, durationMs, particles } = EFFECT_CONFIG.skill;
  return { color, radius, durationMs, particles, motion: 'burst' };
}

/** Ziel-Versatz des Partikels i von n je Bewegung (Pixel, relativ zum Effekt-Ort). */
export function particleOffset(motion: SkillMotion, i: number, n: number, radius: number): [number, number] {
  const spread = n > 1 ? i / (n - 1) - 0.5 : 0;
  switch (motion) {
    case 'rise': return [spread * radius * 1.6, -radius * 2];
    case 'fall': return [spread * radius * 1.6, radius * 1.2];
    case 'wave': return [(i % 2 === 0 ? -1 : 1) * radius * (1.2 + Math.floor(i / 2) * 0.5), 30];
    default: {
      const angle = (i / n) * Math.PI * 2;
      return [Math.cos(angle) * radius * 1.4, Math.sin(angle) * radius * 1.4 - 12];
    }
  }
}
