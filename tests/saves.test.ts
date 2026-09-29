// Heimnetz-Server: /api/save (server/saves.mjs) mit echtem HTTP-Server auf einem freien Port.
import { mkdtempSync, readdirSync, rmSync } from 'node:fs';
import { createServer, type Server } from 'node:http';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { afterAll, beforeAll, describe, expect, it } from 'vitest';

const dir = mkdtempSync(join(tmpdir(), 'k3c-saves-'));
let server: Server;
let base = '';

beforeAll(async () => {
  process.env.K3C_SAVES_DIR = dir;
  // @ts-expect-error reines JS-Modul ohne Typen
  const { handleSave } = await import('../server/saves.mjs');
  server = createServer((req, res) => {
    handleSave(req, res).then((handled: boolean) => handled || res.writeHead(404).end());
  });
  await new Promise<void>((resolve) => server.listen(0, resolve));
  const address = server.address();
  base = `http://localhost:${typeof address === 'object' && address ? address.port : 0}`;
});

afterAll(() => {
  server.close();
  rmSync(dir, { recursive: true, force: true });
});

const save = (campaignId: string, savedAt: string) => ({ version: 1, campaignId, savedAt, seed: 'k3c' });
const post = (body: unknown, slot = 'autosave') =>
  fetch(`${base}/api/save?slot=${slot}`, { method: 'POST', body: typeof body === 'string' ? body : JSON.stringify(body) });

describe('/api/save', () => {
  it('liefert 404, solange es keinen Spielstand gibt', async () => {
    expect((await fetch(`${base}/api/save`)).status).toBe(404);
  });

  it('speichert und lädt einen Spielstand', async () => {
    expect((await post(save('a', '2026-01-01T00:00:00Z'))).status).toBe(200);
    const res = await fetch(`${base}/api/save?slot=autosave`);
    expect(res.status).toBe(200);
    expect(await res.json()).toMatchObject({ campaignId: 'a' });
  });

  it('sichert ein anderes Spiel, bevor es überschrieben wird', async () => {
    await post(save('a', '2026-01-02T00:00:00Z')); // gleiches Spiel: keine Sicherung
    expect(readdirSync(dir)).toEqual(['autosave.json']);
    const res = await post(save('b', '2026-01-03T00:00:00Z'));
    expect((await res.json()).backup).toMatch(/^autosave-2026-01-02/);
    expect(readdirSync(dir).sort()).toHaveLength(2);
    expect(await (await fetch(`${base}/api/save`)).json()).toMatchObject({ campaignId: 'b' });
  });

  it('lehnt Unsinn ab', async () => {
    expect((await post('kaputt')).status).toBe(400);
    expect((await post({ hallo: 1 })).status).toBe(400);
    expect((await post(save('a', 'x'), '../etc')).status).toBe(400);
    expect((await fetch(`${base}/api/save`, { method: 'DELETE' })).status).toBe(405);
  });
});
