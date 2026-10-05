import { describe, expect, it } from 'vitest';
import { fetchMetrics, metricsUrl, nextDelay, type FetchLike } from './monitorApi';

const answer = (status: number, body: unknown = {}): FetchLike => async () => ({
  ok: status >= 200 && status < 300,
  status,
  json: async () => body,
});

describe('fetchMetrics', () => {
  it('schickt das Token als Bearer und since in der URL', async () => {
    let seen: { url: string; auth: string } | null = null;
    const f: FetchLike = async (url, init) => {
      seen = { url, auth: init.headers.Authorization };
      return { ok: true, status: 200, json: async () => ({ startedAt: 1, now: 2 }) };
    };
    const res = await fetchMetrics('geheim', 1234, f);
    expect(seen).toEqual({ url: '/api/metrics?since=1234', auth: 'Bearer geheim' });
    expect(res).toEqual({ kind: 'ok', data: { startedAt: 1, now: 2 } });
    expect(metricsUrl(0)).toBe('/api/metrics');
  });

  it('unterscheidet Diagnose aus, Token falsch und nicht erreichbar', async () => {
    expect(await fetchMetrics('x', 0, answer(404))).toEqual({ kind: 'off' });
    expect(await fetchMetrics('x', 0, answer(401))).toEqual({ kind: 'unauthorized' });
    expect(await fetchMetrics('x', 0, answer(500))).toEqual({ kind: 'offline' });
    const down: FetchLike = () => Promise.reject(new Error('weg'));
    expect(await fetchMetrics('x', 0, down)).toEqual({ kind: 'offline' });
  });
});

describe('nextDelay', () => {
  it('3 s, nach Fehlern verdoppelt bis 30 s', () => {
    expect([0, 1, 2, 3, 4, 10].map(nextDelay)).toEqual([3000, 6000, 12000, 24000, 30000, 30000]);
  });
});
