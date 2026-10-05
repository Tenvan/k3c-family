import { describe, expect, it } from 'vitest';
import { DM_ACTIONS, devUrl, fetchDiagnose, fetchRooms, sendAction, type FetchLike } from './dmApi';

/** Antwortet mit status und body und merkt sich die Anfragen. */
function fake(status: number, body: unknown) {
  const calls: { url: string; init?: { method: string; body: string } }[] = [];
  const f: FetchLike = async (url, init) => {
    calls.push({ url, init });
    return { ok: status < 300, status, json: async () => body };
  };
  return { f, calls };
}

const offline: FetchLike = async () => {
  throw new Error('offline');
};

describe('dmApi (B-232)', () => {
  it('kodiert den Raum-Code in der URL', () => {
    expect(devUrl()).toBe('/api/dev');
    expect(devUrl('A B&')).toBe('/api/dev?room=A%20B%26');
  });

  it('liest Raumliste und Diagnose, ohne Server null', async () => {
    expect(await fetchRooms(fake(200, { dev: true, rooms: [] }).f)).toEqual({ dev: true, rooms: [] });
    expect(await fetchRooms(offline)).toBeNull();
    expect(await fetchDiagnose('KRNZ', fake(404, { error: 'x' }).f)).toBe('gone');
    expect(await fetchDiagnose('KRNZ', offline)).toBeNull();
  });

  // AC-03, AC-05: jede Aktion geht als POST mit JSON an den Raum.
  it('schickt Aktionen als POST', async () => {
    const { f, calls } = fake(200, { ok: true });
    for (const a of DM_ACTIONS) expect(await sendAction('KRNZ', { ...a.action, slot: 0 }, f)).toBeNull();
    const sent = calls.map((c) => JSON.parse(c.init!.body).action as string);
    expect(new Set(sent)).toEqual(new Set(['gold', 'material', 'timescale', 'pause', 'wave', 'phase']));
    expect(calls.every((c) => c.init!.method === 'POST' && c.url === '/api/dev?room=KRNZ')).toBe(true);
  });

  it('meldet Fehler des Servers', async () => {
    expect(await sendAction('KRNZ', { action: 'pause' }, fake(403, { error: 'Nur im Dev-Mode erlaubt' }).f)).toBe(
      'Nur im Dev-Mode erlaubt',
    );
    expect(await sendAction('KRNZ', { action: 'pause' }, offline)).toBe('Server nicht erreichbar');
  });
});
