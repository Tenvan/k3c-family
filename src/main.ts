import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { GameScene, type GameSceneData } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';

// URL-Parameter zum Testen: ?seed=abc&depth=1
const params = new URLSearchParams(window.location.search);
const startData: GameSceneData = {
  seed: params.get('seed') ?? 'k3c',
  depth: Number(params.get('depth') ?? 0),
};

const game = new Phaser.Game({
  type: Phaser.AUTO,
  parent: 'game',
  width: GAME_WIDTH,
  height: GAME_HEIGHT,
  backgroundColor: '#000000',
  scale: { mode: Phaser.Scale.FIT, autoCenter: Phaser.Scale.CENTER_BOTH },
  input: { gamepad: true },
  scene: [],
});

game.scene.add('game', GameScene, true, startData);
game.scene.add('hud', HudScene, false);

// Nur im Dev-Server: Zugriff für Debugging über die Browser-Konsole (window.game).
if (import.meta.env.DEV) (window as unknown as { game: Phaser.Game }).game = game;
