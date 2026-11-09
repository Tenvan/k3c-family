import Phaser from 'phaser';
import { HUB_COLORS, HUB_LAYOUT, HUB_TEXT, HUB_TEXT_OFF, type HubTile, type HubTileId } from './pauseHub';

const TILE_STYLE = { fontFamily: 'sans-serif', fontStyle: 'bold', color: HUB_TEXT };

/** Was eine Kachel beim Zeichnen braucht: wählbar, Beschriftung, Zeile darunter */
export interface HubTileState {
  enabled: boolean;
  label: string;
  sub: string;
}

/**
 * Zeichnet eine Kachelseite (B-768): farbige Flächen mit Auswahlrahmen und zwei Textzeilen je Kachel. Gemeinsam für das Pausenmenü
 * (`OptionsScene`) und die Startseite der Lobby (`LobbyScene`); die Logik steht in `pauseHub.ts`. Neu gezeichnet wird nur bei Änderung.
 */
export class HubView<Id extends string = HubTileId> {
  private readonly gfx: Phaser.GameObjects.Graphics;
  private readonly texts: { label: Phaser.GameObjects.Text; sub: Phaser.GameObjects.Text }[];
  private key = '';

  constructor(
    scene: Phaser.Scene,
    private readonly tiles: readonly HubTile<Id>[],
  ) {
    this.gfx = scene.add.graphics();
    const { pad } = HUB_LAYOUT;
    this.texts = tiles.map((tile) => {
      const wrap = { width: tile.w - 2 * pad };
      const sub = scene.add.text(tile.x + pad, tile.y + tile.h - pad, '', { ...TILE_STYLE, fontSize: `${HUB_LAYOUT.subPx}px`, fontStyle: 'normal', wordWrap: wrap }).setOrigin(0, 1);
      const label = scene.add.text(tile.x + pad, tile.y + tile.h - pad - sub.height - 8, '', { ...TILE_STYLE, fontSize: `${tile.labelPx}px`, wordWrap: wrap }).setOrigin(0, 1);
      return { label, sub };
    });
    this.setVisible(false);
  }

  /** Entfernt alle Flächen und Texte (beim Wechsel zu einem anderen Raster) */
  destroy(): void {
    this.gfx.destroy();
    this.texts.forEach(({ label, sub }) => [label, sub].forEach((o) => o.destroy()));
  }

  setVisible(on: boolean): void {
    this.gfx.setVisible(on);
    this.texts.forEach(({ label, sub }) => [label, sub].forEach((o) => o.setVisible(on)));
    this.key = ''; // beim nächsten Zeichnen neu aufbauen
  }

  /** Alle Kacheln neu zeichnen, wenn sich Auswahl oder Zustand gegenüber `signature` geändert haben */
  draw(selected: number, signature: string, state: (id: Id, index: number) => HubTileState): void {
    const key = `${selected}|${signature}`;
    if (key === this.key) return;
    this.key = key;
    const g = this.gfx.clear();
    this.tiles.forEach((tile, i) => {
      const s = state(tile.id, i);
      g.fillStyle(s.enabled ? tile.color : HUB_COLORS.off, 1).fillRoundedRect(tile.x, tile.y, tile.w, tile.h, 28);
      if (i === selected) g.lineStyle(10, 0xffffff, 1).strokeRoundedRect(tile.x - 6, tile.y - 6, tile.w + 12, tile.h + 12, 32);
      if (tile.id === 'resume') g.fillStyle(HUB_COLORS.night, 1).fillTriangle(tile.x + 40, tile.y + 40, tile.x + 40, tile.y + 150, tile.x + 140, tile.y + 95);
      const { label, sub } = this.texts[i]!;
      const color = s.enabled ? HUB_TEXT : HUB_TEXT_OFF;
      sub.setText(s.sub).setColor(color);
      label.setText(s.label).setColor(color).setY(tile.y + tile.h - HUB_LAYOUT.pad - sub.height - 8);
    });
  }
}
