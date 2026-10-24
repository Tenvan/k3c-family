/**
 * Auswahl- und Befehlslogik der Lobby ohne Phaser (SP08.3): Start-Parameter (B-082), Eintragsliste, Befehle, Hinweise.
 * `LobbyScene` zeichnet nur und gibt Eingaben weiter.
 */
import { t } from '../core/texts';
import type { SaveInfo } from '../core/saveStore';
import type { RoomInfo } from '../online/clientProtocol';
import type { Status } from '../online/clientConnection';
import { GRADES, createOptions, dialogStep, initialDialog, type DialogInput, type DialogState, type Grade, type RoomOptions } from './lobbyOptions';
import { DEFAULT_WORLD, nextSaveName, worldOf } from './saveMenu';

export const DEFAULT_SAVE = 'familie';
const SAVE_FORMAT = /^[a-z0-9-]{1,32}$/;
const ROOM_FORMAT = /^[A-HJ-NP-Z]{4}$/;
const MOCK_FORMAT = /^[0-3]$/;
const PLAYERS_FORMAT = /^[1-4]$/;
const DEPTH_FORMAT = /^[0-9]{1,3}$/;

export interface StartParams {
  autostart: boolean;
  fresh: boolean;
  save: string;
  /** Mock-Slots ohne Eingabe (0–3) */
  mock: number;
  /** Raumcode aus `?room=`, null = keiner */
  room: string | null;
  /** Lokale Spieler beim Beitritt aus `?players=` (1–4, B-353) */
  players: number;
  /** Startstufe eines neuen Spiels aus `?depth=` (Zahl, sonst 0); ob es die Stufe gibt, prüft der Server (`bad_request`, B-095) */
  depth: number;
  /** Seed eines neuen Spiels aus `?seed=`, leer = der Name des Spielstands; die Länge (höchstens 64 Zeichen) prüft der Server (`bad_request`, B-095) */
  seed: string;
  /** Welt eines neuen Spiels aus `?world=`, leer = standard; ob es sie gibt und ob sie erlaubt ist (nur Dev-Mode), prüft der Server (`world_unknown`, `forbidden`) */
  world: string;
}

/** Wertet `?autostart=1&fresh=1&save=NAME&mock=N&room=CODE&players=N&depth=N&seed=TEXT&world=ID` aus; ungültige Werte gelten als nicht gesetzt. */
export function parseStartParams(search: string): StartParams {
  const p = new URLSearchParams(search);
  const save = p.get('save') ?? '';
  const mock = p.get('mock') ?? '';
  const room = (p.get('room') ?? '').toUpperCase();
  const players = p.get('players') ?? '';
  const depth = p.get('depth') ?? '';
  const seed = p.get('seed') ?? '';
  return {
    autostart: p.get('autostart') === '1',
    fresh: p.get('fresh') === '1',
    save: SAVE_FORMAT.test(save) ? save : DEFAULT_SAVE,
    mock: MOCK_FORMAT.test(mock) ? Number(mock) : 0,
    room: ROOM_FORMAT.test(room) ? room : null,
    players: PLAYERS_FORMAT.test(players) ? Number(players) : 1,
    depth: DEPTH_FORMAT.test(depth) ? Number(depth) : 0,
    seed,
    world: p.get('world') ?? '',
  };
}

/** Slots eines Geräts: 0 ist der echte Spieler, 1..mock sind Mock-Slots; `players` lokale Spieler belegen 0..players-1 (B-353) */
export function slotsFor(mock: number, players = 1): number[] {
  return Array.from({ length: Math.max(mock + 1, players) }, (_, i) => i);
}

export type LobbyCommand =
  | { t: 'create'; save: string; fresh: boolean; depth: number; slots: number[]; seed?: string; world?: string; options?: RoomOptions }
  | { t: 'join'; room: string; slots: number[] };

export interface LobbyClient {
  create(save: string, fresh: boolean, depth: number, slots: number[], seed?: string, world?: string, options?: RoomOptions): void;
  join(room: string, slots: number[]): void;
}

export function applyCommand(client: LobbyClient, cmd: LobbyCommand): void {
  if (cmd.t === 'create') client.create(cmd.save, cmd.fresh, cmd.depth, cmd.slots, cmd.seed, cmd.world, cmd.options);
  else client.join(cmd.room, cmd.slots);
}

export type LobbyEntry =
  | { kind: 'play' }
  /** Neuer Spielstand unter dem nächsten freien Namen (BD5.2, B-506) */
  | { kind: 'new'; name: string }
  | { kind: 'room'; room: RoomInfo }
  | { kind: 'save'; save: SaveInfo }
  | { kind: 'retry' }
  | { kind: 'reload' };

/** Höchstens so viele Spielstände unter den Räumen (LB1, Offene Fragen) */
export const MAX_SAVES = 6;

/** Zeitpunkt des Spielstands; ungültige Zeitstempel zählen als ältester Stand (B-382) */
function savedTime(s: SaveInfo): number {
  const ms = Date.parse(s.savedAt);
  return Number.isNaN(ms) ? -Infinity : ms;
}

/**
 * „Spielen“ steht oben, darunter „Neuer Spielstand“ (BD5.2), die Räume des Servers, dann die neuesten Spielstände, die noch nicht als Raum offen sind
 * (der Raumname ist der Name des Spielstands). Ohne Verbindung (B-083) gibt es keine wirkungslosen Einträge:
 * `lost` bietet „Erneut versuchen“, `ended` (an anderer Stelle geöffnet, veraltete Version) „Seite neu laden“, sonst nichts.
 */
export function lobbyEntries(rooms: readonly RoomInfo[], status: Status = 'lobby', saves: readonly SaveInfo[] = []): LobbyEntry[] {
  if (status === 'lost') return [{ kind: 'retry' }];
  if (status === 'ended') return [{ kind: 'reload' }];
  if (status !== 'lobby') return [];
  const open = new Set(rooms.map((r) => r.name));
  const newest = saves
    .filter((s) => !open.has(s.name))
    .sort((a, b) => savedTime(b) - savedTime(a))
    .slice(0, MAX_SAVES);
  return [
    { kind: 'play' },
    { kind: 'new', name: nextSaveName([...saves.map((s) => s.name), ...open]) },
    ...rooms.map((room): LobbyEntry => ({ kind: 'room', room })),
    ...newest.map((save): LobbyEntry => ({ kind: 'save', save })),
  ];
}

export function moveSelection(selected: number, dir: number, count: number): number {
  return count <= 0 ? 0 : (((selected + dir) % count) + count) % count; // am Ende weiter zum Anfang und umgekehrt
}

export function entryLabel(e: LobbyEntry, save: string): string {
  if (e.kind === 'play') return t('lobby.play', { save });
  if (e.kind === 'new') return t('lobby.new', { name: e.name });
  if (e.kind === 'retry') return t('lobby.retry');
  if (e.kind === 'reload') return t('lobby.reload');
  if (e.kind === 'save') return t('lobby.save', { name: e.save.name, world: worldOf(e.save.world), day: e.save.day ?? '–', depths: e.save.depths.join(', ') || '–' });
  const r = e.room;
  const room = t('lobby.room', { code: r.code, name: r.name, depth: r.depth, taken: r.taken, state: t(r.running ? 'lobby.running' : 'lobby.paused') });
  return r.grade ? `${room}  ·  ${gradeName(r.grade)}` : room;
}

/** Name eines Grads; ein unbekannter Grad des Servers bleibt als Kennung stehen (K5.2) */
export function gradeName(grade: string): string {
  return (GRADES as readonly string[]).includes(grade) ? t(`opt.grade.${grade as Grade}`) : grade;
}

/** Zeile unter dem Zeiger (Touch), -1 = keine */
export function rowAt(y: number, top: number, rowHeight: number, count: number): number {
  const row = Math.floor((y - top) / rowHeight);
  return y >= top && row < count ? row : -1;
}

/** Hinweis der Lobby: Verbindungsstand oder Fehlertext des Servers (jeder Fehler-Code bringt seine `message` mit) */
export function lobbyNotice(c: { status: Status; notice: string | null; errorCode?: string | null }): string | null {
  if (c.status === 'connecting') return c.notice ?? t('net.connecting');
  if (c.status === 'lobby' && c.errorCode === 'bad_request') return c.notice ?? t('lobby.badOption'); // abgelehnte Raum-Option (K5.2): Text des Servers zuerst
  return c.notice;
}

/** Hinweis im Spiel bei Abbruch der Verbindung oder angekündigtem Neustart des Servers (B-507), sonst null */
export function gameNotice(c: { status: Status; notice: string | null; errorCode?: string | null }): string | null {
  return c.status === 'reconnecting' || c.errorCode === 'restarting' ? c.notice : null;
}

/** Das Spiel endet, sobald der Client nicht mehr im Raum ist (Raum geschlossen/unbekannt, 120 s ohne Verbindung, an anderer Stelle geöffnet) */
export function leavesGame(status: Status): boolean {
  return status === 'lobby' || status === 'lost' || status === 'ended';
}

/**
 * Wann die Lobby welchen Befehl sendet: Start-Parameter (`?room`, `?autostart`) einmal, Auswahl aus der Liste,
 * `save_not_found` → ein zweiter Versuch mit `fresh: true` (und `depth`, `seed`, `world` der Start-Parameter).
 */
export class LobbyFlow {
  private started = false;
  private retried = false;
  /** Herkunft des gemerkten Befehls: „Spielen“/Autostart dürfen einmal neu anlegen, ein gewählter Spielstand nie */
  private retryable = false;
  private last: LobbyCommand | null = null;
  /** Offener Anlegen-Dialog für „Neuer Spielstand“ (K5.2, B-105), null = Liste */
  dialog: (DialogState & { save: string }) | null = null;
  /** Wählbare Welten des Servers (`GET /api/worlds`, B-566); bis sie da sind gilt nur `standard` */
  worlds: readonly string[] = [DEFAULT_WORLD];

  /** `dev`: Grad „Dev“ im Dialog anbieten (`devAllowed`) */
  constructor(
    private readonly params: StartParams,
    private readonly dev = false,
  ) {}

  /** Bei jedem Frame mit dem Client-Zustand aufrufen; liefert den nächsten Befehl oder null. */
  step(c: { status: Status; errorCode: string | null }): LobbyCommand | null {
    if (c.status !== 'lobby') return null;
    const slots = this.slots();
    if (!this.started && (this.params.room || this.params.autostart)) {
      this.started = true;
      this.retryable = true;
      return this.remember(this.params.room ? { t: 'join', room: this.params.room, slots } : this.create(this.params.fresh));
    }
    // nur für „Spielen“/Autostart: ein gewählter Spielstand existiert, er wird nie leer neu angelegt
    const last = this.last;
    if (c.errorCode === 'save_not_found' && this.retryable && last?.t === 'create' && !last.fresh && !this.retried) {
      this.retried = true;
      return this.remember(this.create(true));
    }
    return null;
  }

  /**
   * Auswahl in der Liste bestätigt; ein Spielstand startet seinen Raum oder tritt ihm bei, falls er schon offen ist (Server).
   * „Erneut versuchen“ und „Seite neu laden“ sind keine Befehle an den Server (null).
   */
  choose(e: LobbyEntry): LobbyCommand | null {
    this.retried = false;
    this.retryable = e.kind === 'play';
    if (e.kind === 'play') return this.remember(this.create(this.params.fresh));
    if (e.kind === 'new') return this.remember(this.create(true, e.name));
    if (e.kind === 'save') return this.remember(this.create(false, e.save.name));
    if (e.kind === 'room') return this.remember({ t: 'join', room: e.room.code, slots: this.slots() });
    return null;
  }

  /** „Neuer Spielstand“ mit Anlegen-Dialog: öffnet den Dialog statt sofort anzulegen (K5.2) */
  openDialog(name: string): void {
    this.dialog = { ...initialDialog(worldOf(this.params.world)), save: name };
  }

  /** Welten des Dialogs: die des Servers, dazu eine per `?world=` vorgegebene (der Server prüft sie beim Anlegen) */
  dialogWorlds(): readonly string[] {
    const w = this.params.world;
    return w && !this.worlds.includes(w) ? [...this.worlds, w] : this.worlds;
  }

  /** Eingabe im offenen Dialog; „Raum anlegen“ liefert den create-Befehl mit den Optionen, „Abbrechen“ schließt. */
  dialogInput(input: DialogInput): LobbyCommand | null {
    if (!this.dialog) return null;
    const { save } = this.dialog;
    const { dialog, action } = dialogStep(this.dialog, input, this.dev, this.dialogWorlds());
    this.dialog = action ? null : { ...dialog, save };
    if (action !== 'create') return null;
    this.retried = false;
    this.retryable = false;
    return this.remember(this.create(true, save, createOptions(dialog.options), dialog.options.world));
  }

  private create(fresh: boolean, save = this.params.save, options?: RoomOptions, chosen?: string): LobbyCommand {
    const { depth, seed } = this.params;
    const world = chosen === undefined ? this.params.world : chosen === DEFAULT_WORLD ? '' : chosen; // `standard` ist der Standard, das Feld entfällt
    return { t: 'create', save, fresh, depth, slots: this.slots(), ...(fresh && seed ? { seed } : {}), ...(fresh && world ? { world } : {}), ...(options ? { options } : {}) };
  }

  private slots(): number[] {
    return slotsFor(this.params.mock, this.params.players);
  }

  private remember(cmd: LobbyCommand): LobbyCommand {
    this.last = cmd;
    return cmd;
  }
}
