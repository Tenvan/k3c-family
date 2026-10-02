import type { Status } from '../online/clientConnection';
import type { World } from '../model/types';

/** Client-Teil des Overlays: genau die lesbaren Felder von `RoomClient`. */
export interface DebugClient {
  status: Status;
  roomCode: string | null;
  deviceId: string;
  tickHz: number;
  snapshotHz: number | null;
  lastSnapshotAt: number | null;
  offlineSince: number | null;
}

export type DebugWorld = {
  cycle: Pick<World['cycle'], 'day'>;
  enemies: unknown[];
  troops: unknown[];
  players: unknown[];
};

export interface DebugInput {
  client: DebugClient;
  protocol: number;
  world: DebugWorld | null;
  fps: number | null;
  /** `env.now()` des Clients (gleiche Uhr wie `lastSnapshotAt`) */
  now: number;
}

const NONE = '–';

const STATUS_TEXT: Record<Status, string> = {
  connecting: 'verbindet…',
  lobby: 'Lobby',
  room: 'verbunden',
  reconnecting: 'getrennt',
  ended: 'beendet',
  lost: 'verloren',
};

const num = (v: number | null, digits = 0): string => (v === null || !Number.isFinite(v) ? NONE : v.toFixed(digits));

function connectionLine(c: DebugClient, protocol: number, now: number): string {
  if (c.status === 'connecting') return STATUS_TEXT.connecting;
  if (c.status === 'reconnecting') {
    const since = c.offlineSince === null ? null : (now - c.offlineSince) / 1000;
    return `getrennt seit ${num(since, 1)} s`;
  }
  return `${STATUS_TEXT[c.status]} · v${protocol} · ${num(c.snapshotHz)} Hz (Soll ${c.tickHz})`;
}

/** Zeilen des Debug-Overlays aus Verbindungs- und Weltzustand (B-093). Reine Funktion, rechnet nichts am Spiel. */
export function debugLines(i: DebugInput): string[] {
  const { client: c, world } = i;
  const lines = [
    `Raum ${c.roomCode ?? NONE} · Gerät ${c.deviceId.slice(0, 4) || NONE}`,
    connectionLine(c, i.protocol, i.now),
  ];
  if (c.status === 'connecting') return [...lines, `${num(i.fps)} FPS`];
  const age = c.lastSnapshotAt === null ? null : i.now - c.lastSnapshotAt;
  lines.push(`letzter Snapshot ${num(age)} ms`);
  if (world) {
    lines.push(`Tag ${world.cycle.day} · ${world.enemies.length} Gegner · ${world.troops.length} Truppen · ${world.players.length} Spieler`);
  }
  lines.push(`${num(i.fps)} FPS`);
  return lines;
}
