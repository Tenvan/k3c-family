import Phaser from 'phaser';
import { GAME_HEIGHT } from '../core/constants';
import { PROTOCOL_VERSION } from '../online/clientProtocol';
import type { RoomClient } from '../online/clientConnection';
import type { World } from '../model/types';
import { DEV_ACTIONS, devMessage, pauseMessage, roomDevMode } from './debugActions';
import { VERSION_KEY, debugLines, type DebugWorld } from './debugOverlay';
import type { GameScene } from './GameScene';
import { HOLD_IDLE, holdStep, listenTaps, type DebugGesture, type HoldState } from './debugGestures';
import { CheatDialog, DEV_FOCUS_KEY, PAD_FOCUS, diagGesture, focusStep, type FocusEdges, type FocusState } from './debugOverlayPanel';

/** Schultertasten (standard mapping). B (1) und View + Menu (8 + 9) bleiben unberührt. */
const PAD_LB = 4;
const PAD_RB = 5;
/** Unterkante der Info-Zeilen: über der Skill-Zeile links unten (GAME_HEIGHT - 56), HUD oben bleibt frei (B-191). */
const TEXT_BOTTOM = GAME_HEIGHT - 70;
// Deutsche Tastatur: keyCode 192 ist Ö (Phaser BACKTICK), 222 ist Ä (Phaser QUOTES).

/**
 * Debug-Anzeige und Cheat-Dialog (B-093, B-231). Ö, RB 3 s halten oder Doppeltap mit einem Finger schaltet die
 * Diagnose (Text links unten; bei offenem Dialog schließt Ö zuerst den Dialog). Ä, LB + RB 3 s halten oder Doppeltap mit zwei Fingern öffnet den Cheat-Dialog: modal,
 * der Raum steht (Dev-Aktion `pause`), bis er schließt. Wird nur erzeugt, wenn `debugEnabled` gilt.
 */
export class DebugOverlay {
  private readonly text: Phaser.GameObjects.Text;
  private readonly diagKey: Phaser.Input.Keyboard.Key | undefined;
  private readonly cheatKey: Phaser.Input.Keyboard.Key | undefined;
  private readonly unlisten: () => void;
  private shown = false;
  private open = false;
  private hold: HoldState = HOLD_IDLE;
  private gestures: DebugGesture[] = [];
  private padPrev = new Set<number>();
  private fs: FocusState = { index: 0, seat: 0 };
  private dialog: CheatDialog | null = null;
  private client: RoomClient | null = null;

  constructor(private readonly scene: Phaser.Scene) {
    const K = Phaser.Input.Keyboard.KeyCodes;
    this.diagKey = scene.input.keyboard?.addKey(K.BACKTICK);
    this.cheatKey = scene.input.keyboard?.addKey(K.QUOTES);
    this.text = scene.add
      .text(20, TEXT_BOTTOM, '', { fontSize: '20px', color: '#9be564', stroke: '#000000', strokeThickness: 4, fontStyle: 'bold' })
      .setOrigin(0, 1)
      .setVisible(false);
    this.unlisten = listenTaps((g) => this.gestures.push(g));
    scene.events.once(Phaser.Scenes.Events.SHUTDOWN, () => this.destroy());
  }

  update(client: RoomClient, world: World | null): void {
    this.client = client;
    for (const g of this.takeGestures()) {
      if (g === 'diag') {
        const next = diagGesture(this.shown, this.open);
        this.shown = next.shown;
        this.setOpen(next.open);
      } else this.setOpen(!this.open);
    }
    this.text.setVisible(this.shown);
    this.updateDialog(client, world);
    if (!this.shown) return;
    const lines = debugLines({ client, protocol: PROTOCOL_VERSION, world, fps: this.scene.game.loop.actualFps, now: performance.now(), delayMs: (this.scene.scene.get('game') as GameScene | null)?.delayMs, version: this.scene.registry.get(VERSION_KEY) as string | undefined });
    this.text.setText(lines);
  }

  /** Gesten dieses Frames: Tasten (Flanke), Schultertasten (halten; bei offenem Dialog schließt LB + RB sofort), Touch. */
  private takeGestures(): DebugGesture[] {
    const out = this.gestures;
    this.gestures = [];
    if (this.diagKey && Phaser.Input.Keyboard.JustDown(this.diagKey)) out.push('diag');
    if (this.cheatKey && Phaser.Input.Keyboard.JustDown(this.cheatKey)) out.push('cheats');
    const pads = this.scene.input.gamepad?.gamepads ?? [];
    const held = (b: number): boolean => pads.some((p) => p?.buttons[b]?.pressed);
    const lb = held(PAD_LB);
    const rb = held(PAD_RB);
    const r = holdStep(this.hold, lb, rb, performance.now(), this.open && lb && rb ? 0 : undefined);
    this.hold = r.state;
    if (r.fire) out.push(r.fire);
    return out;
  }

  /** Öffnen hält den Raum an, Schließen lässt ihn weiterlaufen; die Eingabesperre der Controller geht über die Registry. */
  private setOpen(open: boolean): void {
    if (open === this.open) return;
    this.open = open;
    this.client?.sendDev(pauseMessage(open));
    this.scene.registry.set(DEV_FOCUS_KEY, open);
  }

  private updateDialog(client: RoomClient, world: World | null): void {
    const { state, fire } = focusStep(this.fs, this.padEdges(), this.open, this.slots().length);
    this.fs = state;
    if (fire !== null) this.fire(fire);
    if (this.open) this.dialog ??= new CheatDialog((i) => this.fire(i), () => this.nextSeat(), () => this.setOpen(false));
    const devMode = client.status === 'room' && roomDevMode(world as DebugWorld | null);
    const note = devMode ? '' : 'Server ohne Dev-Mode: Cheats und Pause wirken nicht.';
    this.dialog?.render(this.open, this.fs, `Spieler ${this.seat() + 1}`, note);
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

  /** Neu gedrückte Bedien-Tasten (irgendein Controller). */
  private padEdges(): FocusEdges {
    const pads = this.scene.input.gamepad?.gamepads ?? [];
    const now = new Set<number>();
    for (const b of Object.values(PAD_FOCUS)) if (pads.some((p) => p?.buttons[b]?.pressed)) now.add(b);
    const edges: FocusEdges = {};
    for (const [name, b] of Object.entries(PAD_FOCUS)) edges[name as keyof typeof PAD_FOCUS] = now.has(b) && !this.padPrev.has(b);
    this.padPrev = now;
    return edges;
  }

  /** Szene endet (zurück zur Lobby): Raum weiterlaufen lassen, Dialog und Touch-Lauscher entfernen, Eingabesperre lösen. */
  private destroy(): void {
    this.setOpen(false);
    this.unlisten();
    this.dialog?.destroy();
    this.dialog = null;
    this.scene.registry.set(DEV_FOCUS_KEY, false);
  }
}
