import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '../../core/constants';
import { currentLanguage, nameOf, t } from '../../core/texts';
import { BUILDINGS, TROOPS } from '../../model/data';
import type { Site, World } from '../../model/types';
import { drawBuildSign } from './buildSignView';
import { BUILDING_TEXTURES, HUB_SCAFFOLD_TEXTURE, hubLookTexture, hubTexture, isHubBuilding, siteTexture, textureScale } from './buildingSprites';
import { damageStage, showRubble } from './damageRules';
import { damageTint, drawDamage, drawRubble } from './damageView';
import { fontStyle } from './fontRules';
import { hubLookOf } from './hubGrowth';
import { offersHubUpgrade } from './priceTagRules';
import { buildCost, hubOfferTitle, hubWait } from '../hud/economyLabels';
import { drawScaffoldShape, scaffoldTexture, siteMarker } from './scaffoldView';
import { drawSiteStatus, siteStatus, statusKey } from './siteStatusView';
import { canAfford } from './viewRules';

/**
 * Burg und Bauplätze (GR3.1): zugeordnete Grafik, sonst die Platzhalter-Form. Preisschild, Münz-Slots und Baufortschritt
 * bleiben Zeichnungen darüber. Zeichnet nur.
 */

type View = Phaser.GameObjects.Container;

const G = GROUND_Y;
export const TEXT = { stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' };

export const SITE_SIZE: Record<Site['kind'], [number, number]> = {
  wall: [36, 150],
  tower: [70, 260],
  workshop: [150, 120],
  storage: [130, 100],
  stairsUp: [110, 110],
  stairsDown: [110, 110],
  nursery: [140, 90],
  barracks: [150, 130],
  tavern: [130, 120],
  healer: [100, 100],
  smithy: [120, 110],
  armory: [130, 120],
  ship: [190, 160], // Platzhalter-Größe wie die Grafik `schiff-*` (6 × 5 Kacheln, BS10)
};
const SITE_COLOR: Record<Site['kind'], number> = {
  wall: 0x8d99ae,
  tower: 0x9c6644,
  workshop: 0xbc6c25,
  storage: 0x7f5539,
  stairsUp: 0x6c757d,
  stairsDown: 0x343a40,
  nursery: 0xa7c957,
  barracks: 0x6c584c,
  tavern: 0xb08968,
  healer: 0xe5e5e5,
  smithy: 0x495057,
  armory: 0x5c677d,
  ship: 0x4a6fa5,
};

/** Nur die zugeordneten Gebäude-Grafiken laden (LoadScene.preload) */
export function preloadBuildings(scene: Phaser.Scene): void {
  for (const [key, file] of Object.entries(BUILDING_TEXTURES)) scene.load.image(key, `grafik/${file}`);
}

/** Nach dem Laden: Pixel-Art scharf */
export function prepareBuildings(scene: Phaser.Scene): void {
  for (const key of Object.keys(BUILDING_TEXTURES)) if (scene.textures.exists(key)) scene.textures.get(key).setFilter(Phaser.Textures.FilterMode.NEAREST);
}

const has = (scene: Phaser.Scene, key: string | null): key is string => key !== null && scene.textures.exists(key);

/** Burg (GEB4, GEB11): Bild des Looks und Baugerüst davor, Unterkante am Boden, mittig auf `x`; darüber das Ausbau-Schild (B-591) */
export function createCastleView(scene: Phaser.Scene, x: number): View {
  const picture = scene.add.image(0, 0, '__DEFAULT').setOrigin(0.5, 1).setVisible(false);
  const scaffold = scene.add.image(0, 0, '__DEFAULT').setOrigin(0.5, 1).setVisible(false);
  const label = scene.add.text(0, 0, '', { ...TEXT, ...fontStyle('priceTag') }).setOrigin(0.5, 1);
  return scene.add.container(x, G, [picture, scene.add.graphics(), scaffold, scene.add.graphics(), label]);
}

type CastleWorld = Pick<World, 'hubLevel' | 'hubUpgrade' | 'danger' | 'stock'> & Partial<Pick<World, 'hubStep'>>;

/**
 * Ausbau-Schild über der Burg (B-591): `near` (nächstes Preisschild eines Spielers, `priceTagRules.ts`) zeigt Gold und
 * Material aus `hubUpgrade`; läuft der Ausbau schon (`hubUpgrade.state`), steht der Wartegrund statt des Angebots.
 * Titel mit dem Ausbauschritt („Ausbau 2/3“, `hubStep`, B-592), ohne Schritt „Ausbauen“.
 */
function updateCastleSign(v: View, world: CastleWorld, near: boolean): void {
  const up = world.hubUpgrade;
  const wait = hubWait(world, false);
  const picture = v.getAt(0) as Phaser.GameObjects.Image;
  const h = picture.visible ? picture.displayHeight : 290;
  const title = hubOfferTitle(world);
  const key = [near, up?.gold, up?.paid, title, statusKey(wait), canAfford(world.stock, up?.material ?? {}), currentLanguage(), h].join('|');
  if (v.getData('sign') === key) return;
  v.setData('sign', key);
  const g = (v.getAt(3) as Phaser.GameObjects.Graphics).clear();
  const label = (v.getAt(4) as Phaser.GameObjects.Text).setText('');
  if (wait) drawSiteStatus(g, label, wait, title, -h - 20);
  else if (up && near && offersHubUpgrade(world)) drawPrice(g, label, world, -h - 20, up.gold, up.paid, title, up.material);
}

/**
 * Tauscht die Burg-Grafik, wenn sich der Look der Wachstumsreihe ändert (`hubLookOf`: Hub-Stufe und Ausbauschritt, GEB17.6); fehlt die
 * Grafik des Looks, die Burg der Hub-Stufe, sonst die bisherige Form. Das Gerüst steht, solange ein Bauer den Ausbau baut.
 * Danach das Ausbau-Schild (`near`: Preisschild steht, B-591).
 */
export function updateCastleView(scene: Phaser.Scene, v: View, world: CastleWorld, near = false): void {
  drawCastle(scene, v, world);
  updateCastleSign(v, world, near);
}

function drawCastle(scene: Phaser.Scene, v: View, world: CastleWorld): void {
  const look = hubLookOf(world);
  const stage = `${look.id}-${look.stufe}-${look.scaffold}`;
  if (v.getData('stage') === stage) return;
  v.setData('stage', stage);
  const picture = v.getAt(0) as Phaser.GameObjects.Image;
  const g = (v.getAt(1) as Phaser.GameObjects.Graphics).clear();
  const scaffold = v.getAt(2) as Phaser.GameObjects.Image;
  if (look.scaffold && has(scene, HUB_SCAFFOLD_TEXTURE)) scaffold.setTexture(HUB_SCAFFOLD_TEXTURE).setScale(textureScale(HUB_SCAFFOLD_TEXTURE)).setVisible(true);
  else scaffold.setVisible(false);
  let key = hubLookTexture(look);
  if (!has(scene, key)) key = hubTexture(look.level);
  if (has(scene, key)) {
    picture.setTexture(key).setScale(textureScale(key)).setVisible(true);
    return;
  }
  picture.setVisible(false);
  g.fillStyle(0x6c757d).fillRect(-150, -260, 300, 260).lineStyle(4, 0x343a40).strokeRect(-150, -260, 300, 260);
  g.fillStyle(0x343a40).fillRect(-30, -80, 60, 80);
  for (const dx of [-120, -40, 40, 120]) g.fillStyle(0x6c757d).fillRect(dx - 20, -290, 40, 30).strokeRect(dx - 20, -290, 40, 30);
}

export function createSiteView(scene: Phaser.Scene, s: Site): View {
  const [w, h] = SITE_SIZE[s.kind];
  const picture = scene.add.image(0, 0, '__DEFAULT').setOrigin(0.5, 1).setVisible(false);
  const g = scene.add.graphics();
  const label = scene.add.text(0, 0, '', { ...TEXT, ...fontStyle('priceTag') }).setOrigin(0.5, 1);
  return scene.add.container(s.x * UNIT_PX, G, [picture, g, label]).setDepth(2).setData('size', [w, h]);
}

/** Zeigt die Grafik, falls zugeordnet und geladen; liefert die Höhe für Schilder darüber (sonst Platzhalter-Höhe) */
function showPicture(scene: Phaser.Scene, v: View, s: Site, stage: number): number | null {
  const picture = v.getAt(0) as Phaser.GameObjects.Image;
  let key = siteTexture(s.kind, s.state, stage); // Mauer, Turm und Tor je Ausbau-Stufe (GEB3, B-631), Hub-Gebäude je Hub-Stufe (GEB9)
  if (!has(scene, key)) key = siteTexture(s.kind, s.state); // Stufe nicht geladen: Grundform statt leerer Fläche
  if (!has(scene, key)) {
    picture.setVisible(false);
    return null;
  }
  picture.setTexture(key).setScale(textureScale(key)).setVisible(true);
  return picture.displayHeight;
}

/** Stufe der Grafik: Hub-Gebäude je Hub-Stufe (GEB9), sonst die Stufe des Bauplatzes (GEB3; das Tor hat eigene Stufen, B-631) */
function siteStage(s: Site, world: World): number {
  return isHubBuilding(s.kind) ? (world.hubLevel ?? 1) : (s.level ?? 1);
}

/** `near`: Das Preisschild dieses Bauplatzes steht, weil es für einen Spieler das nächste ist (`priceTagRules.ts`, B-395) */
export function updateSiteView(scene: Phaser.Scene, v: View, s: Site, world: World, near: boolean): void {
  const cost = buildCost(s.kind, world.biome.depth); // Treppen je Tiefe (B-631)
  const wait = siteStatus(s, world.danger, world.biome.depth);
  const stage = siteStage(s, world);
  const marker = siteMarker(s); // Bauschild oder Gerüst im Bau, nie beides (B-586)
  const key = [s.state, s.paidGold, Math.round(s.buildProgress * 20), Math.round((s.hp / s.maxHp) * 20), s.bows, s.bowPaidGold, near, canAfford(world.stock, cost), currentLanguage(), statusKey(wait), stage, s.ruin === true, marker, s.clearing === true].join('|'); // stage: Mauer, Turm und Tor je Stufe (GEB3, B-631)
  if (v.getData('key') === key) return;
  v.setData('key', key);

  const g = v.getAt(1) as Phaser.GameObjects.Graphics;
  const label = v.getAt(2) as Phaser.GameObjects.Text;
  g.clear();
  label.setText('');
  const [w, size] = v.getData('size') as [number, number];
  const picture = showPicture(scene, v, s, stage);
  const h = picture ?? size;

  const image = v.getAt(0) as Phaser.GameObjects.Image;
  image.setTint(damageTint(damageStage(s))); // Schaden tönt das Sprite (WZ14.6), unbeschädigt ungetönt
  if (s.state === 'built') {
    if (picture === null) drawBuiltPlaceholder(g, s, w, h);
    drawDamageOverlay(g, s, picture === null ? w : image.displayWidth, h, picture === null);
    drawBuiltSigns(g, label, s, world, h, near);
    if (wait && label.text === '') drawSiteStatus(g, label, wait, siteName(s.kind), -h - 20); // laufender Ausbau
    return;
  }

  // Noch nicht gebaut: Bauschild (B-584) nur bei payable, im Bau das Gerüst (B-586) mit Fortschrittsbalken darüber
  if (marker === 'sign') drawBuildSign(g, s.kind);
  const top = marker === 'scaffold' ? showScaffold(scene, v, g, s.kind, w, h) : h;
  if (s.buildProgress > 0) drawBar(g, -top - 12, 80, s.buildProgress, 0xdda15e);
  if (s.state === 'unpaid' && near) drawPriceOrClearing(g, label, s, world, h, cost);
  if (wait) drawSiteStatus(g, label, wait, siteName(s.kind), -top - 20);
}

/** Gerüst der Größenklasse als Bild, ohne geladene Grafik die gezeichnete Form; liefert die Höhe für Balken und Status darüber */
function showScaffold(scene: Phaser.Scene, v: View, g: Phaser.GameObjects.Graphics, kind: Site['kind'], w: number, h: number): number {
  const key = scaffoldTexture(kind);
  if (!has(scene, key)) {
    drawScaffoldShape(g, w, h);
    return h;
  }
  const picture = (v.getAt(0) as Phaser.GameObjects.Image).setTexture(key).setScale(textureScale(key)).setVisible(true);
  return picture.displayHeight;
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

/** Risse über dem Gebäude, an Ruinen Trümmer am Fuß (WZ14.6); `shape`: Platzhalter-Form statt Bild der Breite `w` */
function drawDamageOverlay(g: Phaser.GameObjects.Graphics, s: Site, w: number, h: number, shape: boolean): void {
  drawDamage(g, damageStage(s), shape ? w : w * 0.6, h, shape);
  if (showRubble(s)) drawRubble(g, w);
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
/** Preis am unbezahlten Platz, solange Bäume oder Felsen im Freiraum stehen stattdessen „erst roden“ (B-636, B-652) */
function drawPriceOrClearing(g: Phaser.GameObjects.Graphics, label: Phaser.GameObjects.Text, s: Site, world: World, h: number, cost: Record<string, number | undefined>): void {
  if (s.clearing === true) label.setPosition(0, -h - 20).setText(t('site.clearing', { name: siteName(s.kind) })).setColor('#ffd166');
  else drawPrice(g, label, world, -h - 20, cost.gold ?? 0, s.paidGold, siteName(s.kind), cost);
}

function drawPrice(g: Phaser.GameObjects.Graphics, label: Phaser.GameObjects.Text, world: Pick<World, 'stock'>, y: number, total: number, paid: number, name: string, cost: Record<string, number | undefined>): void {
  const perRow = 10;
  for (let i = 0; i < total; i++) {
    const row = Math.floor(i / perRow);
    const inRow = Math.min(perRow, total - row * perRow);
    const cx = (i % perRow) * 22 - ((inRow - 1) * 22) / 2;
    const cy = y - row * 22;
    g.lineStyle(2, 0xffd166).strokeCircle(cx, cy, 8);
    if (i < paid) g.fillStyle(0xffd166).fillCircle(cx, cy, 8);
  }
  const material = (['wood', 'stone', 'copper', 'iron', 'crystal'] as const).filter((r) => cost[r]).map((r) => `${cost[r]} ${nameOf('res', r, r)}`); // Eisen, Kristall: Hub-Ausbau (B-591)
  const enough = canAfford(world.stock, cost);
  label.setPosition(0, y - Math.ceil(total / perRow) * 22).setText([name, ...material].join(' · ')).setColor(enough ? '#ffffff' : '#ff8fa3');
}

function drawBar(g: Phaser.GameObjects.Graphics, y: number, width: number, fraction: number, color: number): void {
  g.fillStyle(0x000000, 0.6).fillRect(-width / 2, y, width, 8);
  g.fillStyle(color).fillRect(-width / 2, y, width * Phaser.Math.Clamp(fraction, 0, 1), 8);
}

export const resourceName = (r: 'wood' | 'stone' | 'copper'): string => nameOf('res', r, r);
export const siteName = (kind: Site['kind']): string => nameOf('site', kind, BUILDINGS[kind].name);
