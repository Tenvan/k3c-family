import Phaser from 'phaser';
import { guideSeen } from '../core/guideSeen';
import { loadSettings } from '../core/settings';
import { t } from '../core/texts';
import type { Device } from '../input/slotBindings';
import { screenAt } from './actionOverlay';
import { fontStyle } from './fontRules';
import { glyphOf } from './glyphs';
import { GlyphRow, type RowPart } from './glyphView';
import { guideImage, stepGuide, type GuideHint } from './guideHints';
import type { RadarCell } from './radarView';

const STYLE = { stroke: '#000000', strokeThickness: 6, fontStyle: 'bold', ...fontStyle('playerValue') }; // 28 px, HUD-fest (Q03)
/** Höhe über dem Boden in Welt-Pixeln: über Münze und Bauplatz bzw. über dem Aktionen-Overlay des Monarchen */
const OBJECT_PX = 160;
const PLAYER_PX = 370;

/** Hinweis als Zeilenteile; `{key}` im Text wird zur Glyph der Bestätigen-Taste des Geräts, ein Bild steht davor (B-294) */
function partsOf(h: GuideHint, device: Device): RowPart[] {
  const [before = '', after] = t(`guide.${h.id}`, {}).split('{key}');
  const text: RowPart[] = after === undefined ? [before] : [before, glyphOf('confirm', device), after];
  const image = guideImage(h.id);
  return image ? [image, ...text] : text;
}

/**
 * Geführte erste Nacht (S6.3, B-148): je Spielerfeld der nächste ungesehene Hinweis über seinem Objekt, mit Glyph (S6.2).
 * Rein zum Ansehen, nimmt keine Eingabe; abschaltbar in den Optionen (`guideHints`).
 */
export class GuideOverlay {
  private rows: GlyphRow[] = [];
  private shown: (GuideHint | null)[] = [];

  constructor(private readonly scene: Phaser.Scene) {}

  /** Argumente wie `ActionOverlay.draw` */
  draw(cells: readonly RadarCell[], cameras: readonly Phaser.Cameras.Scene2D.Camera[], slotOf: (seat: number) => number | undefined, deviceOf: (slot: number) => Device): void {
    this.rows.forEach((r) => r.box.setVisible(false));
    const on = loadSettings().guideHints; // je Frame gelesen wie in `GameScene`: die Optionen wirken sofort
    cells.forEach(({ cell, monarch, world }, i) => {
      const p = monarch === null ? undefined : world?.players.find((q) => q.index === monarch);
      const slot = slotOf(cell.seat);
      const h = on && p && world ? stepGuide(this.shown[i] ?? null, world, p, guideSeen.ids, (id) => guideSeen.mark(id)) : null;
      this.shown[i] = h;
      const at = h && cameras[i] ? screenAt(cell, cameras[i].worldView, h.x, h.target === null ? PLAYER_PX : OBJECT_PX) : null;
      if (!h || !at || slot === undefined) return;
      (this.rows[i] ??= new GlyphRow(this.scene, STYLE, 'bottom')).set(partsOf(h, deviceOf(slot)), at.x, at.y);
    });
  }
}
