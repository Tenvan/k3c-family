import Phaser from 'phaser';
import { GAME_WIDTH, GROUND_Y as GAME_GROUND_Y } from '@game/core/constants';
import type { KampfWorld } from '@game/scenes/kampfAnzeige';
import { KampfHud, WarnView } from '@game/scenes/kampfView';
import { GROUND_Y } from '../playback/figureView';

// Boss-Anzeigen des Spiels auf der Bühne (B-469, WZ23.3): `KampfHud` (Boss-Leiste, Phasen-Text, Event-Banner) und `WarnView`
// (Warnkreis) aus `src/scenes/kampfView.ts` zeichnen den Beispielzustand; hier wird nur platziert. Das HUD des Spiels liegt
// bildschirmfest über der Szene, die Kamera der Bühne zoomt und scrollt aber: Ein Container ohne Scroll-Faktor rechnet den
// Kamera-Zoom heraus, sodass das HUD den Bildschirm wie im Spiel füllt (Breite = Spielbreite). Der Warnkreis liegt in der Welt,
// auf der Bodenlinie der Figuren (Studio und Spiel rechnen die Bodenlinie verschieden).

export class KampfOverlay {
  private box: Phaser.GameObjects.Container | null = null;
  private layer: Phaser.GameObjects.Layer | null = null;

  constructor(private readonly scene: Phaser.Scene) {}

  /** Zeigt den Zustand (null = nichts). */
  show(world: KampfWorld | null): void {
    this.clear();
    if (!world) return;
    const before = new Set(this.scene.children.list);
    new KampfHud(this.scene).draw(world);
    const hud = this.scene.children.list.filter((o) => !before.has(o));
    this.box = this.scene.add.container(0, 0, hud).setScrollFactor(0).setDepth(1000);
    this.layer = this.scene.add.layer().setDepth(5);
    new WarnView(this.scene, this.layer).sync(world);
    // Layer haben keine Lage: die Zeichnung des Warnkreises selbst auf die Bodenlinie der Figuren verschieben
    for (const o of this.layer.list) (o as Phaser.GameObjects.Graphics).setY(GROUND_Y - GAME_GROUND_Y);
    this.place();
  }

  /** HUD an den aktuellen Kamera-Zoom anpassen (jeden Frame: Zoom und Fenstergröße ändern sich). */
  place(): void {
    if (!this.box) return;
    const cam = this.scene.cameras.main;
    const { width, height } = this.scene.scale;
    const z = cam.zoom;
    // ohne Scroll-Faktor wirkt nur der Zoom um die Bildmitte: Bildschirm = (x − Mitte) · z + Mitte
    this.box.setScale(width / GAME_WIDTH / z).setPosition((0 - width / 2) / z + width / 2, (0 - height / 2) / z + height / 2);
  }

  clear(): void {
    this.box?.destroy(); // samt der Objekte des HUD
    this.layer?.destroy(true);
    this.box = this.layer = null;
  }
}
