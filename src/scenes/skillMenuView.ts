import Phaser from 'phaser';
import { nameOf, t } from '../core/texts';
import { MONARCH } from '../model/data';
import type { Player } from '../model/types';
import { fontStyle } from './fontRules';
import { slotLabels, type HintDevice } from './glyphs';
import type { RadarCell } from './radarView';
import { HudBox } from './hudElements';
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
function slotBar(p: Player, device: HintDevice): string {
  const labels = slotLabels(device).slice(1, 5);
  return slotViews(p)
    .map((s, i) => `${labels[i]} ${s.skill ? `${skillName(s.skill)} ${s.cooldown > 0 ? `${Math.ceil(s.cooldown)} s` : t('skill.ready')}` : t('skill.free')}`)
    .join('  ·  ');
}

function menuText(p: Player, cursor: number, device: HintDevice): string {
  const entries = menuEntries(p);
  const from = Math.max(0, Math.min(cursor - Math.floor(WINDOW / 2), entries.length - WINDOW));
  const rows = entries.slice(from, from + WINDOW).map((e, i) => entryText(e, from + i === cursor));
  const menu = slotLabels(device)[5] ?? '';
  return [t('skill.title', { n: p.points }), ...rows, t(`skill.hint.${device}`, { menu })].join('\n');
}

/**
 * Zeichnet je Spielerfeld die Skill-Leiste (immer, als HUD-Element `skills:<Zelle>`, Lage aus `hudLayout`) und das
 * Skill-Menü (wenn offen, mittig im Feld). Nur Zeichnen, die Logik liegt in `skillMenuLogic.ts`.
 */
export class SkillMenuLayer {
  /** Skill-Leiste je Zellen-Index */
  readonly bars = new Map<number, HudBox>();
  private menus: Phaser.GameObjects.Text[] = [];

  constructor(private readonly scene: Phaser.Scene) {}

  /** `slotOf`: lokaler Slot zum Platz der Zelle (`cell.seat`), `deviceOf`: Gerät des Spielers im Slot */
  draw(cells: readonly RadarCell[], menus: SkillMenus, slotOf: (seat: number) => number | undefined, deviceOf: (slot: number) => HintDevice): void {
    for (const b of this.bars.values()) b.text.setVisible(false);
    this.menus.forEach((o) => o.setVisible(false));
    let n = 0;
    for (const [i, { cell, monarch, world }] of cells.entries()) {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      const slot = slotOf(cell.seat);
      if (!p || slot === undefined) continue;
      const device = deviceOf(slot);
      let bar = this.bars.get(i);
      if (!bar) this.bars.set(i, (bar = new HudBox(this.scene, this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue') }))));
      bar.text.setText(slotBar(p, device)).setVisible(true);
      const menu = (this.menus[n] ??= this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue'), backgroundColor: '#0c1024cc', padding: { x: 16, y: 12 } }).setOrigin(0.5));
      n += 1;
      if (!menus.isOpen(slot)) continue;
      menu.setText(menuText(p, menus.cursor(slot), device)).setPosition(cell.x + cell.w / 2, cell.y + cell.h / 2 - 40).setVisible(true); // etwas hoch: unten steht der Beitritts-Hinweis
    }
  }
}
