import type { Server as HttpServer } from 'node:http';
import type { Server as HttpsServer } from 'node:https';
import { WebSocketServer } from 'ws';
import { ONLINE_PATH, TICK_HZ, sanitizeInput, sanitizeRoom, type ClientMessage, type ServerMessage } from './protocol';
import { Rooms, type Room } from './room';

/** Räume und Takt gelten für den ganzen Prozess, auch wenn HTTP und HTTPS beide angehängt werden. */
const rooms = new Rooms();
let ticking = false;

/** Hängt den Online-Modus (WebSocket unter /ws) an einen bestehenden HTTP(S)-Server. Kein Phaser, läuft in Node. */
export function attachOnline(server: HttpServer | HttpsServer): void {
  const wss = new WebSocketServer({ noServer: true, maxPayload: 4096 });

  server.on('upgrade', (req, socket, head) => {
    if (new URL(req.url ?? '/', 'http://x').pathname !== ONLINE_PATH) return; // z.B. Vite-HMR bleibt unberührt
    wss.handleUpgrade(req, socket, head, (ws) => wss.emit('connection', ws));
  });

  wss.on('connection', (ws) => {
    let joined: { room: Room; clientId: string } | null = null;
    const conn = {
      send: (m: string) => {
        if (ws.readyState === ws.OPEN) ws.send(m);
      },
    };
    const fail = (message: string) => {
      ws.send(JSON.stringify({ t: 'error', message } satisfies ServerMessage));
      ws.close();
    };

    ws.on('message', (data) => {
      let msg: ClientMessage;
      try {
        msg = JSON.parse(String(data)) as ClientMessage;
      } catch {
        return;
      }
      if (msg.t === 'join' && !joined) {
        const code = sanitizeRoom(msg.room);
        if (!code || typeof msg.clientId !== 'string' || msg.clientId.length > 64) return fail('Ungültiger Raum');
        const depth = Math.max(0, Math.min(2, Number(msg.depth) || 0));
        const room = rooms.get(code, { depth, fast: msg.fast === true, seed: sanitizeRoom(msg.seed) ?? undefined });
        if (room.join(msg.clientId, conn) === null) return fail('Raum ist voll');
        joined = { room, clientId: msg.clientId };
      } else if (msg.t === 'input' && joined) {
        joined.room.input(joined.clientId, sanitizeInput(msg));
      }
    });
    ws.on('close', () => joined?.room.leave(joined.clientId, conn));
    ws.on('error', () => ws.terminate());
  });

  if (ticking) return;
  ticking = true;
  let last = Date.now();
  setInterval(() => {
    const now = Date.now();
    rooms.tick(Math.min((now - last) / 1000, 0.1));
    last = now;
  }, 1000 / TICK_HZ).unref();
}
