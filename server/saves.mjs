// Spielstand auf dem Heimnetz-Server: GET/PUT /api/save liest bzw. schreibt saves/k3c.json.
// Wird vom Heimnetz-Server (server.mjs) und vom Vite-Dev-Server (vite.config.ts) gemeinsam genutzt.
// Das Spiel fällt auf localStorage zurück, wenn der Server fehlt (z.B. GitHub Pages).
import { mkdir, readFile, rename, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

export const SAVES_DIR = join(dirname(fileURLToPath(import.meta.url)), '..', 'saves');
const MAX_BYTES = 1024 * 1024;
const SLOT = 'k3c.json';

/**
 * Behandelt GET/PUT /api/save. Gibt true zurück, wenn der Request bearbeitet wurde.
 * @param {import('node:http').IncomingMessage} req
 * @param {import('node:http').ServerResponse} res
 * @param {string} [dir] Ordner für die Spielstände (Tests)
 */
export async function handleSave(req, res, dir = SAVES_DIR) {
  if (new URL(req.url ?? '/', 'http://x').pathname !== '/api/save') return false;
  const file = join(dir, SLOT);
  const json = { 'Content-Type': 'application/json; charset=utf-8', 'Cache-Control': 'no-store' };

  if (req.method === 'GET') {
    const data = await readFile(file, 'utf8').catch(() => null);
    // 204 statt 404: noch kein Stand ist kein Fehler (sonst meldet die Browser-Konsole jeden Neustart rot).
    if (data === null) res.writeHead(204, json).end();
    else res.writeHead(200, json).end(data);
    return true;
  }

  if (req.method !== 'PUT' && req.method !== 'POST') {
    res.writeHead(405, { Allow: 'GET, PUT, POST' }).end();
    return true;
  }

  let body = '';
  for await (const chunk of req) {
    body += chunk;
    if (body.length > MAX_BYTES) {
      res.writeHead(413).end('Spielstand zu groß');
      return true;
    }
  }

  let save;
  try {
    save = JSON.parse(body);
  } catch {
    res.writeHead(400).end('Kein gültiges JSON');
    return true;
  }
  if (!save || typeof save !== 'object' || typeof save.version !== 'number') {
    res.writeHead(400).end('Kein Spielstand');
    return true;
  }

  // Erst in eine Temp-Datei, dann umbenennen: ein Abbruch mitten im Schreiben zerstört den alten Stand nicht.
  await mkdir(dir, { recursive: true });
  const tmp = `${file}.tmp`;
  await writeFile(tmp, JSON.stringify(save));
  await rename(tmp, file);
  res.writeHead(200, json).end(JSON.stringify({ ok: true }));
  return true;
}
