import type { Status } from '../online/clientConnection';
import type { ErrorCode } from '../online/clientProtocol';
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
  /** Hinweis und Code des letzten Server-Fehlers (`forbidden` nach einer Dev-Aktion ohne Dev-Mode) */
  notice: string | null;
  errorCode: ErrorCode | null;
}

export type DebugWorld = {
  cycle: Pick<World['cycle'], 'day'>;
  enemies: unknown[];
  troops: unknown[];
  players: unknown[];
  /** Zeitraffer des Raums, nur im Dev-Mode im Zustand (docs/protocol.md › Dev-Aktionen) */
  devTimescale?: number;
  /** Dev-Pause des Raums (Cheat-Dialog, B-231) */
  devPaused?: boolean;
};

export interface DebugInput {
  client: DebugClient;
  protocol: number;
  world: DebugWorld | null;
  fps: number | null;
  /** `env.now()` des Clients (gleiche Uhr wie `lastSnapshotAt`) */
  now: number;
  /** Verzögerung der Zeitleiste in ms (B-277); fehlt = keine Zeile */
  delayMs?: number;
  /** Versionszeile `Client … · Server …` (src/core/version.ts); fehlt = keine Zeile */
  version?: string;
}

/** Schlüssel der Versionszeile in der Phaser-Registry (gesetzt in src/main.ts). */
export const VERSION_KEY = 'versionLine';

/** Entwicklungsphase: Overlay ist standardmäßig verfügbar, `?dev=0` schaltet es ab (B-093, Revision 2). `search` ist `location.search`. */
export const debugEnabled = (search: string): boolean => new URLSearchParams(search).get('dev') !== '0';

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
  if (c.status === 'connecting') return [...lines, `${num(i.fps)} FPS`, ...(i.version ? [i.version] : [])];
  const age = c.lastSnapshotAt === null ? null : i.now - c.lastSnapshotAt;
  lines.push(`letzter Snapshot ${num(age)} ms`);
  if (i.delayMs !== undefined) lines.push(`Puffer ${num(i.delayMs)} ms`);
  if (world) {
    lines.push(`Tag ${world.cycle.day} · ${world.enemies.length} Gegner · ${world.troops.length} Truppen · ${world.players.length} Spieler`);
    if (world.devTimescale !== undefined) lines.push(world.devPaused ? 'Raum angehalten' : `Zeit ${world.devTimescale}×`);
  }
  if (c.errorCode === 'forbidden') lines.push(`Dev abgelehnt: ${c.notice ?? ''}`);
  lines.push(`${num(i.fps)} FPS`);
  if (i.version) lines.push(i.version);
  return lines;
}
