/**
 * Szenario aus dem Menü starten, ohne die Seite neu zu laden (B-768, BD11.3): setzt die `RunConfig` auf genau die Werte des
 * Szenarios und startet die Lobby neu; sie liest die Konfiguration beim Start (`autostart`, `save`, `world`, `players`, `dev` …) und
 * legt oder betritt den Raum selbst. Hat die Konfiguration kein `autostart`, zeigt die Lobby ihre Startseite.
 */
import { setRun, type RunParams } from '../core/runConfig';
import type { RoomClient } from '../online/clientConnection';
import type { LobbySceneData } from './LobbyScene';

/** Was `startRun` vom Szenen-Manager braucht (Teil von `Phaser.Scenes.ScenePlugin`) */
export interface SceneStarter {
  scene: { start(key: string, data?: object): unknown };
}

export function startRun(host: SceneStarter, client: RoomClient, params: RunParams): void {
  setRun(params);
  const data: LobbySceneData = { client };
  host.scene.start('lobby', data);
}
