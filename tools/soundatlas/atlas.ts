// Packt die erste Präferenz jedes Effekt-Slots aus data/sounds.json (Bus sfx, ohne Schleifen) zu einem Sound-Atlas
// public/audio/atlas.ogg und atlas.mp3 mit atlas.json (Start und Dauer je Slot, SO6.2, B-494). ffmpeg dekodiert und
// kodiert; die Reihenfolge folgt der Slot-ID, die Ausgabe hängt nur von den Eingaben ab.

import { spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, statSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { notFound, resolveExe } from '../checkdiff/exe';

const SAMPLE_RATE = 44100;
// Stille zwischen zwei Effekten: fängt die Encoder-Verzögerung von MP3 (~25 ms) und Rundung ab.
const GAP_SECONDS = 0.15;
// Spielen als Schleife je Spieler und gehören nicht in den Atlas.
const LOOP_SLOTS = new Set(['sfx:ride']);
const COMMENT = 'Erzeugt von task gen:soundatlas (tools/soundatlas) aus data/sounds.json; nicht von Hand ändern.';

export interface Slot {
  id: string;
  art: string;
  bus: string;
  wahl?: { datei: string; sprite?: string }[];
}

/** Entry ist ein Effekt im Atlas: Slot und Datei unter public/. */
export interface Entry {
  slot: string;
  file: string;
}

/** Sprite ist Start und Dauer in Sekunden (auf Millisekunden gerundet). */
export interface Sprite {
  start: number;
  duration: number;
}

const message = (e: unknown): string => (e instanceof Error ? e.message : String(e));

/** pickEntries wählt rein die erste Präferenz jedes Effekt-Slots ohne Atlas-Sprite, sortiert nach Slot-ID. */
export function pickEntries(slots: Slot[]): Entry[] {
  const out: Entry[] = [];
  for (const s of slots) {
    const first = s.wahl?.[0];
    if (s.art !== 'sound' || s.bus !== 'sfx' || LOOP_SLOTS.has(s.id) || !first || first.sprite) continue;
    out.push({ slot: s.id, file: first.datei ?? '' });
  }
  return out.sort((a, b) => (a.slot < b.slot ? -1 : a.slot > b.slot ? 1 : 0));
}

const seconds = (samples: number): number => Math.round((samples / SAMPLE_RATE) * 1000) / 1000;

/** layout rechnet rein Start und Dauer je Slot und die Zahl der Stille-Samples vor jedem Effekt. */
export function layout(entries: Entry[], samples: number[]): { sprites: Map<string, Sprite>; pads: number[] } {
  const gap = Math.round(GAP_SECONDS * SAMPLE_RATE);
  const sprites = new Map<string, Sprite>();
  const pads: number[] = [];
  let pos = 0;
  entries.forEach((entry, i) => {
    pads.push(i > 0 ? gap : 0);
    pos += pads[i];
    sprites.set(entry.slot, { start: seconds(pos), duration: seconds(samples[i]) });
    pos += samples[i];
  });
  return { sprites, pads };
}

/** atlasJson schreibt den Index wie Go: Dateien und Sprites nach Schlüssel sortiert, zwei Leerzeichen. */
export function atlasJson(sprites: Map<string, Sprite>): string {
  const sorted = [...sprites].sort(([a], [b]) => (a < b ? -1 : 1));
  const index = { _comment: COMMENT, files: { mp3: 'atlas.mp3', ogg: 'atlas.ogg' }, sprites: Object.fromEntries(sorted) };
  return `${JSON.stringify(index, null, 2)}\n`;
}

/** isLocal meldet, ob ein Pfad innerhalb seines Ausgangsordners bleibt (wie filepath.IsLocal). */
export function isLocal(file: string): boolean {
  if (file === '' || /^[\\/]/.test(file) || file.includes(':')) return false;
  const normal = path.posix.normalize(file.replaceAll('\\', '/'));
  if (normal === '..' || normal.startsWith('../')) return false;
  return !normal.split('/').some((part) => /^(con|prn|aux|nul|com\d|lpt\d)(\.|$)/i.test(part));
}

function ffmpegExe(ffmpeg: string): string {
  if (/[\\/]/.test(ffmpeg)) return path.resolve(ffmpeg);
  const found = resolveExe(ffmpeg.replace(/\.exe$/i, ''));
  if (!found) throw new Error(notFound(ffmpeg));
  return found;
}

function runFfmpeg(ffmpeg: string, args: string[]) {
  const res = spawnSync(ffmpegExe(ffmpeg), args, { maxBuffer: 512 * 1024 * 1024, windowsHide: true });
  if (res.error) throw new Error(res.error.message);
  return { failed: res.status === 0 ? '' : `exit status ${res.status ?? res.signal}`, out: res.stdout, err: res.stderr };
}

/** Liefert die Datei als PCM s16le mono 44,1 kHz. */
function decode(ffmpeg: string, file: string): Buffer {
  statSync(file);
  const args = ['-v', 'error', '-i', file, '-ac', '1', '-ar', String(SAMPLE_RATE), '-f', 's16le', '-'];
  const res = runFfmpeg(ffmpeg, args);
  if (res.failed) throw new Error(`ffmpeg: ${res.failed} ${res.err.toString('utf8').trim()}`);
  if (res.out.length === 0) throw new Error('leer');
  return res.out;
}

function encode(ffmpeg: string, raw: string, tmp: string): void {
  const input = [
    '-v', 'error', '-y', '-f', 's16le', '-ar', String(SAMPLE_RATE), '-ac', '1', '-i', raw,
    '-map_metadata', '-1', '-fflags', '+bitexact', '-flags:a', '+bitexact',
  ];
  const encodings: [name: string, args: string[]][] = [
    ['atlas.ogg', ['-c:a', 'libvorbis', '-q:a', '3', '-serial_offset', '1']],
    ['atlas.mp3', ['-c:a', 'libmp3lame', '-b:a', '64k']],
  ];
  for (const [name, args] of encodings) {
    const res = runFfmpeg(ffmpeg, [...input, ...args, path.join(tmp, name)]);
    if (res.failed) throw new Error(`${name}: ${res.failed} ${Buffer.concat([res.out, res.err]).toString('utf8').trim()}`);
  }
}

/** Kodiert erst in temporäre Dateien und benennt danach um: ein Fehler hinterlässt keinen halben Atlas. */
function write(root: string, ffmpeg: string, pcm: Buffer, sprites: Map<string, Sprite>): void {
  const dir = path.join(root, 'public', 'audio');
  const tmpBase = path.join(root, '.work', 'tmp'); // Regel „Schreiben nur im Repo“
  mkdirSync(tmpBase, { recursive: true });
  const tmp = mkdtempSync(path.join(tmpBase, 'soundatlas-'));
  try {
    const raw = path.join(tmp, 'atlas.raw');
    writeFileSync(raw, pcm);
    encode(ffmpeg, raw, tmp);
    writeFileSync(path.join(tmp, 'atlas.json'), atlasJson(sprites));
    // erst alle drei neben die Ziele kopieren, dann umbenennen: die drei Dateien passen immer zusammen
    const names = ['atlas.ogg', 'atlas.mp3', 'atlas.json'];
    for (const name of names) writeFileSync(path.join(dir, `${name}.tmp`), readFileSync(path.join(tmp, name)));
    for (const name of names) renameSync(path.join(dir, `${name}.tmp`), path.join(dir, name));
  } finally {
    rmSync(tmp, { recursive: true, force: true, maxRetries: 10, retryDelay: 100 });
  }
}

function readSlots(root: string): Slot[] {
  const raw = readFileSync(path.join(root, 'data', 'sounds.json'), 'utf8');
  try {
    return (JSON.parse(raw) as { slots?: Slot[] }).slots ?? [];
  } catch (e) {
    throw new Error(`data/sounds.json: ${message(e)}`);
  }
}

function decodeEntry(root: string, ffmpeg: string, entry: Entry): Buffer {
  if (!isLocal(entry.file)) {
    throw new Error(`${entry.slot}: Pfad ${JSON.stringify(entry.file)} liegt nicht unter public/`);
  }
  try {
    return decode(ffmpeg, path.join(root, 'public', entry.file));
  } catch (e) {
    throw new Error(`${entry.slot} (${entry.file}): ${message(e)}`);
  }
}

/** run baut den Atlas des Projekts in `root` und liefert die Zahl der Effekte. */
export function run(root: string, ffmpeg: string): number {
  const entries = pickEntries(readSlots(root));
  if (entries.length === 0) throw new Error('keine Effekt-Slots');
  const pcm = entries.map((entry) => decodeEntry(root, ffmpeg, entry));
  const { sprites, pads } = layout(entries, pcm.map((b) => Math.floor(b.length / 2)));
  const all = Buffer.concat(pcm.flatMap((b, i) => [Buffer.alloc(pads[i] * 2), b]));
  write(root, ffmpeg, all, sprites);
  return entries.length;
}
