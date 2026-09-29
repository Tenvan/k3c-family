import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH, MAX_PLAYERS, UNIT_PX } from '../core/constants';
import type { GameScene } from './GameScene';

/** Bildschirmfeste Anzeigen über allen Split-Screen-Kameras. */
export class HudScene extends Phaser.Scene {
  private info!: Phaser.GameObjects.Text;
  private joinHint!: Phaser.GameObjects.Text;
  private playerLabels: Phaser.GameObjects.Text[] = [];
  private fps!: Phaser.GameObjects.Text;

  constructor() {
    super('hud');
  }

  create(): void {
    const game = this.scene.get('game') as GameScene;
    const style = { fontSize: '26px', color: '#ffffff', stroke: '#000000', strokeThickness: 5 };

    this.info = this.add
      .text(GAME_WIDTH - 20, 16, `${game.biome.name} · Seed "${game.seed}" · ${game.level.widthUnits} Units`, style)
      .setOrigin(1, 0);
    this.joinHint = this.add
      .text(GAME_WIDTH / 2, GAME_HEIGHT / 2, 'Drücke  A  (Controller) oder  Leertaste  zum Beitreten', { ...style, fontSize: '44px' })
      .setOrigin(0.5);
    this.fps = this.add.text(20, GAME_HEIGHT - 44, '', { ...style, fontSize: '20px' });
    this.add
      .text(GAME_WIDTH - 20, GAME_HEIGHT - 44, 'Dev: N = neuer Seed · 1/2/3 = Tiefe · F / RS = Vollbild', { ...style, fontSize: '20px' })
      .setOrigin(1, 0);
  }

  update(): void {
    const game = this.scene.get('game') as GameScene;
    const players = game.players;
    const stripHeight = GAME_HEIGHT / Math.max(1, players.length);

    this.joinHint.setVisible(players.length < MAX_PLAYERS);
    this.joinHint.setY(players.length === 0 ? GAME_HEIGHT / 2 : GAME_HEIGHT - 110);
    this.joinHint.setFontSize(players.length === 0 ? 44 : 28);
    this.info.setVisible(players.length < 2);

    players.forEach((monarch, i) => {
      let label = this.playerLabels[i];
      if (!label) {
        label = this.add.text(20, 0, '', { fontSize: '26px', color: '#ffffff', stroke: '#000000', strokeThickness: 5 });
        this.playerLabels[i] = label;
      }
      label.setY(i * stripHeight + 16);
      label.setText(`P${i + 1} · ${monarch.controls.label} · x ${Math.round(monarch.x / UNIT_PX)}`);
    });

    this.fps.setText(`${Math.round(this.game.loop.actualFps)} FPS`);
  }
}
