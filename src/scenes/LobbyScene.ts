import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { deleteSave, listSaves, type SaveInfo } from '../core/saveStore';
import { t } from '../core/texts';
import type { RoomClient, Status } from '../online/clientConnection';
import type { GameSceneData } from './GameScene';
import { LobbyFlow, applyCommand, entryLabel, lobbyEntries, lobbyNotice, moveSelection, parseStartParams, rowAt, slotsFor, type StartParams } from './lobbyLogic';
import { DeleteDialog, deleteNotice, worldOf } from './saveMenu';
import { VERSION_KEY } from './debug/debugOverlay';
import { musicDirector } from '../audio/music';

export interface LobbySceneData {
  client: RoomClient;
  /** true nach einem Spiel: `?autostart` und `?room` gelten nur für den ersten Start */
  returned?: boolean;
}

const STYLE = { fontSize: '36px', color: '#ffffff', stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
const TOP = 200;
const ROW_H = 64;
const STICK = 0.5;
/** Pad-Tasten (standard mapping): A bestätigt, X fragt nach dem Löschen eines Spielstands, D-Pad oder Stick wählt; B bleibt frei (Edge-Zurück) */
const PAD_A = 0;
const PAD_X = 2;
const PAD_UP = 12;
const PAD_DOWN = 13;

/** Gerade gehaltene Lobby-Tasten aller Pads: 'a', 'x', 'up', 'down' */
function heldPadKeys(pads: readonly (Phaser.Input.Gamepad.Gamepad | null)[]): Set<string> {
  const held = new Set<string>();
  for (const pad of pads) {
    if (!pad) continue;
    const pressed = (i: number) => pad.buttons[i]?.pressed;
    if (pressed(PAD_A)) held.add('a');
    if (pressed(PAD_X)) held.add('x');
    if (pressed(PAD_UP) || pad.leftStick.y < -STICK) held.add('up');
    if (pressed(PAD_DOWN) || pad.leftStick.y > STICK) held.add('down');
  }
  return held;
}

/**
 * Raumliste (Spielmenü): Welt, „Spielen“, „Neuer Spielstand“, die Räume und die Spielstände des Servers. Bedienung per
 * Pfeile/Stick und A/Enter oder Tippen; X/Entf auf einem Spielstand fragt nach dem Löschen (BD5.2). Zeichnet nur, Logik in
 * `lobbyLogic.ts` und `saveMenu.ts`.
 */
export class LobbyScene extends Phaser.Scene {
  private client!: RoomClient;
  private params!: StartParams;
  private flow!: LobbyFlow;
  private selected = 0;
  private rows: Phaser.GameObjects.Text[] = [];
  private notice!: Phaser.GameObjects.Text;
  private keys!: Record<'up' | 'down' | 'w' | 's' | 'enter' | 'space' | 'del', Phaser.Input.Keyboard.Key>;
  private padHeld = new Set<string>();
  private tapped: number | null = null;
  private saves: SaveInfo[] = [];
  /** Zustand im letzten Bild: Spielstände nur beim Wechsel nach `lobby` laden, nicht jedes Bild */
  private lastStatus: Status | null = null;
  /** Rückfrage vor dem Löschen eines Spielstands (BD5.2) */
  private dialog = new DeleteDialog();
  /** Hinweis nach dem Löschen; verdrängt den Verbindungshinweis, bis etwas anderes gewählt wird */
  private message: string | null = null;

  constructor() {
    super('lobby');
  }

  init(data: LobbySceneData): void {
    this.client = data.client;
    const parsed = parseStartParams(window.location.search);
    this.params = data.returned ? { ...parsed, autostart: false, room: null } : parsed;
    this.flow = new LobbyFlow(this.params);
    this.selected = 0;
    this.rows = [];
    this.padHeld = new Set();
    this.tapped = null;
    this.saves = [];
    this.lastStatus = null;
    this.dialog = new DeleteDialog();
    this.message = null;
  }

  create(): void {
    const K = Phaser.Input.Keyboard.KeyCodes;
    this.keys = this.input.keyboard!.addKeys({ up: K.UP, down: K.DOWN, w: K.W, s: K.S, enter: K.ENTER, space: K.SPACE, del: K.DELETE }) as LobbyScene['keys'];
    this.add.text(GAME_WIDTH / 2, 80, t('lobby.title'), { ...STYLE, fontSize: '56px', strokeThickness: 10 }).setOrigin(0.5);
    this.notice = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 90, '', { ...STYLE, fontSize: '30px', color: '#ffd166', align: 'center' }).setOrigin(0.5);
    // Versionen in einer Zeile zwischen Titel und Liste (Nebeninfo, 24 px nach rules/bedienung.md § 2)
    const versions = ((this.registry.get(VERSION_KEY) as string | undefined) ?? '').replace('\n', ' · ');
    this.add.text(GAME_WIDTH / 2, 128, versions, { ...STYLE, fontSize: '24px', strokeThickness: 4, fontStyle: 'normal', color: '#c8d0e0' }).setOrigin(0.5);
    // Welt eines neuen Spiels (`?world=`, sonst standard); die Welt eines Spielstands steht in seiner Zeile (B-506)
    this.add.text(GAME_WIDTH / 2, 166, t('lobby.world', { world: worldOf(this.params.world) }), { ...STYLE, fontSize: '28px', strokeThickness: 4 }).setOrigin(0.5);
    this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 36, t('lobby.hint'), { ...STYLE, fontSize: '20px', strokeThickness: 4 }).setOrigin(0.5);
    // Touch über das Fenster: das Touch-Overlay des Spiels liegt über dem Canvas, die Ereignisse laufen aber bis hierher.
    const rows = () => (this.dialog.open ? this.dialog.labels().length : lobbyEntries(this.client.rooms, this.client.status, this.saves).length);
    const onTap = (e: PointerEvent) => (this.tapped = rowAt(this.scale.transformY(e.pageY), TOP, ROW_H, rows()));
    window.addEventListener('pointerdown', onTap);
    this.events.once('shutdown', () => window.removeEventListener('pointerdown', onTap));
  }

  update(): void {
    musicDirector().lobby();
    const client = this.client;
    if (client.status === 'room') return this.enterGame();
    if (client.status === 'lobby' && this.lastStatus !== 'lobby') void listSaves().then((saves) => (this.saves = saves));
    this.lastStatus = client.status;
    const first = this.flow.step(client);
    if (first) applyCommand(client, first);

    const entries = lobbyEntries(client.rooms, client.status, this.saves);
    const input = this.poll();
    if (client.status !== 'lobby') this.dialog.close();
    if (this.dialog.open) return this.updateDialog(input);
    this.selected = moveSelection(this.selected, input.dir, entries.length);
    let chosen = input.confirm ? this.selected : null;
    if (this.tapped !== null) {
      if (this.tapped >= 0) chosen = this.selected = this.tapped;
      this.tapped = null;
    }
    const entry = chosen === null ? undefined : entries[chosen];
    if (entry) {
      this.message = null;
      this.activate(entry);
    }
    const marked = entries[this.selected];
    if (input.del && marked?.kind === 'save') {
      this.message = null;
      this.dialog.toggle(marked.save.name);
    }
    this.draw(entries);
  }

  /** Rückfrage offen: Richtung wählt, A/Enter oder Tippen bestätigt, X/Entf bricht ab; nur „Löschen“ löscht (BD5.2) */
  private updateDialog(input: { dir: number; confirm: boolean; del: boolean }): void {
    this.dialog.move(input.dir);
    let name: string | null = null;
    if (this.tapped !== null) {
      if (this.tapped >= 0) name = this.dialog.confirm(this.tapped);
      this.tapped = null;
    } else if (input.confirm) name = this.dialog.confirm();
    else if (input.del) this.dialog.close();
    if (name) this.remove(name);
    this.drawDialog();
  }

  private remove(name: string): void {
    void deleteSave(name)
      .then((result) => {
        this.message = deleteNotice(name, result);
        return listSaves();
      })
      .then((saves) => (this.saves = saves));
  }

  private drawDialog(): void {
    if (!this.dialog.open) return;
    this.dialog.labels().forEach((label, i) => {
      const row = (this.rows[i] ??= this.add.text(GAME_WIDTH / 2, TOP + i * ROW_H + ROW_H / 2, '', STYLE).setOrigin(0.5));
      row.setText(`${i === this.dialog.selected ? '▶  ' : '    '}${label}`).setColor(i === this.dialog.selected ? '#ffd166' : '#ffffff');
    });
    this.rows.slice(this.dialog.labels().length).forEach((r) => r.setText(''));
    this.notice.setText(t('lobby.deleteAsk', { name: this.dialog.name ?? '' }));
  }

  private activate(entry: ReturnType<typeof lobbyEntries>[number]): void {
    if (entry.kind === 'retry') this.client.retry();
    else if (entry.kind === 'reload') window.location.reload();
    else {
      const cmd = this.flow.choose(entry);
      if (cmd) applyCommand(this.client, cmd);
    }
  }

  private enterGame(): void {
    const data: GameSceneData = { client: this.client, mock: slotsFor(this.params.mock).slice(1) };
    this.scene.start('game', data);
  }

  private draw(entries: ReturnType<typeof lobbyEntries>): void {
    const usable = entries.length > 0;
    entries.forEach((e, i) => {
      const row = (this.rows[i] ??= this.add.text(GAME_WIDTH / 2, TOP + i * ROW_H + ROW_H / 2, '', STYLE).setOrigin(0.5));
      row.setText(`${i === this.selected ? '▶  ' : '    '}${entryLabel(e, this.params.save)}`).setColor(i === this.selected && usable ? '#ffd166' : '#ffffff');
    });
    this.rows.slice(entries.length).forEach((r) => r.setText(''));
    this.notice.setText(this.message ?? lobbyNotice(this.client) ?? '');
  }

  /** Eingaben der Tastatur und aller Pads: Richtung (-1/0/1), Bestätigung und Löschen (X/Entf), je einmal pro Druck */
  private poll(): { dir: number; confirm: boolean; del: boolean } {
    const down = Phaser.Input.Keyboard.JustDown;
    const k = this.keys;
    let dir = (down(k.down) || down(k.s) ? 1 : 0) - (down(k.up) || down(k.w) ? 1 : 0);
    let confirm = down(k.enter) || down(k.space);
    let del = down(k.del);

    const held = heldPadKeys(this.input.gamepad?.gamepads ?? []);
    const edge = (name: string) => held.has(name) && !this.padHeld.has(name);
    dir += (edge('down') ? 1 : 0) - (edge('up') ? 1 : 0);
    confirm ||= edge('a');
    del ||= edge('x');
    this.padHeld = held;
    return { dir: Math.sign(dir), confirm, del };
  }
}
