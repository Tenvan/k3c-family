import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { installPageChrome } from './core/shell';
import { OnlineClient } from './online/client';
import { GameScene, type GameSceneData } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';

// URL-Parameter zum Testen: ?seed=abc&depth=1&fast=1&dev=1
const params = new URLSearchParams(window.location.search);
// ?fast=1: Tag/Nacht 8x schneller · ?dev=1: Dev-Tasten (G Gold, H Material, T Zeitsprung) auch im Build
const startData: GameSceneData = {
  seed: params.get('seed') ?? 'k3c',
  depth: Number(params.get('depth') ?? 0),
  fast: params.has('fast'),
  dev: import.meta.env.DEV || params.has('dev'),
};

// ?online=RAUM: Mit anderen Geräten im selben Raum auf dem Server spielen (ein Monarch pro Gerät)
const onlineRoom = params.get('online');

/** Online: erst verbinden, dann starten. Seed, Tiefe und Tempo bestimmt der Raum. */
async function start(): Promise<void> {
  if (onlineRoom !== null) {
    const status = document.createElement('p');
    status.style.cssText = 'position:fixed;inset:0;display:grid;place-items:center;margin:0;color:#f1f3f9;font:24px system-ui,sans-serif;text-align:center;padding:0 24px';
    status.textContent = 'Verbinde …';
    document.body.append(status);
    try {
      const client = await OnlineClient.connect(onlineRoom || 'familie', { depth: startData.depth, fast: !!startData.fast, seed: params.get('seed') ?? undefined });
      Object.assign(startData, { seed: client.seed, depth: client.depth, fast: client.fast, online: client });
      status.remove();
    } catch (err) {
      status.textContent = `Online nicht möglich: ${(err as Error).message}`;
      return;
    }
  }
  launch();
}

function launch(): void {
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
}

installPageChrome();
void start();
