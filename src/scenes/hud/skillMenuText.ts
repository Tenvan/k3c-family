import { nameOf, t } from '../../core/texts';
import { MONARCH } from '../../model/data';
import type { Player } from '../../model/types';
import { slotLabels, type HintDevice } from '../hints/glyphs';
import { menuEntries, skillName, slotViews, type MenuEntry } from './skillMenuLogic';

/**
 * Texte der Skill-Leiste und des Skill-Menüs, ohne Phaser (B-755): Die Leiste im HUD nennt keine Tasten, das Menü zeigt
 * die Slot-Taste des Geräts vor dem Skill. Die Tasten kommen aus der Belegung (`glyphs.ts` › `slotLabels`).
 */

/** Sichtbare Menüzeilen um den Cursor (passt auch ins Viertel bei 28 px) */
export const WINDOW = 5;
const TIERS = ['', 'I', 'II', 'III', 'IV'];

const lineName = (line: string): string => nameOf('line', line, (MONARCH.lines as Record<string, { name: string }>)[line]?.name ?? line);

/** Skill-ID → Taste seines Slots (Slot 1–4 = Beschriftungen 1–4 von `slotLabels`); Skills ohne Slot fehlen */
export function slotKeys(p: Player, device: HintDevice): Map<string, string> {
  const labels = slotLabels(device).slice(1, 5);
  return new Map(slotViews(p).flatMap((s, i) => (s.skill && labels[i] ? [[s.skill, labels[i]!] as [string, string]] : [])));
}

/** Menüzeile: Cursor, Taste (nur belegter Skill) vor Linie und Skillname, dann Zustand */
export function entryText(e: MenuEntry, selected: boolean, key?: string): string {
  const mark = selected ? '▶ ' : '   ';
  if (e.id === 'respec') return mark + t('skill.respec');
  const state = e.learned ? t('skill.learned') : e.learnable ? t('skill.learnable') : t('skill.locked');
  const passive = e.passive ? ` (${t('skill.passive')})` : '';
  return `${mark}${key ? `[${key}] ` : ''}${lineName(e.line)} ${TIERS[e.tier] ?? e.tier} · ${skillName(e.id)}${passive}  ·  ${state}`;
}

/** Skill-Leiste im HUD: Skill und Abklingzeit je Slot, ohne Taste */
export function slotBar(p: Player): string {
  return slotViews(p)
    .map((s) => (s.skill ? `${skillName(s.skill)} ${s.cooldown > 0 ? `${Math.ceil(s.cooldown)} s` : t('skill.ready')}` : t('skill.free')))
    .join('  ·  ');
}

export function menuText(p: Player, cursor: number, device: HintDevice): string {
  const entries = menuEntries(p), keys = slotKeys(p, device);
  const from = Math.max(0, Math.min(cursor - Math.floor(WINDOW / 2), entries.length - WINDOW));
  const rows = entries.slice(from, from + WINDOW).map((e, i) => entryText(e, from + i === cursor, keys.get(e.id)));
  const menu = slotLabels(device)[5] ?? '';
  return [t('skill.title', { n: p.points }), ...rows, t(`skill.hint.${device}`, { menu })].join('\n');
}
