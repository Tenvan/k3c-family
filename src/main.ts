import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { clientLog, installClientLog } from './core/clientLog';
import { installPageChrome } from './core/shell';
import { CLIENT, fetchServerBuild, versionLine, type BuildInfo } from './core/version';
import { antialiasOff, rendererRequest } from './core/renderInfo';
import { installRenderLog } from './core/renderLogInstall';
import { installContextGuard } from './core/contextGuardInstall';
import { showNoServer } from './landing/serverCheck';
import { createRoomClient } from './online/clientConnection';
import { GameScene } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';
import { LoadScene } from './scenes/LoadScene';
import { LobbyScene } from './scenes/LobbyScene';
import { OptionsScene } from './scenes/OptionsScene';
import { VERSION_KEY } from './scenes/debug/debugOverlay';
import { runSearch } from './core/runConfig';

// Seitenrahmen sofort (Home-Button, Zurück-Falle für B).
installPageChrome({ padNav: false }); // eigene Pad-Bedienung bzw. Phaser-Spiel
installClientLog();

function start(server: BuildInfo): void {
  // Der Server rechnet, der Browser zeichnet; Raumwahl und Start-Parameter übernimmt die LobbyScene.
  const client = createRoomClient();

  // Renderer: Standard AUTO (WebGL, sonst Canvas); `?renderer=webgl|canvas` erzwingt einen, `?aa=0` schaltet die MSAA-Glättung ab (B-761)
  const want = rendererRequest(runSearch());
  const game = new Phaser.Game({
    type: want === 'webgl' ? Phaser.WEBGL : want === 'canvas' ? Phaser.CANVAS : Phaser.AUTO,
    render: { powerPreference: 'high-performance', antialiasGL: !antialiasOff(runSearch()) },
    parent: 'game',
    width: GAME_WIDTH,
    height: GAME_HEIGHT,
    backgroundColor: '#000000',
    scale: { mode: Phaser.Scale.FIT, autoCenter: Phaser.Scale.CENTER_BOTH },
    input: { gamepad: true },
    scene: [],
  });

  // Versionen für Lobby und Debug-Overlay, je eine Zeile für Client und Server (beide lesen sie aus der Registry)
  game.registry.set(VERSION_KEY, versionLine(CLIENT, server, '\n'));
  clientLog('info', `🚀 Phaser gestartet (${game.renderer.type === Phaser.WEBGL ? 'WebGL' : '⚠️ Canvas'}, gewünscht: ${want})`);
  installRenderLog(game, versionLine(CLIENT, server, ' | '));
  installContextGuard(game, () => client.roomCode); // Kontextverlust: Neustart mit Canvas, höchstens einmal je Sitzung (B-763)
  game.scene.add('load', LoadScene, true, { client }); // lädt den Atlas, startet dann die Lobby
  game.scene.add('lobby', LobbyScene, false);
  game.scene.add('game', GameScene, false);
  game.scene.add('hud', HudScene, false);
  game.scene.add('options', OptionsScene, false);

  // Nur im Dev-Server oder mit ?dev=1: Zugriff für Debugging über die Browser-Konsole (window.game).
  if (import.meta.env.DEV || new URLSearchParams(runSearch()).has('dev')) (window as unknown as { game: Phaser.Game; client: typeof client }).game = Object.assign(game, { client });
}

// Ohne Go-Server (z. B. GitHub Pages) statt der Lobby ein Hinweis (B-032).
void fetchServerBuild().then((server) => (server ? start(server) : showNoServer(document.getElementById('game')!)));
