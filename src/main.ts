import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from './core/constants';
import { clientLog, installClientLog } from './core/clientLog';
import { installPageChrome } from './core/shell';
import { CLIENT, fetchServerBuild, versionLine, type BuildInfo } from './core/version';
import { antialiasOff, rendererRequest } from './core/renderInfo';
import { installRenderLog } from './core/renderLogInstall';
import { installContextGuard } from './core/contextGuardInstall';
import { installPanelHost, panelOpen } from './core/panelHost';
import { showNoServer } from './core/serverCheck';
import { reloadIfStale } from './core/fresh';
import { installFullscreenKeeper } from './core/fullscreenKeeper';
import { createRoomClient } from './online/clientConnection';
import { GameScene } from './scenes/GameScene';
import { HudScene } from './scenes/HudScene';
import { LoadScene } from './scenes/LoadScene';
import { LobbyScene } from './scenes/LobbyScene';
import { OptionsScene } from './scenes/OptionsScene';
import { VERSION_KEY } from './scenes/debug/debugOverlay';
import { uniqueRun } from './scenes/menuPages';
import { startRun } from './scenes/runStart';
import { runSearch } from './core/runConfig';

// Seitenrahmen sofort (Home-Button, Zurück-Falle für B).
installPageChrome({ padNav: false, home: false }); // eigene Pad-Bedienung; das Spiel ist die oberste Seite (kein Home)
installFullscreenKeeper(); // Vollbild bei der ersten Eingabe anfordern und nach einem Verlust erneut (BD11.6)
installClientLog();

/** Neuere Fassung am Server? Dann einmal je Version frisch laden, nie im Spiel oder im Werkzeug-Panel (B-767) */
function checkFresh(game: Phaser.Game | null): void {
  if (game && (panelOpen() || !game.scene.isActive('lobby'))) return;
  void fetchServerBuild().then((server) => reloadIfStale(CLIENT, server));
}

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
  // Werkzeug-Seiten öffnen als Panel im Menü: das Spiel pausiert die Schleife, ein Szenario aus dem Panel startet im selben Dokument (BD11.5)
  installPanelHost({
    onOpen: () => game.loop.sleep(),
    onClose: () => game.loop.wake(),
    onRun: (params) => startRun(game.scene.getScene('lobby'), client, uniqueRun(params)),
  });
  game.scene.add('load', LoadScene, true, { client }); // lädt den Atlas, startet dann die Lobby
  game.scene.add('lobby', LobbyScene, false);
  game.scene.add('game', GameScene, false);
  game.scene.add('hud', HudScene, false);
  game.scene.add('options', OptionsScene, false);

  setInterval(() => checkFresh(game), 60_000); // das Spiel bleibt auf der Xbox tagelang offen: neue Fassung des Servers erkennen

  // Nur im Dev-Server oder mit ?dev=1: Zugriff für Debugging über die Browser-Konsole (window.game).
  if (import.meta.env.DEV || new URLSearchParams(runSearch()).has('dev')) (window as unknown as { game: Phaser.Game; client: typeof client }).game = Object.assign(game, { client });
}

// Ohne Go-Server (z. B. GitHub Pages) statt der Lobby ein Hinweis (B-032).
void fetchServerBuild().then((server) => {
  if (!server) return showNoServer(document.getElementById('game')!);
  if (reloadIfStale(CLIENT, server)) return; // alter Client im Zwischenspeicher: frisch laden, bevor das Spiel startet
  start(server);
});
