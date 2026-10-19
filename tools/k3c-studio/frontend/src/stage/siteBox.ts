import Phaser from 'phaser';
import { GROUND_Y, UNIT_PX } from '@game/core/constants';
import { SITE_SIZE } from '@game/scenes/render/siteView';
import type { Site } from '@game/model/types';

// Umriss eines Bauplatzes für Rahmen und Namen im Gebäude-Planer: aus der Grafik, die der Renderer wirklich zeigt (sichtbare
// Pixel, ohne transparenten Rand), damit der Rahmen auch nach einem Tausch der Grafiken (Asset-Sets) passt. Ohne Grafik gilt
// die Platzhalter-Größe des Spiels.

export interface Box {
  left: number;
  right: number;
  top: number;
  bottom: number;
}

const ALPHA_MIN = 24;
const opaque = new Map<string, readonly [number, number, number, number] | null>();

/** Sichtbarer Bereich der Textur in Bildpunkten des Ausschnitts (links, oben, rechts, unten); null, wenn nicht lesbar. */
function opaqueBounds(frame: Phaser.Textures.Frame): readonly [number, number, number, number] | null {
  const id = `${frame.texture.key}|${frame.name}`;
  if (opaque.has(id)) return opaque.get(id) ?? null;
  let bounds: readonly [number, number, number, number] | null = null;
  try {
    const canvas = document.createElement('canvas');
    canvas.width = frame.cutWidth;
    canvas.height = frame.cutHeight;
    const ctx = canvas.getContext('2d', { willReadFrequently: true });
    ctx?.drawImage(frame.source.image as CanvasImageSource, frame.cutX, frame.cutY, frame.cutWidth, frame.cutHeight, 0, 0, frame.cutWidth, frame.cutHeight);
    const data = ctx?.getImageData(0, 0, frame.cutWidth, frame.cutHeight).data;
    if (data) bounds = scan(data, frame.cutWidth, frame.cutHeight);
  } catch { /* Pixel nicht lesbar: der Rahmen nimmt das ganze Bild */ }
  opaque.set(id, bounds);
  return bounds;
}

function scan(data: Uint8ClampedArray, w: number, h: number): readonly [number, number, number, number] | null {
  let x0 = w; let y0 = h; let x1 = -1; let y1 = -1;
  for (let y = 0; y < h; y++) {
    for (let x = 0; x < w; x++) {
      if (data[(y * w + x) * 4 + 3]! < ALPHA_MIN) continue;
      if (x < x0) x0 = x;
      if (x > x1) x1 = x;
      if (y < y0) y0 = y;
      y1 = y;
    }
  }
  return x1 < 0 ? null : [x0, y0, x1 + 1, y1 + 1];
}

/** Umriss in Welt-Pixeln des Bauplatzes `site`, dessen Ansicht (Container des Renderers) in `views` liegt. */
export function siteBox(site: Pick<Site, 'kind' | 'x'>, views: readonly Phaser.GameObjects.GameObject[]): Box {
  const x = site.x * UNIT_PX;
  const [w, h] = SITE_SIZE[site.kind] ?? [100, 100];
  const fallback: Box = { left: x - w / 2, right: x + w / 2, top: GROUND_Y - h, bottom: GROUND_Y };
  const view = views.find((o): o is Phaser.GameObjects.Container => o instanceof Phaser.GameObjects.Container && o.x === x && o.y === GROUND_Y && sameSize(o, w, h));
  const picture = view?.getAt(0);
  if (!(picture instanceof Phaser.GameObjects.Image) || !picture.visible) return fallback;
  const b = opaqueBounds(picture.frame) ?? [0, 0, picture.width, picture.height];
  const s = picture.scaleX;
  return {
    left: x + (b[0] - picture.width / 2) * s,
    right: x + (b[2] - picture.width / 2) * s,
    top: GROUND_Y - (picture.height - b[1]) * s,
    bottom: GROUND_Y - (picture.height - b[3]) * s,
  };
}

function sameSize(view: Phaser.GameObjects.Container, w: number, h: number): boolean {
  const size = view.getData('size') as [number, number] | undefined;
  return size?.[0] === w && size[1] === h;
}
