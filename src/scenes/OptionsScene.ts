import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { audioCore } from '../audio/audioCore';
import { playUi, uiSoundFor } from '../audio/ui';
import { guideSeen } from '../core/guideSeen';
import { loadSettings, saveSettings, type Settings } from '../core/settings';
import { setLanguage, t } from '../core/texts';
import {
  LEAVE_KEY,
  MenuPress,
  OPTION_PAD_KEYS as PAD,
  REMOVE_PLAYER_KEY,
  SETTINGS_IDS,
  applyOption,
  moveOption,
  optionLabel,
  syncVolumes,
  tapOption,
  OPTIONS_LAYOUT,
  KEYS_LAYOUT,
  worldLine,
  type OptionDir,
  type OptionResult,
  type OptionsSceneData,
} from './optionsLogic';
import { HUB_COLORS, HUB_LAYOUT, PAUSE_TILES, hubAction, hubChoice, hubDir, hubEnabled, hubLabel, hubSub, hubTileAt, moveHub, type HubAction, type HubDir } from './pauseHub';
import { HubView } from './hubView';
import { pauseButton } from './hud/pauseButton';
import { keyTable } from './keyTable';
import { GlyphRow } from './hints/glyphView';

const { top: TOP, rowH: ROW_H, rowPx: FONT_PX } = OPTIONS_LAYOUT; // Zeilen der Seite „Optionen“ über dem Hinweis unten, Zeilen ≥ 28 px (Q03)
const STICK = 0.5;
const STYLE = { fontFamily: 'sans-serif', fontStyle: 'bold', color: '#ffffff', stroke: '#000000', strokeThickness: 6 };

/**
 * Touch-Overlay (`.k3c-zone`, `.k3c-touch`) und ☰ verdecken die Zeilen (B-336): solange die Szene offen ist ausblenden.
 * `src/input/touchInput.ts` (PLAT) bleibt unverändert; die Rückgabe stellt die vorigen Werte wieder her.
 */
function hideTouchControls(): () => void {
  const els = [...document.querySelectorAll<HTMLElement>('.k3c-zone, .k3c-touch')];
  const before = els.map((el) => el.style.display);
  els.forEach((el) => (el.style.display = 'none'));
  const restoreButton = pauseButton().suspend();
  return () => {
    els.forEach((el, i) => (el.style.display = before[i] ?? ''));
    restoreButton();
  };
}

function keyAction(k: OptionsScene['keys']): OptionDir | null {
  const down = Phaser.Input.Keyboard.JustDown;
  if (down(k.left)) return 'left';
  if (down(k.right)) return 'right';
  return down(k.enter) || down(k.space) ? 'confirm' : null;
}

/** Zwei Seiten: das Pausenmenü als Kachelseite (`hub`, B-768) und darunter die Optionen (`settings`) */
type Mode = 'hub' | 'settings';

/**
 * Pause und Optionen (Client-Anteil, S5.2): liegt über dem Spiel, die Eingaben der Monarchen ruhen (`GameScene`).
 * Jedes Gerät (Tastatur, Pads, Tippen) steuert sie; Menu/Esc geht eine Seite zurück, auf der Kachelseite ins Spiel.
 * Jede Änderung der Optionen wird sofort gespeichert.
 */
export class OptionsScene extends Phaser.Scene {
  private settings!: Settings;
  private mode: Mode = 'hub';
  /** Auswahl auf der Seite „Optionen“ (Zeile) und auf der Kachelseite (Kachel) */
  private selected = 0;
  private hubSel = 0;
  private rows: Phaser.GameObjects.Text[] = [];
  private title!: Phaser.GameObjects.Text;
  private hint!: Phaser.GameObjects.Text;
  private hubView!: HubView;
  /** Von der Startseite der Lobby geöffnet (B-768): nur die Optionen, Menu/Esc schließt die Szene */
  private standalone = false;
  private keys!: Record<'up' | 'down' | 'w' | 's' | 'left' | 'right' | 'enter' | 'space' | 'esc', Phaser.Input.Keyboard.Key>;
  private padHeld = new Set<string>();
  private menu = new MenuPress();
  private tapped: { row: number; dir: OptionDir } | null = null;
  private tappedTile: number | null = null;
  /** Slots, die „Spieler abmelden“ treffen kann (leer = ausgegraut), und der gewählte davon (B-411) */
  private targets: number[] = [];
  private choice = 0;
  /** Welt des Raums (BD5.2) */
  private world: string | undefined;
  /** Seite „Tastenbelegung“ (B-741) offen? Die Texte der Tabelle entstehen beim Öffnen */
  private keysTable: Phaser.GameObjects.GameObject[] | null = null;

  constructor() {
    super('options');
  }

  init(data?: Partial<OptionsSceneData>): void {
    this.targets = data?.targets ?? [];
    this.world = data?.world;
    this.standalone = data?.standalone ?? false;
    this.choice = 0;
  }

  create(): void {
    this.settings = loadSettings();
    setLanguage(this.settings.language);
    this.mode = this.standalone ? 'settings' : 'hub';
    this.selected = 0;
    this.hubSel = 0;
    this.keysTable = null;
    this.padHeld = new Set();
    this.menu = new MenuPress();
    this.tapped = null;
    this.tappedTile = null;
    const K = Phaser.Input.Keyboard.KeyCodes;
    this.keys = this.input.keyboard!.addKeys({ up: K.UP, down: K.DOWN, w: K.W, s: K.S, left: K.LEFT, right: K.RIGHT, enter: K.ENTER, space: K.SPACE, esc: K.ESC }) as OptionsScene['keys'];
    this.add.rectangle(0, 0, GAME_WIDTH, GAME_HEIGHT, HUB_COLORS.night, 0.93).setOrigin(0);
    this.title = this.add.text(GAME_WIDTH / 2, OPTIONS_LAYOUT.titleY, '', { ...STYLE, fontSize: `${OPTIONS_LAYOUT.titlePx}px`, strokeThickness: 10 }).setOrigin(0.5);
    this.add.text(GAME_WIDTH / 2, OPTIONS_LAYOUT.worldY, worldLine(this.world), { ...STYLE, fontSize: `${OPTIONS_LAYOUT.worldPx}px`, strokeThickness: 6 }).setOrigin(0.5);
    this.hint = this.add.text(GAME_WIDTH / 2, HUB_LAYOUT.hintY, '', { ...STYLE, fontSize: `${HUB_LAYOUT.hintPx}px`, strokeThickness: 4 }).setOrigin(0.5);
    this.rows = SETTINGS_IDS.map((_, i) => this.add.text(GAME_WIDTH / 2, TOP + i * ROW_H, '', { ...STYLE, fontSize: `${FONT_PX}px` }).setOrigin(0.5));
    this.createHub();
    this.refreshView();
    // Touch über das Fenster, wie in der Lobby; das Touch-Overlay ist ausgeblendet, solange die Szene offen ist.
    const onTap = (e: PointerEvent) => {
      const x = this.scale.transformX(e.pageX);
      const y = this.scale.transformY(e.pageY);
      if (this.mode === 'hub') {
        const tile = hubTileAt(PAUSE_TILES, x, y);
        if (tile >= 0) this.tappedTile = tile;
        return;
      }
      const row = Math.round((y - TOP) / ROW_H);
      const id = SETTINGS_IDS[row];
      if (id) this.tapped = { row, dir: tapOption(id, x, GAME_WIDTH) };
    };
    window.addEventListener('pointerdown', onTap);
    const restoreTouch = hideTouchControls();
    this.events.once('shutdown', () => {
      window.removeEventListener('pointerdown', onTap);
      restoreTouch();
    });
  }

  /** Kachelseite anlegen; gezeichnet wird sie in `drawHub` */
  private createHub(): void {
    this.hubView = new HubView(this, PAUSE_TILES);
  }

  /** Zeigt je Seite die richtigen Teile: Kacheln, Optionszeilen oder die Tastentabelle */
  private refreshView(): void {
    const hub = this.mode === 'hub' && !this.keysTable;
    const settings = this.mode === 'settings' && !this.keysTable;
    this.hubView.setVisible(hub);
    this.rows.forEach((row) => row.setVisible(settings));
    if (this.keysTable) return;
    this.title.setText(hub ? t('hub.title') : t('opt.title'));
    this.hint.setText(hub ? t('hub.hint') : t('opt.hint'));
  }

  update(): void {
    const { dir, act, close } = this.poll();
    if (this.keysTable) return this.updateKeys(act !== null || close || !!this.tapped || this.tappedTile !== null);
    if (this.mode === 'hub') return this.updateHub(dir, act, close);
    playUi(uiSoundFor({ dir, side: act === 'left' || act === 'right' ? 1 : 0, confirm: act === 'confirm' || !!this.tapped })); // UI-Klick (B-575)
    this.selected = moveOption(this.selected, dir, SETTINGS_IDS.length);
    let want: OptionDir | null = act;
    if (this.tapped) {
      this.selected = this.tapped.row;
      want = this.tapped.dir;
      this.tapped = null;
    }
    if (want) this.applyResult(applyOption(this.settings, SETTINGS_IDS[this.selected]!, want));
    if (close) return this.backToHub();
    this.title.setText(t('opt.title'));
    this.hint.setText(t('opt.hint'));
    this.rows.forEach((row, i) => {
      const id = SETTINGS_IDS[i]!;
      row.setText(`${i === this.selected ? '▶  ' : '    '}${optionLabel(this.settings, id)}`).setColor(i === this.selected ? '#ffd166' : '#ffffff');
    });
  }

  /** Kachelseite: Pfeile und Stick bewegen die Auswahl in vier Richtungen, A/Enter bestätigt, Menu/Esc geht ins Spiel */
  private updateHub(dir: number, act: OptionDir | null, close: boolean): void {
    playUi(uiSoundFor({ dir, side: act === 'left' || act === 'right' ? 1 : 0, confirm: act === 'confirm' || this.tappedTile !== null })); // UI-Klick (B-575)
    const confirm = this.moveHubSelection(hubDir(dir, act)) || act === 'confirm';
    const action = confirm ? hubAction(PAUSE_TILES[this.hubSel]!.id, this.targets) : null;
    if (action && this.runHub(action)) return;
    if (close) return this.close();
    if (this.mode === 'hub') this.drawHub();
  }

  /** Tippen wählt und bestätigt die Kachel (`true`); eine Richtung bewegt die Auswahl oder wählt das Ziel von „Spieler abmelden“ */
  private moveHubSelection(d: HubDir | null): boolean {
    if (this.tappedTile !== null) {
      this.hubSel = this.tappedTile;
      this.tappedTile = null;
      return true;
    }
    if (!d) return false;
    const choice = hubChoice(PAUSE_TILES[this.hubSel]!.id, d, this.choice, this.targets.length);
    if (choice !== null) this.choice = choice;
    else this.hubSel = moveHub(PAUSE_TILES, this.hubSel, d);
    return false;
  }

  /** Wirkt eine bestätigte Kachel; `true` = die Szene ist für diesen Frame fertig (geschlossen oder Raum verlassen) */
  private runHub(action: HubAction): boolean {
    switch (action) {
      case 'close':
        this.close();
        return true;
      case 'settings':
        this.mode = 'settings';
        this.selected = 0;
        this.refreshView();
        return false;
      case 'keys':
        this.openKeys();
        return false;
      case 'removePlayer':
        this.registry.set(REMOVE_PLAYER_KEY, this.targets[this.choice] ?? null); // GameScene meldet genau diesen Slot ab (B-411)
        this.close();
        return true;
      case 'leave':
        this.registry.set(LEAVE_KEY, true); // GameScene verlässt den Raum und schließt diese Szene
        return true;
      default:
        return false; // Einträge der Startseite (Lobby) gibt es im Pausenmenü nicht
    }
  }

  /** Zeichnet die Kacheln neu, wenn sich Auswahl, Ziele oder Sprache geändert haben */
  private drawHub(): void {
    this.hubView.draw(this.hubSel, `${this.targets.length}|${this.choice}|${this.settings.language}`, (id) => ({
      enabled: hubEnabled(id, this.targets),
      label: hubLabel(id),
      sub: hubSub(id, this.targets, this.choice),
    }));
  }

  /** Optionen-Seite: Einstellungen speichern und wirken lassen */
  private applyResult(result: OptionResult): void {
    if (result.settings === this.settings) return;
    if (result.settings.guideHints && !this.settings.guideHints) guideSeen.reset(); // wieder an = alle Hinweise erneut (Q11)
    saveSettings((this.settings = result.settings));
    setLanguage(this.settings.language);
    syncVolumes(this.settings, audioCore()); // Regler wirken sofort auf Musik, Effekte und Ambient
  }

  /** Tabelle der Tastenbelegung zeichnen (B-741): Kacheln und Zeilen ruhen, bis die Seite geschlossen wird */
  private openKeys(): void {
    const { head, rows, padGlyphs } = keyTable();
    const L = KEYS_LAYOUT;
    const style = { ...STYLE, fontSize: `${L.px}px`, strokeThickness: 4 };
    const objs: Phaser.GameObjects.GameObject[] = [];
    [head, ...rows].forEach((cells, r) => {
      const y = L.top + r * L.rowH;
      cells.forEach((txt, c) => {
        const glyph = r > 0 && c === 1 ? padGlyphs[r - 1] : null; // Controller-Spalte: Symbol statt Text (A, X, LB …)
        if (glyph) objs.push(new GlyphRow(this, style, 'top').set([glyph], L.colX[0], y - Math.ceil(L.px * 1.3) / 2).box);
        else objs.push(this.add.text(c === 0 ? L.labelX : L.colX[c - 1]!, y, txt, { ...style, color: r === 0 ? '#ffd166' : '#ffffff' }).setOrigin(c === 0 ? 0 : 0.5, 0.5));
      });
    });
    this.keysTable = objs;
    this.refreshView();
    this.title.setText(t('opt.keys'));
    this.hint.setText(t('opt.keysHint'));
  }

  /** Seite offen: jede Taste oder jedes Tippen führt zurück zur Kachelseite */
  private updateKeys(back: boolean): void {
    this.tapped = null;
    this.tappedTile = null;
    if (!back) return;
    this.keysTable?.forEach((txt) => txt.destroy());
    this.keysTable = null;
    this.refreshView();
  }

  /** Von den Optionen zurück zu den Kacheln (Menu/Esc) */
  private backToHub(): void {
    if (this.standalone) return this.close();
    this.mode = 'hub';
    this.refreshView();
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
