import { describe, expect, it } from 'vitest';
import { deleteSave, listSaves, listWorlds } from './saveStore';

const respond = (body: unknown, ok = true) => (() => Promise.resolve({ ok, json: () => Promise.resolve(body) } as Response)) as typeof fetch;

describe('listSaves (LB1.1, B-037/AC-04)', () => {
  it('liefert geprüfte Einträge und lässt unlesbare weg', async () => {
    const saves = await listSaves(
      respond([
        { name: 'alt', savedAt: '2026-10-01T10:00:00Z', version: 3, depths: [0] },
        { name: 'familie', savedAt: '2026-10-04T22:00:00Z', version: 4, day: 2, phase: 'night', depths: [0, 1] },
        { name: 'nord', savedAt: '2026-10-22T10:00:00Z', version: 8, world: 'nordmeer', depths: [0] },
        { name: 'kaputt', error: 'ungültig' },
        { name: 'halb', savedAt: 5, depths: [0] },
      ]),
    );
    expect(saves).toEqual([
      { name: 'alt', savedAt: '2026-10-01T10:00:00Z', depths: [0] },
      { name: 'familie', savedAt: '2026-10-04T22:00:00Z', day: 2, depths: [0, 1] },
      { name: 'nord', savedAt: '2026-10-22T10:00:00Z', world: 'nordmeer', depths: [0] },
    ]);
  });

  it('Fehler, Server-Fehler und falsches Format ergeben eine leere Liste', async () => {
    expect(await listSaves((() => Promise.reject(new Error('weg'))) as typeof fetch)).toEqual([]);
    expect(await listSaves(respond(null, false))).toEqual([]);
    expect(await listSaves(respond({ name: 'x' }))).toEqual([]);
  });
});

describe('deleteSave (BD5.2, B-506/AC-02)', () => {
  const status = (code: number) => {
    const calls: [string, RequestInit | undefined][] = [];
    const fn = ((url: string, init?: RequestInit) => {
      calls.push([url, init]);
      return Promise.resolve({ ok: code === 200, status: code } as Response);
    }) as typeof fetch;
    return { fn, calls };
  };

  it('schickt DELETE für genau diesen Namen', async () => {
    const { fn, calls } = status(200);
    expect(await deleteSave('nord', fn)).toBe('ok');
    expect(calls.map(([url, init]) => [url, init?.method])).toEqual([['api/save?slot=nord', 'DELETE']]);
  });

  it('jede Antwort ergibt ein Ergebnis, nichts bleibt still', async () => {
    expect(await deleteSave('nord', status(409).fn)).toBe('in_use');
    expect(await deleteSave('nord', status(404).fn)).toBe('not_found');
    expect(await deleteSave('nord', status(500).fn)).toBe('failed');
    expect(await deleteSave('nord', (() => Promise.reject(new Error('weg'))) as typeof fetch)).toBe('failed');
  });
});

describe('listWorlds (BD8.1, B-566/AC-01)', () => {
  it('liefert die IDs des Servers ohne Doppelte und unlesbare Einträge', async () => {
    expect(await listWorlds(respond([{ id: 'standard', name: 'Standard' }, { id: 'nordmeer', name: 'Nordmeer' }, { id: 'nordmeer' }, { name: 'x' }, 7]))).toEqual(['standard', 'nordmeer']);
  });

  it('Fehler, Server-Fehler, leere Liste und falsches Format ergeben nur standard', async () => {
    expect(await listWorlds((() => Promise.reject(new Error('weg'))) as typeof fetch)).toEqual(['standard']);
    expect(await listWorlds(respond([], false))).toEqual(['standard']);
    expect(await listWorlds(respond([]))).toEqual(['standard']);
    expect(await listWorlds(respond({ id: 'x' }))).toEqual(['standard']);
  });
});
