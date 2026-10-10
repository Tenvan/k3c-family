import { describe, expect, it } from 'vitest';
import { listSaves } from './saveStore';

const respond = (body: unknown, ok = true) => (() => Promise.resolve({ ok, json: () => Promise.resolve(body) } as Response)) as typeof fetch;

describe('listSaves (LB1.1, B-037/AC-04)', () => {
  it('liefert geprüfte Einträge und lässt unlesbare weg', async () => {
    const saves = await listSaves(
      respond([
        { name: 'alt', savedAt: '2026-10-01T10:00:00Z', version: 3, depths: [0] },
        { name: 'familie', savedAt: '2026-10-04T22:00:00Z', version: 4, day: 2, phase: 'night', depths: [0, 1] },
        { name: 'kaputt', error: 'ungültig' },
        { name: 'halb', savedAt: 5, depths: [0] },
      ]),
    );
    expect(saves).toEqual([
      { name: 'alt', savedAt: '2026-10-01T10:00:00Z', depths: [0] },
      { name: 'familie', savedAt: '2026-10-04T22:00:00Z', day: 2, depths: [0, 1] },
    ]);
  });

  it('Fehler, Server-Fehler und falsches Format ergeben eine leere Liste', async () => {
    expect(await listSaves((() => Promise.reject(new Error('weg'))) as typeof fetch)).toEqual([]);
    expect(await listSaves(respond(null, false))).toEqual([]);
    expect(await listSaves(respond({ name: 'x' }))).toEqual([]);
  });
});
