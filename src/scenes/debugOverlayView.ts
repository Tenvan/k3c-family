import Phaser from 'phaser';
import { PROTOCOL_VERSION } from '../online/clientProtocol';
import type { RoomClient } from '../online/clientConnection';
import type { World } from '../model/types';
import { DEV_ACTIONS, actionsVisible, devMessage, roomDevMode } from './debugActions';
import { VERSION_KEY, debugLines, type DebugWorld } from './debugOverlay';
import { DEV_FOCUS_KEY, DevActionPanel, PAD_FOCUS, focusStep, type FocusEdges, type FocusState } from './debugOverlayPanel';

/** Linker Stick (Klick), standard mapping. B (1) und View + Menu (8 + 9) bleiben unberührt. */
const PAD_LS = 10;
// keyCode 192 ist auf deutscher Tastatur Ö (Phaser nennt ihn BACKTICK, US-Layout: `).

/**
 * Debug-Overlay (B-093): Text oben links, Ö (Tastatur) oder Klick auf den linken Stick schaltet um. D wäre „laufen“, F3 ist im Browser belegt.
 * Im Dev-Mode des Raums zusätzlich die Dev-Aktionen (B-179, `debugOverlayPanel.ts`). Wird nur erzeugt, wenn `debugEnabled` gilt.
 */
export class DebugOverlay {
  private readonly text: Phaser.GameObjects.Text;
  private readonly key: Phaser.Input.Keyboard.Key | undefined;
  private padHeld = false;
  private shown = false;
  private padPrev = new Set<number>();
  private fs: FocusState = { focus: false, index: 0, seat: 0 };
  private panel: DevActionPanel | null = null;
  private client: RoomClient | null = null;

  constructor(private readonly scene: Phaser.Scene) {
    this.key = scene.input.keyboard?.addKey(Phaser.Input.Keyboard.KeyCodes.BACKTICK);
    this.text = scene.add
      .text(20, 96, '', { fontSize: '20px', color: '#9be564', stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' })
      .setVisible(false);
    scene.events.once(Phaser.Scenes.Events.SHUTDOWN, () => this.destroy());
  }

  update(client: RoomClient, world: World | null): void {
    this.client = client;
    if (this.toggled()) this.shown = !this.shown;
    this.text.setVisible(this.shown);
    this.updateActions(client, world);
    if (!this.shown) return;
    const lines = debugLines({ client, protocol: PROTOCOL_VERSION, world, fps: this.scene.game.loop.actualFps, now: performance.now(), version: this.scene.registry.get(VERSION_KEY) as string | undefined });
    this.text.setText(lines);
  }

  /** Aktionsliste nur bei offenem Overlay im Dev-Mode des Raums; Controller-Fokus per RB, Eingabesperre über die Registry. */
  private updateActions(client: RoomClient, world: World | null): void {
    const devMode = client.status === 'room' && roomDevMode(world as DebugWorld | null);
    const visible = actionsVisible({ overlayOn: this.shown, devMode });
    const { state, fire } = focusStep(this.fs, this.padEdges(), visible, this.slots().length);
    this.fs = state;
    if (fire !== null) this.fire(fire);
    this.scene.registry.set(DEV_FOCUS_KEY, state.focus);
    if (visible) this.panel ??= new DevActionPanel((i) => this.fire(i), () => this.nextSeat());
    this.panel?.render(visible, this.fs, `Spieler ${this.seat() + 1}`);
  }

  /** Lokale Slots des Geräts, aufsteigend (Gold und Material gehen an den gewählten). */
  private slots(): number[] {
    return (this.client?.you ?? []).map((s) => s.slot).sort((a, b) => a - b);
  }

  private seat(): number {
    return this.fs.seat % Math.max(1, this.slots().length);
  }

  private nextSeat(): void {
    this.fs = { ...this.fs, seat: (this.seat() + 1) % Math.max(1, this.slots().length) };
  }

  private fire(index: number): void {
    const message = devMessage(DEV_ACTIONS[index]?.key ?? '', this.slots()[this.seat()] ?? 0);
    if (message) this.client?.sendDev(message);
  }

  /** Neu gedrückte Fokus-Tasten (irgendein Controller). */
  private padEdges(): FocusEdges {
    const pads = this.scene.input.gamepad?.gamepads ?? [];
    const now = new Set<number>();
    for (const b of Object.values(PAD_FOCUS)) if (pads.some((p) => p?.buttons[b]?.pressed)) now.add(b);
    const edges: FocusEdges = {};
    for (const [name, b] of Object.entries(PAD_FOCUS)) edges[name as keyof typeof PAD_FOCUS] = now.has(b) && !this.padPrev.has(b);
    this.padPrev = now;
    return edges;
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

  /** Szene endet (zurück zur Lobby): Schaltflächen entfernen, Eingabesperre lösen. */
  private destroy(): void {
    this.panel?.destroy();
    this.panel = null;
    this.scene.registry.set(DEV_FOCUS_KEY, false);
  }
}
