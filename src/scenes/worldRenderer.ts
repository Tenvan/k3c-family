import Phaser from 'phaser';
import { GROUND_Y, PLAYER_COLORS, UNIT_PX } from '../core/constants';
import { BUILDINGS, TROOPS } from '../world/sim/data';
import { canAfford } from '../world/sim/economy';
import { hasDepth } from '../world/sim/travel';
import { isOnTower } from '../world/sim/units';
import type { Coin, Enemy, Pickup, Player, Projectile, ResourceNode, Site, Troop, World } from '../world/sim/types';

/**
 * Zeichnet den Simulationszustand mit Platzhalter-Formen. Hält pro Entität (id) ein Phaser-Objekt
 * und legt an/entfernt sie passend zum Zustand. Später durch Sprites ersetzen, die Logik bleibt.
 */

const U = UNIT_PX;
const G = GROUND_Y;
const PRICE_TAG_RANGE = 6;
const TEXT = { fontSize: '22px', color: '#ffffff', stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' };

const SITE_SIZE: Record<Site['kind'], [number, number]> = {
  wall: [36, 150],
  tower: [70, 260],
  workshop: [150, 120],
  stairsUp: [110, 110],
  stairsDown: [110, 110],
};
const SITE_COLOR: Record<Site['kind'], number> = {
  wall: 0x8d99ae,
  tower: 0x9c6644,
  workshop: 0xbc6c25,
  stairsUp: 0x6c757d,
  stairsDown: 0x343a40,
};

const ENEMY_COLORS: Record<string, number> = {
  greed: 0x5a189a,
  wolf: 0x6c757d,
  goblin: 0x2b9348,
  goblinArcher: 0x007f5f,
  skeleton: 0xe9ecef,
  bat: 0x3c096c,
  caveTroll: 0x7f5539,
  zombie: 0x606c38,
  ratSwarm: 0x8d6e63,
  mineGhost: 0xcaf0f8,
};

type View = Phaser.GameObjects.Container;

/** Synchronisiert eine Liste von Entitäten mit Phaser-Containern. */
class Layer<T extends { id: number }> {
  readonly views = new Map<number, View>();
  constructor(
    private readonly create: (t: T) => View,
    private readonly update: (v: View, t: T) => void,
    /** true = Objekt neu aufbauen (z.B. Landstreicher wurde zum Bauer) */
    private readonly rebuild?: (v: View, t: T) => boolean,
  ) {}

  sync(items: readonly T[]): void {
    const seen = new Set<number>();
    for (const item of items) {
      seen.add(item.id);
      let view = this.views.get(item.id);
      if (view && this.rebuild?.(view, item)) {
        view.destroy();
        view = undefined;
      }
      if (!view) {
        view = this.create(item);
        this.views.set(item.id, view);
      }
      this.update(view, item);
    }
    for (const [id, view] of this.views) {
      if (!seen.has(id)) {
        view.destroy();
        this.views.delete(id);
      }
    }
  }
}

/** Kleiner Balken (HP, Fortschritt). Gibt ein Rectangle zurück, dessen Breite man per setBar() setzt. */
function bar(scene: Phaser.Scene, y: number, width: number, color: number): Phaser.GameObjects.Rectangle[] {
  const bg = scene.add.rectangle(0, y, width, 8, 0x000000, 0.6).setOrigin(0.5);
  const fg = scene.add.rectangle(-width / 2, y, width, 8, color).setOrigin(0, 0.5);
  return [bg, fg];
}

function setBar(v: View, index: number, fraction: number, visible = true): void {
  const bg = v.getAt(index) as Phaser.GameObjects.Rectangle;
  const fg = v.getAt(index + 1) as Phaser.GameObjects.Rectangle;
  bg.setVisible(visible);
  fg.setVisible(visible);
  fg.width = bg.width * Phaser.Math.Clamp(fraction, 0, 1);
}

export class WorldRenderer {
  private readonly players: Layer<Player>;
  private readonly troops: Layer<Troop>;
  private readonly enemies: Layer<Enemy>;
  private readonly coins: Layer<Coin>;
  private readonly nodes: Layer<ResourceNode>;
  private readonly sites: Layer<Site>;
  private readonly projectiles: Layer<Projectile>;
  private readonly pickups: Layer<Pickup>;
  private readonly castleHp: View;
  private readonly lastGold = new Map<number, { gold: number; at: number }>();

  constructor(
    private readonly scene: Phaser.Scene,
    private world: World,
  ) {
    this.drawStatic();
    this.castleHp = scene.add.container(world.castle.x * U, G - 300, bar(scene, 0, 240, 0x52b788)).setDepth(5);

    this.nodes = new Layer((n) => this.createNode(n), (v, n) => this.updateNode(v, n));
    this.sites = new Layer((s) => this.createSite(s), (v, s) => this.updateSite(v, s));
    this.pickups = new Layer((p) => this.createPickup(p), () => {});
    this.coins = new Layer((c) => scene.add.container(c.x * U, G - 8, [scene.add.circle(0, 0, 8, 0xffd166).setStrokeStyle(2, 0x9c6644)]).setDepth(6), () => {});
    this.troops = new Layer(
      (t) => this.createTroop(t),
      (v, t) => this.updateTroop(v, t),
      (v, t) => v.getData('kind') !== t.kind,
    );
    this.enemies = new Layer((e) => this.createEnemy(e), (v, e) => this.updateEnemy(v, e));
    this.players = new Layer((p) => this.createPlayer(p), (v, p) => this.updatePlayer(v, p));
    this.projectiles = new Layer(
      (p) => scene.add.container(p.x * U, G - 70, [scene.add.rectangle(0, 0, 26, 3, p.team === 'player' ? 0xffffff : 0xff4d6d)]).setDepth(9),
      (v, p) => v.setX(p.x * U),
    );
  }

  /** Kamera-Ziel für Spieler i */
  playerView(index: number): View | undefined {
    const p = this.world.players[index];
    return p ? this.players.views.get(p.id) : undefined;
  }

  sync(world: World): void {
    this.world = world;
    this.nodes.sync(world.nodes);
    this.sites.sync(world.sites);
    this.pickups.sync(world.pickups);
    this.coins.sync(world.coins);
    this.troops.sync(world.troops);
    this.enemies.sync(world.enemies);
    this.players.sync(world.players);
    this.projectiles.sync(world.projectiles);
    setBar(this.castleHp, 0, world.castle.hp / world.castle.maxHp, world.castle.hp < world.castle.maxHp);
  }

  // ---------- statische Objekte ----------

  private drawStatic(): void {
    const { scene, world } = this;
    const x = world.castle.x * U;
    scene.add.rectangle(x, G - 130, 300, 260, 0x6c757d).setStrokeStyle(4, 0x343a40);
    scene.add.rectangle(x, G - 40, 60, 80, 0x343a40);
    for (const dx of [-120, -40, 40, 120]) scene.add.rectangle(x + dx, G - 275, 40, 30, 0x6c757d).setStrokeStyle(4, 0x343a40);

    for (const e of world.level.entities) {
      const ex = e.x * U;
      if (e.kind === 'portal') {
        scene.add.ellipse(ex, G - 110, 110, 220, 0x7b2cbf).setStrokeStyle(6, 0x240046);
      } else if (e.kind === 'exit') {
        scene.add.rectangle(ex, G - 90, 160, 180, 0x111111).setStrokeStyle(6, 0x555555);
        const deeper = hasDepth(world.biome.depth + 1);
        scene.add.text(ex, G - 210, deeper ? `Tiefe ${world.biome.depth + 1}\nalle hierher` : 'verschüttet', { ...TEXT, align: 'center' }).setOrigin(0.5);
      } else if (e.kind === 'bush') {
        scene.add.circle(ex, G - 14, 18, 0x40916c); // Deko
      } else if (e.kind === 'recruitCamp') {
        scene.add.triangle(ex, G - 45, 0, 90, 60, 0, 120, 90, 0xe9c46a).setStrokeStyle(3, 0x7f5539);
        scene.add.circle(ex + 90, G - 10, 14, 0xf77f00); // Lagerfeuer
      }
    }
  }

  // ---------- Ressourcen ----------

  private createNode(n: ResourceNode): View {
    const s = this.scene;
    const parts: Phaser.GameObjects.GameObject[] = [];
    if (n.kind === 'tree') {
      parts.push(s.add.rectangle(0, -40, 14, 80, 0x6b4226), s.add.triangle(0, -110, 0, 90, 40, 0, 80, 90, 0x2d6a4f));
    } else if (n.kind === 'rock') {
      parts.push(s.add.ellipse(0, -16, 50, 34, 0x8d99ae));
    } else if (n.kind === 'copperOre') {
      parts.push(s.add.ellipse(0, -16, 44, 30, 0xb87333).setStrokeStyle(3, 0x6d3f1f));
    } else {
      parts.push(s.add.circle(0, -14, 18, 0x40916c)); // Busch
    }
    const flag = s.add.rectangle(10, -170, 24, 14, 0xd00000).setOrigin(0, 0.5);
    const pole = s.add.rectangle(10, -160, 3, 30, 0xffffff);
    return s.add.container(n.x * U, G, [...parts, pole, flag, ...bar(s, -190, 50, 0xffd166)]);
  }

  private updateNode(v: View, n: ResourceNode): void {
    const i = v.length - 4;
    (v.getAt(i) as Phaser.GameObjects.Rectangle).setVisible(n.marked);
    (v.getAt(i + 1) as Phaser.GameObjects.Rectangle).setVisible(n.marked);
    setBar(v, i + 2, n.progress, n.progress > 0);
    v.setScale(1, 1 - n.progress * 0.3);
  }

  // ---------- Bauplätze ----------

  private createSite(s: Site): View {
    const g = this.scene.add.graphics();
    const label = this.scene.add.text(0, 0, '', { ...TEXT, fontSize: '20px' }).setOrigin(0.5, 1);
    return this.scene.add.container(s.x * U, G, [g, label]).setDepth(2);
  }

  private updateSite(v: View, s: Site): void {
    const near = this.world.players.some((p) => p.respawnIn <= 0 && Math.abs(p.x - s.x) < PRICE_TAG_RANGE);
    const data = BUILDINGS[s.kind];
    const key = [s.state, s.paidGold, Math.round(s.buildProgress * 20), Math.round((s.hp / s.maxHp) * 20), s.bows, s.bowPaidGold, near, canAfford(this.world.stock, data.cost)].join('|');
    if (v.getData('key') === key) return;
    v.setData('key', key);

    const g = v.getAt(0) as Phaser.GameObjects.Graphics;
    const label = v.getAt(1) as Phaser.GameObjects.Text;
    g.clear();
    label.setText('');
    const [w, h] = SITE_SIZE[s.kind];

    if (s.state === 'built') {
      const color = SITE_COLOR[s.kind];
      g.fillStyle(color).fillRect(-w / 2, -h, w, h).lineStyle(4, 0x343a40).strokeRect(-w / 2, -h, w, h);
      if (s.kind === 'tower') g.fillStyle(0x6f4518).fillRect(-w / 2 - 15, -h - 10, w + 30, 14);
      if (s.kind === 'stairsUp' || s.kind === 'stairsDown') {
        // Stufen als Treppe, dazu ein Schild wohin es geht
        for (let i = 0; i < 5; i++) g.fillStyle(0x495057).fillRect(-w / 2 + i * (w / 5), -((i + 1) * h) / 5, w / 5, ((i + 1) * h) / 5);
        label.setPosition(0, -h - 12).setText(s.kind === 'stairsUp' ? 'Treppe hoch' : 'Treppe runter').setColor('#ffd166');
      }
      if (s.hp < s.maxHp) this.drawBar(g, -h - 30, 80, s.hp / s.maxHp, 0x52b788);
      if (s.kind === 'workshop') {
        for (let i = 0; i < s.bows; i++) g.lineStyle(4, 0xffd166).beginPath().arc(-40 + i * 30, -60, 14, -Math.PI / 2, Math.PI / 2).strokePath();
        const price = TROOPS.archer.cost ?? {};
        if (near && s.bows < (data.bowRack ?? 0)) this.drawPrice(g, label, -h - 20, price.gold ?? 0, s.bowPaidGold, 'Bogen', price);
      }
      return;
    }

    // Noch nicht gebaut: Umriss + Preisschild bzw. Baufortschritt
    g.lineStyle(3, 0xffffff, 0.5).strokeRect(-w / 2, -h, w, h);
    if (s.buildProgress > 0) {
      g.fillStyle(0xdda15e, 0.8).fillRect(-w / 2, -h * s.buildProgress, w, h * s.buildProgress);
    }
    if (s.state === 'unpaid' && near) this.drawPrice(g, label, -h - 20, data.cost.gold ?? 0, s.paidGold, data.name, data.cost);
    if (s.state === 'waitingMaterial') label.setPosition(0, -h - 20).setText(`${data.name}: ${missing(this.world.stock, data.cost)}`).setColor('#ff8fa3');
    if (s.state === 'waitingWorker') label.setPosition(0, -h - 20).setText(s.buildProgress > 0 ? '' : `${data.name}: wartet auf Bauer`).setColor('#ffffff');
  }

  /** Münz-Slots (K2C): gefüllt = bezahlt. Baumaterial steht darüber. */
  private drawPrice(g: Phaser.GameObjects.Graphics, label: Phaser.GameObjects.Text, y: number, total: number, paid: number, name: string, cost: Record<string, number | undefined>): void {
    const perRow = 10;
    for (let i = 0; i < total; i++) {
      const row = Math.floor(i / perRow);
      const inRow = Math.min(perRow, total - row * perRow);
      const cx = (i % perRow) * 22 - ((inRow - 1) * 22) / 2;
      const cy = y - row * 22;
      g.lineStyle(2, 0xffd166).strokeCircle(cx, cy, 8);
      if (i < paid) g.fillStyle(0xffd166).fillCircle(cx, cy, 8);
    }
    const material = (['wood', 'stone', 'copper'] as const).filter((r) => cost[r]).map((r) => `${cost[r]} ${RESOURCE_NAMES[r]}`);
    const enough = canAfford(this.world.stock, cost);
    label.setPosition(0, y - Math.ceil(total / perRow) * 22).setText([name, ...material].join(' · ')).setColor(enough ? '#ffffff' : '#ff8fa3');
  }

  private drawBar(g: Phaser.GameObjects.Graphics, y: number, width: number, fraction: number, color: number): void {
    g.fillStyle(0x000000, 0.6).fillRect(-width / 2, y, width, 8);
    g.fillStyle(color).fillRect(-width / 2, y, width * Phaser.Math.Clamp(fraction, 0, 1), 8);
  }

  // ---------- Truppen ----------

  private createTroop(t: Troop): View {
    const s = this.scene;
    const color = t.kind === 'vagrant' ? 0x8d8d8d : t.kind === 'peasant' ? 0xbc8a5f : 0x40916c;
    const body = s.add.rectangle(0, -30, 22, 60, color).setStrokeStyle(2, 0x000000);
    const head = s.add.circle(0, -70, 12, 0xf1c27d).setStrokeStyle(2, 0x000000);
    const extra =
      t.kind === 'archer'
        ? s.add.arc(14, -40, 18, -80, 80, false).setStrokeStyle(4, 0x6b4226).setClosePath(false)
        : s.add.rectangle(0, -84, 30, 6, t.kind === 'peasant' ? 0xe9c46a : 0x5c5c5c);
    const load = s.add.rectangle(-16, -44, 16, 24, 0x6b4226).setVisible(false);
    return s.add.container(t.x * U, G, [body, head, extra, load, ...bar(s, -100, 36, 0x52b788)]).setDepth(7).setData('kind', t.kind);
  }

  private updateTroop(v: View, t: Troop): void {
    const dx = t.x * U - v.x;
    if (Math.abs(dx) > 0.5) v.scaleX = Math.sign(dx);
    v.setPosition(t.x * U, isOnTower(this.world, t) ? G - 270 : G);
    (v.getAt(3) as Phaser.GameObjects.Rectangle).setVisible(t.job?.type === 'carry');
    setBar(v, 4, t.hp / t.maxHp, t.hp < t.maxHp);
  }

  // ---------- Gegner ----------

  private createEnemy(e: Enemy): View {
    const s = this.scene;
    const elite = e.maxHp >= 150 || e.traits.includes('ranged');
    const [w, h] = e.traits.includes('flying') ? [40, 26] : elite ? [60, 90] : [40, 50];
    const y = e.traits.includes('flying') ? -130 : -h / 2;
    const body = s.add.ellipse(0, y, w, h, ENEMY_COLORS[e.kind] ?? 0xff00ff).setStrokeStyle(3, 0x10002b);
    const eye1 = s.add.circle(-8, y - h / 5, 5, 0xffffff);
    const eye2 = s.add.circle(8, y - h / 5, 5, 0xffffff);
    const loot = s.add.circle(0, y - h / 2 - 14, 10, 0xffd166).setVisible(false);
    return s.add.container(e.x * U, G, [body, eye1, eye2, loot, ...bar(s, y - h / 2 - 30, 44, 0xe63946)]).setDepth(8);
  }

  private updateEnemy(v: View, e: Enemy): void {
    const dx = e.x * U - v.x;
    if (Math.abs(dx) > 0.5) v.scaleX = Math.sign(dx);
    v.setX(e.x * U);
    (v.getAt(3) as Phaser.GameObjects.Arc).setVisible(e.carriedGold > 0);
    setBar(v, 4, e.hp / e.maxHp, e.hp < e.maxHp);
    v.setAlpha(e.fleeing ? 0.7 : 1);
  }

  // ---------- Pickups ----------

  private createPickup(p: Pickup): View {
    const s = this.scene;
    const shape =
      p.kind === 'chest'
        ? s.add.rectangle(0, -18, 44, 36, 0x9c6644).setStrokeStyle(3, 0x5c3d2e)
        : s.add.star(0, -60, 5, 8, 18, 0xffd60a).setStrokeStyle(2, 0xffffff);
    return s.add.container(p.x * U, G, [shape]).setDepth(3);
  }

  // ---------- Monarchen ----------

  private createPlayer(p: Player): View {
    const s = this.scene;
    const color = PLAYER_COLORS[p.index % PLAYER_COLORS.length];
    const body = s.add.rectangle(0, -40, 36, 80, color).setStrokeStyle(3, 0x000000);
    const crown = s.add.rectangle(0, -88, 28, 12, 0xffd166).setStrokeStyle(2, 0x000000);
    const purse = s.add.text(0, -110, '', { ...TEXT, color: '#ffd166', fontSize: '26px' }).setOrigin(0.5, 1);
    return s.add.container(p.x * U, G, [body, crown, purse, ...bar(s, -150, 50, 0x52b788)]).setDepth(10);
  }

  private updatePlayer(v: View, p: Player): void {
    v.setPosition(p.x * U, G);
    (v.getAt(0) as Phaser.GameObjects.Rectangle).scaleX = p.facing;
    v.setAlpha(p.respawnIn > 0 ? 0.25 : 1);

    // Münzbeutel über dem Kopf: beim Bezahlen und kurz nach jeder Änderung
    const now = this.world.time;
    const last = this.lastGold.get(p.id);
    if (!last || last.gold !== p.gold) this.lastGold.set(p.id, { gold: p.gold, at: last ? now : -99 });
    const recent = now - (this.lastGold.get(p.id)?.at ?? -99) < 2;
    const purse = v.getAt(2) as Phaser.GameObjects.Text;
    purse.setText(`${p.gold} Gold`).setVisible(p.paying || recent);
    setBar(v, 3, p.hp / p.maxHp, p.hp < p.maxHp && p.respawnIn <= 0);
  }
}

export const RESOURCE_NAMES = { wood: 'Holz', stone: 'Stein', copper: 'Kupfer' } as const;

function missing(stock: World['stock'], cost: Record<string, number | undefined>): string {
  const parts = (['wood', 'stone', 'copper'] as const)
    .filter((r) => (cost[r] ?? 0) > stock[r])
    .map((r) => `${(cost[r] ?? 0) - stock[r]} ${RESOURCE_NAMES[r]} fehlt`);
  return parts.join(', ') || 'bereit';
}
