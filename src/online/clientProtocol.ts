import type { LevelLayout } from '../model/types';
import type { GameEvent, ResourceKind, World } from '../model/types';

/** Protokoll v8 aus Sicht des Browsers (Vertrag: docs/protocol.md, Beispiele: testdata/protocol/). */

export const PROTOCOL_VERSION = 8;
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
  | 'forbidden'
  | 'world_unknown'
  | 'restarting';

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
  /** Index der Stufe: gehört zum Strom mit diesem `stage` (B-176). Der Server sendet es immer; im Typ optional, weil Szenen-Tests Plätze ohne Stufe bauen. */
  stage?: number;
}

export type MonarchState = 'taken' | 'waiting' | 'free';

export interface SlotInput {
  slot: number;
  moveX: number;
  sprint: boolean;
  pay: boolean;
  /** Gehalten: Schlag, sobald bereit (fehlt = false). */
  attack?: boolean;
  /** Gehalten: Skill-Slot 1 bis 4 feuert, sobald bereit; 0 = keiner (fehlt = 0). */
  skill?: number;
}

/** Dynamischer Weltzustand: alles außer dem Statischen (kommt mit `level`), dazu `events` und `depth`. */
export type StaticKey = 'seed' | 'biome' | 'level' | 'rng' | 'widthUnits';
/** `devTimescale`: Faktor des Zeitraffers, `devPaused`: Dev-Pause, beide nur im Dev-Mode des Servers (docs/protocol.md › Dev-Aktionen). */
export type WorldState = Omit<World, StaticKey> & { events: GameEvent[]; depth: number; devTimescale?: number; devPaused?: boolean };

export type ServerMessage =
  | { t: 'welcome'; v: number; tickHz: number; limits: Limits }
  | { t: 'rooms'; rooms: RoomInfo[] }
  /** `world`: Welt-ID des Raums (docs/rules/welten.md § 6). */
  | { t: 'joined'; room: string; name: string; world: string; you: SlotSeat[] }
  | { t: 'level'; stage: number; depth: number; layout: LevelLayout }
  | { t: 'snap'; stage: number; tick: number; ack: number; s: WorldState }
  | { t: 'delta'; stage: number; tick: number; ack: number; s: Record<string, unknown> }
  | { t: 'seats'; you: SlotSeat[]; monarchs: MonarchState[] }
  | { t: 'error'; code: ErrorCode; message: string };

export type ClientMessage =
  | { t: 'hello'; v: number; device: string }
  /** `world`: Welt-ID eines neuen Stands, fehlt = standard; eine andere nur im Dev-Mode des Servers (sonst `forbidden`). */
  | { t: 'create'; save: string; fresh: boolean; depth?: number; seed?: string; world?: string; slots: number[]; grade?: string; goal?: string; defeat?: string }
  | { t: 'join'; room: string; slots: number[] }
  | { t: 'addSlot'; slot: number }
  | { t: 'removeSlot'; slot: number }
  | { t: 'input'; seq: number; p: SlotInput[] }
  | { t: 'learn'; slot: number; skill: string }
  | { t: 'respec'; slot: number }
  | { t: 'leave' }
  /** Pause-Menü offen bzw. geschlossen (v8, BD7.3): Gerät pausieren bzw. fortsetzen, nur im Raum. */
  | { t: 'pause' }
  | { t: 'resume' }
  | DevMessage;

/** Dev-Aktionen (nur Dev-Mode am Server, sonst `forbidden`; docs/protocol.md › Dev-Aktionen). */
export type DevMessage =
  | { t: 'dev'; action: 'gold'; slot: number; amount: number }
  | { t: 'dev'; action: 'material'; slot: number; resource: ResourceKind; amount: number }
  | { t: 'dev'; action: 'timescale'; factor: number }
  | { t: 'dev'; action: 'pause'; paused: boolean }
  | { t: 'dev'; action: 'maxStock'; amount: number }
  | { t: 'dev'; action: 'fillStock' }
  | { t: 'dev'; action: 'cheapBuild'; on: boolean }
  | { t: 'dev'; action: 'buildAll'; level: number }
  | { t: 'dev'; action: 'timescaleAuto'; on: boolean }
  | { t: 'dev'; action: 'nextPhase' }
  | { t: 'dev'; action: 'castleFall'; slot: number }
  /** Grad des Raums, gilt ab der nächsten Welle; nur im Dev-Mode (B-080, KB2.6) */
  | { t: 'dev'; action: 'grade'; grade: string };
