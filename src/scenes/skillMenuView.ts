import Phaser from 'phaser';
import { nameOf, t } from '../core/texts';
import { MONARCH } from '../model/data';
import type { Player } from '../model/types';
import { slotBindings, type Device } from '../input/slotBindings';
import { fontStyle } from './fontRules';
import type { RadarCell } from './radarView';
import { menuEntries, skillName, slotViews, type MenuEntry, type SkillMenus } from './skillMenuLogic';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
/** Sichtbare Menüzeilen um den Cursor (passt auch ins Viertel bei 28 px) */
const WINDOW = 5;
const TIERS = ['', 'I', 'II', 'III', 'IV'];

const lineName = (line: string): string => nameOf('line', line, (MONARCH.lines as Record<string, { name: string }>)[line]?.name ?? line);

function entryText(e: MenuEntry, selected: boolean): string {
  const mark = selected ? '▶ ' : '   ';
  if (e.id === 'respec') return mark + t('skill.respec');
  const state = e.learned ? t('skill.learned') : e.learnable ? t('skill.learnable') : t('skill.locked');
  const passive = e.passive ? ` (${t('skill.passive')})` : '';
  return `${mark}${lineName(e.line)} ${TIERS[e.tier] ?? e.tier} · ${skillName(e.id)}${passive}  ·  ${state}`;
}

/** Skill-Leiste: Taste, Skill und Abklingzeit je Slot */
function slotBar(p: Player, device: Device): string {
  const labels = slotBindings(device).slice(1, 5).map((b) => b.label);
  return slotViews(p)
    .map((s, i) => `${labels[i]} ${s.skill ? `${skillName(s.skill)} ${s.cooldown > 0 ? `${Math.ceil(s.cooldown)} s` : t('skill.ready')}` : t('skill.free')}`)
    .join('  ·  ');
}

function menuText(p: Player, cursor: number, device: Device): string {
  const entries = menuEntries(p);
  const from = Math.max(0, Math.min(cursor - Math.floor(WINDOW / 2), entries.length - WINDOW));
  const rows = entries.slice(from, from + WINDOW).map((e, i) => entryText(e, from + i === cursor));
  const menu = slotBindings(device).find((b) => b.action === 'skillMenu')?.label ?? '';
  return [t('skill.title', { n: p.points }), ...rows, t(`skill.hint.${device}`, { menu })].join('\n');
}

/** Zeichnet je Spielerfeld die Skill-Leiste (immer) und das Skill-Menü (wenn offen). Nur Zeichnen, die Logik liegt in `skillMenuLogic.ts`. */
export class SkillMenuLayer {
  private bars: Phaser.GameObjects.Text[] = [];
  private menus: Phaser.GameObjects.Text[] = [];

  constructor(private readonly scene: Phaser.Scene) {}

  /** `slotOf`: lokaler Slot zum Platz der Zelle (`cell.seat`), `deviceOf`: Gerät des Spielers im Slot */
  draw(cells: readonly RadarCell[], menus: SkillMenus, slotOf: (seat: number) => number | undefined, deviceOf: (slot: number) => Device): void {
    [...this.bars, ...this.menus].forEach((o) => o.setVisible(false));
    let n = 0;
    for (const { cell, monarch, world } of cells) {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      const slot = slotOf(cell.seat);
      if (!p || slot === undefined) continue;
      const device = deviceOf(slot);
      const bar = (this.bars[n] ??= this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }).setOrigin(0, 1));
      bar.setText(slotBar(p, device)).setPosition(cell.x + 24, cell.y + cell.h - 24).setVisible(true);
      const menu = (this.menus[n] ??= this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue'), backgroundColor: '#0c1024cc', padding: { x: 16, y: 12 } }).setOrigin(0.5));
      n += 1;
      if (!menus.isOpen(slot)) continue;
      menu.setText(menuText(p, menus.cursor(slot), device)).setPosition(cell.x + cell.w / 2, cell.y + cell.h / 2 - 40).setVisible(true); // etwas hoch: unten steht der Beitritts-Hinweis
    }
  }
}
