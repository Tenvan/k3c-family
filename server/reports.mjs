// Nimmt Testberichte der Gamepad-Testseite entgegen und speichert sie als JSON in reports/.
// Wird vom Heimnetz-Server (server.mjs) und vom Vite-Dev-Server (vite.config.ts) gemeinsam genutzt.
import { mkdir, writeFile } from 'node:fs/promises';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

export const REPORTS_DIR = join(dirname(fileURLToPath(import.meta.url)), '..', 'reports');
const MAX_BYTES = 256 * 1024;

/**
 * Behandelt POST /api/report. Gibt true zurück, wenn der Request bearbeitet wurde.
 * @param {import('node:http').IncomingMessage} req
 * @param {import('node:http').ServerResponse} res
 */
export async function handleReport(req, res) {
  if (req.method !== 'POST' || req.url !== '/api/report') return false;

  let body = '';
  for await (const chunk of req) {
    body += chunk;
    if (body.length > MAX_BYTES) {
      res.writeHead(413).end('Bericht zu groß');
      return true;
    }
  }

  let report;
  try {
    report = JSON.parse(body);
  } catch {
    res.writeHead(400).end('Kein gültiges JSON');
    return true;
  }

  report.receivedAt = new Date().toISOString();
  report.remoteAddress = req.socket.remoteAddress;
  await mkdir(REPORTS_DIR, { recursive: true });
  const file = join(REPORTS_DIR, `gamepad-${report.receivedAt.replace(/[:.]/g, '-')}.json`);
  await writeFile(file, JSON.stringify(report, null, 2));
  console.log(`[report] gespeichert: ${file}`);

  res.writeHead(200, { 'Content-Type': 'application/json' }).end(JSON.stringify({ ok: true, file }));
  return true;
}
