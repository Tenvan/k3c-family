/**
 * Bot-Eingabe für Testläufe (B-349, TR2): `game.html?botfeed=<ws-Adresse>&players=n` nimmt die Eingaben der n lokalen
 * Spieler aus dem Bot-Feed der Workbench (`tools/k3c-dev/internal/botfeed`). Der Client entscheidet nichts, er spielt
 * je Slot das letzte Kommando ab, bis das nächste kommt. Ohne Verbindung gibt es keine Eingabe (die Monarchen stehen).
 */
import { clientLog } from '../core/clientLog';
import type { Action, PlayerInput } from './playerInput';
import { SKILL_ACTIONS } from './slotBindings';

/** Feed-Nachricht (Server → Client), Felder wie `sim.PlayerCommand` (B-349 › Notizen) */
export interface FeedCommand {
  slot: number;
  moveX: number;
  sprint: boolean;
  pay: boolean;
  attack: boolean;
  skill: number;
}

export const MAX_BOT_PLAYERS = 4;
/** Pause vor dem nächsten Verbindungsversuch, wenn der Feed abbricht */
export const RETRY_MS = 1000;
const LOCAL_HOSTS = new Set(['127.0.0.1', 'localhost', '[::1]']);

export type BotFeedParams = { kind: 'none' } | { kind: 'rejected'; url: string } | { kind: 'feed'; url: string; players: number };

/** Wertet `?botfeed=…&players=n` aus. Erlaubt sind ws/wss auf Loopback oder dem Host der Seite, `players` 1–4 (sonst 1). */
export function parseBotFeed(search: string, pageHost: string): BotFeedParams {
  const p = new URLSearchParams(search);
  const url = p.get('botfeed');
  if (url === null) return { kind: 'none' };
  if (!allowedFeed(url, pageHost)) return { kind: 'rejected', url };
  const n = Number(p.get('players'));
  return { kind: 'feed', url, players: Number.isInteger(n) && n >= 1 && n <= MAX_BOT_PLAYERS ? n : 1 };
}

function allowedFeed(url: string, pageHost: string): boolean {
  let u: URL;
  try {
    u = new URL(url);
  } catch {
    return false;
  }
  if (u.protocol !== 'ws:' && u.protocol !== 'wss:') return false;
  return LOCAL_HOSTS.has(u.hostname) || (pageHost !== '' && u.hostname === pageHost);
}

/** Prüft eine Nachricht; ungültige Felder machen sie ungültig, `moveX` wird auf -1…1 begrenzt. */
export function parseCommand(data: unknown): FeedCommand | null {
  let m: unknown;
  try {
    m = typeof data === 'string' ? JSON.parse(data) : null;
  } catch {
    return null;
  }
  if (typeof m !== 'object' || m === null) return null;
  const c = m as Record<string, unknown>;
  const slot = c.slot;
  if (!Number.isInteger(slot) || (slot as number) < 0 || (slot as number) >= MAX_BOT_PLAYERS) return null;
  const moveX = typeof c.moveX === 'number' && Number.isFinite(c.moveX) ? Math.max(-1, Math.min(1, c.moveX)) : 0;
  const skill = Number.isInteger(c.skill) && (c.skill as number) >= 0 && (c.skill as number) <= SKILL_ACTIONS.length ? (c.skill as number) : 0;
  return { slot: slot as number, moveX, sprint: c.sprint === true, pay: c.pay === true, attack: c.attack === true, skill };
}

/** Das Nötige eines WebSockets (im Test ersetzbar) */
export interface FeedSocket {
  onopen: (() => void) | null;
  onmessage: ((e: { data: unknown }) => void) | null;
  onclose: (() => void) | null;
  close(): void;
}

export interface FeedDeps {
  open: (url: string) => FeedSocket;
  later: (fn: () => void, ms: number) => void;
}

const browserDeps: FeedDeps = {
  open: (url) => new WebSocket(url) as unknown as FeedSocket,
  later: (fn, ms) => void setTimeout(fn, ms),
};

/** Verbindung zum Bot-Feed: hält das letzte Kommando je Slot, verbindet nach einem Abbruch neu. */
export class BotFeed {
  private readonly last = new Map<number, FeedCommand>();
  private socket: FeedSocket | null = null;
  private stopped = false;

  constructor(
    readonly url: string,
    private readonly deps: FeedDeps = browserDeps,
  ) {
    this.connect();
  }

  /** Letztes Kommando des Slots, null ohne Verbindung oder bevor eines kam */
  command(slot: number): FeedCommand | null {
    return this.last.get(slot) ?? null;
  }

  /** Nimmt eine Nachricht an (sonst über den WebSocket) */
  receive(data: unknown): void {
    const cmd = parseCommand(data);
    if (cmd) this.last.set(cmd.slot, cmd);
  }

  stop(): void {
    this.stopped = true;
    this.socket?.close();
  }

  private connect(): void {
    const s = this.deps.open(this.url);
    this.socket = s;
    s.onopen = () => clientLog('info', `🔌 Bot-Feed verbunden: ${this.url}`);
    s.onmessage = (e) => this.receive(e.data);
    s.onclose = () => {
      if (this.socket !== s) return;
      this.last.clear(); // Feed weg: die Monarchen stehen
      this.socket = null;
      if (this.stopped) return;
      clientLog('warn', `👋 Bot-Feed getrennt, neuer Versuch in ${RETRY_MS} ms: ${this.url}`);
      this.deps.later(() => this.stopped || this.connect(), RETRY_MS);
    };
  }
}

const IDLE: FeedCommand = { slot: 0, moveX: 0, sprint: false, pay: false, attack: false, skill: 0 };

/** Eingabe eines lokalen Slots aus dem Bot-Feed; Bezahlen = `confirm`, Schlag = `attack`, Skill k = `skill<k>`. */
export class BotInput implements PlayerInput {
  readonly label: string;
  private previous = new Set<Action>();
  private current = new Set<Action>();
  private cmd: FeedCommand = IDLE;

  constructor(
    readonly slot: number,
    private readonly feed: Pick<BotFeed, 'command'>,
  ) {
    this.label = `Bot ${slot + 1}`;
  }

  update(): void {
    this.cmd = this.feed.command(this.slot) ?? IDLE;
    this.previous = this.current;
    this.current = held(this.cmd);
  }

  moveX(): number {
    return this.cmd.moveX;
  }

  sprint(): boolean {
    return this.cmd.sprint;
  }

  justPressed(action: Action): boolean {
    return this.current.has(action) && !this.previous.has(action);
  }

  held(action: Action): boolean {
    return this.current.has(action);
  }
}

function held(c: FeedCommand): Set<Action> {
  const s = new Set<Action>();
  if (c.pay) s.add('confirm');
  if (c.attack) s.add('attack');
  const skill = SKILL_ACTIONS[c.skill - 1];
  if (skill) s.add(skill);
  return s;
}

/**
 * Bot-Eingaben der Seite, eine je lokalem Spieler (Slot 0…n-1). Ohne `?botfeed` keine, die Eingabe bleibt wie sie ist;
 * ein fremder Host wird abgelehnt und mit 🚫 geloggt.
 */
export function botInputs(search: string, pageHost: string, deps: FeedDeps = browserDeps): BotInput[] {
  const p = parseBotFeed(search, pageHost);
  if (p.kind === 'none') return [];
  if (p.kind === 'rejected') {
    clientLog('warn', `🚫 Bot-Feed abgelehnt, nur Loopback oder ${pageHost || 'der Host der Seite'}: ${p.url}`);
    return [];
  }
  clientLog('info', `🚀 Bot-Eingabe für ${p.players} Spieler: ${p.url}`);
  const feed = new BotFeed(p.url, deps);
  return Array.from({ length: p.players }, (_, slot) => new BotInput(slot, feed));
}
