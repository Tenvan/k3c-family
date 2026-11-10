import Phaser from 'phaser';
import { clientLog } from './clientLog';
import { readGameRenderInfo } from './gameRenderInfo';
import { rendererRequest } from './renderInfo';
import { RENDER_LOG_KEY, RenderLog } from './renderLog';
import { runSearch } from './runConfig';

/**
 * Render-Log eines Phaser-Spiels (B-762): Verlauf je Frame und Kontext-Ereignisse. Die Startzeile schreibt der Aufrufer nach dem Laden
 * über die Registry (`RENDER_LOG_KEY`: `{ log, version }`). Gibt das `RenderLog` zurück.
 */
export function installRenderLog(game: Phaser.Game, version: string): RenderLog {
  const log = new RenderLog({
    log: clientLog,
    now: () => performance.now(),
    info: () => readGameRenderInfo(game),
    heapMiB: () => {
      const m = (performance as unknown as { memory?: { usedJSHeapSize: number } }).memory;
      return m ? m.usedJSHeapSize / (1024 * 1024) : null;
    },
  });
  game.registry.set(RENDER_LOG_KEY, { log, version });
  game.events.on('poststep', () => log.frame(game.loop.delta, game.loop.actualFps));
  const watch = (): void => {
    const canvas = game.canvas;
    canvas.addEventListener('webglcontextlost', () => log.context('lost', game.loop.actualFps));
    canvas.addEventListener('webglcontextrestored', () => log.context('restored', game.loop.actualFps));
    if (game.renderer.type !== Phaser.WEBGL && rendererRequest(runSearch()) !== 'canvas') log.context('fallback', null);
  };
  if (game.isBooted) watch();
  else game.events.once('ready', watch);
  return log;
}
