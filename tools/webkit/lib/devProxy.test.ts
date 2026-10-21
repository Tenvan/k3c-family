/// <reference types="node" />
import { createServer, type Server } from 'node:net';
import { afterEach, describe, expect, it } from 'vitest';
import { EventEmitter } from 'node:events';
import { portOpen, waitForBackend, type DevServerLike } from './devProxy';

let servers: Server[] = [];
afterEach(() => {
  servers.forEach((s) => s.close());
  servers = [];
});

function listen(): Promise<number> {
  return new Promise((resolve) => {
    const s = createServer((c) => c.end());
    servers.push(s);
    s.listen(0, '127.0.0.1', () => resolve((s.address() as { port: number }).port));
  });
}

async function freePort(): Promise<number> {
  const port = await listen();
  servers.pop()!.close();
  return port;
}

/** Ruft die Middleware des Plugins für url auf und liefert, ob next kam bzw. welcher Status gesetzt wurde. */
function handlerFor(target: string, waitMs: number) {
  let handler: Parameters<DevServerLike['middlewares']['use']>[0] = () => {};
  waitForBackend(target, { waitMs, everyMs: 20 }).configureServer({ middlewares: { use: (fn) => (handler = fn) } });
  return handler;
}

/** fakeRes ist eine Antwort mit close-Ereignis; end meldet den Status. */
function fakeRes(onEnd: (status: number) => void) {
  const res = Object.assign(new EventEmitter(), { statusCode: 200, setHeader() {}, end() { onEnd(res.statusCode); } });
  return res;
}

function call(target: string, url: string, waitMs: number): Promise<{ next: boolean; status?: number }> {
  const handler = handlerFor(target, waitMs);
  return new Promise((resolve) => {
    handler({ url } as never, fakeRes((status) => resolve({ next: false, status })) as never, () => resolve({ next: true }));
  });
}

describe('devProxy', () => {
  it('portOpen erkennt einen offenen und einen geschlossenen Port', async () => {
    expect(await portOpen('127.0.0.1', await listen())).toBe(true);
    expect(await portOpen('127.0.0.1', await freePort())).toBe(false);
  });

  it('lässt /api durch, sobald das Backend lauscht', async () => {
    expect(await call(`http://127.0.0.1:${await listen()}`, '/api/x', 1000)).toEqual({ next: true });
  });

  it('wartet auf ein später startendes Backend', async () => {
    const port = await freePort();
    setTimeout(() => {
      const s = createServer((c) => c.end());
      servers.push(s);
      s.listen(port, '127.0.0.1');
    }, 150);
    expect(await call(`http://127.0.0.1:${port}`, '/api/x', 3000)).toEqual({ next: true });
  });

  it('antwortet nach der Frist mit 503', async () => {
    expect(await call(`http://127.0.0.1:${await freePort()}`, '/api/x', 100)).toEqual({ next: false, status: 503 });
  });

  it('andere Pfade gehen sofort weiter', async () => {
    expect(await call(`http://127.0.0.1:${await freePort()}`, '/index.html', 100)).toEqual({ next: true });
  });

  it('ein geschlossener Client bekommt weder next noch 503', async () => {
    const handler = handlerFor(`http://127.0.0.1:${await freePort()}`, 150);
    let called = false;
    const res = fakeRes(() => (called = true));
    handler({ url: '/api/events' } as never, res as never, () => (called = true));
    res.emit('close');
    await new Promise((r) => setTimeout(r, 400));
    expect(called).toBe(false);
  });

  it('/apix ist kein /api', async () => {
    expect(await call(`http://127.0.0.1:${await freePort()}`, '/apix', 100)).toEqual({ next: true });
  });
});
