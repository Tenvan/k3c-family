import { BIOMES, type BiomeConfig } from '../model/biome';
import type { LevelLayout } from '../model/types';
import { clientLog } from '../core/clientLog';
import { t } from '../core/texts';
import { applyDelta } from './clientDelta';
import { LatencyMeter, type LatencyStats } from './clientLatency';
import {
  INPUT_KEEPALIVE_MS,
  PROTOCOL_VERSION,
  WS_PATH,
  type ClientMessage,
  type DevMessage,
  type ErrorCode,
  type Limits,
  type MonarchState,
  type RoomInfo,
  type ServerMessage,
  type SlotInput,
  type SlotSeat,
  type WorldState,
} from './clientProtocol';

/**
 * Client für Protokoll v2 (docs/protocol.md), ohne Phaser. Zustandsautomat:
 * `connecting` → `lobby` ↔ `room`; bei Abbruch `reconnecting`; endgültig `ended` (replaced, version) oder `lost` (Server nicht erreichbar).
 */

const DEVICE_KEY = 'k3c-client';
const MAX_DEVICE_ID = 64;
const RETRY_FIRST_MS = 500;
const RETRY_MAX_MS = 4000;
/** So lange versucht der Client nach einem Abbruch, sich neu zu verbinden. */
export const RECONNECT_LIMIT_MS = 120_000;
/** Anzahl Snapshots für die Takt-Messung des Debug-Overlays */
const SNAPSHOT_WINDOW = 30;
/** Kleinster Abstand zweier geänderter Eingaben (angenommen: unter einem Bild bei 60 FPS, B-277) */
const INPUT_MIN_GAP_MS = 8;
/** Raster der analogen Laufrichtung (Stufen je Unit, also 0,05): feineres Zittern des Sticks zählt nicht als Änderung (angenommen) */
const MOVE_STEPS = 20;

/** `moveX` auf 1 / `MOVE_STEPS` gerastert, für Vergleich und Versand (`+ 0` macht aus -0 eine 0). */
export const roundMoves = (p: SlotInput[]): SlotInput[] => p.map((i) => ({ ...i, moveX: Math.round(i.moveX * MOVE_STEPS) / MOVE_STEPS + 0 }));

export interface SocketLike {
  send(data: string): void;
  close(): void;
  onopen: (() => void) | null;
  onmessage: ((e: { data: unknown }) => void) | null;
  onclose: (() => void) | null;
}

/** Alles, was Tests ersetzen: Verbindung, Uhr, Zeitgeber, Geräte-ID. */
export interface ClientEnv {
  connect(): SocketLike;
  now(): number;
  setTimer(fn: () => void, ms: number): unknown;
  clearTimer(handle: unknown): void;
  deviceId: string;
}

export type Status = 'connecting' | 'lobby' | 'room' | 'reconnecting' | 'ended' | 'lost';

/** Ein Tick: voller Zustand (nicht verändern, Teile werden mit früheren Frames geteilt). */
export interface Frame {
  tick: number;
  ack: number;
  /** `env.now()` beim Empfang, Grundlage der Interpolation */
  receivedAt: number;
  state: WorldState;
}

export interface LevelInfo {
  depth: number;
  layout: LevelLayout;
  biome: BiomeConfig;
}

function randomId(): string {
  const bytes = crypto.getRandomValues(new Uint8Array(16));
  return Array.from(bytes, (b) => b.toString(16).padStart(2, '0')).join('');
}

/** Dauerhafte Geräte-ID: im `localStorage`, ohne Speicher eine neue Zufalls-ID (`randomUUID` fehlt in unsicheren Kontexten, z. B. http im Heimnetz). */
export function getDeviceId(storage?: Pick<Storage, 'getItem' | 'setItem'> | null): string {
  try {
    let id = storage?.getItem(DEVICE_KEY);
    if (!id || id.length > MAX_DEVICE_ID) {
      id = randomId();
      storage?.setItem(DEVICE_KEY, id);
    }
    return id;
  } catch {
    return randomId();
  }
}

export class RoomClient {
  status: Status = 'connecting';
  /** Hinweis für die Anzeige (Fehlertext des Servers oder Verbindungsstatus), null = keiner */
  notice: string | null = null;
  /** Code des Fehlers hinter `notice` (nur bei Fehlern des Servers), sonst null */
  errorCode: ErrorCode | null = null;
  rooms: RoomInfo[] = [];
  limits: Limits | null = null;
  tickHz = 30;
  roomCode: string | null = null;
  roomName = '';
  you: SlotSeat[] = [];
  monarchs: MonarchState[] = [];
  level: LevelInfo | null = null;
  /** Wird nach jeder Änderung aufgerufen (Anzeige aktualisieren) */
  onChange: (() => void) | null = null;

  private sock!: SocketLike;
  private closedByUs = false;
  private timer: unknown = null;
  private downSince = 0;
  private attempt = 0;
  private seq = 0;
  private lastInput = '';
  private lastInputAt = -Infinity;
  private slotsWanted: number[] = [];
  private state: Record<string, unknown> | null = null;
  private frames: Frame[] = [];
  private snapshotTimes: number[] = [];
  private loggedStatus: Status | null = null;
  private framesSinceLevel = 0;
  private readonly latencyMeter = new LatencyMeter();

  constructor(private readonly env: ClientEnv) {
    this.open();
  }

  get deviceId(): string {
    return this.env.deviceId;
  }

  /** `env.now()` beim letzten Snapshot, null bis zum ersten (Debug-Overlay). */
  get lastSnapshotAt(): number | null {
    return this.snapshotTimes[this.snapshotTimes.length - 1] ?? null;
  }

  /** Latenz Eingabe → Zustand (Mittel, p95) der letzten 60 s, null ohne Messung (Debug-Overlay, B-181). */
  get latency(): LatencyStats | null {
    return this.latencyMeter.stats(this.env.now());
  }

  /** Gemessener Snapshot-Takt über die letzten Snapshots, null ohne zwei Messpunkte (Debug-Overlay). */
  get snapshotHz(): number | null {
    const n = this.snapshotTimes.length;
    const span = n > 1 ? this.snapshotTimes[n - 1]! - this.snapshotTimes[0]! : 0;
    return span > 0 ? ((n - 1) * 1000) / span : null;
  }

  /** `env.now()` des Verbindungsverlusts, solange wiederverbunden wird, sonst null (Debug-Overlay). */
  get offlineSince(): number | null {
    return this.status === 'reconnecting' ? this.downSince : null;
  }

  create(save: string, fresh: boolean, depth: number, slots: number[]): void {
    if (this.status !== 'lobby') return;
    this.slotsWanted = slots;
    this.notice = null;
    this.errorCode = null;
    this.send({ t: 'create', save, fresh, ...(fresh ? { depth } : {}), slots });
  }

  join(room: string, slots: number[]): void {
    if (this.status !== 'lobby') return;
    this.slotsWanted = slots;
    this.notice = null;
    this.errorCode = null;
    this.send({ t: 'join', room, slots });
  }

  /** Dev-Aktion (nur im Dev-Mode des Servers wirksam, sonst `forbidden`; docs/protocol.md › Dev-Aktionen), nur im Raum. */
  sendDev(message: DevMessage): void {
    if (this.status === 'room') this.send(message);
  }

  addSlot(slot: number): void {
    if (this.status === 'room') this.send({ t: 'addSlot', slot });
  }

  removeSlot(slot: number): void {
    if (this.status === 'room') this.send({ t: 'removeSlot', slot });
  }

  /** Skill für den Monarchen des Slots lernen (Skill-ID aus `data/monarch.json`), nur im Raum. */
  learn(slot: number, skill: string): void {
    if (this.status === 'room') this.send({ t: 'learn', slot, skill });
  }

  /** Skill-Verteilung des Slots zurücksetzen (nur am Tag an der Burg, prüft der Server), nur im Raum. */
  respec(slot: number): void {
    if (this.status === 'room') this.send({ t: 'respec', slot });
  }

  /** Raum bewusst verlassen: zurück zur Raumliste, kein Wiederverbinden. */
  leave(): void {
    if (this.status !== 'room') return;
    this.send({ t: 'leave' });
    this.clearRoom();
    this.status = 'lobby';
    this.changed();
  }

  /**
   * Eingaben der lokalen Slots. Sendet bei Änderung sofort (Abstand mindestens `INPUT_MIN_GAP_MS`), sonst spätestens nach 500 ms.
   * Aufruf in jedem Frame ist vorgesehen.
   */
  sendInput(raw: SlotInput[]): void {
    if (this.status !== 'room') return;
    const p = roundMoves(raw);
    const json = JSON.stringify(p);
    const sinceLast = this.env.now() - this.lastInputAt;
    const changed = json !== this.lastInput && sinceLast >= INPUT_MIN_GAP_MS;
    if (!changed && sinceLast < INPUT_KEEPALIVE_MS) return;
    this.lastInput = json;
    this.lastInputAt = this.env.now();
    this.send({ t: 'input', seq: ++this.seq, p });
    this.latencyMeter.sent(this.seq, this.lastInputAt);
  }

  /** Alle seit dem letzten Aufruf empfangenen Ticks, älteste zuerst. */
  takeFrames(): Frame[] {
    const frames = this.frames;
    this.frames = [];
    return frames;
  }

  /** Nach `lost` (Server nicht erreichbar) erneut versuchen. */
  retry(): void {
    if (this.status !== 'lost') return;
    this.status = 'connecting';
    this.notice = null;
    this.downSince = this.env.now();
    this.attempt = 0;
    this.open();
    this.changed();
  }

  close(): void {
    this.closedByUs = true;
    this.stopTimer();
    this.sock.close();
  }

  private open(): void {
    const sock = this.env.connect();
    this.sock = sock;
    sock.onopen = () => {
      if (sock !== this.sock) return;
      this.seq = 0;
      this.lastInput = '';
      this.lastInputAt = -Infinity;
      this.latencyMeter.reset();
      this.send({ t: 'hello', v: PROTOCOL_VERSION, device: this.env.deviceId });
    };
    sock.onmessage = (e) => {
      if (sock === this.sock) this.handle(e.data);
    };
    sock.onclose = () => {
      if (sock === this.sock) this.dropped();
    };
  }

  private send(message: ClientMessage): void {
    this.sock.send(JSON.stringify(message));
  }

  private changed(): void {
    if (this.status !== this.loggedStatus) {
      this.loggedStatus = this.status;
      clientLog(this.status === 'lost' ? 'error' : 'info', `🔌 Verbindung: ${this.status}`, { room: this.roomCode, notice: this.notice, errorCode: this.errorCode });
    }
    this.onChange?.();
  }

  private stopTimer(): void {
    if (this.timer !== null) this.env.clearTimer(this.timer);
    this.timer = null;
  }

  private clearRoom(): void {
    this.roomCode = null;
    this.roomName = '';
    this.you = [];
    this.monarchs = [];
    this.level = null;
    this.state = null;
    this.frames = [];
    this.slotsWanted = [];
  }

  /** Verbindung weg ohne Abmeldung: neu verbinden, bis `RECONNECT_LIMIT_MS` um sind. */
  private dropped(): void {
    if (this.closedByUs || this.status === 'ended' || this.status === 'lost') return;
    if (this.status !== 'reconnecting') {
      this.status = 'reconnecting';
      this.notice = t('net.reconnecting');
      this.downSince = this.env.now();
      this.attempt = 0;
      clientLog('warn', '🔁 Verbindung abgebrochen, verbinde neu', { room: this.roomCode });
      this.changed();
    }
    if (this.env.now() - this.downSince >= RECONNECT_LIMIT_MS) {
      this.clearRoom();
      this.status = 'lost';
      this.notice = t('net.lost');
      this.changed();
      return;
    }
    const delay = Math.min(RETRY_FIRST_MS * 2 ** this.attempt++, RETRY_MAX_MS);
    this.timer = this.env.setTimer(() => {
      this.timer = null;
      this.open();
    }, delay);
  }

  private end(notice: string): void {
    this.status = 'ended';
    this.notice = notice;
    this.closedByUs = true;
    this.stopTimer();
    this.sock.close();
  }

  private handle(data: unknown): void {
    let msg: ServerMessage;
    try {
      msg = JSON.parse(String(data)) as ServerMessage;
    } catch {
      clientLog('warn', '💥 Nachricht des Servers nicht lesbar', { start: String(data).slice(0, 120) });
      return;
    }
    this.apply(msg);
    this.changed();
  }

  private apply(msg: ServerMessage): void {
    switch (msg.t) {
      case 'welcome':
        this.limits = msg.limits;
        this.tickHz = msg.tickHz;
        if (this.roomCode) this.send({ t: 'join', room: this.roomCode, slots: this.slotsWanted });
        else this.setConnected();
        break;
      case 'rooms':
        this.rooms = msg.rooms;
        break;
      case 'joined':
        this.clearRoom();
        this.roomCode = msg.room;
        this.roomName = msg.name;
        this.you = msg.you;
        this.slotsWanted = msg.you.map((s) => s.slot);
        this.setConnected('room');
        break;
      case 'level':
        this.setLevel(msg.depth, msg.layout);
        break;
      case 'snap':
        this.state = msg.s as unknown as Record<string, unknown>;
        this.pushFrame(msg.tick, msg.ack);
        break;
      case 'delta':
        if (!this.state) return void console.warn('delta ohne snap verworfen');
        this.state = applyDelta(this.state, msg.s);
        this.pushFrame(msg.tick, msg.ack);
        break;
      case 'seats':
        this.you = msg.you;
        this.monarchs = msg.monarchs;
        this.slotsWanted = msg.you.map((s) => s.slot);
        break;
      case 'error':
        this.fail(msg.code, msg.message);
        break;
    }
  }

  private setConnected(status: 'lobby' | 'room' = 'lobby'): void {
    this.status = status;
    this.notice = null;
    this.errorCode = null;
  }

  private setLevel(depth: number, layout: LevelLayout): void {
    const biome = BIOMES.find((b) => b.id === layout.biomeId);
    this.state = null;
    this.frames = [];
    if (!biome) {
      this.notice = t('net.unknownBiome');
      this.level = null;
      clientLog('error', `💥 Unbekanntes Biom ${layout.biomeId}`, { depth });
      return;
    }
    this.level = { depth, layout, biome };
    this.framesSinceLevel = 0;
    clientLog('info', '📂 Level empfangen', { depth, biome: biome.id, width: layout.widthUnits });
  }

  private pushFrame(tick: number, ack: number): void {
    const receivedAt = this.env.now();
    this.latencyMeter.acked(ack, receivedAt);
    if (this.framesSinceLevel++ === 0) clientLog('info', '✅ Erster Zustand nach Level', { tick, sites: (this.state as unknown as WorldState).sites?.map((s) => s.kind) });
    this.frames.push({ tick, ack, receivedAt, state: this.state as unknown as WorldState });
    this.snapshotTimes.push(receivedAt);
    if (this.snapshotTimes.length > SNAPSHOT_WINDOW) this.snapshotTimes.shift();
  }

  /** Verhalten je Fehler-Code (docs/protocol.md › Fehler-Codes). */
  private fail(code: string, message: string): void {
    clientLog(code === 'bad_request' ? 'error' : 'warn', `🚫 Server-Fehler ${code}: ${message}`, { status: this.status, room: this.roomCode });
    switch (code) {
      case 'replaced':
        return this.end(t('net.replaced'));
      case 'version':
        return this.end(t('net.version'));
      case 'bad_request':
        return void console.warn(`bad_request: ${message}`);
      case 'forbidden':
        break; // dev ohne Dev-Mode: Hinweis zeigen, Raum und Verbindung bleiben
      case 'room_closed':
      case 'room_not_found':
        this.clearRoom();
        this.setConnected();
        break;
      default:
        // room_full, too_many_slots, too_many_rooms, save_exists, save_not_found: bleibt in Liste bzw. Raum
        if (this.status === 'reconnecting') {
          this.clearRoom();
          this.setConnected();
        }
    }
    this.notice = message;
    this.errorCode = code as ErrorCode;
  }
}

/** Client für den Browser: `ws(s)://<Host>/ws`, Geräte-ID aus dem `localStorage`. */
export function createRoomClient(): RoomClient {
  const scheme = location.protocol === 'https:' ? 'wss' : 'ws';
  let storage: Storage | null = null;
  try {
    storage = localStorage;
  } catch {
    // Speicher gesperrt: Geräte-ID gilt nur bis zum Neuladen
  }
  return new RoomClient({
    connect: () => new WebSocket(`${scheme}://${location.host}${WS_PATH}`) as unknown as SocketLike,
    now: () => performance.now(),
    setTimer: (fn, ms) => setTimeout(fn, ms),
    clearTimer: (h) => clearTimeout(h as number),
    deviceId: getDeviceId(storage),
  });
}
