import Phaser from 'phaser';
import { t } from '../../core/texts';
import { fontStyle } from '../render/fontRules';
import type { RadarCell } from './radarView';

/**
 * Pause-Anzeige (AZ1.2, B-333): Meldet der Zustand `devPaused`, steht in jeder Spieler-Zelle mittig „Pausiert“ auf einem
 * halbtransparenten Band. Der Client zeigt nur an, was der Server meldet; ohne das Feld (Couch-Raum) erscheint nichts.
 */

const BAND_HEIGHT = 110;
const BAND_ALPHA = 0.55;
/** über dem Radar und den HUD-Elementen, unter dem Verbindungs-Hinweis (der ersetzt das Band ohnehin) */
const DEPTH = 40;

/** Zeigen? Nur bei gemeldeter Dev-Pause und ohne Verbindungs-Hinweis, der hat Vorrang. */
export function pauseShown(world: object | null | undefined, away: string | null | undefined): boolean {
  // Der Welt-Typ kennt `devPaused` nicht; der Zustand des Servers trägt es nur im Dev-Mode (wie `debugOverlay.ts`)
  return (world as { devPaused?: boolean } | null | undefined)?.devPaused === true && !away;
}

export class PauseBanner {
  private readonly band: Phaser.GameObjects.Graphics;
  private readonly labels: Phaser.GameObjects.Text[] = [];

  constructor(private readonly scene: Phaser.Scene) {
    this.band = scene.add.graphics().setDepth(DEPTH).setVisible(false);
  }

  /** Band und Text je Spieler-Zelle; `shown = false` blendet alles aus */
  draw(cells: readonly RadarCell[], shown: boolean): void {
    this.band.clear().setVisible(shown);
    this.labels.forEach((l) => l.setVisible(false));
    if (!shown) return;
    cells
      .filter((c) => c.cell.kind === 'player')
      .forEach(({ cell }, i) => {
        const cy = cell.y + cell.h / 2;
        this.band.fillStyle(0x000000, BAND_ALPHA).fillRect(cell.x, cy - BAND_HEIGHT / 2, cell.w, BAND_HEIGHT);
        const label = (this.labels[i] ??= this.scene.add
          .text(0, 0, t('hud.paused'), { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold', ...fontStyle('banner') })
          .setOrigin(0.5)
          .setDepth(DEPTH + 1));
        label.setText(t('hud.paused')).setPosition(cell.x + cell.w / 2, cy).setVisible(true);
      });
  }
}
