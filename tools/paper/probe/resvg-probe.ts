// resvg-Probe (PV1.3, AC-05): rendert die Quell-SVGs aus fixtures/svg/ mit @resvg/resvg-js auf 192 px Höhe und prüft,
// dass zwei Läufe byte-gleich sind. Mit --write schreibt sie fixtures/render/<asset>/<name>.png, hashes.json und die
// Fixture-Manifeste. Aufruf: bun tools/paper/probe/resvg-probe.ts [--write]
import { createHash } from 'node:crypto';
import { mkdirSync, readdirSync, readFileSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';
import { Resvg } from '@resvg/resvg-js';
import { renderKey, RENDERER_VERSION, specHash, stilHash } from '../schema/hash';

const PAPER = join(import.meta.dirname, '..');
export const SVG_DIR = join(PAPER, 'fixtures/svg');
export const RENDER_DIR = join(PAPER, 'fixtures/render');
export const HOEHE = 192;

/** Quell-SVGs: Name ohne Endung → Asset (Teil vor dem ersten `-`). */
export const quellen = () => readdirSync(SVG_DIR).filter((f) => f.endsWith('.svg')).sort()
  .map((f) => ({ name: f.slice(0, -4), asset: f.split('-')[0], datei: join(SVG_DIR, f) }));

/** Rendert eine SVG-Datei; Zeilenenden zählen nicht (Git unter Windows checkt mit CRLF aus). */
export function render(datei: string): { png: Buffer; breite: number; hoehe: number } {
  const svg = readFileSync(datei, 'utf8').replace(/\r\n/g, '\n');
  const r = new Resvg(svg, { fitTo: { mode: 'height', value: HOEHE }, font: { loadSystemFonts: false } }).render();
  return { png: Buffer.from(r.asPng()), breite: r.width, hoehe: r.height };
}

export const sha256 = (b: Buffer | string) => createHash('sha256').update(b).digest('hex');

/** Manifest-Rahmen der Fixtures: eine Animation je Quell-SVG, nur Frame 0 (Format: docs/standards/paper-spec.md). */
const ANIMATIONEN: Record<string, Record<string, string>> = {
  monarch: { idle: 'monarch-idle-0', reiten: 'monarch-galopp-0' },
  pferd: { galopp: 'pferd-galopp-0' },
  mauer: { stand: 'mauer-1', 'stand.s2': 'mauer-2' },
};

function manifest(asset: string, hashes: Record<string, string>, groesse: { breite: number; hoehe: number }) {
  const spec = JSON.parse(readFileSync(join(PAPER, 'fixtures/specs', `${asset}.paper.json`), 'utf8'));
  const stil = JSON.parse(readFileSync(join(PAPER, '../../public/grafik/stil.paper.json'), 'utf8'));
  const sh = specHash(spec), st = stilHash(stil);
  const animationen = Object.fromEntries(Object.entries(ANIMATIONEN[asset]).map(([name, quelle]) =>
    [name, { frames: 1, fps: spec.animationen?.[name.split('.')[0]]?.fps ?? 8, datei: `${quelle}.png`, hashes: [hashes[`${asset}/${quelle}.png`]] }]));
  return { id: spec.id, version: spec.version, specHash: sh, stilHash: st, renderer: RENDERER_VERSION, renderKey: renderKey(sh, st),
    rahmen: spec.ausgabe.rahmen, frameW: groesse.breite, frameH: groesse.hoehe, animationen, varianten: Object.keys(spec.varianten ?? {}) };
}

function schreibeAlles() {
  const hashes: Record<string, string> = {};
  const groessen: Record<string, { breite: number; hoehe: number }> = {};
  for (const q of quellen()) {
    const a = render(q.datei), b = render(q.datei);
    if (!a.png.equals(b.png)) throw new Error(`${q.name}: zwei Läufe nicht byte-gleich`);
    mkdirSync(join(RENDER_DIR, q.asset), { recursive: true });
    writeFileSync(join(RENDER_DIR, q.asset, `${q.name}.png`), a.png);
    hashes[`${q.asset}/${q.name}.png`] = sha256(a.png);
    groessen[q.asset] = { breite: a.breite, hoehe: a.hoehe };
  }
  writeFileSync(join(RENDER_DIR, 'hashes.json'), JSON.stringify(hashes, null, 2) + '\n');
  for (const asset of Object.keys(ANIMATIONEN))
    writeFileSync(join(RENDER_DIR, asset, 'manifest.json'), JSON.stringify(manifest(asset, hashes, groessen[asset]), null, 2) + '\n');
  return hashes;
}

if (import.meta.main) {
  if (process.argv.includes('--write')) console.log(JSON.stringify({ ok: true, hashes: schreibeAlles() }, null, 2));
  else for (const q of quellen()) console.log(q.name, sha256(render(q.datei).png) === sha256(render(q.datei).png) ? 'byte-gleich' : 'ABWEICHUNG');
}
