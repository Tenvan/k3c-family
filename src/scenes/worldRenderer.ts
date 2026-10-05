import Phaser from 'phaser';
import { GROUND_Y, PLAYER_COLORS, UNIT_PX } from '../core/constants';
import { t } from '../core/texts';
import { fontStyle } from './fontRules';
import { createRider, updateRider } from './mountView';
import { objectImage } from './objectView';
import { TEXT, createSiteView, drawCastle, updateSiteView } from './siteView';
import { hasDepth, isOnTower } from './viewRules';
import { ENEMY_SPRITES, PLAYER_SPRITES, TROOP_SPRITES, face, makeSprite, playAnim, spriteTop } from './sprites';
import type { Coin, Enemy, Pickup, Player, Projectile, ResourceNode, Site, Troop, World } from '../model/types';

/**
 * Zeichnet den Simulationszustand: Figuren als animierte Sprites (sprites.ts), Burg und Bauplätze über siteView.ts,
 * Ressourcen, Portal, Truhe und Münzen über objectView.ts. Hält pro Entität (id) ein Phaser-Objekt und legt an/entfernt sie.
 */

const U = UNIT_PX;
const G = GROUND_Y;

type View = Phaser.GameObjects.Container;
type Sprite = Phaser.GameObjects.Sprite;

/** Ab dieser Bewegung pro Frame (Pixel) gilt eine Figur als laufend */
const MOVING_PX = 0.5;

/**
 * Laufen/Stehen/Angriff aus der Bewegung und dem Angriffs-Cooldown ableiten. Ein Angriff setzt den
 * Cooldown hoch, das erkennt man am Sprung gegenüber dem letzten Frame. Gibt die Laufrichtung zurück.
 */
function animate(v: View, sprite: Sprite, x: number, cooldown: number | null): number {
  const dx = x - v.x;
  const moving = Math.abs(dx) > MOVING_PX;
  const prev = v.getData('cooldown') as number | undefined;
  v.setData('cooldown', cooldown);
  if (moving) face(sprite, Math.sign(dx));
  if (cooldown !== null && prev !== undefined && cooldown > prev + 1e-6) playAnim(sprite, 'attack', true);
  else playAnim(sprite, moving ? 'run' : 'idle');
  return moving ? Math.sign(dx) : 0;
}

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
    /** Ebene der Stufe: alles, was dieser Renderer zeichnet, liegt darin (je Kamera ein-/ausblendbar, S4.1) */
    private readonly root: Phaser.GameObjects.Layer,
  ) {
    this.drawStatic();
    this.castleHp = this.put(scene.add.container(world.castle.x * U, G - 300, bar(scene, 0, 240, 0x52b788)).setDepth(5));

    this.nodes = this.layer((n) => this.createNode(n), (v, n) => this.updateNode(v, n));
    this.sites = this.layer((s) => createSiteView(scene, s), (v, s) => updateSiteView(scene, v, s, this.world));
    this.pickups = this.layer((p) => this.createPickup(p), () => {});
    this.coins = this.layer((c) => scene.add.container(c.x * U, G - 8, [objectImage(scene, 'coin', 0, 8) ?? scene.add.circle(0, 0, 8, 0xffd166).setStrokeStyle(2, 0x9c6644)]).setDepth(6), () => {});
    this.troops = this.layer(
      (t) => this.createTroop(t),
      (v, t) => this.updateTroop(v, t),
      (v, t) => v.getData('kind') !== t.kind,
    );
    this.enemies = this.layer((e) => this.createEnemy(e), (v, e) => this.updateEnemy(v, e));
    this.players = this.layer((p) => this.createPlayer(p), (v, p) => this.updatePlayer(v, p));
    this.projectiles = this.layer(
      (p) => scene.add.container(p.x * U, G - 70, [scene.add.rectangle(0, 0, 26, 3, p.team === 'player' ? 0xffffff : 0xff4d6d)]).setDepth(9),
      (v, p) => v.setX(p.x * U),
    );
  }

  private put<T extends Phaser.GameObjects.GameObject>(o: T): T {
    this.root.add(o);
    return o;
  }

  private layer<T extends { id: number }>(create: (t: T) => View, update: (v: View, t: T) => void, rebuild?: (v: View, t: T) => boolean): Layer<T> {
    return new Layer((t: T) => this.put(create(t)), update, rebuild);
  }

  /** Kamera-Ziel für Spieler i */
  playerView(index: number): View | undefined {
    const p = this.world.players.find((p) => p.index === index);
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
    drawCastle(scene, world.castle.x * U, (o) => this.put(o));

    for (const e of world.level.entities) {
      const ex = e.x * U;
      if (e.kind === 'portal') {
        this.put(objectImage(scene, 'portal', ex, G) ?? scene.add.ellipse(ex, G - 110, 110, 220, 0x7b2cbf).setStrokeStyle(6, 0x240046));
      } else if (e.kind === 'exit') {
        this.put(objectImage(scene, 'exit', ex, G) ?? scene.add.rectangle(ex, G - 90, 160, 180, 0x111111).setStrokeStyle(6, 0x555555));
        const deeper = hasDepth(world.biome.depth + 1);
        this.put(scene.add.text(ex, G - 210, deeper ? t('exit.deeper', { depth: world.biome.depth + 1 }) : t('exit.blocked'), { ...TEXT, ...fontStyle('exitSign'), align: 'center' }).setOrigin(0.5));
      } else if (e.kind === 'bush') {
        this.put(objectImage(scene, 'node:bush', ex, G) ?? scene.add.circle(ex, G - 14, 18, 0x40916c)); // Deko
      } else if (e.kind === 'recruitCamp') {
        this.put(objectImage(scene, 'camp:recruit', ex, G) ?? scene.add.triangle(ex, G - 45, 0, 90, 60, 0, 120, 90, 0xe9c46a).setStrokeStyle(3, 0x7f5539));
        this.put(objectImage(scene, 'camp:recruit:fire', ex + 90, G) ?? scene.add.circle(ex + 90, G - 10, 14, 0xf77f00)); // Lagerfeuer
      }
    }
  }

  // ---------- Ressourcen ----------

  private createNode(n: ResourceNode): View {
    const s = this.scene;
    const parts: Phaser.GameObjects.GameObject[] = [];
    const sprite = objectImage(s, `node:${n.kind}`, 0, 0); // Grafik aus der Zuordnung (GR3.2), sonst Platzhalter-Form
    if (sprite) parts.push(sprite);
    else if (n.kind === 'tree') {
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

  // ---------- Truppen ----------

  private createTroop(t: Troop): View {
    const s = this.scene;
    const sprite = makeSprite(s, TROOP_SPRITES[t.kind]);
    const top = spriteTop(sprite);
    const load = s.add.rectangle(0, top * 0.55, 16, 24, 0x6b4226).setStrokeStyle(2, 0x3b2414).setVisible(false);
    return s.add.container(t.x * U, G, [sprite, load, ...bar(s, top - 16, 36, 0x52b788)]).setDepth(7).setData('kind', t.kind);
  }

  private updateTroop(v: View, t: Troop): void {
    const sprite = v.getAt(0) as Sprite;
    const x = t.x * U;
    // Nur Kämpfer nutzen den Cooldown für Angriffe, bei Landstreichern ist es die Wartezeit beim Umherwandern
    const dir = animate(v, sprite, x, t.kind === 'archer' ? t.cooldown : null);
    if (t.kind === 'archer' && dir === 0) face(sprite, this.nearestEnemyDir(t.x));
    // Bauer bei der Arbeit (Holz hacken, bauen): Schlag-Animation in Schleife
    const working = t.kind === 'peasant' && dir === 0 && (t.job?.type === 'gather' || t.job?.type === 'build') && Math.abs(t.targetX - t.x) < 0.5;
    if (working) playAnim(sprite, 'attack');
    v.setPosition(x, isOnTower(this.world, t) ? G - 270 : G);
    const load = v.getAt(1) as Phaser.GameObjects.Rectangle;
    load.setVisible(t.job?.type === 'carry').setX(sprite.flipX ? 14 : -14);
    setBar(v, 2, t.hp / t.maxHp, t.hp < t.maxHp);
  }

  /** Richtung zum nächsten Gegner (0 = keiner da) */
  private nearestEnemyDir(x: number): number {
    let best: Enemy | null = null;
    for (const e of this.world.enemies) if (!best || Math.abs(e.x - x) < Math.abs(best.x - x)) best = e;
    return best ? Math.sign(best.x - x) : 0;
  }

  // ---------- Gegner ----------

  private createEnemy(e: Enemy): View {
    const s = this.scene;
    const sprite = makeSprite(s, ENEMY_SPRITES[e.kind] ?? ENEMY_SPRITES.goblin);
    const top = spriteTop(sprite);
    const loot = s.add.circle(0, top - 12, 10, 0xffd166).setStrokeStyle(2, 0x9c6644).setVisible(false);
    return s.add.container(e.x * U, G, [sprite, loot, ...bar(s, top - 32, 44, 0xe63946)]).setDepth(8);
  }

  private updateEnemy(v: View, e: Enemy): void {
    animate(v, v.getAt(0) as Sprite, e.x * U, e.cooldown);
    v.setX(e.x * U);
    (v.getAt(1) as Phaser.GameObjects.Arc).setVisible(e.carriedGold > 0);
    setBar(v, 2, e.hp / e.maxHp, e.hp < e.maxHp);
    v.setAlpha(e.fleeing ? 0.7 : 1);
  }

  // ---------- Pickups ----------

  private createPickup(p: Pickup): View {
    const s = this.scene;
    const chest = p.kind === 'chest';
    const shape = objectImage(s, `pickup:${p.kind}`, 0, chest ? 0 : -44) ?? (chest ? s.add.rectangle(0, -18, 44, 36, 0x9c6644).setStrokeStyle(3, 0x5c3d2e) : s.add.star(0, -60, 5, 8, 18, 0xffd60a).setStrokeStyle(2, 0xffffff));
    return s.add.container(p.x * U, G, [shape]).setDepth(3);
  }

  // ---------- Monarchen ----------

  private createPlayer(p: Player): View {
    const s = this.scene;
    // Beritten (mountView) oder, bei unbekanntem Reittier, die bisherige Figur
    const sprite = createRider(s, p) ?? makeSprite(s, PLAYER_SPRITES[p.index % PLAYER_SPRITES.length]);
    const top = sprite.getData('top') as number;
    // Farbiger Punkt unter den Füßen: welcher Monarch gehört zu wem
    const color = PLAYER_COLORS[p.index % PLAYER_COLORS.length];
    const marker = s.add.ellipse(0, 4, 56, 12, color, 0.8);
    const purse = s.add.text(0, top - 10, '', { ...TEXT, ...fontStyle('purse') }).setOrigin(0.5, 1);
    return s.add.container(p.x * U, G, [marker, sprite, purse, ...bar(s, top - 50, 50, 0x52b788)]).setDepth(10);
  }

  private updatePlayer(v: View, p: Player): void {
    const fig = v.getAt(1) as Sprite | Phaser.GameObjects.Container;
    if (fig instanceof Phaser.GameObjects.Container) updateRider(fig, p);
    else {
      face(fig, p.facing);
      playAnim(fig, Math.abs(p.vx) > 0.05 ? 'run' : 'idle');
    }
    v.setPosition(p.x * U, G);
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

export { resourceName, siteName } from './siteView';
