// Spielstände im Heimnetz: GET/POST /api/save?slot=autosave -> saves/<slot>.json
// Wird vom Heimnetz-Server (server.mjs) und vom Vite-Dev-Server (vite.config.ts) gemeinsam genutzt.
// Überschreibt ein Spielstand ein ANDERES Spiel (andere campaignId), wird das alte vorher als Sicherung umbenannt.
import { mkdir, readFile, rename, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

export const SAVES_DIR = process.env.K3C_SAVES_DIR ?? join(dirname(fileURLToPath(import.meta.url)), '..', 'saves');
const MAX_BYTES = 1024 * 1024;
const SLOT = /^[a-z0-9-]{1,32}$/;

const json = (res, status, body) => res.writeHead(status, { 'Content-Type': 'application/json; charset=utf-8', 'Cache-Control': 'no-store' }).end(JSON.stringify(body));

/**
 * Behandelt /api/save. Gibt true zurück, wenn der Request bearbeitet wurde.
 * @param {import('node:http').IncomingMessage} req
 * @param {import('node:http').ServerResponse} res
 */
export async function handleSave(req, res) {
  const url = new URL(req.url ?? '/', 'http://x');
  if (url.pathname !== '/api/save') return false;

  const slot = url.searchParams.get('slot') ?? 'autosave';
  if (!SLOT.test(slot)) return json(res, 400, { error: 'Ungültiger Slot' }), true;
  const file = join(SAVES_DIR, `${slot}.json`);

  if (req.method === 'GET') {
    const data = await readFile(file, 'utf8').catch(() => null);
    if (data === null) return json(res, 404, { error: 'Kein Spielstand' }), true;
    res.writeHead(200, { 'Content-Type': 'application/json; charset=utf-8', 'Cache-Control': 'no-store' }).end(data);
    return true;
  }

  if (req.method !== 'POST') return json(res, 405, { error: 'Nur GET/POST' }), true;

  let body = '';
  for await (const chunk of req) {
    body += chunk;
    if (body.length > MAX_BYTES) return json(res, 413, { error: 'Spielstand zu groß' }), true;
  }
  let save;
  try {
    save = JSON.parse(body);
  } catch {
    return json(res, 400, { error: 'Kein gültiges JSON' }), true;
  }
  if (!save || typeof save.campaignId !== 'string' || typeof save.version !== 'number') {
    return json(res, 400, { error: 'Kein Spielstand' }), true;
  }

  await mkdir(SAVES_DIR, { recursive: true });
  const previous = await readFile(file, 'utf8').then(JSON.parse).catch(() => null);
  let backup = null;
  if (previous && previous.campaignId !== save.campaignId) {
    backup = `${slot}-${String(previous.savedAt ?? Date.now()).replace(/[:.]/g, '-')}.json`;
    await rename(file, join(SAVES_DIR, backup));
    console.log(`[save] anderes Spiel, Sicherung: ${backup}`);
  }
  await writeFile(file, JSON.stringify(save));
  json(res, 200, { ok: true, slot, backup });
  return true;
}
