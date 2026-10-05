import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { guideSeen } from '../core/guideSeen';
import { loadSettings, saveSettings, type Settings } from '../core/settings';
import { setLanguage, t } from '../core/texts';
import {
  LEAVE_KEY,
  MenuPress,
  OPTION_IDS,
  OPTION_PAD_KEYS as PAD,
  applyOption,
  moveOption,
  optionLabel,
  tapDir,
  type OptionDir,
} from './optionsLogic';

const TOP = 230;
const ROW_H = 86; // 9 Zeilen über dem Hinweis unten
const FONT_PX = 40; // ≥ 28 px (Q03)
const STICK = 0.5;
const STYLE = { fontFamily: 'sans-serif', fontStyle: 'bold', color: '#ffffff', stroke: '#000000', strokeThickness: 6 };

function keyAction(k: OptionsScene['keys']): OptionDir | null {
  const down = Phaser.Input.Keyboard.JustDown;
  if (down(k.left)) return 'left';
  if (down(k.right)) return 'right';
  return down(k.enter) || down(k.space) ? 'confirm' : null;
}

/**
 * Optionen und Pause (Client-Anteil, S5.2): liegt über dem Spiel, die Eingaben der Monarchen ruhen (`GameScene`).
 * Jedes Gerät (Tastatur, Pads, Tippen) steuert sie; Menu/Esc oder „Weiter“ schließt. Jede Änderung wird sofort gespeichert.
 */
export class OptionsScene extends Phaser.Scene {
  private settings!: Settings;
  private selected = 0;
  private rows: Phaser.GameObjects.Text[] = [];
  private title!: Phaser.GameObjects.Text;
  private hint!: Phaser.GameObjects.Text;
  private keys!: Record<'up' | 'down' | 'w' | 's' | 'left' | 'right' | 'enter' | 'space' | 'esc', Phaser.Input.Keyboard.Key>;
  private padHeld = new Set<string>();
  private menu = new MenuPress();
  private tapped: { row: number; dir: OptionDir } | null = null;

  constructor() {
    super('options');
  }

  create(): void {
    this.settings = loadSettings();
    setLanguage(this.settings.language);
    this.selected = 0;
    this.padHeld = new Set();
    this.menu = new MenuPress();
    this.tapped = null;
    const K = Phaser.Input.Keyboard.KeyCodes;
    this.keys = this.input.keyboard!.addKeys({ up: K.UP, down: K.DOWN, w: K.W, s: K.S, left: K.LEFT, right: K.RIGHT, enter: K.ENTER, space: K.SPACE, esc: K.ESC }) as OptionsScene['keys'];
    this.add.rectangle(0, 0, GAME_WIDTH, GAME_HEIGHT, 0x000000, 0.8).setOrigin(0);
    this.title = this.add.text(GAME_WIDTH / 2, 130, '', { ...STYLE, fontSize: '64px', strokeThickness: 10 }).setOrigin(0.5);
    this.hint = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 60, '', { ...STYLE, fontSize: '28px', strokeThickness: 4 }).setOrigin(0.5);
    this.rows = OPTION_IDS.map((_, i) => this.add.text(GAME_WIDTH / 2, TOP + i * ROW_H, '', { ...STYLE, fontSize: `${FONT_PX}px` }).setOrigin(0.5));
    // Touch über das Fenster, wie in der Lobby: das Touch-Overlay liegt über dem Canvas, die Ereignisse laufen bis hierher.
    const onTap = (e: PointerEvent) => {
      const row = Math.round((this.scale.transformY(e.pageY) - TOP) / ROW_H);
      if (row >= 0 && row < OPTION_IDS.length) this.tapped = { row, dir: tapDir(this.scale.transformX(e.pageX), GAME_WIDTH) };
    };
    window.addEventListener('pointerdown', onTap);
    this.events.once('shutdown', () => window.removeEventListener('pointerdown', onTap));
  }

  update(): void {
    const { dir, act, close } = this.poll();
    this.selected = moveOption(this.selected, dir);
    let want: OptionDir | null = act;
    if (this.tapped) {
      this.selected = this.tapped.row;
      want = this.tapped.dir;
      this.tapped = null;
    }
    if (want) {
      const result = applyOption(this.settings, OPTION_IDS[this.selected]!, want);
      if (result.settings !== this.settings) {
        if (result.settings.guideHints && !this.settings.guideHints) guideSeen.reset(); // wieder an = alle Hinweise erneut (Q11)
        saveSettings((this.settings = result.settings));
        setLanguage(this.settings.language);
      }
      if (result.close) return this.close();
      if (result.leave) {
        this.registry.set(LEAVE_KEY, true); // GameScene verlässt den Raum und schließt diese Szene
        return;
      }
    }
    if (close) return this.close();
    this.title.setText(t('opt.title'));
    this.hint.setText(t('opt.hint'));
    this.rows.forEach((row, i) => {
      const id = OPTION_IDS[i]!;
      row.setText(`${i === this.selected ? '▶  ' : '    '}${optionLabel(this.settings, id)}`).setColor(i === this.selected ? '#ffd166' : '#ffffff');
    });
  }

  private close(): void {
    this.scene.stop('options');
  }

  /** Tastatur und alle Pads: Auswahl (-1/0/1), Aktion (Links/Rechts/Bestätigen) und „schließen“, je einmal pro Druck. */
  private poll(): { dir: number; act: OptionDir | null; close: boolean } {
    const down = Phaser.Input.Keyboard.JustDown;
    const k = this.keys;
    let dir = (down(k.down) || down(k.s) ? 1 : 0) - (down(k.up) || down(k.w) ? 1 : 0);
    let act = keyAction(k);
    let close = down(k.esc);

    const held = this.heldPadKeys();
    const edge = (name: string) => held.has(name) && !this.padHeld.has(name);
    dir += (edge('down') ? 1 : 0) - (edge('up') ? 1 : 0);
    act ??= edge('left') ? 'left' : edge('right') ? 'right' : edge('a') ? 'confirm' : null;
    close ||= this.menu.update(held.has('menu'), held.has('view'), performance.now());
    this.padHeld = held;
    return { dir: Math.sign(dir), act, close };
  }

  /** Gerade gehaltene Tasten aller Pads (B bleibt unbelegt). */
  private heldPadKeys(): Set<string> {
    const held = new Set<string>();
    const names: [string, number][] = [['a', PAD.A], ['view', PAD.VIEW], ['menu', PAD.MENU], ['up', PAD.UP], ['down', PAD.DOWN], ['left', PAD.LEFT], ['right', PAD.RIGHT]];
    for (const pad of this.input.gamepad?.gamepads ?? []) {
      if (!pad) continue;
      for (const [name, index] of names) if (pad.buttons[index]?.pressed) held.add(name);
      if (pad.leftStick.y < -STICK) held.add('up');
      if (pad.leftStick.y > STICK) held.add('down');
      if (pad.leftStick.x < -STICK) held.add('left');
      if (pad.leftStick.x > STICK) held.add('right');
    }
    return held;
  }
}
