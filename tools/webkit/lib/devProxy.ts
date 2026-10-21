/// <reference types="node" />
// Läuft nur in vite.config.ts (Node), nie im Browser-Bundle; die Referenz bringt die Node-Typen für den Typecheck der
// Frontends (bewusst in Kauf genommen: `process`/`Buffer` in src/ fallen dort nicht mehr als Fehler auf).
import type { IncomingMessage, ServerResponse } from 'node:http';
import { connect } from 'node:net';

// Vite-Plugin der Client/Server-Entwicklung (`task <app>:dev`): Das Backend startet langsamer als Vite und ist bei jedem
// Neubau kurz weg (engine/devrun). Statt `/api` sofort scheitern zu lassen (Proxy-Fehler mit Stacktrace, rote Oberfläche),
// hält dieses Plugin jede `/api`-Anfrage zurück, bis der Port des Backends Verbindungen annimmt; erst nach `waitMs`
// antwortet es mit 503 und einem kurzen Text. Schließt der Client vorher (Reload, EventSource.close), passiert nichts
// mehr. Alle Anfragen teilen eine Prüfung je Backend, ein offener Port gilt `okMs` lang ohne neue Prüfung.

/** Optionen von waitForBackend; Standard: 30 s warten, alle 250 ms prüfen, offen gilt 1 s, Präfix `/api`. */
export interface WaitOptions {
  waitMs?: number;
  everyMs?: number;
  okMs?: number;
  prefix?: string;
}

/** portOpen meldet, ob host:port gerade eine TCP-Verbindung annimmt. */
export function portOpen(host: string, port: number, timeoutMs = 500): Promise<boolean> {
  return new Promise((resolve) => {
    const socket = connect({ host, port });
    const done = (ok: boolean) => {
      socket.destroy();
      resolve(ok);
    };
    socket.setTimeout(timeoutMs, () => done(false));
    socket.once('connect', () => done(true));
    socket.once('error', () => done(false));
  });
}

/** Der Teil des Vite-Dev-Servers, den das Plugin braucht. Strukturell statt `import type { Plugin } from 'vite'`:
 *  tools/webkit hat kein eigenes node_modules, der Vite-Typ der Wurzel passt nicht zu dem jedes Frontends. */
export interface DevServerLike {
  middlewares: { use(fn: (req: IncomingMessage, res: ServerResponse, next: () => void) => void): unknown };
}

/** Das Plugin als Struktur, die Vites `Plugin` erfüllt. */
export interface WaitPlugin {
  name: string;
  apply: 'serve';
  configureServer(server: DevServerLike): void;
}

/** backendProbe teilt die Prüfung eines Backends: höchstens eine Verbindung gleichzeitig, offen gilt okMs lang. */
function backendProbe(host: string, port: number, okMs: number) {
  let openUntil = 0;
  let running: Promise<boolean> | null = null;
  return (): Promise<boolean> => {
    if (Date.now() < openUntil) return Promise.resolve(true);
    running ??= portOpen(host, port).then((ok) => {
      running = null;
      if (ok) openUntil = Date.now() + okMs;
      return ok;
    });
    return running;
  };
}

/** waitForBackend hält `/api`-Anfragen zurück, bis das Backend unter target (z. B. http://127.0.0.1:8192) bereit ist. */
export function waitForBackend(target: string, options: WaitOptions = {}): WaitPlugin {
  const { waitMs = 30_000, everyMs = 250, okMs = 1_000, prefix = '/api' } = options;
  const url = new URL(target);
  const host = url.hostname.replace(/^\[(.*)\]$/, '$1'); // [::1] → ::1
  const probe = backendProbe(host, Number(url.port || (url.protocol === 'https:' ? 443 : 80)), okMs);
  const matches = (path = '') => path === prefix || path.startsWith(`${prefix}/`) || path.startsWith(`${prefix}?`);
  return {
    name: 'k3c-wait-for-backend',
    apply: 'serve',
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        if (!matches(req.url)) return next();
        let gone = false;
        res.once('close', () => (gone = true));
        const until = Date.now() + waitMs;
        const attempt = async (): Promise<void> => {
          if (gone) return;
          if (await probe()) return gone ? undefined : next();
          if (gone) return;
          if (Date.now() >= until) {
            res.statusCode = 503;
            res.setHeader('Content-Type', 'application/json; charset=utf-8');
            res.end(JSON.stringify({ error: `Backend ${url.host} antwortet nicht (läuft task <app>:dev:server?)` }));
            return;
          }
          setTimeout(() => void attempt(), everyMs);
        };
        void attempt();
      });
    },
  };
}
