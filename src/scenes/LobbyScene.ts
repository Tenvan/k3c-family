import Phaser from 'phaser';
import { GAME_HEIGHT, GAME_WIDTH } from '../core/constants';
import { heldPadKeys, type LobbyKey } from './lobbyPad';
import { clientLog } from '../core/clientLog';
import { closeRoom, deleteSave, listSaves, listWorlds, type SaveInfo } from '../core/saveStore';
import { t } from '../core/texts';
import type { RoomClient, Status } from '../online/clientConnection';
import type { GameSceneData } from './GameScene';
import { LobbyFlow, applyCommand, entryLabel, gradeName, lobbyEntries, lobbyNotice, moveSelection, parseStartParams, rowAt, slotsFor, type StartParams } from './lobbyLogic';
import { DIALOG_ROWS, devAllowed, type DialogState } from './lobbyOptions';
import { DeleteDialog, closeNotice, deleteNotice, worldOf } from './saveMenu';
import { VERSION_KEY } from './debug/debugOverlay';
import { musicDirector } from '../audio/music';
import { playUi, uiSoundFor } from '../audio/ui';
import { openPage } from '../core/shell';
import { HubView } from './hubView';
import { HUB_COLORS, START_TILES, gridTiles, hubAction, hubDir, hubLabel, hubSub, hubTileAt, moveHub, type HubAction, type HubTile } from './pauseHub';
import { CATEGORY_COLOR, categoryLabel, categorySub, menuCategories, uniqueRun, type CategoryId, type MenuEntry } from './menuPages';
import { startRun } from './runStart';
import { runSearch } from '../core/runConfig';

export interface LobbySceneData {
  client: RoomClient;
  /** true nach einem Spiel: `?autostart` und `?room` gelten nur für den ersten Start */
  returned?: boolean;
}

const STYLE = { fontSize: '36px', color: '#ffffff', stroke: '#000000', strokeThickness: 6, fontStyle: 'bold' };
/** Ansichten der Lobby: Startseite (Kacheln, Einstieg), Raum- und Spielstandliste, Seitenliste (B-768) */
type View = 'hub' | 'list' | 'pages';
const TOP = 200;
const ROW_H = 64;
/** Höchstzahl geloggter Tastendrücke je Lobby-Besuch (Rate-Limit des Client-Logs) */
const PAD_LOG_MAX = 30;

/**
 * Raumliste (Spielmenü): Welt, „Spielen“, „Neuer Spielstand“, die Räume und die Spielstände des Servers. Bedienung per
 * Pfeile/Stick und A/Enter oder Tippen; X/Entf auf einem Spielstand fragt nach dem Löschen (BD5.2), auf einem Raum nach dem Schließen (BD9.2). Zeichnet nur, Logik in
 * `lobbyLogic.ts` und `saveMenu.ts`.
 */
export class LobbyScene extends Phaser.Scene {
  private client!: RoomClient;
  private params!: StartParams;
  private flow!: LobbyFlow;
  private selected = 0;
  private rows: Phaser.GameObjects.Text[] = [];
  private notice!: Phaser.GameObjects.Text;
  private keys!: Record<'up' | 'down' | 'left' | 'right' | 'w' | 's' | 'a' | 'd' | 'enter' | 'space' | 'del' | 'esc', Phaser.Input.Keyboard.Key>;
  private hint!: Phaser.GameObjects.Text;
  private padHeld = new Set<string>();
  /** Pad-Tasten im letzten Bild und Zahl der geloggten Tastendrücke (Diagnose: kommt X auf dem Gerät an?) */
  private padButtons = new Set<string>();
  private padLogged = 0;
  private tapped: number | null = null;
  private tappedTile: number | null = null;
  private view: View = 'hub';
  private hubSel = 0;
  /** Seiten (BD11.4): Kategorie offen (`null` = die Kategorien selbst), Auswahl im Raster, Raster und seine Kacheln */
  private category: CategoryId | null = null;
  private gridSel = 0;
  private gridKey = '';
  private gridView: HubView<string> | null = null;
  private gridTiles: readonly HubTile<string>[] = [];
  private hubView!: HubView;
  private hubShown = false;
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
    const parsed = parseStartParams(runSearch());
    this.params = data.returned ? { ...parsed, autostart: false, room: null } : parsed;
    this.flow = new LobbyFlow(this.params, devAllowed(runSearch()));
    this.selected = 0;
    this.rows = [];
    this.padHeld = new Set();
    this.tapped = null;
    this.tappedTile = null;
    this.view = 'hub';
    this.hubSel = 0;
    this.category = null;
    this.gridSel = 0;
    this.gridKey = '';
    this.gridView = null;
    this.gridTiles = [];
    this.hubShown = false;
    this.saves = [];
    this.lastStatus = null;
    this.dialog = new DeleteDialog();
    this.message = null;
  }

  create(): void {
    const K = Phaser.Input.Keyboard.KeyCodes;
    this.keys = this.input.keyboard!.addKeys({ up: K.UP, down: K.DOWN, left: K.LEFT, right: K.RIGHT, w: K.W, s: K.S, a: K.A, d: K.D, enter: K.ENTER, space: K.SPACE, del: K.DELETE, esc: K.ESC }) as LobbyScene['keys'];
    this.cameras.main.setBackgroundColor(HUB_COLORS.night); // Nachtblau der Landingpage statt Schwarz
    this.add.text(GAME_WIDTH / 2, 80, t('lobby.title'), { ...STYLE, fontSize: '56px', strokeThickness: 10 }).setOrigin(0.5);
    this.notice = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 90, '', { ...STYLE, fontSize: '30px', color: '#ffd166', align: 'center' }).setOrigin(0.5);
    // Versionen in einer Zeile zwischen Titel und Liste (Nebeninfo, 24 px nach rules/bedienung.md § 2)
    const versions = ((this.registry.get(VERSION_KEY) as string | undefined) ?? '').replace('\n', ' · ');
    this.add.text(GAME_WIDTH / 2, 128, versions, { ...STYLE, fontSize: '24px', strokeThickness: 4, fontStyle: 'normal', color: '#c8d0e0' }).setOrigin(0.5);
    // Welt eines neuen Spiels (`?world=`, sonst standard); die Welt eines Spielstands steht in seiner Zeile (B-506)
    this.add.text(GAME_WIDTH / 2, 166, t('lobby.world', { world: worldOf(this.params.world) }), { ...STYLE, fontSize: '28px', strokeThickness: 4 }).setOrigin(0.5);
    this.hint = this.add.text(GAME_WIDTH / 2, GAME_HEIGHT - 36, t('lobby.hint'), { ...STYLE, fontSize: '20px', strokeThickness: 4 }).setOrigin(0.5);
    // Touch über das Fenster: das Touch-Overlay des Spiels liegt über dem Canvas, die Ereignisse laufen aber bis hierher.
    const rows = () => (this.dialog.open ? this.dialog.labels().length : lobbyEntries(this.client.rooms, this.client.status, this.saves).length);
    const onTap = (e: PointerEvent) => {
      const y = this.scale.transformY(e.pageY);
      const view = this.currentView();
      if (view !== 'list' && !this.flow.dialog && !this.dialog.open) {
        this.tappedTile = hubTileAt(view === 'hub' ? START_TILES : this.gridTiles, this.scale.transformX(e.pageX), y);
        return;
      }
      this.tapped = this.flow.dialog ? rowAt(y, TOP + ROW_H, ROW_H, DIALOG_ROWS.length) : rowAt(y, TOP, ROW_H, rows());
    };
    window.addEventListener('pointerdown', onTap);
    this.hubView = new HubView(this, START_TILES);
    this.events.once('shutdown', () => window.removeEventListener('pointerdown', onTap));
  }

  update(): void {
    musicDirector().lobby();
    const client = this.client;
    if (client.status === 'room') return this.enterGame();
    if (this.scene.isActive('options')) return void this.poll(); // Optionen offen (Startseite): Eingaben verwerfen, damit sie danach nicht nachwirken
    this.loadLists(client.status);
    this.lastStatus = client.status;
    const first = this.flow.step(client);
    if (first) applyCommand(client, first);

    const entries = lobbyEntries(client.rooms, client.status, this.saves);
    const input = this.poll();
    playUi(uiSoundFor({ ...input, confirm: input.confirm || (this.tapped !== null && this.tapped >= 0) })); // UI-Klick (B-575)
    if (this.updateDialogs(input)) return this.showHub(false);
    const view = this.currentView();
    if (view !== 'pages') this.clearGrid();
    if (view === 'hub') return this.updateHub(input, entries);
    this.showHub(false);
    if (view === 'pages') return this.updatePages(input);
    if (input.back && client.status === 'lobby') {
      this.view = 'hub';
      return;
    }
    this.updateList(input, entries);
  }

  /** Raum- und Spielstandliste: Auswahl, Bestätigen oder Tippen, X/Entf löscht oder schließt */
  private updateList(input: { dir: number; confirm: boolean; del: boolean }, entries: ReturnType<typeof lobbyEntries>): void {
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
    if (input.del) this.askRemove(entries[this.selected]);
    this.draw(entries);
  }

  /** Ohne Verbindung zeigt die Lobby die Liste (mit „Erneut versuchen“ und „Seite neu laden“), sonst die gewählte Ansicht */
  private currentView(): View {
    return this.client.status === 'lobby' ? this.view : 'list';
  }

  /** Kachelseite ein- oder ausblenden, nur beim Wechsel (sonst würde sie jedes Bild neu gezeichnet) */
  private showHub(on: boolean): void {
    if (on === this.hubShown) return;
    this.hubShown = on;
    this.hubView.setVisible(on);
    if (on) this.rows.forEach((r) => r.setText(''));
  }

  /** Startseite: Pfeile und Stick bewegen die Auswahl in vier Richtungen, A/Enter oder Tippen bestätigt */
  private updateHub(input: { dir: number; side: number; confirm: boolean }, entries: ReturnType<typeof lobbyEntries>): void {
    this.showHub(true);
    let confirm = input.confirm;
    if (this.tappedTile !== null) {
      if (this.tappedTile >= 0) {
        this.hubSel = this.tappedTile;
        confirm = true;
      }
      this.tappedTile = null;
    } else {
      const d = hubDir(input.dir, input.side < 0 ? 'left' : input.side > 0 ? 'right' : null);
      if (d) this.hubSel = moveHub(START_TILES, this.hubSel, d);
    }
    const action = confirm ? hubAction(START_TILES[this.hubSel]!.id, []) : null;
    if (action) this.runStart(action, entries);
    this.hubView.draw(this.hubSel, this.params.save, (id) => ({ enabled: true, label: hubLabel(id), sub: hubSub(id, [], 0, this.params.save) }));
    this.notice.setText(this.message ?? lobbyNotice(this.client) ?? '');
    this.hint.setText(t('hub.startHint'));
  }

  /** Wirkt eine bestätigte Kachel der Startseite */
  private runStart(action: HubAction, entries: ReturnType<typeof lobbyEntries>): void {
    this.message = null;
    if (action === 'close') this.activate({ kind: 'play' }); // Fortsetzen: der Spielstand der Start-Parameter
    else if (action === 'new') {
      const entry = entries.find((e) => e.kind === 'new');
      if (entry) this.activate(entry);
    } else if (action === 'join') {
      this.view = 'list';
      this.selected = Math.min(2, Math.max(0, entries.length - 1)); // gleich auf den ersten Raum oder Spielstand
    } else if (action === 'pages') {
      this.view = 'pages';
      this.category = null;
      this.gridSel = 0;
    } else if (action === 'settings') this.scene.launch('options', { targets: [], standalone: true });
  }

  /** Seiten (BD11.4): erst die Kategorien, dann ihre Einträge; Menu/Esc geht eine Ebene zurück */
  private updatePages(input: { dir: number; side: number; confirm: boolean; back: boolean }): void {
    const cats = menuCategories();
    const open = this.category === null ? undefined : cats.find((c) => c.id === this.category);
    const items = open ? open.entries.map((e) => ({ id: e.id, color: CATEGORY_COLOR[open.id] })) : cats.map((c) => ({ id: c.id as string, color: c.entries.length > 0 ? CATEGORY_COLOR[c.id] : HUB_COLORS.off }));
    this.setGrid(`${this.category}|${items.map((i) => i.id).join(',')}`, gridTiles(items));
    if (this.moveGrid(input)) this.confirmPage(open?.entries[this.gridSel], cats[this.gridSel]);
    if (input.back) this.pagesBack();
    this.drawPages(cats, open);
  }

  /** Auswahl im Raster der Seiten: Richtungen, Tippen; `true` = bestätigt */
  private moveGrid(input: { dir: number; side: number; confirm: boolean }): boolean {
    if (this.tappedTile !== null) {
      const tile = this.tappedTile;
      this.tappedTile = null;
      if (tile < 0) return false;
      this.gridSel = tile;
      return true;
    }
    const d = hubDir(input.dir, input.side < 0 ? 'left' : input.side > 0 ? 'right' : null);
    if (d) this.gridSel = moveHub(this.gridTiles, this.gridSel, d);
    return input.confirm;
  }

  private drawPages(cats: readonly { id: CategoryId; entries: readonly MenuEntry[] }[], open: { entries: readonly MenuEntry[] } | undefined): void {
    this.gridView?.draw(this.gridSel, this.gridKey, (_id, i) =>
      open ? { enabled: true, label: open.entries[i]!.title, sub: '' } : { enabled: cats[i]!.entries.length > 0, label: categoryLabel(cats[i]!.id), sub: categorySub(cats[i]!) },
    );
    this.notice.setText(open?.entries[this.gridSel]?.description ?? '');
    this.hint.setText(t('hub.listHint'));
  }

  /** Bestätigt: eine Kategorie öffnen oder einen Eintrag ausführen */
  private confirmPage(entry: MenuEntry | undefined, category: { id: CategoryId; entries: readonly MenuEntry[] } | undefined): void {
    if (entry) return this.runEntry(entry);
    if (category && category.entries.length > 0) {
      this.category = category.id;
      this.gridSel = 0;
    }
  }

  /** Seite öffnen, Szenario starten (ohne Neuladen) oder das eigene Fenster des Dungeon Masters öffnen */
  private runEntry(entry: MenuEntry): void {
    const target = entry.target;
    if (target.kind === 'page') openPage(target.href);
    else if (target.kind === 'external') window.open(target.href, '_blank', 'noopener');
    else startRun(this, this.client, uniqueRun(target.params));
  }

  /** Eine Ebene zurück: aus einer Kategorie zu den Kategorien (mit ihr als Auswahl), von dort zur Startseite */
  private pagesBack(): void {
    if (this.category === null) {
      this.view = 'hub';
      return;
    }
    const index = menuCategories().findIndex((c) => c.id === this.category);
    this.category = null;
    this.gridSel = Math.max(0, index);
  }

  /** Raster der Seiten bei Änderung neu aufbauen (andere Kacheln oder Ebene) */
  private setGrid(key: string, tiles: readonly HubTile<string>[]): void {
    if (key === this.gridKey && this.gridView) return;
    this.clearGrid();
    this.rows.forEach((r) => r.setText(''));
    this.gridKey = key;
    this.gridTiles = tiles;
    this.gridView = new HubView<string>(this, tiles);
    this.gridView.setVisible(true);
  }

  private clearGrid(): void {
    this.gridView?.destroy();
    this.gridView = null;
    this.gridKey = '';
    this.gridTiles = [];
  }

  /** X/Entf: Rückfrage zum Löschen eines Spielstands oder zum Schließen eines Raums (BD9.2); andere Einträge ignorieren es */
  private askRemove(marked: ReturnType<typeof lobbyEntries>[number] | undefined): void {
    if (marked?.kind !== 'save' && marked?.kind !== 'room') return;
    this.message = null;
    if (marked.kind === 'save') this.dialog.toggle(marked.save.name);
    else this.dialog.toggle(marked.room.name, 'room', marked.room.taken);
  }

  /** Spielstände und wählbare Welten des Servers laden, nur beim Wechsel nach `lobby` */
  private loadLists(status: Status): void {
    if (status !== 'lobby' || this.lastStatus === 'lobby') return;
    void listSaves().then((saves) => (this.saves = saves));
    void listWorlds().then((worlds) => (this.flow.worlds = worlds));
  }

  /** Offene Dialoge (Anlegen, Löschen) bekommen die Eingabe; außerhalb der Lobby schließen beide. true = ein Dialog ist offen */
  private updateDialogs(input: { dir: number; side: number; confirm: boolean; del: boolean }): boolean {
    if (this.client.status !== 'lobby') {
      this.dialog.close();
      this.flow.dialog = null;
    }
    if (this.flow.dialog) this.updateOptions(input);
    else if (this.dialog.open) this.updateDialog(input);
    else return false;
    return true;
  }

  /** Rückfrage offen: Richtung wählt, A/Enter oder Tippen bestätigt, X/Entf bricht ab; nur „Löschen“ löscht (BD5.2) */
  private updateDialog(input: { dir: number; confirm: boolean; del: boolean }): void {
    this.dialog.move(input.dir);
    const kind = this.dialog.kind;
    let name: string | null = null;
    if (this.tapped !== null) {
      if (this.tapped >= 0) name = this.dialog.confirm(this.tapped);
      this.tapped = null;
    } else if (input.confirm) name = this.dialog.confirm();
    else if (input.del) this.dialog.close();
    if (name) this.remove(name, kind);
    this.drawDialog();
  }

  /** Anlegen-Dialog (K5.2): Eingabe an `LobbyFlow`, Klick wählt die Zeile und bestätigt sie */
  private updateOptions(input: { dir: number; side: number; confirm: boolean }): void {
    const row = this.flow.dialog!.row;
    if (this.tapped !== null) {
      if (this.tapped >= 0) Object.assign(input, { dir: this.tapped - row, side: 0, confirm: true });
      this.tapped = null;
    }
    const cmd = this.flow.dialogInput(input);
    if (cmd) applyCommand(this.client, cmd);
    if (this.flow.dialog) this.drawOptions(this.flow.dialog);
  }

  private drawOptions(d: DialogState): void {
    const o = d.options;
    const labels = [
      t('lobby.dialog.title'),
      t('lobby.dialog.grade', { value: gradeName(o.grade) }),
      t('lobby.dialog.goal', { value: t(`opt.goal.${o.goal}`) }),
      t('lobby.dialog.defeat', { value: t(`opt.defeat.${o.defeat}`) }),
      t('lobby.dialog.world', { value: o.world }),
      t('lobby.dialog.create'),
      t('lobby.dialog.cancel'),
    ];
    labels.forEach((label, i) => {
      const selected = i === d.row + 1;
      const row = (this.rows[i] ??= this.add.text(GAME_WIDTH / 2, TOP + i * ROW_H + ROW_H / 2, '', STYLE).setOrigin(0.5));
      row.setText(i === 0 ? label : `${selected ? '▶  ' : '    '}${label}`).setColor(selected ? '#ffd166' : i === 0 ? '#c8d0e0' : '#ffffff');
    });
    this.rows.slice(labels.length).forEach((r) => r.setText(''));
    this.notice.setText(lobbyNotice(this.client) ?? '');
    this.hint.setText(t('lobby.dialog.hint'));
  }

  /** Spielstand löschen oder offenen Raum schließen (BD9.2); danach Spielstände neu laden (die Raumliste schickt der Server selbst) */
  private remove(name: string, kind: 'save' | 'room'): void {
    const done = kind === 'room' ? closeRoom(name).then((r) => closeNotice(name, r)) : deleteSave(name).then((r) => deleteNotice(name, r));
    void done
      .then((message) => {
        this.message = message;
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
    this.notice.setText(this.dialog.ask());
  }

  private activate(entry: ReturnType<typeof lobbyEntries>[number]): void {
    if (entry.kind === 'retry') this.client.retry();
    else if (entry.kind === 'reload') window.location.reload();
    else if (entry.kind === 'new') this.flow.openDialog(entry.name); // Grad, Ziel, Niederlage-Modus wählen (K5.2)
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
    this.hint.setText(t('lobby.hint'));
  }

  /** Neu gedrückte Pad-Tasten (Index) ins Client-Log, höchstens `PAD_LOG_MAX` je Lobby-Besuch: zeigt auf dem Gerät, welche Tasten ankommen */
  private logPadButtons(): void {
    const now = new Set<string>();
    for (const [p, pad] of (this.input.gamepad?.gamepads ?? []).entries()) {
      pad?.buttons.forEach((b, i) => b.pressed && now.add(`${p}:${i}`));
    }
    for (const key of now) {
      if (this.padButtons.has(key) || this.padLogged >= PAD_LOG_MAX) continue;
      this.padLogged++;
      clientLog('info', `🎮 Lobby: Pad ${key.split(':')[0]} Taste ${key.split(':')[1]}`);
    }
    this.padButtons = now;
  }

  /** Eingaben der Tastatur und aller Pads: Richtung (-1/0/1), Bestätigung und Löschen (X/Entf), je einmal pro Druck */
  private poll(): { dir: number; side: number; confirm: boolean; del: boolean; back: boolean } {
    const down = Phaser.Input.Keyboard.JustDown;
    const k = this.keys;
    const pressed = (...keys: Phaser.Input.Keyboard.Key[]) => (keys.some((key) => down(key)) ? 1 : 0);
    let dir = pressed(k.down, k.s) - pressed(k.up, k.w);
    let confirm = pressed(k.enter, k.space) > 0;
    let del = pressed(k.del) > 0;
    let side = pressed(k.right, k.d) - pressed(k.left, k.a);
    let back = pressed(k.esc) > 0;

    this.logPadButtons();
    const held = heldPadKeys(this.input.gamepad?.gamepads ?? []);
    const edge = (name: LobbyKey) => (held.has(name) && !this.padHeld.has(name) ? 1 : 0);
    dir += edge('down') - edge('up');
    confirm ||= edge('a') > 0;
    del ||= edge('x') > 0;
    side += edge('right') - edge('left');
    back ||= edge('back') > 0;
    this.padHeld = held;
    return { dir: Math.sign(dir), side: Math.sign(side), confirm, del, back };
  }
}
