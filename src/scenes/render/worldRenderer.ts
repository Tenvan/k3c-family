import Phaser from 'phaser';
import { GROUND_Y, PLAYER_COLORS, UNIT_PX } from '../../core/constants';
import { t } from '../../core/texts';
import { fontStyle } from './fontRules';
import { type BackWall, backWalls } from './backWallRules';
import { createBackWallView, updateBackWallView } from './backWallView';
import { BuergerView } from './citizenView';
import { createRider, updateRider } from './mountView';
import { campSprite, nodeSprite } from './natureSprites';
import { objectImage } from './objectView';
import { PENNANT_MAX_HEIGHT, markerOf, noteMarkers, pennantColor } from './pennant';
import { type Plot, isPlantationTree, plantationPlots } from './plantation';
import { priceTagSites } from './priceTagRules';
import { TEXT, createCastleView, createSiteView, updateCastleView, updateSiteView } from './siteView';
import { hasDepth, isOnTower } from './viewRules';
import { type MonarchFrame, pickMonarchAnim } from './monarchAnim';
import { type AnimName, ENEMY_SPRITES, PLAYER_SPRITES, TROOP_SPRITES, face, hasAnim, makeSprite, playAnim, spriteTop } from './sprites';
import { CHEST_OPEN } from './worldSprites';
import { WarnView } from '../kampfView';
import type { Coin, Enemy, GameEvent, Pickup, Player, Projectile, ResourceNode, Site, Troop, World } from '../../model/types';

/**
 * Zeichnet den Simulationszustand: Figuren als animierte Sprites (sprites.ts), Burg und Bauplätze über siteView.ts,
 * Ressourcen, Portal, Truhe und Münzen über objectView.ts. Hält pro Entität (id) ein Phaser-Objekt und legt an/entfernt sie.
 */

const U = UNIT_PX;
const G = GROUND_Y;
/** Setzling steht vor der Farm (Bauplätze: Tiefe 2); der hohe Plantage-Baum steht wie der Wald dahinter (GEB10) */
const PLANTATION_DEPTH = 3;
/** Bildhöhe, auf die Fahne und Balken ohne Grafik zielen (Platzhalter-Form) */
const PLACEHOLDER_NODE_HEIGHT = 158;
/** Abstand der Ausgang-Beschriftung über der Oberkante des Ausgangs */
const SIGN_GAP = 24;

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
    /** Abschied, wenn die Entität fehlt (Standard: sofort weg) */
    private readonly remove: (v: View) => void = (v) => v.destroy(),
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
        this.remove(view);
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
  /** leere Plantage-Plätze (Setzling, GEB5) */
  private readonly plots: Layer<Plot>;
  private readonly backWalls: Layer<BackWall>;
  private readonly projectiles: Layer<Projectile>;
  private readonly pickups: Layer<Pickup>;
  private readonly castle: View;
  private readonly castleHp: View;
  private readonly citizens: BuergerView;
  private readonly warn: WarnView;
  /** Bauplätze, deren Preisschild steht (nur das nächste je Spieler, B-395) */
  private priceTags = new Set<number>();
  /** Knoten-ID → Spieler, der zuletzt eine Markier-Münze gab (Wimpel-Farbe, GEB10) */
  private readonly markers = new Map<number, number>();

  constructor(
    private readonly scene: Phaser.Scene,
    private world: World,
    /** Ebene der Stufe: alles, was dieser Renderer zeichnet, liegt darin (je Kamera ein-/ausblendbar, S4.1) */
    private readonly root: Phaser.GameObjects.Layer,
  ) {
    this.castle = this.put(createCastleView(scene, world.castle.x * U));
    updateCastleView(scene, this.castle, world);
    this.drawStatic();
    // über der Burg (288 px hoch, GEB4)
    this.castleHp = this.put(scene.add.container(world.castle.x * U, G - 310, bar(scene, 0, 240, 0x52b788)).setDepth(5));

    this.citizens = new BuergerView(scene, root);
    this.warn = new WarnView(scene, root);
    this.nodes = this.layer((n) => this.createNode(n), (v, n) => this.updateNode(v, n));
    this.sites = this.layer((s) => createSiteView(scene, s), (v, s) => updateSiteView(scene, v, s, this.world, this.priceTags.has(s.id)));
    this.plots = this.layer((p) => this.createPlot(p), () => {});
    this.backWalls = this.layer(() => createBackWallView(scene), (v, b) => updateBackWallView(scene, v, b));
    this.pickups = this.layer((p) => this.createPickup(p), () => {}, undefined, (v) => this.openChest(v));
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

  private layer<T extends { id: number }>(create: (t: T) => View, update: (v: View, t: T) => void, rebuild?: (v: View, t: T) => boolean, remove?: (v: View) => void): Layer<T> {
    return new Layer((t: T) => this.put(create(t)), update, rebuild, remove);
  }

  /** Ereignisse eines Frames dieser Stufe: wer welchen Knoten markiert (Wimpel-Farbe, GEB10) */
  noteEvents(events: readonly GameEvent[]): void {
    noteMarkers(this.markers, this.world.nodes, events);
  }

  /** Kamera-Ziel für Spieler i */
  playerView(index: number): View | undefined {
    const p = this.world.players.find((p) => p.index === index);
    return p ? this.players.views.get(p.id) : undefined;
  }

  /** Der Monarch von Spieler `id` holt aus (S9.7, B-396): Angriffs-Animation der Figur bzw. des Reiters, läuft zu Ende, dann wieder Stehen/Laufen */
  swing(id: number): void {
    const sprite = monarchSprite(this.players.views.get(id));
    if (sprite) playAnim(sprite, 'attack', true);
  }

  /** Monarch von Spieler-Index `index` gibt eine Münze (`coinGive`, GR9.3): Wurf-Animation beim nächsten Zeichnen */
  give(index: number): void {
    const p = this.world.players.find((q) => q.index === index);
    this.players.views.get(p?.id ?? -1)?.setData('cue', 'give');
  }

  /** `hidden`: Index freier oder wartender Monarchen (kein Spieler steuert sie), sie werden nicht gezeichnet (B-411). */
  sync(world: World, hidden: ReadonlySet<number> = new Set()): void {
    this.world = world;
    this.priceTags = priceTagSites(world);
    this.nodes.sync(world.nodes);
    this.sites.sync(world.sites);
    this.plots.sync(plantationPlots(world.sites, world.nodes).filter((p) => p.tree === null));
    this.backWalls.sync(backWalls(world.sites, world.hubX));
    this.pickups.sync(world.pickups);
    this.coins.sync(world.coins);
    this.troops.sync(world.troops);
    this.enemies.sync(world.enemies);
    this.players.sync(hidden.size === 0 ? world.players : world.players.filter((p) => !hidden.has(p.index)));
    this.projectiles.sync(world.projectiles);
    this.citizens.sync(world);
    this.warn.sync(world);
    updateCastleView(this.scene, this.castle, world); // Burg wächst mit Hub-Stufe und Ausbau-Stand (GEB4 AC-05, GEB11)
    setBar(this.castleHp, 0, world.castle.hp / world.castle.maxHp, world.castle.hp < world.castle.maxHp);
  }

  // ---------- statische Objekte ----------

  private drawStatic(): void {
    const { scene, world } = this;
    for (const e of world.level.entities) {
      const ex = e.x * U;
      if (e.kind === 'portal') {
        this.put(objectImage(scene, 'portal', ex, G) ?? scene.add.ellipse(ex, G - 110, 110, 220, 0x7b2cbf).setStrokeStyle(6, 0x240046));
      } else if (e.kind === 'exit') {
        const door = objectImage(scene, 'exit', ex, G) ?? scene.add.rectangle(ex, G - 90, 160, 180, 0x111111).setStrokeStyle(6, 0x555555);
        this.put(door);
        const deeper = hasDepth(world.biome.depth + 1);
        this.put(scene.add.text(ex, G - door.displayHeight - SIGN_GAP, deeper ? t('exit.deeper', { depth: world.biome.depth + 1 }) : t('exit.blocked'), { ...TEXT, ...fontStyle('exitSign'), align: 'center' }).setOrigin(0.5));
      } else if (e.kind === 'bush') {
        this.put(objectImage(scene, 'node:bush', ex, G, nodeSprite('bush', e.x, world.biome.id)) ?? scene.add.circle(ex, G - 14, 18, 0x40916c)); // Deko
      } else if (e.kind === 'recruitCamp') {
        this.put(objectImage(scene, 'camp:recruit', ex, G, campSprite(world.biome.id)) ?? scene.add.triangle(ex, G - 45, 0, 90, 60, 0, 120, 90, 0xe9c46a).setStrokeStyle(3, 0x7f5539));
      }
    }
  }

  // ---------- Ressourcen ----------

  /** Setzling auf einem leeren Plantage-Platz, vor der Farm; ohne geladene Grafik bleibt der Platz leer */
  private createPlot(p: Plot): View {
    const image = this.scene.textures.exists('grafik:setzling') ? [this.scene.add.image(0, 0, 'grafik:setzling').setOrigin(0.5, 1)] : [];
    return this.scene.add.container(p.x * U, G, image).setDepth(PLANTATION_DEPTH);
  }

  private createNode(n: ResourceNode): View {
    const s = this.scene;
    const parts: Phaser.GameObjects.GameObject[] = [];
    // Plantage-Baum (steht auf einem Platz): eigene niedrige Pappel vor der Farm (B-568); Waldbäume bleiben bei der Zuordnung (GEB6)
    const pappel = isPlantationTree(n, this.world.sites) && s.textures.exists('grafik:pappel-niedrig');
    // Baumform aus der Position, Look aus dem Biom (GEB6); unbekannte Art oder fehlende Grafik: Platzhalter-Form
    const sprite = pappel ? s.add.image(0, 0, 'grafik:pappel-niedrig').setOrigin(0.5, 1) : objectImage(s, `node:${n.kind}`, 0, 0, nodeSprite(n.kind, n.x, this.world.biome.id));
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
    // Wimpel und Balken sitzen über der Bildhöhe, höchstens auf Baumhöhe (Wurzeln reichen bis an den Bildrand, GEB10)
    const top = -Math.min(sprite ? sprite.displayHeight : PLACEHOLDER_NODE_HEIGHT, PENNANT_MAX_HEIGHT);
    const pole = s.add.rectangle(10, top - 2, 3, 30, 0xffffff);
    const pennant = s.add.triangle(11, top - 12, 0, 0, 26, 7, 0, 14, pennantColor(undefined)).setOrigin(0, 0.5).setStrokeStyle(1, 0x000000, 0.5);
    const view = s.add.container(n.x * U, G, [...parts, pole, pennant, ...bar(s, top - 32, 50, 0xffd166)]);
    return pappel ? view.setDepth(PLANTATION_DEPTH) : view; // vor der Farm (Tiefe 2), wie der Setzling
  }

  private updateNode(v: View, n: ResourceNode): void {
    const i = v.length - 4;
    (v.getAt(i) as Phaser.GameObjects.Rectangle).setVisible(n.marked);
    (v.getAt(i + 1) as Phaser.GameObjects.Triangle).setVisible(n.marked).setFillStyle(pennantColor(markerOf(n, this.markers)));
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

  /** Aufgehobene Truhe: kurz offen, dann ausblenden (GEB10); andere Pickups und fehlende Grafik verschwinden sofort */
  private openChest(v: View): void {
    const image = v.getAt(0);
    const open = image instanceof Phaser.GameObjects.Sprite ? CHEST_OPEN[image.texture.key] : undefined;
    if (!open || !this.scene.textures.exists(open)) {
      v.destroy();
      return;
    }
    (image as Phaser.GameObjects.Sprite).setTexture(open);
    this.scene.tweens.add({ targets: v, alpha: 0, delay: 300, duration: 500, onComplete: () => v.destroy() });
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
    return s.add.container(p.x * U, G, [marker, sprite, ...bar(s, top - 50, 50, 0x52b788)]).setDepth(10); // Gold steht nur im HUD (B-398)
  }

  private updatePlayer(v: View, p: Player): void {
    const fig = v.getAt(1) as Sprite | Phaser.GameObjects.Container;
    const sprite = monarchSprite(v);
    const cur: MonarchFrame = { respawnIn: p.respawnIn, hp: p.hp, moving: Math.abs(p.vx) > 0.05 };
    // Beritten bleibt der Tod beim Rückfall (Deckkraft): der gekippte Reiter läge im weggeschnittenen Teil unter dem Sattel
    const mounted = fig instanceof Phaser.GameObjects.Container;
    const has = (a: AnimName) => sprite !== undefined && hasAnim(sprite, a) && !(mounted && a === 'death');
    const pick = pickMonarchAnim(v.getData('monarch') as MonarchFrame | undefined, cur, v.getData('cue') === 'give' ? 'give' : null, has);
    v.setData('monarch', cur).setData('cue', null);
    if (fig instanceof Phaser.GameObjects.Container) updateRider(fig, p, pick.anim, pick.restart);
    else {
      face(fig, p.facing);
      playAnim(fig, pick.anim, pick.restart);
    }
    v.setPosition(p.x * U, G);
    v.setAlpha(pick.alpha);
    if (p.stealthFor) v.setAlpha(Math.min(pick.alpha, 0.35)); // Smoke Bomb: fast durchsichtig (B-577)

    setBar(v, 2, p.hp / p.maxHp, p.hp < p.maxHp && p.respawnIn <= 0);
  }
}

/** Figur eines Monarchen (zu Fuß) bzw. der Reiter-Sprite (beritten, Index 1 im Reiter-Container) */
function monarchSprite(v: View | undefined): Sprite | undefined {
  const fig = v?.getAt(1);
  const sprite = fig instanceof Phaser.GameObjects.Container ? fig.getAt(1) : fig;
  return sprite instanceof Phaser.GameObjects.Sprite ? sprite : undefined;
}

export { resourceName, siteName } from './siteView';
