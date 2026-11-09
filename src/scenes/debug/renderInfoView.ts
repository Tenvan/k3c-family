import Phaser from 'phaser';
import { GAME_WIDTH } from '../../core/constants';
import { FrameAverage, canvasWarning, gfxEnabled, renderInfoLines, type RenderInfo } from '../../core/renderInfo';
import { readGameRenderInfo } from '../../core/gameRenderInfo';
import { loadSettings } from '../../core/settings';

/** Oben rechts unter der Leiste, klein genug, um das Spiel nicht zu verdecken */
const POS = { x: GAME_WIDTH - 16, y: 110 };
/** Texturen und GPU ändern sich selten: neu lesen alle 2 s statt je Frame */
const REFRESH_MS = 2000;

/**
 * Technik-Anzeige (B-761): Renderer-Typ, GPU, Texturen, FPS, Ø Frametime. Standard an (Menü › Technik-Anzeige schaltet ab, `?gfx=0` ebenso, `?gfx=1` erzwingt an).
 * Bei Canvas-Fallback rot und mit Warnzeile. Gehört der `HudScene`, liest nur.
 */
export class RenderInfoView {
  private readonly text: Phaser.GameObjects.Text;
  private readonly frames = new FrameAverage(60);
  private info: RenderInfo | null = null;
  private readAt = -Infinity;

  constructor(private readonly scene: Phaser.Scene) {
    this.text = scene.add
      .text(POS.x, POS.y, '', { fontSize: '20px', color: '#9be564', stroke: '#000000', strokeThickness: 4, fontStyle: 'bold', align: 'right', backgroundColor: '#000000aa' })
      .setOrigin(1, 0)
      .setScrollFactor(0)
      .setDepth(10_000)
      .setVisible(false);
  }

  /** Das Text-Objekt, z. B. damit eine zweite Kamera es ausblendet */
  get object(): Phaser.GameObjects.Text {
    return this.text;
  }

  update(): void {
    const on = gfxEnabled(location.search, loadSettings().techInfo);
    this.text.setVisible(on);
    if (!on) return;
    const loop = this.scene.game.loop;
    this.frames.add(loop.delta);
    const now = performance.now();
    if (!this.info || now - this.readAt > REFRESH_MS) {
      this.info = this.read();
      this.readAt = now;
    }
    this.text
      .setColor(canvasWarning(this.info) ? '#ff5555' : '#9be564')
      .setText(renderInfoLines(this.info, loop.actualFps, this.frames.mean));
  }

  private read(): RenderInfo {
    return readGameRenderInfo(this.scene.game);
  }
}
