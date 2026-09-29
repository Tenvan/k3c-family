import { biomeForDepth } from '../world/biome';
import { IDLE, type PlayerCommand, type World } from '../world/sim/types';
import { addPlayer, createWorld, step } from '../world/sim/world';
import { MAX_ONLINE_PLAYERS, snapshotWorld, type ServerMessage } from './protocol';

/** Kanal zu einem Client (WebSocket im Server, direkter Aufruf in Tests). */
export interface Conn {
  send(message: string): void;
}

interface Member {
  index: number;
  conn: Conn | null;
  cmd: PlayerCommand;
}

export interface RoomOptions {
  depth: number;
  fast: boolean;
  seed?: string;
}

/**
 * Ein Spielraum: eine Welt, mehrere Verbindungen. Wer mit derselben clientId zurückkommt, bekommt seinen Monarchen wieder.
 * Ohne verbundene Spieler pausiert die Welt.
 */
export class Room {
  readonly world: World;
  readonly seed: string;
  readonly depth: number;
  readonly fast: boolean;
  private readonly members = new Map<string, Member>();
  emptySince: number | null = null;

  constructor(readonly code: string, options: RoomOptions) {
    this.seed = options.seed ?? code;
    this.depth = options.depth;
    this.fast = options.fast;
    this.world = createWorld(biomeForDepth(this.depth), this.seed, { cycleSpeed: this.fast ? 8 : 1 });
  }

  get connected(): number {
    return [...this.members.values()].filter((m) => m.conn).length;
  }

  /** Index des Monarchen oder null, wenn der Raum voll ist. */
  join(clientId: string, conn: Conn): number | null {
    let member = this.members.get(clientId);
    if (!member) {
      if (this.members.size >= MAX_ONLINE_PLAYERS) return null;
      const player = addPlayer(this.world);
      member = { index: player.index, conn, cmd: IDLE };
      this.members.set(clientId, member);
    }
    member.conn = conn;
    member.cmd = IDLE;
    this.emptySince = null;
    const welcome: ServerMessage = { t: 'welcome', you: member.index, room: this.code, seed: this.seed, depth: this.depth, fast: this.fast };
    conn.send(JSON.stringify(welcome));
    return member.index;
  }

  leave(clientId: string, conn: Conn): void {
    const member = this.members.get(clientId);
    if (!member || member.conn !== conn) return; // schon durch eine neuere Verbindung ersetzt
    member.conn = null;
    member.cmd = IDLE;
    if (this.connected === 0) this.emptySince = Date.now();
  }

  input(clientId: string, cmd: PlayerCommand): void {
    const member = this.members.get(clientId);
    if (member) member.cmd = cmd;
  }

  /** Ein Tick: rechnen und den Zustand an alle schicken. */
  tick(dt: number): void {
    if (this.connected === 0) return;
    const commands: PlayerCommand[] = [];
    for (const m of this.members.values()) commands[m.index] = m.cmd;
    step(this.world, commands, dt);
    const message = JSON.stringify({ t: 'state', s: snapshotWorld(this.world) } satisfies ServerMessage);
    for (const m of this.members.values()) m.conn?.send(message);
  }
}

/** Alle Räume; leere Räume werden nach einer Weile verworfen. */
export class Rooms {
  private readonly rooms = new Map<string, Room>();

  constructor(private readonly keepEmptyMs = 10 * 60 * 1000) {}

  get(code: string, options: RoomOptions): Room {
    let room = this.rooms.get(code);
    if (!room) {
      room = new Room(code, options);
      this.rooms.set(code, room);
    }
    return room;
  }

  tick(dt: number): void {
    const now = Date.now();
    for (const [code, room] of this.rooms) {
      room.tick(dt);
      if (room.emptySince !== null && now - room.emptySince > this.keepEmptyMs) this.rooms.delete(code);
    }
  }
}
