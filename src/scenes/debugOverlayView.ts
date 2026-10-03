import Phaser from 'phaser';
import { PROTOCOL_VERSION } from '../online/clientProtocol';
import type { RoomClient } from '../online/clientConnection';
import type { World } from '../model/types';
import { VERSION_KEY, debugLines } from './debugOverlay';

/** Linker Stick (Klick), standard mapping. B (1) und View + Menu (8 + 9) bleiben unberührt. */
const PAD_LS = 10;
// keyCode 192 ist auf deutscher Tastatur Ö (Phaser nennt ihn BACKTICK, US-Layout: `).

/**
 * Debug-Overlay (B-093): Text oben links, Ö (Tastatur) oder Klick auf den linken Stick schaltet um. D wäre „laufen“, F3 ist im Browser belegt.
 * Nur Lesezugriff; die Zeilen kommen aus `debugLines`. Wird nur erzeugt, wenn `debugEnabled` gilt (Standard an, `?dev=0` aus).
 */
export class DebugOverlay {
  private readonly text: Phaser.GameObjects.Text;
  private readonly key: Phaser.Input.Keyboard.Key | undefined;
  private padHeld = false;
  private shown = false;

  constructor(private readonly scene: Phaser.Scene) {
    this.key = scene.input.keyboard?.addKey(Phaser.Input.Keyboard.KeyCodes.BACKTICK);
    this.text = scene.add
      .text(20, 96, '', { fontSize: '20px', color: '#9be564', stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' })
      .setVisible(false);
  }

  update(client: RoomClient, world: World | null): void {
    if (this.toggled()) this.shown = !this.shown;
    this.text.setVisible(this.shown);
    if (!this.shown) return;
    const lines = debugLines({ client, protocol: PROTOCOL_VERSION, world, fps: this.scene.game.loop.actualFps, now: performance.now(), version: this.scene.registry.get(VERSION_KEY) as string | undefined });
    this.text.setText(lines);
  }

  /** Flanke von Ö oder LS (irgendein Controller). */
  private toggled(): boolean {
    const keyHit = !!this.key && Phaser.Input.Keyboard.JustDown(this.key);
    const pads = this.scene.input.gamepad?.gamepads ?? [];
    const padDown = pads.some((p) => p?.buttons[PAD_LS]?.pressed);
    const padHit = padDown && !this.padHeld;
    this.padHeld = padDown;
    return keyHit || padHit;
  }
}
