import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '../core/constants';
import type { Troop, World } from '../model/types';
import { healerRanges, merchantSpot, professionLabel } from './buergerAnzeige';
import { fontStyle } from './fontRules';
import { TEXT } from './siteView';
import { isOnTower } from './viewRules';

/**
 * Bürger zeichnen (W6.3, B-126): Berufssymbol am Bauern, Händlerfigur im Hub, Reichweite der Heilplätze. Platzhalter-Formen
 * und Namen als Text, bis GR3 Sprites einbindet. Alles kommt aus dem Zustand (`buergerAnzeige.ts`); fehlt ein Feld,
 * entsteht nichts.
 */

const G = GROUND_Y;
/** Platzhalter-Farbe je Symbolname aus `professionLabel` */
const ICON_COLOR: Record<string, number> = { 'prof-miner': 0x8d99ae, 'prof-builder': 0xe9c46a, 'prof-craftsman': 0xb87333, 'prof-unknown': 0xcccccc };
/** Höhe des Berufssymbols über dem Boden (über Kopf und Lebensbalken) */
const BADGE_Y = -150;

type View = Phaser.GameObjects.Container;

export class BuergerView {
  private readonly badges = new Map<number, View>();
  private readonly healers: Phaser.GameObjects.Ellipse[] = [];
  private merchant: View | null = null;

  constructor(
    private readonly scene: Phaser.Scene,
    private readonly root: Phaser.GameObjects.Layer,
  ) {}

  sync(world: World): void {
    this.syncBadges(world);
    this.syncMerchant(world);
    this.syncHealers(world);
  }

  private syncBadges(world: World): void {
    const seen = new Set<number>();
    for (const tr of world.troops) {
      const label = professionLabel(tr.profession, true);
      if (!label) continue;
      seen.add(tr.id);
      let v = this.badges.get(tr.id);
      if (!v) {
        v = this.make(label.icon, label.text);
        this.badges.set(tr.id, v);
      }
      this.place(v, tr, world);
    }
    for (const [id, v] of this.badges) {
      if (seen.has(id)) continue;
      v.destroy();
      this.badges.delete(id);
    }
  }

  private place(v: View, tr: Troop, world: World): void {
    v.setPosition(tr.x * UNIT_PX, isOnTower(world, tr) ? G - 270 : G);
  }

  private make(icon: string, text: string): View {
    const s = this.scene;
    const shape = s.add.circle(0, BADGE_Y, 12, ICON_COLOR[icon] ?? 0xcccccc).setStrokeStyle(3, 0x000000);
    const name = s.add.text(0, BADGE_Y - 18, text, { ...TEXT, ...fontStyle('citizenLabel') }).setOrigin(0.5, 1);
    const v = s.add.container(0, G, [shape, name]).setDepth(11);
    this.root.add(v);
    return v;
  }

  private syncMerchant(world: World): void {
    const spot = merchantSpot(world);
    if (!spot) {
      this.merchant?.destroy();
      this.merchant = null;
      return;
    }
    if (!this.merchant) {
      const s = this.scene;
      const body = s.add.rectangle(0, -40, 36, 80, 0x7b2cbf).setStrokeStyle(3, 0x240046);
      const head = s.add.circle(0, -92, 14, 0xf1c27d).setStrokeStyle(3, 0x240046);
      const label = s.add.text(0, -116, '', { ...TEXT, ...fontStyle('citizenLabel') }).setOrigin(0.5, 1);
      this.merchant = s.add.container(0, G, [body, head, label]).setDepth(6);
      this.root.add(this.merchant);
    }
    this.merchant.setX(spot.x);
    (this.merchant.getAt(2) as Phaser.GameObjects.Text).setText(spot.text);
  }

  private syncHealers(world: World): void {
    const ranges = healerRanges(world);
    while (this.healers.length > ranges.length) this.healers.pop()?.destroy();
    ranges.forEach((r, i) => {
      let e = this.healers[i];
      if (!e) {
        e = this.scene.add.ellipse(0, G, 1, 24, 0x52b788, 0.2).setStrokeStyle(3, 0x52b788, 0.7).setDepth(1);
        this.root.add(e);
        this.healers.push(e);
      }
      e.setPosition(r.x, G).setSize(r.radiusPx * 2, 24);
    });
  }
}
