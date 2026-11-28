// Paketprüfung (B-1137): Pflichtdateien, Pflichtordner, verbotene Reste und EXE-Größen. `pruefe` arbeitet auf einer
// Liste von Einträgen (Tests ohne Platte), `listEntries` liest sie aus dem Paketordner.

import { lstatSync, readdirSync, readFileSync, statSync } from 'node:fs';
import path from 'node:path';

/** Entry ist eine Datei oder ein Ordner, `path` relativ zum Paketordner mit `/`. */
export interface Entry {
  path: string;
  dir: boolean;
  size: number;
}

// Pflichtdateien des Rust-Pakets (relativ zum Paketordner).
const REQUIRED_FILES = [
  'k3c-game.exe', 'k3c-devwin.exe', 'README.txt', 'LICENSE', 'LICENSE-ASSETS', 'LIZENZEN.txt', 'public/schrift/LICENSE',
];

// Größte erlaubte Fenster-EXE (RU56, B-1135: Schwelle 5 MB, kein GUI-Framework).
const MAX_DEVWIN_BYTES = 5 * (1 << 20);

// Pflichtordner müssen mindestens eine Datei enthalten, deren Name auf das Suffix endet (leer = beliebig).
// data/worlds hält die Welten des Pakets (B-1210).
const REQUIRED_DIRS: [dir: string, suffix: string][] = [
  ['data', '.json'], ['data/worlds', '.json'], ['public/grafik', 'index.paper.json'], ['public/audio', ''], ['public/rig', ''],
];

const FORBIDDEN_EXT = ['.go', '.rs', '.pdb', '.tmp'];
const FORBIDDEN_NAMES = ['go.mod', 'go.sum', 'assets.txt', 'Cargo.toml', 'Cargo.lock'];
const FORBIDDEN_DIRS = ['saves', 'reports', '.git', '.work', 'target', 'trace'];
// Entstehen beim Spielen aus dem Paket (Spielstände, Berichte, Aufzeichnung): im Entwicklungsordner erlaubt (-lokal),
// im Paket zum Weitergeben nicht.
const RUNTIME_DIRS = ['saves', 'reports', 'trace'];

const baseName = (p: string): string => p.slice(p.lastIndexOf('/') + 1);

/** Ordner, in die nicht abgestiegen wird (geduldete Laufzeit-Ordner und verbotene Ordner). */
function skipsDir(p: string, lokal: boolean): boolean {
  return (lokal && RUNTIME_DIRS.includes(p)) || FORBIDDEN_DIRS.includes(baseName(p));
}

/** forbiddenFile nennt den Grund, warum eine Datei nicht ins Paket gehört (Go-Reste, Quellen, Entwickler-Dateien). */
function forbiddenFile(p: string): string {
  const base = baseName(p);
  const ext = FORBIDDEN_EXT.find((e) => base.endsWith(e));
  if (ext) return `Endung ${ext}`;
  if (FORBIDDEN_NAMES.includes(base)) return 'Entwickler-Datei';
  if (p.startsWith('data/') && base.endsWith('.md')) return 'Doku unter data/';
  return '';
}

// Wie %.1f in Go: Hälften zur geraden Ziffer (Bytes·10 / 2^20 ist exakt).
function mib(bytes: number): string {
  const scaled = (bytes * 10) / (1 << 20);
  let tenths = Math.floor(scaled);
  const rest = scaled - tenths;
  if (rest > 0.5 || (rest === 0.5 && tenths % 2 === 1)) tenths++;
  return (tenths / 10).toFixed(1);
}

const tooBig = (file: string, size: number, max: number): string =>
  `${file} zu groß: ${mib(size)} MiB, erlaubt ${mib(max)} MiB`;

/** missing prüft Pflichtdateien, Pflichtordner und EXE-Größen gegen die Dateiliste. */
function missing(files: Map<string, number>, maxExeBytes: number): string[] {
  const mangel = REQUIRED_FILES.filter((f) => !files.has(f)).map((f) => `Pflichtdatei fehlt: ${f}`);
  const game = files.get('k3c-game.exe') ?? 0;
  if (maxExeBytes > 0 && game > maxExeBytes) mangel.push(tooBig('k3c-game.exe', game, maxExeBytes));
  const devwin = files.get('k3c-devwin.exe') ?? 0;
  if (devwin > MAX_DEVWIN_BYTES) mangel.push(tooBig('k3c-devwin.exe', devwin, MAX_DEVWIN_BYTES));
  const names = [...files.keys()];
  for (const [dir, suffix] of REQUIRED_DIRS) {
    if (!names.some((f) => f.startsWith(`${dir}/`) && f.endsWith(suffix))) {
      mangel.push(`Pflichtordner leer oder fehlt: ${dir} (erwartet *${suffix})`);
    }
  }
  return mangel;
}

// Reihenfolge wie filepath.WalkDir: je Ebene nach Name, ein Ordner vor seinem Inhalt.
function walkOrder(a: Entry, b: Entry): number {
  const left = a.path.split('/');
  const right = b.path.split('/');
  for (let i = 0; i < left.length && i < right.length; i++) {
    if (left[i] !== right[i]) return left[i] < right[i] ? -1 : 1;
  }
  return left.length - right.length;
}

export interface Check {
  maxExeBytes: number;
  lokal: boolean;
  /** Inhalt von LIZENZEN.txt, undefined wenn nicht lesbar. */
  lizenzen: () => string | undefined;
}

/** pruefe liefert je Mangel eine Zeile (leer = vollständig); n ist die Zahl der geprüften Dateien. */
export function pruefe(entries: Entry[], check: Check): { mangel: string[]; n: number } {
  const mangel: string[] = [];
  const files = new Map<string, number>();
  const skipped: string[] = [];
  for (const entry of [...entries].sort(walkOrder)) {
    if (skipped.some((dir) => entry.path.startsWith(`${dir}/`))) continue;
    if (entry.dir) {
      if (!skipsDir(entry.path, check.lokal)) continue;
      skipped.push(entry.path);
      if (!(check.lokal && RUNTIME_DIRS.includes(entry.path))) mangel.push(`verbotener Ordner: ${entry.path}`);
      continue;
    }
    files.set(entry.path, entry.size);
    const reason = forbiddenFile(entry.path);
    if (reason) mangel.push(`verbotene Datei: ${entry.path} (${reason})`);
  }
  mangel.push(...missing(files, check.maxExeBytes));
  if (files.has('LIZENZEN.txt') && !check.lizenzen()?.includes('\n==== ')) {
    mangel.push('LIZENZEN.txt enthält keinen Crate-Abschnitt');
  }
  return { mangel, n: files.size };
}

/** listEntries liest den Paketordner; Links werden nicht verfolgt, übersprungene Ordner nicht geöffnet. */
export function listEntries(root: string, lokal: boolean, rel = ''): Entry[] {
  const res: Entry[] = [];
  for (const item of readdirSync(path.join(root, rel), { withFileTypes: true })) {
    const p = rel ? `${rel}/${item.name}` : item.name;
    if (item.isDirectory()) {
      res.push({ path: p, dir: true, size: 0 });
      if (!skipsDir(p, lokal)) res.push(...listEntries(root, lokal, p));
    } else {
      res.push({ path: p, dir: false, size: lstatSync(path.join(root, p)).size });
    }
  }
  return res;
}

function readText(file: string): string | undefined {
  try {
    return readFileSync(file, 'utf8');
  } catch {
    return undefined;
  }
}

function isDirectory(dir: string): boolean {
  try {
    return statSync(dir).isDirectory();
  } catch {
    return false;
  }
}

function checkDir(dir: string, maxExeBytes: number, lokal: boolean): { mangel: string[]; n: number } {
  try {
    const lizenzen = () => readText(path.join(dir, 'LIZENZEN.txt'));
    return pruefe(listEntries(dir, lokal), { maxExeBytes, lokal, lizenzen });
  } catch (e) {
    return { mangel: [`Paketordner nicht lesbar: ${e instanceof Error ? e.message : String(e)}`], n: 0 };
  }
}

/** pruefen meldet die Mängel eines Paketordners nach `out` und wirft, wenn es welche gibt. */
export function pruefen(dir: string, maxExeMib: number, lokal: boolean, out: (text: string) => void): void {
  if (!isDirectory(dir)) throw new Error(`paketordner ${dir} fehlt (erst task rust:game:build)`);
  const { mangel, n } = checkDir(dir, maxExeMib * (1 << 20), lokal);
  if (mangel.length > 0) {
    for (const m of mangel) out(`❌ ${m}\n`);
    throw new Error(`paket ${dir} unvollständig (${mangel.length} Mängel)`);
  }
  out(`✅ Paket ${dir} vollständig (${n} Dateien)\n`);
}
