import type Phaser from 'phaser';

/**
 * Kamera-Culling der Welt (PF1.2, B-194): Phaser zeichnet Container ohne eigenes Culling in jeder Kamera, auch weit
 * außerhalb ihres Ausschnitts. Gemessen (PF1.1) liegen von rund 145 Welt-Containern nur rund 25 in einer der Kameras.
 * Ein Container außerhalb des Ausschnitts einer Kamera bekommt deren Bit in `cameraFilter`, je Kamera getrennt.
 */

/** Rand in Welt-Pixeln links und rechts vom Ausschnitt: breiteste Objekte (Burg, Hub-Gebäude, Gerüste) ragen nicht hinein */
export const CULL_MARGIN = 400;

export interface CullView {
  id: number;
  left: number;
  right: number;
}

/** Filter-Bits für ein Objekt an `x`: die Kameras, deren Ausschnitt (mit Rand) es nicht berührt */
export function cullMask(x: number, views: readonly CullView[], margin = CULL_MARGIN): number {
  let mask = 0;
  for (const v of views) if (x < v.left - margin || x > v.right + margin) mask |= v.id;
  return mask;
}

/**
 * Setzt den Filter der Welt-Container einer Stufen-Ebene neu; nur Container mit Scroll-Faktor 1 (Hintergründe und
 * bildschirmfeste Teile bleiben unberührt). `cams` sind die Kameras der Szene, ihr `worldView` gilt für diesen Frame.
 */
export function cullLayer(layer: Phaser.GameObjects.Layer, cams: readonly Phaser.Cameras.Scene2D.Camera[]): void {
  const views = cams.map((c) => ({ id: c.id, left: c.worldView.x, right: c.worldView.right }));
  for (const o of layer.list) {
    const c = o as Phaser.GameObjects.Container;
    if (c.type !== 'Container' || c.scrollFactorX !== 1) continue;
    c.cameraFilter = cullMask(c.x, views);
  }
}
