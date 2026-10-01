import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { installPageChrome } from './core/shell';
import { createRoomClient } from './online/clientConnection';
import { GameScene } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';

// Seitenrahmen sofort (Home-Button, Zurück-Falle für B).
installPageChrome();

// Übergangslösung bis zur Lobby (SP08.3): Nach dem Verbinden wird der Spielstand `familie` geöffnet (oder neu erstellt),
// `?save=NAME` wählt einen anderen, `?room=CODE` tritt einem Raum bei. Der Server rechnet, der Browser zeichnet.
const params = new URLSearchParams(window.location.search);
const saveParam = params.get('save') ?? '';
const save = /^[a-z0-9-]{1,32}$/.test(saveParam) ? saveParam : 'familie';
const room = params.get('room');

const client = createRoomClient();
let requested = false;
let created = false;
client.onChange = () => {
  if (client.status !== 'lobby') return;
  if (!requested) {
    requested = true;
    if (room) client.join(room, [0]);
    else client.create(save, false, 0, [0]);
  } else if (!room && !created && client.errorCode === 'save_not_found') {
    created = true; // gibt es den Spielstand noch nicht, einmal neu anlegen
    client.create(save, true, 0, [0]);
  }
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

game.scene.add('game', GameScene, true, { client });
game.scene.add('hud', HudScene, false);

// Nur im Dev-Server: Zugriff für Debugging über die Browser-Konsole (window.game).
if (import.meta.env.DEV) (window as unknown as { game: Phaser.Game; client: typeof client }).game = Object.assign(game, { client });
