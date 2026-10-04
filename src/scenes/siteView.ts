import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '../core/constants';
import { currentLanguage, nameOf, t } from '../core/texts';
import { BUILDINGS, TROOPS } from '../model/data';
import type { Site, World } from '../model/types';
import { BUILDING_TEXTURES, STAIRS_UP_TILES, hubTexture, siteTexture } from './buildingSprites';
import { fontStyle } from './fontRules';
import { canAfford } from './viewRules';

/**
 * Burg und Bauplätze (GR3.1): zugeordnete Grafik, sonst die Platzhalter-Form. Preisschild, Münz-Slots und Baufortschritt
 * bleiben Zeichnungen darüber. Zeichnet nur.
 */

type View = Phaser.GameObjects.Container;
type Picture = Phaser.GameObjects.Image | Phaser.GameObjects.TileSprite | Phaser.GameObjects.Container;

const G = GROUND_Y;
/** Pixel-Art ganzzahlig ×2 (Q13: 16-px-Raster auf UNIT_PX = 32) */
const ZOOM = 2;
const PRICE_TAG_RANGE = 6;
export const TEXT = { stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' };

const SITE_SIZE: Record<Site['kind'], [number, number]> = {
  wall: [36, 150],
  tower: [70, 260],
  gate: [40, 160],
  workshop: [150, 120],
  storage: [130, 100],
  stairsUp: [110, 110],
  stairsDown: [110, 110],
  farm: [140, 90],
  barracks: [150, 130],
  tavern: [130, 120],
  healer: [100, 100],
  smithy: [120, 110],
  armory: [130, 120],
};
const SITE_COLOR: Record<Site['kind'], number> = {
  wall: 0x8d99ae,
  tower: 0x9c6644,
  gate: 0x7f5539,
  workshop: 0xbc6c25,
  storage: 0x7f5539,
  stairsUp: 0x6c757d,
  stairsDown: 0x343a40,
  farm: 0xa7c957,
  barracks: 0x6c584c,
  tavern: 0xb08968,
  healer: 0xe5e5e5,
  smithy: 0x495057,
  armory: 0x5c677d,
};

/** Ausschnitte aus Kachelsätzen (Pixel im Bild): Burg aus dem Fort-Tileset, Verlies-Eingang aus dem Dungeon-Bild */
const FRAMES: Record<string, [string, number, number, number, number][]> = {
  'grafik:fort-tileset': [['column', 0, 16, 32, 96], ['wall', 48, 48, 96, 64], ['merlon', 48, 16, 32, 16]],
  'grafik:stairsDown': [['entrance', 96, 0, 176, 112]],
};

/** Nur die zugeordneten Gebäude-Grafiken laden (LoadScene.preload) */
export function preloadBuildings(scene: Phaser.Scene): void {
  for (const [key, file] of Object.entries(BUILDING_TEXTURES)) scene.load.image(key, `grafik/${file}`);
}

/** Nach dem Laden: Pixel-Art scharf und Ausschnitte als Frames anlegen */
export function prepareBuildings(scene: Phaser.Scene): void {
  for (const key of Object.keys(BUILDING_TEXTURES)) {
    if (!scene.textures.exists(key)) continue;
    const texture = scene.textures.get(key);
    texture.setFilter(Phaser.Textures.FilterMode.NEAREST);
    for (const [name, x, y, w, h] of FRAMES[key] ?? []) if (!texture.has(name)) texture.add(name, 0, x, y, w, h);
  }
}

const has = (scene: Phaser.Scene, key: string | null): key is string => key !== null && scene.textures.exists(key);

/** Burg (Hub-Stufe 1): Säulen links/rechts, Mauer mit Zinnen, Banner. Ohne Grafik die bisherige Form. */
export function drawCastle(scene: Phaser.Scene, x: number, put: (o: Phaser.GameObjects.GameObject) => void): void {
  const key = hubTexture();
  if (!has(scene, key)) {
    put(scene.add.rectangle(x, G - 130, 300, 260, 0x6c757d).setStrokeStyle(4, 0x343a40));
    put(scene.add.rectangle(x, G - 40, 60, 80, 0x343a40));
    for (const dx of [-120, -40, 40, 120]) put(scene.add.rectangle(x + dx, G - 275, 40, 30, 0x6c757d).setStrokeStyle(4, 0x343a40));
    return;
  }
  const part = (px: number, py: number, frame: string): void => put(scene.add.image(x + px, py, key, frame).setOrigin(0.5, 1).setScale(ZOOM));
  part(-128, G, 'column');
  part(128, G, 'column');
  part(0, G, 'wall');
  for (const dx of [-64, 0, 64]) part(dx, G - 128, 'merlon');
  if (has(scene, 'grafik:fort-banner')) put(scene.add.image(x, G - 120, 'grafik:fort-banner').setOrigin(0.5, 0).setScale(ZOOM));
}

export function createSiteView(scene: Phaser.Scene, s: Site): View {
  const [w, h] = SITE_SIZE[s.kind];
  // Mauer: 16-px-Kachel gestapelt, sonst ein Bild
  const picture: Picture = s.kind === 'stairsUp' ? stairsUp(scene) : s.kind === 'wall' ? scene.add.tileSprite(0, 0, 16, 80, '__DEFAULT').setOrigin(0.5, 1) : scene.add.image(0, 0, '__DEFAULT').setOrigin(0.5, 1);
  picture.setScale(ZOOM).setVisible(false);
  const g = scene.add.graphics();
  const label = scene.add.text(0, 0, '', { ...TEXT, ...fontStyle('priceTag') }).setOrigin(0.5, 1);
  return scene.add.container(s.x * UNIT_PX, G, [picture, g, label]).setDepth(2).setData('size', [w, h]);
}

/** Treppe hoch aus den Kacheln (je 16 nach rechts und 16 höher); fehlt eine Kachel, bleibt sie leer → Platzhalter */
function stairsUp(scene: Phaser.Scene): Phaser.GameObjects.Container {
  if (!STAIRS_UP_TILES.every((key) => scene.textures.exists(key))) return scene.add.container(0, 0);
  const mid = (STAIRS_UP_TILES.length - 1) / 2;
  return scene.add.container(0, 0, STAIRS_UP_TILES.map((key, i) => scene.add.image((i - mid) * 16, -i * 16, key).setOrigin(0.5, 1)));
}

/** Zeigt die Grafik, falls zugeordnet und geladen; liefert die Höhe für Schilder darüber (sonst Platzhalter-Höhe) */
function showPicture(scene: Phaser.Scene, v: View, s: Site): number | null {
  const picture = v.getAt(0) as Picture;
  const key = siteTexture(s.kind, s.state);
  const frame = key === null ? undefined : FRAMES[key]?.[0]?.[0];
  const empty = picture instanceof Phaser.GameObjects.Container && picture.length === 0;
  if (!has(scene, key) || empty) {
    picture.setVisible(false);
    return null;
  }
  if (picture instanceof Phaser.GameObjects.Container) {
    picture.setVisible(true);
    return picture.getBounds().height;
  }
  picture.setTexture(key, frame).setVisible(true);
  return picture.displayHeight;
}

export function updateSiteView(scene: Phaser.Scene, v: View, s: Site, world: World): void {
  const near = world.players.some((p) => p.respawnIn <= 0 && Math.abs(p.x - s.x) < PRICE_TAG_RANGE);
  const data = BUILDINGS[s.kind];
  const key = [s.state, s.paidGold, Math.round(s.buildProgress * 20), Math.round((s.hp / s.maxHp) * 20), s.bows, s.bowPaidGold, near, canAfford(world.stock, data.cost), currentLanguage()].join('|');
  if (v.getData('key') === key) return;
  v.setData('key', key);

  const g = v.getAt(1) as Phaser.GameObjects.Graphics;
  const label = v.getAt(2) as Phaser.GameObjects.Text;
  g.clear();
  label.setText('');
  const [w, size] = v.getData('size') as [number, number];
  const picture = showPicture(scene, v, s);
  const h = picture ?? size;

  if (s.state === 'built') {
    if (picture === null) drawBuiltPlaceholder(g, s, w, h);
    drawBuiltSigns(g, label, s, world, h, near);
    return;
  }

  // Noch nicht gebaut: Umriss + Preisschild bzw. Baufortschritt
  g.lineStyle(3, 0xffffff, 0.5).strokeRect(-w / 2, -h, w, h);
  if (s.buildProgress > 0) {
    g.fillStyle(0xdda15e, 0.8).fillRect(-w / 2, -h * s.buildProgress, w, h * s.buildProgress);
  }
  if (s.state === 'unpaid' && near) drawPrice(g, label, world, -h - 20, data.cost.gold ?? 0, s.paidGold, siteName(s.kind), data.cost);
  if (s.state === 'waitingMaterial') label.setPosition(0, -h - 20).setText(`${siteName(s.kind)}: ${missing(world.stock, data.cost)}`).setColor('#ff8fa3');
  if (s.state === 'waitingWorker') label.setPosition(0, -h - 20).setText(s.buildProgress > 0 ? '' : t('site.waitingWorker', { name: siteName(s.kind) })).setColor('#ffffff');
}

/** Über einem gebauten Gebäude: Treppen-Schild, Lebensbalken, Bögen und Bogen-Preis der Werkstatt */
function drawBuiltSigns(g: Phaser.GameObjects.Graphics, label: Phaser.GameObjects.Text, s: Site, world: World, h: number, near: boolean): void {
  if (s.kind === 'stairsUp' || s.kind === 'stairsDown') label.setPosition(0, -h - 12).setText(siteName(s.kind)).setColor('#ffd166');
  if (s.hp < s.maxHp) drawBar(g, -h - 30, 80, s.hp / s.maxHp, 0x52b788);
  if (s.kind !== 'workshop') return;
  for (let i = 0; i < s.bows; i++) g.lineStyle(4, 0xffd166).beginPath().arc(-40 + i * 30, -60, 14, -Math.PI / 2, Math.PI / 2).strokePath();
  const price = TROOPS.archer.cost ?? {};
  if (near && s.bows < (BUILDINGS.workshop.bowRack ?? 0)) drawPrice(g, label, world, -h - 20, price.gold ?? 0, s.bowPaidGold, t('site.bow'), price);
}

/** Platzhalter-Form eines gebauten Gebäudes ohne Grafik */
function drawBuiltPlaceholder(g: Phaser.GameObjects.Graphics, s: Site, w: number, h: number): void {
  g.fillStyle(SITE_COLOR[s.kind]).fillRect(-w / 2, -h, w, h).lineStyle(4, 0x343a40).strokeRect(-w / 2, -h, w, h);
  if (s.kind === 'tower') g.fillStyle(0x6f4518).fillRect(-w / 2 - 15, -h - 10, w + 30, 14);
  if (s.kind === 'stairsUp' || s.kind === 'stairsDown') {
    // Stufen als Treppe
    for (let i = 0; i < 5; i++) g.fillStyle(0x495057).fillRect(-w / 2 + i * (w / 5), -((i + 1) * h) / 5, w / 5, ((i + 1) * h) / 5);
  }
}

/** Münz-Slots (K2C): gefüllt = bezahlt. Baumaterial steht darüber. */
function drawPrice(g: Phaser.GameObjects.Graphics, label: Phaser.GameObjects.Text, world: World, y: number, total: number, paid: number, name: string, cost: Record<string, number | undefined>): void {
  const perRow = 10;
  for (let i = 0; i < total; i++) {
    const row = Math.floor(i / perRow);
    const inRow = Math.min(perRow, total - row * perRow);
    const cx = (i % perRow) * 22 - ((inRow - 1) * 22) / 2;
    const cy = y - row * 22;
    g.lineStyle(2, 0xffd166).strokeCircle(cx, cy, 8);
    if (i < paid) g.fillStyle(0xffd166).fillCircle(cx, cy, 8);
  }
  const material = (['wood', 'stone', 'copper'] as const).filter((r) => cost[r]).map((r) => `${cost[r]} ${resourceName(r)}`);
  const enough = canAfford(world.stock, cost);
  label.setPosition(0, y - Math.ceil(total / perRow) * 22).setText([name, ...material].join(' · ')).setColor(enough ? '#ffffff' : '#ff8fa3');
}

function drawBar(g: Phaser.GameObjects.Graphics, y: number, width: number, fraction: number, color: number): void {
  g.fillStyle(0x000000, 0.6).fillRect(-width / 2, y, width, 8);
  g.fillStyle(color).fillRect(-width / 2, y, width * Phaser.Math.Clamp(fraction, 0, 1), 8);
}

export const resourceName = (r: 'wood' | 'stone' | 'copper'): string => nameOf('res', r, r);
export const siteName = (kind: Site['kind']): string => nameOf('site', kind, BUILDINGS[kind].name);

function missing(stock: World['stock'], cost: Record<string, number | undefined>): string {
  const parts = (['wood', 'stone', 'copper'] as const)
    .filter((r) => (cost[r] ?? 0) > stock[r])
    .map((r) => t('site.missing', { n: (cost[r] ?? 0) - stock[r], res: resourceName(r) }));
  return parts.join(', ') || t('site.ready');
}
