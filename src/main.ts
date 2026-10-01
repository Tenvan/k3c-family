import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { installPageChrome } from './core/shell';
import { createRoomClient } from './online/clientConnection';
import { GameScene } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';
import { LobbyScene } from './scenes/LobbyScene';

// Seitenrahmen sofort (Home-Button, Zurück-Falle für B).
installPageChrome();

// Der Server rechnet, der Browser zeichnet; Raumwahl und Start-Parameter übernimmt die LobbyScene.
const client = createRoomClient();

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

game.scene.add('lobby', LobbyScene, true, { client });
game.scene.add('game', GameScene, false);
game.scene.add('hud', HudScene, false);

// Nur im Dev-Server oder mit ?dev=1: Zugriff für Debugging über die Browser-Konsole (window.game).
if (import.meta.env.DEV || new URLSearchParams(location.search).has('dev')) (window as unknown as { game: Phaser.Game; client: typeof client }).game = Object.assign(game, { client });
