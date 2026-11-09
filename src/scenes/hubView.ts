import Phaser from 'phaser';
import { HUB_COLORS, HUB_LAYOUT, HUB_TEXT, HUB_TEXT_OFF, type HubTile, type HubTileId } from './pauseHub';
import { LINE, MATERIAL, SHADOW, SHADOW_LAYERS, STRIP_INSET, STRIP_PAD, cutPolygon, seedOf, shifted, stripHeight, toneOfColor, type Pt } from './paperCard';

const TILE_STYLE = { fontFamily: 'sans-serif', fontStyle: 'bold', color: HUB_TEXT };

/** Was eine Kachel beim Zeichnen braucht: wählbar, Beschriftung, Zeile darunter */
export interface HubTileState {
  enabled: boolean;
  label: string;
  sub: string;
}

/** Illustration einer Kachel aus Grafiken des Spiels; die Objekte gehören danach der Ansicht (sie räumt sie auf) */
export type HubArt<Id extends string> = (scene: Phaser.Scene, tile: HubTile<Id>) => Phaser.GameObjects.GameObject[];

/** Punkte als Phaser-Punkte (nur `x` und `y` werden gelesen) */
const vec = (points: readonly Pt[]): Phaser.Math.Vector2[] => points as unknown as Phaser.Math.Vector2[];

const fill = (g: Phaser.GameObjects.Graphics, points: readonly Pt[], color: number, alpha = 1): void => {
  g.fillStyle(color, alpha).fillPoints(vec(points), true, true);
};

/**
 * Zeichnet eine Kachelseite im Papierschnitt-Stil (B-768): je Kachel ein von Hand geschnittenes Stück Papier in einem Material des
 * Spiels mit weichem Schlagschatten, ein Leinen-Schild mit der Beschriftung und die Illustration aus dem Spiel. Die Auswahl hat einen
 * weißen Papierrand und höheren Schatten. Gemeinsam für das Pausenmenü (`OptionsScene`) und die Lobby (`LobbyScene`); die Logik steht in
 * `pauseHub.ts`. Neu gezeichnet wird nur bei Änderung.
 */
export class HubView<Id extends string = HubTileId> {
  private readonly cards: Phaser.GameObjects.Graphics;
  private readonly strips: Phaser.GameObjects.Graphics;
  private readonly texts: { label: Phaser.GameObjects.Text; sub: Phaser.GameObjects.Text }[];
  private readonly art: Phaser.GameObjects.GameObject[][];
  private key = '';

  constructor(
    scene: Phaser.Scene,
    private readonly tiles: readonly HubTile<Id>[],
    art?: HubArt<Id>,
  ) {
    this.cards = scene.add.graphics();
    this.art = tiles.map((tile) => art?.(scene, tile) ?? []);
    this.strips = scene.add.graphics();
    const { pad } = HUB_LAYOUT;
    this.texts = tiles.map((tile) => {
      const wrap = { width: tile.w - 2 * (STRIP_INSET + pad) };
      const sub = scene.add.text(tile.x + STRIP_INSET + pad, tile.y + tile.h - pad, '', { ...TILE_STYLE, fontSize: `${HUB_LAYOUT.subPx}px`, fontStyle: 'normal', wordWrap: wrap }).setOrigin(0, 1);
      const label = scene.add.text(tile.x + STRIP_INSET + pad, tile.y + tile.h - pad - sub.height - 8, '', { ...TILE_STYLE, fontSize: `${tile.labelPx}px`, wordWrap: wrap }).setOrigin(0, 1);
      return { label, sub };
    });
    this.setVisible(false);
  }

  /** Entfernt alle Flächen, Bilder und Texte (beim Wechsel zu einem anderen Raster) */
  destroy(): void {
    this.cards.destroy();
    this.strips.destroy();
    this.art.flat().forEach((o) => o.destroy());
    this.texts.forEach(({ label, sub }) => [label, sub].forEach((o) => o.destroy()));
  }

  setVisible(on: boolean): void {
    this.cards.setVisible(on);
    this.strips.setVisible(on);
    this.art.flat().forEach((o) => (o as Phaser.GameObjects.Sprite).setVisible?.(on));
    this.texts.forEach(({ label, sub }) => [label, sub].forEach((o) => o.setVisible(on)));
    this.key = ''; // beim nächsten Zeichnen neu aufbauen
  }

  /** Alle Kacheln neu zeichnen, wenn sich Auswahl oder Zustand gegenüber `signature` geändert haben */
  draw(selected: number, signature: string, state: (id: Id, index: number) => HubTileState): void {
    const key = `${selected}|${signature}`;
    if (key === this.key) return;
    this.key = key;
    this.cards.clear();
    this.strips.clear();
    this.tiles.forEach((tile, i) => {
      const s = state(tile.id, i);
      const { label, sub } = this.texts[i]!;
      const color = s.enabled ? HUB_TEXT : HUB_TEXT_OFF;
      sub.setText(s.sub).setColor(color);
      label.setText(s.label).setColor(color);
      const textH = label.height + (s.sub ? sub.height + 6 : 0);
      const bottom = tile.y + tile.h - STRIP_INSET - STRIP_PAD;
      sub.setY(bottom);
      label.setY(bottom - (s.sub ? sub.height + 6 : 0));
      this.drawCard(tile, i === selected, s.enabled);
      this.drawStrip(tile, textH);
    });
  }

  /** Karte: Schatten, bei Auswahl weißer Papierrand, Papier im Material, Falz unten links und helle Oberkante (Licht von rechts oben) */
  private drawCard(tile: HubTile<Id>, selected: boolean, enabled: boolean): void {
    const g = this.cards;
    const seed = seedOf(String(tile.id));
    const body = cutPolygon(tile.x, tile.y, tile.w, tile.h, seed);
    const tone = enabled ? toneOfColor(tile.color) : MATERIAL.stoff;
    const border = selected ? cutPolygon(tile.x - 9, tile.y - 9, tile.w + 18, tile.h + 18, seed + 1) : null;
    const lift = selected ? 1.5 : 1;
    for (const { d, alpha } of SHADOW_LAYERS) fill(g, shifted(border ?? body, d * 0.6 * lift, d * lift), SHADOW, alpha);
    if (border) fill(g, border, 0xffffff);
    fill(g, body, tone.basis);
    g.fillStyle(tone.schatten, 0.38).fillTriangle(tile.x + 2, tile.y + tile.h * 0.5, tile.x + 2, tile.y + tile.h - 2, tile.x + tile.w * 0.34, tile.y + tile.h - 2);
    g.lineStyle(4, tone.licht, 0.85).lineBetween(tile.x + tile.w * 0.4, tile.y + 2, tile.x + tile.w - 10, tile.y + 2);
  }

  /** Leinen-Schild unten in der Kachel, leicht schief eingesetzt, mit eigenem Schatten */
  private drawStrip(tile: HubTile<Id>, textH: number): void {
    const g = this.strips;
    const h = stripHeight(textH);
    const x = tile.x + STRIP_INSET;
    const y = tile.y + tile.h - STRIP_INSET - h;
    const poly = cutPolygon(x, y, tile.w - 2 * STRIP_INSET, h, seedOf(`${tile.id}:strip`), 120, 1.4, 4);
    for (const { d, alpha } of SHADOW_LAYERS) fill(g, shifted(poly, d * 0.25, d * 0.4), SHADOW, alpha * 0.8);
    fill(g, poly, MATERIAL.leinen.licht);
    g.lineStyle(2, LINE, 0.18).strokePoints(vec(poly), true, true);
  }
}

export { HUB_COLORS };
