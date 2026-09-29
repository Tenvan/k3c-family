import { ONLINE_PATH, type ClientMessage, type ServerMessage, type Snapshot } from './protocol';
import type { PlayerCommand } from '../world/sim/types';

export interface OnlineOptions {
  depth: number;
  fast: boolean;
  seed?: string;
}

/** Verbindung zum Spiel-Server: Eingabe senden, Weltzustand empfangen. */
export class OnlineClient {
  private latest: Snapshot | null = null;
  private lastSent = '';
  private lastSentAt = 0;
  /** Verbindung weg (Server beendet, WLAN weg) */
  closed = false;

  private constructor(
    private readonly ws: WebSocket,
    readonly you: number,
    readonly room: string,
    readonly seed: string,
    readonly depth: number,
    readonly fast: boolean,
  ) {}

  static connect(room: string, options: OnlineOptions): Promise<OnlineClient> {
    return new Promise((resolve, reject) => {
      const scheme = location.protocol === 'https:' ? 'wss' : 'ws';
      const ws = new WebSocket(`${scheme}://${location.host}${ONLINE_PATH}`);
      let client: OnlineClient | null = null;
      const fail = (err: Error) => {
        clearTimeout(timer);
        ws.close();
        reject(err);
      };
      const timer = setTimeout(() => fail(new Error('Keine Antwort vom Server')), 8000);
      ws.onopen = () => ws.send(JSON.stringify({ t: 'join', room, clientId: clientId(), ...options } satisfies ClientMessage));
      ws.onerror = () => fail(new Error('Server nicht erreichbar (läuft er mit npm run dev / npm run serve?)'));
      ws.onmessage = (e) => {
        const msg = JSON.parse(String(e.data)) as ServerMessage;
        if (msg.t === 'welcome') {
          clearTimeout(timer);
          client = new OnlineClient(ws, msg.you, msg.room, msg.seed, msg.depth, msg.fast);
          ws.onerror = null;
          resolve(client);
        } else if (msg.t === 'state') {
          if (client) client.latest = msg.s;
        } else if (msg.t === 'error') {
          fail(new Error(msg.message));
        }
      };
      ws.onclose = () => {
        if (client) client.closed = true;
        else fail(new Error('Verbindung getrennt'));
      };
    });
  }

  /** Neuester Zustand seit dem letzten Aufruf (oder null). */
  takeState(): Snapshot | null {
    const s = this.latest;
    this.latest = null;
    return s;
  }

  /** Nur bei Änderung senden, dazu alle 250 ms als Lebenszeichen. */
  sendInput(cmd: PlayerCommand): void {
    if (this.ws.readyState !== WebSocket.OPEN) return;
    const text = JSON.stringify({ t: 'input', ...cmd } satisfies ClientMessage);
    const now = performance.now();
    if (text === this.lastSent && now - this.lastSentAt < 250) return;
    this.lastSent = text;
    this.lastSentAt = now;
    this.ws.send(text);
  }
}

/** Stabile Kennung pro Browser, damit man nach einem Verbindungsabbruch seinen Monarchen wiederbekommt. */
function clientId(): string {
  try {
    let id = localStorage.getItem('k3c-client');
    if (!id) localStorage.setItem('k3c-client', (id = crypto.randomUUID()));
    return id;
  } catch {
    return crypto.randomUUID();
  }
}
