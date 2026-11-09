import Phaser from 'phaser';
import { readRenderInfo, rendererRequest, type RenderInfo } from './renderInfo';

type Source = Parameters<typeof readRenderInfo>[0];

/** Liest Renderer und Texturen eines laufenden Phaser-Spiels (Technik-Anzeige B-761 und Render-Log B-762 teilen sich diese Messung). */
export function readGameRenderInfo(game: Phaser.Game): RenderInfo {
  return readRenderInfo({
    renderer: game.renderer as unknown as Source['renderer'],
    webglType: Phaser.WEBGL,
    textureList: Object.values(game.textures.list) as unknown as Source['textureList'],
    devicePixelRatio: window.devicePixelRatio,
    viewport: `${window.innerWidth}×${window.innerHeight}`,
    request: rendererRequest(location.search),
  });
}
