/**
 * Auswahl- und Befehlslogik der Lobby ohne Phaser (SP08.3): Start-Parameter (B-082), Eintragsliste, Befehle, Hinweise.
 * `LobbyScene` zeichnet nur und gibt Eingaben weiter.
 */
import { t } from '../core/texts';
import type { SaveInfo } from '../core/saveStore';
import type { RoomInfo } from '../online/clientProtocol';
import type { Status } from '../online/clientConnection';

export const DEFAULT_SAVE = 'familie';
const SAVE_FORMAT = /^[a-z0-9-]{1,32}$/;
const ROOM_FORMAT = /^[A-HJ-NP-Z]{4}$/;
const MOCK_FORMAT = /^[0-3]$/;
const PLAYERS_FORMAT = /^[1-4]$/;

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
}

/** Wertet `?autostart=1&fresh=1&save=NAME&mock=N&room=CODE&players=N` aus; ungültige Werte gelten als nicht gesetzt. */
export function parseStartParams(search: string): StartParams {
  const p = new URLSearchParams(search);
  const save = p.get('save') ?? '';
  const mock = p.get('mock') ?? '';
  const room = (p.get('room') ?? '').toUpperCase();
  const players = p.get('players') ?? '';
  return {
    autostart: p.get('autostart') === '1',
    fresh: p.get('fresh') === '1',
    save: SAVE_FORMAT.test(save) ? save : DEFAULT_SAVE,
    mock: MOCK_FORMAT.test(mock) ? Number(mock) : 0,
    room: ROOM_FORMAT.test(room) ? room : null,
    players: PLAYERS_FORMAT.test(players) ? Number(players) : 1,
  };
}

/** Slots eines Geräts: 0 ist der echte Spieler, 1..mock sind Mock-Slots; `players` lokale Spieler belegen 0..players-1 (B-353) */
export function slotsFor(mock: number, players = 1): number[] {
  return Array.from({ length: Math.max(mock + 1, players) }, (_, i) => i);
}

export type LobbyCommand =
  | { t: 'create'; save: string; fresh: boolean; depth: number; slots: number[] }
  | { t: 'join'; room: string; slots: number[] };

export interface LobbyClient {
  create(save: string, fresh: boolean, depth: number, slots: number[]): void;
  join(room: string, slots: number[]): void;
}

export function applyCommand(client: LobbyClient, cmd: LobbyCommand): void {
  if (cmd.t === 'create') client.create(cmd.save, cmd.fresh, cmd.depth, cmd.slots);
  else client.join(cmd.room, cmd.slots);
}

export type LobbyEntry =
  | { kind: 'play' }
  | { kind: 'room'; room: RoomInfo }
  | { kind: 'save'; save: SaveInfo }
  | { kind: 'retry' }
  | { kind: 'reload' };

/** Höchstens so viele Spielstände unter den Räumen (LB1, Offene Fragen) */
export const MAX_SAVES = 6;

/**
 * „Spielen“ steht oben, darunter die Räume des Servers, dann die neuesten Spielstände, die noch nicht als Raum offen sind
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
    .sort((a, b) => Date.parse(b.savedAt) - Date.parse(a.savedAt))
    .slice(0, MAX_SAVES);
  return [
    { kind: 'play' },
    ...rooms.map((room): LobbyEntry => ({ kind: 'room', room })),
    ...newest.map((save): LobbyEntry => ({ kind: 'save', save })),
  ];
}

export function moveSelection(selected: number, dir: number, count: number): number {
  return Math.max(0, Math.min(count - 1, selected + dir));
}

export function entryLabel(e: LobbyEntry, save: string): string {
  if (e.kind === 'play') return t('lobby.play', { save });
  if (e.kind === 'retry') return t('lobby.retry');
  if (e.kind === 'reload') return t('lobby.reload');
  if (e.kind === 'save') return t('lobby.save', { name: e.save.name, day: e.save.day ?? '–', depths: e.save.depths.join(', ') || '–' });
  const r = e.room;
  return t('lobby.room', { code: r.code, name: r.name, depth: r.depth, taken: r.taken, state: t(r.running ? 'lobby.running' : 'lobby.paused') });
}

/** Zeile unter dem Zeiger (Touch), -1 = keine */
export function rowAt(y: number, top: number, rowHeight: number, count: number): number {
  const row = Math.floor((y - top) / rowHeight);
  return y >= top && row < count ? row : -1;
}

/** Hinweis der Lobby: Verbindungsstand oder Fehlertext des Servers (jeder Fehler-Code bringt seine `message` mit) */
export function lobbyNotice(c: { status: Status; notice: string | null }): string | null {
  if (c.status === 'connecting') return c.notice ?? t('net.connecting');
  return c.notice;
}

/** Hinweis im Spiel bei Abbruch der Verbindung, sonst null */
export function gameNotice(c: { status: Status; notice: string | null }): string | null {
  return c.status === 'reconnecting' ? c.notice : null;
}

/** Das Spiel endet, sobald der Client nicht mehr im Raum ist (Raum geschlossen/unbekannt, 120 s ohne Verbindung, an anderer Stelle geöffnet) */
export function leavesGame(status: Status): boolean {
  return status === 'lobby' || status === 'lost' || status === 'ended';
}

/**
 * Wann die Lobby welchen Befehl sendet: Start-Parameter (`?room`, `?autostart`) einmal, Auswahl aus der Liste,
 * `save_not_found` → ein zweiter Versuch mit `fresh: true`, `depth: 0`.
 */
export class LobbyFlow {
  private started = false;
  private retried = false;
  private last: LobbyCommand | null = null;

  constructor(private readonly params: StartParams) {}

  /** Bei jedem Frame mit dem Client-Zustand aufrufen; liefert den nächsten Befehl oder null. */
  step(c: { status: Status; errorCode: string | null }): LobbyCommand | null {
    if (c.status !== 'lobby') return null;
    const slots = this.slots();
    if (!this.started && (this.params.room || this.params.autostart)) {
      this.started = true;
      return this.remember(this.params.room ? { t: 'join', room: this.params.room, slots } : this.create(this.params.fresh));
    }
    // nur für „Spielen“: ein gewählter Spielstand existiert, er wird nie leer neu angelegt
    const last = this.last;
    if (c.errorCode === 'save_not_found' && last?.t === 'create' && !last.fresh && last.save === this.params.save && !this.retried) {
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
    if (e.kind === 'play') return this.remember(this.create(this.params.fresh));
    if (e.kind === 'save') return this.remember(this.create(false, e.save.name));
    if (e.kind === 'room') return this.remember({ t: 'join', room: e.room.code, slots: this.slots() });
    return null;
  }

  private create(fresh: boolean, save = this.params.save): LobbyCommand {
    return { t: 'create', save, fresh, depth: 0, slots: this.slots() };
  }

  private slots(): number[] {
    return slotsFor(this.params.mock, this.params.players);
  }

  private remember(cmd: LobbyCommand): LobbyCommand {
    this.last = cmd;
    return cmd;
  }
}
