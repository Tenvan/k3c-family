import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { loadSave } from './core/saveStore';
import { installPageChrome } from './core/shell';
import { GameScene, type GameSceneData } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';

installPageChrome();

// URL-Parameter zum Testen: ?seed=abc&depth=1&fast=1&dev=1&new=1
const params = new URLSearchParams(window.location.search);
// Ohne ?seed / ?depth ist das der Familien-Spielstand: laden, weiterspielen, automatisch speichern.
// ?new=1 verwirft ihn und startet neu. ?seed / ?depth sind freies Spiel ohne Speichern.
const persist = !params.has('seed') && !params.has('depth');
const startNew = params.has('new');
if (startNew) {
  // Sonst würde jeder Reload wieder neu anfangen.
  params.delete('new');
  history.replaceState(null, '', `${window.location.pathname}${params.toString() ? `?${params}` : ''}`);
}
// ?fast=1: Tag/Nacht 8x schneller · ?dev=1: Dev-Tasten (G Gold, H Material, T Zeitsprung) auch im Build
const startData: GameSceneData = {
  seed: params.get('seed') ?? 'k3c',
  depth: Number(params.get('depth') ?? 0),
  fast: params.has('fast'),
  dev: import.meta.env.DEV || params.has('dev'),
  persist,
  save: persist && !startNew ? await loadSave() : null,
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
