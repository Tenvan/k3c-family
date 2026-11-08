import Phaser from 'phaser';
import { fontStyle } from '../render/fontRules';
import type { HintDevice } from '../hints/glyphs';
import type { RadarCell } from './radarView';
import { HudBox } from './hudElements';
import type { SkillMenus } from './skillMenuLogic';
import { menuText, slotBar } from './skillMenuText';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
/** Schrift des Menüs je Feldbreite: moderat größer als 28 px, nie darunter (Q03); `fit` schrumpft bis der Text passt */
const MENU_PX_FULL = 36;
const MENU_PX_HALF = 32;
const MENU_MIN_PX = 28;
const MENU_PAD = { x: 24, y: 18 } as const;

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
      bar.text.setText(slotBar(p)).setVisible(true);
      const menu = (this.menus[n] ??= this.scene.add.text(0, 0, '', { ...STYLE, ...fontStyle('playerValue'), backgroundColor: '#0c1024cc', padding: MENU_PAD }).setOrigin(0.5));
      n += 1;
      if (!menus.isOpen(slot)) continue;
      menu.setText(menuText(p, menus.cursor(slot), device)).setPosition(cell.x + cell.w / 2, cell.y + cell.h / 2 - 40).setVisible(true); // etwas hoch: unten steht der Beitritts-Hinweis
      fit(menu, cell);
    }
  }
}

/** Größte Schrift (36 px im vollen, 32 px im halben Feld, mindestens 28 px), bei der das Menü ins Feld passt */
function fit(menu: Phaser.GameObjects.Text, cell: { w: number; h: number }): void {
  for (let px = cell.w >= 1400 ? MENU_PX_FULL : MENU_PX_HALF; px >= MENU_MIN_PX; px -= 2) {
    menu.setFontSize(px);
    if (menu.width <= cell.w * 0.96 && menu.height <= cell.h * 0.9) return;
  }
  menu.setFontSize(MENU_MIN_PX);
}
