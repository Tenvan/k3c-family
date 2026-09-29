import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { fetchSave } from './core/saveStore';
import { installPageChrome } from './core/shell';
import { GameScene, type GameSceneData } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';

// Seitenrahmen sofort (Home-Button, Zurück-Falle für B), nicht erst nach dem Laden des Spielstands.
installPageChrome();

// URL-Parameter: ?seed=abc&depth=1 (Start-Stufe), ?fast=1 (Tag/Nacht 8x schneller),
// ?dev=1 (Dev-Tasten G/H/T/S auch im Build), ?save=1 (automatisch speichern), ?continue=1 (Spielstand laden)
const params = new URLSearchParams(window.location.search);
const save = params.has('continue') ? await fetchSave() : null;
const startData: GameSceneData = {
  seed: params.get('seed') ?? 'k3c',
  depth: Number(params.get('depth') ?? 0),
  fast: params.has('fast'),
  dev: import.meta.env.DEV || params.has('dev'),
  save,
  persist: params.has('save') || params.has('continue'),
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
