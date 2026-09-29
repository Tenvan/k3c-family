// Server-Handler für den Spielstand (server/saves.mjs) mit Fake-Requests.
import { mkdtempSync, readFileSync, rmSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { Readable } from 'node:stream';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { handleSave } from '../server/saves.mjs';

interface FakeResponse {
  status: number;
  body: string;
  writeHead(status: number): FakeResponse;
  end(body?: string): FakeResponse;
}

function request(method: string, url: string, body = '') {
  return Object.assign(Readable.from(body ? [body] : []), { method, url });
}

function response(): FakeResponse {
  return {
    status: 0,
    body: '',
    writeHead(status) {
      this.status = status;
      return this;
    },
    end(body = '') {
      this.body = body;
      return this;
    },
  };
}

async function call(method: string, body = '', url = '/api/save') {
  const res = response();
  const handled: boolean = await handleSave(request(method, url, body), res, dir);
  return { handled, ...res };
}

let dir: string;
beforeEach(() => (dir = mkdtempSync(join(tmpdir(), 'k3c-saves-'))));
afterEach(() => rmSync(dir, { recursive: true, force: true }));

describe('/api/save', () => {
  it('ignoriert andere Pfade', async () => {
    expect((await call('GET', '', '/api/report')).handled).toBe(false);
  });

  it('meldet 204, solange nichts gespeichert ist', async () => {
    expect(await call('GET')).toMatchObject({ handled: true, status: 204, body: '' });
  });

  it('speichert und liefert den Stand wieder aus', async () => {
    const save = { version: 1, seed: 'k3c', time: 42 };
    expect(await call('PUT', JSON.stringify(save))).toMatchObject({ status: 200 });
    expect(JSON.parse(readFileSync(join(dir, 'k3c.json'), 'utf8'))).toEqual(save);
    const got = await call('GET', '', '/api/save?t=1');
    expect(got.status).toBe(200);
    expect(JSON.parse(got.body)).toEqual(save);
  });

  it('lehnt kaputte Daten ab und behält den alten Stand', async () => {
    await call('PUT', JSON.stringify({ version: 1, seed: 'alt' }));
    expect((await call('PUT', 'kaputt')).status).toBe(400);
    expect((await call('PUT', '{"foo":1}')).status).toBe(400);
    expect(JSON.parse((await call('GET')).body).seed).toBe('alt');
  });

  it('lehnt andere Methoden ab', async () => {
    expect((await call('DELETE')).status).toBe(405);
  });
});
