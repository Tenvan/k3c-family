import { afterEach, describe, expect, it, vi } from 'vitest';
import { httpBackend } from './http';
import type { ServerState } from './types';

const state: ServerState = { running: true, starting: false, port: '8098', urls: ['http://localhost:8098/'], message: 'Server läuft.', log: ['a'] };

function fakeFetch(res: Partial<Response> & { json?: () => Promise<unknown> }) {
  const fn = vi.fn().mockResolvedValue({ ok: true, status: 200, json: () => Promise.resolve(state), ...res });
  vi.stubGlobal('fetch', fn);
  return fn;
}

afterEach(() => vi.unstubAllGlobals());

describe('HTTP-Backend des Starters', () => {
  it('liest den Zustand mit der Pflichtkopfzeile', async () => {
    const fetch = fakeFetch({});
    const b = httpBackend();
    expect(b.mock).toBe(false);
    expect(await b.state()).toEqual(state);
    expect(fetch).toHaveBeenCalledWith('/api/starter/state', { method: 'GET', headers: { 'X-K3C-Starter': '1' } });
  });

  it('Start und Stop sind POST-Aufrufe', async () => {
    const fetch = fakeFetch({});
    const b = httpBackend();
    await b.start();
    await b.stop();
    expect(fetch).toHaveBeenNthCalledWith(1, '/api/starter/start', { method: 'POST', headers: { 'X-K3C-Starter': '1' } });
    expect(fetch).toHaveBeenNthCalledWith(2, '/api/starter/stop', { method: 'POST', headers: { 'X-K3C-Starter': '1' } });
  });

  it('meldet einen Fehlerstatus mit Hinweis auf den Dev-Task', async () => {
    fakeFetch({ ok: false, status: 502 });
    await expect(httpBackend().state()).rejects.toThrow(/HTTP 502.*k3c-starter:dev/);
  });

  it('reicht Netzwerkfehler weiter', async () => {
    vi.stubGlobal('fetch', vi.fn().mockRejectedValue(new TypeError('failed to fetch')));
    await expect(httpBackend().start()).rejects.toThrow('failed to fetch');
  });
});
