// Heimnetz-Server: liefert den Produktions-Build (dist/) aus und nimmt Testberichte an.
// Start: npm run serve   (baut vorher)   oder   npm start   (nur Server)
//
// HTTP  auf Port 8080 (K3C_HTTP_PORT)
// HTTPS auf Port 8443 (K3C_HTTPS_PORT), nur wenn certs/key.pem + certs/cert.pem existieren.
//   Nötig, falls Edge auf der Xbox die Gamepad API nur in "secure contexts" erlaubt. Anleitung: README.md
import { createServer as createHttp } from 'node:http';
import { createServer as createHttps } from 'node:https';
import { existsSync, readFileSync } from 'node:fs';
import { readFile, stat } from 'node:fs/promises';
import { networkInterfaces } from 'node:os';
import { dirname, extname, join, normalize, sep } from 'node:path';
import { fileURLToPath } from 'node:url';
import { handleReport } from './reports.mjs';

const ROOT = join(dirname(fileURLToPath(import.meta.url)), '..');
const DIST = join(ROOT, 'dist');
const CERTS = join(ROOT, 'certs');
const HTTP_PORT = Number(process.env.K3C_HTTP_PORT ?? 8080);
const HTTPS_PORT = Number(process.env.K3C_HTTPS_PORT ?? 8443);

const MIME = {
  '.html': 'text/html; charset=utf-8',
  '.js': 'text/javascript; charset=utf-8',
  '.css': 'text/css; charset=utf-8',
  '.json': 'application/json; charset=utf-8',
  '.png': 'image/png',
  '.jpg': 'image/jpeg',
  '.svg': 'image/svg+xml',
  '.ogg': 'audio/ogg',
  '.mp3': 'audio/mpeg',
  '.wav': 'audio/wav',
  '.woff2': 'font/woff2',
};

if (!existsSync(join(DIST, 'index.html'))) {
  console.error('dist/ fehlt. Erst bauen: npm run build   (oder direkt: npm run serve)');
  process.exit(1);
}

/** @type {import('node:http').RequestListener} */
async function handle(req, res) {
  try {
    if (await handleReport(req, res)) return;
    if (req.method !== 'GET' && req.method !== 'HEAD') return void res.writeHead(405).end();

    const path = decodeURIComponent(new URL(req.url ?? '/', 'http://x').pathname);
    let file = normalize(join(DIST, path));
    if (file !== DIST && !file.startsWith(DIST + sep)) return void res.writeHead(403).end();
    if ((await stat(file).catch(() => null))?.isDirectory()) file = join(file, 'index.html');

    const data = await readFile(file).catch(() => null);
    if (!data) return void res.writeHead(404).end('Nicht gefunden');
    res.writeHead(200, {
      'Content-Type': MIME[extname(file)] ?? 'application/octet-stream',
      // HTML nie cachen (neue Builds sofort sichtbar), gehashte Assets dauerhaft.
      'Cache-Control': file.endsWith('.html') ? 'no-cache' : 'public, max-age=31536000, immutable',
    });
    res.end(req.method === 'HEAD' ? undefined : data);
  } catch (err) {
    console.error(err);
    if (!res.headersSent) res.writeHead(500).end();
  }
}

const lanAddresses = Object.values(networkInterfaces())
  .flat()
  .filter((a) => a && a.family === 'IPv4' && !a.internal)
  .map((a) => a.address);

createHttp(handle).listen(HTTP_PORT, () => {
  console.log(`K3C läuft (HTTP):`);
  for (const ip of ['localhost', ...lanAddresses]) console.log(`  http://${ip}:${HTTP_PORT}/   Test: http://${ip}:${HTTP_PORT}/gamepad-test.html`);
});

const key = join(CERTS, 'key.pem');
const cert = join(CERTS, 'cert.pem');
if (existsSync(key) && existsSync(cert)) {
  createHttps({ key: readFileSync(key), cert: readFileSync(cert) }, handle).listen(HTTPS_PORT, () => {
    console.log(`K3C läuft (HTTPS):`);
    for (const ip of lanAddresses) console.log(`  https://${ip}:${HTTPS_PORT}/`);
  });
} else {
  console.log(`(Kein HTTPS: certs/key.pem + certs/cert.pem fehlen. Nur nötig, falls die Xbox HTTPS verlangt.)`);
}
