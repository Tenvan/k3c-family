/**
 * Auswahl- und Befehlslogik der Lobby ohne Phaser (SP08.3): Start-Parameter (B-082), Eintragsliste, Befehle, Hinweise.
 * `LobbyScene` zeichnet nur und gibt Eingaben weiter.
 */
import type { RoomInfo } from '../online/clientProtocol';
import type { Status } from '../online/clientConnection';

export const DEFAULT_SAVE = 'familie';
const SAVE_FORMAT = /^[a-z0-9-]{1,32}$/;
const ROOM_FORMAT = /^[A-HJ-NP-Z]{4}$/;
const MOCK_FORMAT = /^[0-3]$/;

export interface StartParams {
  autostart: boolean;
  fresh: boolean;
  save: string;
  /** Mock-Slots ohne Eingabe (0–3) */
  mock: number;
  /** Raumcode aus `?room=`, null = keiner */
  room: string | null;
}

/** Wertet `?autostart=1&fresh=1&save=NAME&mock=N&room=CODE` aus; ungültige Werte gelten als nicht gesetzt. */
export function parseStartParams(search: string): StartParams {
  const p = new URLSearchParams(search);
  const save = p.get('save') ?? '';
  const mock = p.get('mock') ?? '';
  const room = (p.get('room') ?? '').toUpperCase();
  return {
    autostart: p.get('autostart') === '1',
    fresh: p.get('fresh') === '1',
    save: SAVE_FORMAT.test(save) ? save : DEFAULT_SAVE,
    mock: MOCK_FORMAT.test(mock) ? Number(mock) : 0,
    room: ROOM_FORMAT.test(room) ? room : null,
  };
}

/** Slots eines Geräts: 0 ist der echte Spieler, 1..mock sind Mock-Slots */
export function slotsFor(mock: number): number[] {
  return Array.from({ length: mock + 1 }, (_, i) => i);
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

export type LobbyEntry = { kind: 'play' } | { kind: 'room'; room: RoomInfo };

/** „Spielen“ steht immer oben, darunter die Räume des Servers */
export function lobbyEntries(rooms: readonly RoomInfo[]): LobbyEntry[] {
  return [{ kind: 'play' }, ...rooms.map((room): LobbyEntry => ({ kind: 'room', room }))];
}

export function moveSelection(selected: number, dir: number, count: number): number {
  return Math.max(0, Math.min(count - 1, selected + dir));
}

export function entryLabel(e: LobbyEntry, save: string): string {
  if (e.kind === 'play') return `Spielen  (${save})`;
  const r = e.room;
  return `${r.code}  ${r.name}  ·  Stufe ${r.depth}  ·  ${r.taken}/4 Plätze  ·  ${r.running ? 'läuft' : 'pausiert'}`;
}

/** Zeile unter dem Zeiger (Touch), -1 = keine */
export function rowAt(y: number, top: number, rowHeight: number, count: number): number {
  const row = Math.floor((y - top) / rowHeight);
  return y >= top && row < count ? row : -1;
}

/** Hinweis der Lobby: Verbindungsstand oder Fehlertext des Servers (jeder Fehler-Code bringt seine `message` mit) */
export function lobbyNotice(c: { status: Status; notice: string | null }): string | null {
  if (c.status === 'connecting') return c.notice ?? 'Verbinde …';
  if (c.status === 'lost') return `${c.notice ?? ''} – Taste drücken für neuen Versuch`;
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
    const slots = slotsFor(this.params.mock);
    if (!this.started && (this.params.room || this.params.autostart)) {
      this.started = true;
      return this.remember(this.params.room ? { t: 'join', room: this.params.room, slots } : this.create(this.params.fresh));
    }
    if (c.errorCode === 'save_not_found' && this.last?.t === 'create' && !this.last.fresh && !this.retried) {
      this.retried = true;
      return this.remember(this.create(true));
    }
    return null;
  }

  /** Auswahl in der Liste bestätigt */
  choose(e: LobbyEntry): LobbyCommand {
    this.retried = false;
    return this.remember(e.kind === 'play' ? this.create(this.params.fresh) : { t: 'join', room: e.room.code, slots: slotsFor(this.params.mock) });
  }

  private create(fresh: boolean): LobbyCommand {
    return { t: 'create', save: this.params.save, fresh, depth: 0, slots: slotsFor(this.params.mock) };
  }

  private remember(cmd: LobbyCommand): LobbyCommand {
    this.last = cmd;
    return cmd;
  }
}
