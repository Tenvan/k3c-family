import Phaser from 'phaser';
import { guideSeen } from '../core/guideSeen';
import { loadSettings } from '../core/settings';
import { t } from '../core/texts';
import { screenAt } from './actionOverlay';
import { fontStyle } from './fontRules';
import { glyphOf, type HintDevice } from './glyphs';
import { GlyphRow, type RowPart } from './glyphView';
import { guideImage, MOON_IMAGE, stepGuide, usableGuideImage, type GuideHint } from './guideHints';
import { clientLog } from '../core/clientLog';
import type { RadarCell } from './radarView';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold', ...fontStyle('playerValue') }; // 28 px, HUD-fest (Q03)
/** Höhe über dem Boden in Welt-Pixeln: über Münze und Bauplatz bzw. über dem Aktionen-Overlay des Monarchen */
const OBJECT_PX = 160;
const PLAYER_PX = 370;

const MOON_PX = 28;

/** Mond-Bild einmal als Canvas-Textur aus dem Emoji 🌙 (B-370, kein Pack hat einen Mond); ohne Canvas steht nur Text */
function ensureMoon(scene: Phaser.Scene): void {
  if (scene.textures.exists(MOON_IMAGE)) return;
  const tex = scene.textures.createCanvas(MOON_IMAGE, MOON_PX, MOON_PX);
  const c = tex?.getContext();
  if (!tex || !c) return;
  c.font = `${MOON_PX - 6}px "Segoe UI Emoji", "Noto Color Emoji", "Apple Color Emoji", sans-serif`;
  c.textAlign = 'center';
  c.textBaseline = 'middle';
  c.fillText('🌙', MOON_PX / 2, MOON_PX / 2 + 1);
  tex.refresh();
}

/** Hinweis als Zeilenteile; `{key}` im Text wird zur Glyph der Bestätigen-Taste des Geräts, ein Bild steht davor (B-294) */
function partsOf(h: GuideHint, device: HintDevice, usable: (h: GuideHint) => ReturnType<typeof guideImage>): RowPart[] {
  const [before = '', after] = t(`guide.${h.id}`, {}).split('{key}');
  const text: RowPart[] = after === undefined ? [before] : [before, glyphOf('confirm', device), after];
  const image = usable(h);
  return image ? [image, ...text] : text;
}

/**
 * Geführte erste Nacht (S6.3, B-148): je Spielerfeld der nächste ungesehene Hinweis über seinem Objekt, mit Glyph (S6.2).
 * Rein zum Ansehen, nimmt keine Eingabe; abschaltbar in den Optionen (`guideHints`).
 */
export class GuideOverlay {
  private rows: GlyphRow[] = [];
  private shown: (GuideHint | null)[] = [];
  private readonly warned = new Set<string>();

  constructor(private readonly scene: Phaser.Scene) {}

  /** Bild des Hinweises, sobald seine Textur da ist; fehlt sie, steht der Hinweis ohne Bild und ein Log 🐛 meldet es einmal (B-393) */
  private imageOf(h: GuideHint): ReturnType<typeof guideImage> {
    if (h.id === 'dusk') ensureMoon(this.scene);
    const exists = (key: string) => this.scene.textures.exists(key);
    const warn = (key: string) => clientLog('warn', '🐛 Hinweis-Bild fehlt: Textur nicht geladen', { key, hint: h.id });
    return usableGuideImage(guideImage(h.id), exists, warn, this.warned);
  }

  /** Argumente wie `ActionOverlay.draw` */
  draw(cells: readonly RadarCell[], cameras: readonly Phaser.Cameras.Scene2D.Camera[], slotOf: (seat: number) => number | undefined, deviceOf: (slot: number) => HintDevice): void {
    this.rows.forEach((r) => r.box.setVisible(false));
    const on = loadSettings().guideHints; // je Frame gelesen wie in `GameScene`: die Optionen wirken sofort
    cells.forEach(({ cell, monarch, world }, i) => {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      const slot = slotOf(cell.seat);
      const h = on && p && world ? stepGuide(this.shown[i] ?? null, world, p, guideSeen.ids, (id) => guideSeen.mark(id)) : null;
      this.shown[i] = h;
      const at = h && cameras[i] ? screenAt(cell, cameras[i].worldView, h.x, h.target === null ? PLAYER_PX : OBJECT_PX) : null;
      if (!h || !at || slot === undefined) return;
      (this.rows[i] ??= new GlyphRow(this.scene, STYLE, 'bottom')).set(partsOf(h, deviceOf(slot), (hint) => this.imageOf(hint)), at.x, at.y);
    });
  }
}
