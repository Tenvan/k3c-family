import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { RENDER_LOG_KEY, logLoadError, type RenderLog } from '../core/renderLog';
import { textureMiB } from '../core/renderInfo';
import type { RoomClient } from '../online/clientConnection';
import { barFill, errorText, progressText } from './loadLogic';
import { prepareWorld, preloadWorld } from './render/objectView';
import { prepareBuildings, preloadBuildings } from './render/siteView';
import { createSpriteAnims, preloadSprites } from './render/sprites';

const STYLE = { fontSize: '36px', color: '#ffffff', stroke: '#000000', strokeThickness: 6, fontStyle: 'bold', align: 'center' };
const BAR_W = 800;
const BAR_H = 28;

/** Lädt den Figuren-Atlas und die Gebäude-Grafiken mit Fortschrittsbalken, bei einem Ladefehler eine Meldung mit Dateinamen; danach Lobby. Zeichnet nur. */
export class LoadScene extends Phaser.Scene {
  private client!: RoomClient;

  constructor() {
    super('load');
  }

  init(data: { client: RoomClient }): void {
    this.client = data.client;
  }

  preload(): void {
    const x = (GAME_WIDTH - BAR_W) / 2;
    const y = GAME_HEIGHT / 2;
    this.add.rectangle(GAME_WIDTH / 2, y, BAR_W + 8, BAR_H + 8, 0x333333).setStrokeStyle(2, 0xffffff);
    const fill = this.add.rectangle(x, y, 1, BAR_H, 0xffd700).setOrigin(0, 0.5);
    const label = this.add.text(GAME_WIDTH / 2, y - 70, progressText(0), STYLE).setOrigin(0.5);
    const failed: string[] = [];
    const startedAt = performance.now();
    let files = 0;
    this.load.on('filecomplete', () => files++);

    this.load.on('progress', (v: number) => {
      fill.width = Math.max(1, barFill(v, BAR_W));
      label.setText(progressText(v));
    });
    // Phaser meldet nach Fehlern trotzdem „complete“: Fehler merken, statt weiterzumachen.
    this.load.on('loaderror', (file: Phaser.Loader.File) => {
      failed.push(String(file.url));
      logLoadError(String(file.url), file.xhrLoader?.status ?? 0);
      label.setText(errorText(failed)).setColor('#ff6060');
      fill.setFillStyle(0xff4040);
    });
    this.load.once('complete', () => {
      if (failed.length > 0) return;
      createSpriteAnims(this);
      prepareBuildings(this);
      prepareWorld(this);
      const entry = this.registry.get(RENDER_LOG_KEY) as { log: RenderLog; version: string } | undefined;
      entry?.log.loaded('start', files, textureMiB(Object.values(this.textures.list) as unknown as Parameters<typeof textureMiB>[0]), performance.now() - startedAt);
      entry?.log.start({ userAgent: navigator.userAgent, version: entry.version });
      this.scene.start('lobby', { client: this.client });
    });
    preloadSprites(this);
    preloadBuildings(this);
    preloadWorld(this);
  }
}
