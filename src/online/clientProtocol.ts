import type { LevelLayout } from '../model/types';
import type { GameEvent, ResourceKind, World } from '../model/types';

/** Protokoll v3 aus Sicht des Browsers (Vertrag: docs/protocol.md, Beispiele: testdata/protocol/). */

export const PROTOCOL_VERSION = 3;
export const WS_PATH = '/ws';
/** Ein Gerät sendet mindestens alle 500 ms eine `input` zur Bestätigung. */
export const INPUT_KEEPALIVE_MS = 500;

export type ErrorCode =
  | 'room_full'
  | 'too_many_slots'
  | 'too_many_rooms'
  | 'room_not_found'
  | 'save_exists'
  | 'save_not_found'
  | 'room_closed'
  | 'replaced'
  | 'version'
  | 'bad_request'
  | 'forbidden';

export interface Limits {
  monarchsPerRoom: number;
  slotsPerDevice: number;
  rooms: number;
}

export interface RoomInfo {
  code: string;
  name: string;
  depth: number;
  grade: string;
  taken: number;
  free: number;
  running: boolean;
}

export interface SlotSeat {
  slot: number;
  monarch: number;
  /** Tiefe der Stufe, in der der Monarch steht. */
  depth: number;
}

export type MonarchState = 'taken' | 'waiting' | 'free';

export interface SlotInput {
  slot: number;
  moveX: number;
  sprint: boolean;
  pay: boolean;
}

/** Dynamischer Weltzustand: alles außer dem Statischen (kommt mit `level`), dazu `events` und `depth`. */
export type StaticKey = 'seed' | 'biome' | 'level' | 'rng' | 'widthUnits';
/** `devTimescale`: Faktor des Zeitraffers, nur im Dev-Mode des Servers (docs/protocol.md › Dev-Aktionen). */
export type WorldState = Omit<World, StaticKey> & { events: GameEvent[]; depth: number; devTimescale?: number };

export type ServerMessage =
  | { t: 'welcome'; v: number; tickHz: number; limits: Limits }
  | { t: 'rooms'; rooms: RoomInfo[] }
  | { t: 'joined'; room: string; name: string; you: SlotSeat[] }
  | { t: 'level'; depth: number; layout: LevelLayout }
  | { t: 'snap'; tick: number; ack: number; s: WorldState }
  | { t: 'delta'; tick: number; ack: number; s: Record<string, unknown> }
  | { t: 'seats'; you: SlotSeat[]; monarchs: MonarchState[] }
  | { t: 'error'; code: ErrorCode; message: string };

export type ClientMessage =
  | { t: 'hello'; v: number; device: string }
  | { t: 'create'; save: string; fresh: boolean; depth?: number; slots: number[]; grade?: string; goal?: string; defeat?: string }
  | { t: 'join'; room: string; slots: number[] }
  | { t: 'addSlot'; slot: number }
  | { t: 'removeSlot'; slot: number }
  | { t: 'input'; seq: number; p: SlotInput[] }
  | { t: 'leave' }
  | DevMessage;

/** Rohstoffe der Aktion `material`: alle, die der Server kennt (`engine/sim` › `stockField`), auch Eisen und Kristall. */
export type DevResource = ResourceKind | 'iron' | 'crystal';

/** Dev-Aktionen (nur Dev-Mode am Server, sonst `forbidden`; docs/protocol.md › Dev-Aktionen). */
export type DevMessage =
  | { t: 'dev'; action: 'gold'; slot: number; amount: number }
  | { t: 'dev'; action: 'material'; slot: number; resource: DevResource; amount: number }
  | { t: 'dev'; action: 'timescale'; factor: number };
