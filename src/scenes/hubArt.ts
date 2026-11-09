/**
 * Illustrationen der Kacheln aus Grafiken des Spiels (B-768): Monarch, Burg, Truppen, Portal, Truhe, Ausgang. Sie stehen als ausgeschnittene
 * Papierfiguren auf der Kachel, die große Kachel lässt den Monarchen über den Rand hinausragen. Fehlt eine Grafik (noch nicht geladen),
 * bleibt die Kachel ohne Bild. Zeichnet nur; welche Grafik zu welcher Kachel gehört, steht in `HUB_ART` (Daten, getestet).
 */
import Phaser from 'phaser';
import { objectImage } from './render/objectView';
import { PLAYER_SPRITES, TROOP_SPRITES, makeSprite } from './render/sprites';
import type { HubTile, HubTileId } from './pauseHub';
import type { CategoryId } from './menuPages';
import { ART_BASE, HUB_ART, type ArtPiece } from './hubArtData';

type Placed = Phaser.GameObjects.Sprite | Phaser.GameObjects.Image;

/** Auf Zielhöhe skalieren: Verhältnis aus dem Bild, ohne das Seitenverhältnis zu ändern */
function fitHeight(obj: Placed, targetPx: number): Placed {
  const k = targetPx / Math.max(1, obj.displayHeight);
  return obj.setScale(obj.scaleX * k, obj.scaleY * k);
}

function place(scene: Phaser.Scene, piece: ArtPiece, x: number, y: number, targetPx: number): Placed | null {
  let made: Placed | null = null;
  if (piece.kind === 'player') made = makeSprite(scene, PLAYER_SPRITES[piece.slot % PLAYER_SPRITES.length]!).setPosition(x, y);
  else if (piece.kind === 'troop') {
    const spec = TROOP_SPRITES[piece.troop];
    made = spec ? makeSprite(scene, spec).setPosition(x, y) : null;
  } else if (piece.kind === 'object') made = objectImage(scene, piece.id, x, y);
  else if (scene.textures.exists(piece.key)) made = scene.add.image(x, y, piece.key).setOrigin(0.5, 1);
  return made ? fitHeight(made, targetPx) : null;
}

/**
 * Fabrik für `HubView`: stellt die Illustration einer Kachel auf. `tile.id` ist die Kachel- oder Kategorie-ID; Kacheln ohne Eintrag
 * in `HUB_ART` (Einträge der Werkzeuge) bekommen kein Bild.
 */
export function hubArt(scene: Phaser.Scene, tile: HubTile<string>): Phaser.GameObjects.GameObject[] {
  const art = HUB_ART[tile.id as HubTileId | CategoryId];
  if (!art) return [];
  const big = tile.w > 600;
  const baseY = tile.y + tile.h * (1 - (big ? ART_BASE.big : ART_BASE.small));
  const areaH = baseY - tile.y;
  const out: Phaser.GameObjects.GameObject[] = [];
  for (const { piece, at } of art.pieces) {
    const target = (piece.kind === 'texture' ? 0.82 : big ? 1.12 : 0.9) * areaH; // die große Kachel lässt den Monarchen über den Rand ragen
    const obj = place(scene, piece, tile.x + tile.w * at, baseY, target);
    if (obj) out.push(obj);
  }
  return out;
}
