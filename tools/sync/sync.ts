// Kopiert Dateien und Ordner ins Spiel-Paket (dist/k3c-game, `task rust:game:build`): je Argument `quelle=ziel` eine
// Datei oder einen Ordner (rekursiv, zusammengeführt). Eine Datei wird nur kopiert, wenn sie im Ziel fehlt oder Größe
// bzw. Änderungszeit abweichen; so ersetzt ein neuer Build geänderte Grafik und lässt den Rest liegen.
// Eigenes Werkzeug statt cp: Task nutzt unter Windows ein eingebautes cp ohne -u und ohne Zusammenführen von Ordnern.

import { lstatSync, mkdirSync, readdirSync, readFileSync, statSync, utimesSync, writeFileSync } from 'node:fs';
import path from 'node:path';

// Node setzt Zeiten nur als Sekunden in double (rund 250 ns Auflösung heute), Go auf die Nanosekunde. Eine
// Abweichung unter 1 µs gilt darum als gleich; echte Änderungen liegen weit darüber.
const SLACK_NS = 1000n;

export interface Streams {
  out: (text: string) => void;
  err: (text: string) => void;
}

const std: Streams = {
  out: (text) => void process.stdout.write(text),
  err: (text) => void process.stderr.write(text),
};

const message = (e: unknown): string => (e instanceof Error ? e.message : String(e));

function unchanged(to: string, size: bigint, mtimeNs: bigint): boolean {
  try {
    const target = statSync(to, { bigint: true });
    const diff = target.mtimeNs - mtimeNs;
    return target.size === size && diff < SLACK_NS && diff > -SLACK_NS;
  } catch {
    return false;
  }
}

/** syncFile kopiert from nach to, wenn to fehlt oder Größe bzw. Änderungszeit abweichen; die Zeit wird übernommen. */
export function syncFile(from: string, to: string): boolean {
  const source = statSync(from, { bigint: true });
  if (unchanged(to, source.size, source.mtimeNs)) return false;
  mkdirSync(path.dirname(to), { recursive: true });
  try {
    writeFileSync(to, readFileSync(from));
  } catch (e) {
    throw new Error(`${from} → ${to}: ${message(e)} (läuft das Spiel dort noch?)`);
  }
  const seconds = Number(source.mtimeNs) / 1e9;
  utimesSync(to, seconds, seconds);
  return true;
}

function walk(from: string, to: string, isDir: boolean): number {
  if (!isDir) return syncFile(from, to) ? 1 : 0;
  mkdirSync(to, { recursive: true });
  const entries = readdirSync(from, { withFileTypes: true }).sort((a, b) => (a.name < b.name ? -1 : 1));
  let n = 0;
  for (const entry of entries) {
    n += walk(path.join(from, entry.name), path.join(to, entry.name), entry.isDirectory());
  }
  return n;
}

/** syncPath bringt dst auf den Stand von src (Datei oder Ordner) und liefert die Zahl kopierter Dateien. */
export function syncPath(src: string, dst: string): number {
  return walk(src, dst, lstatSync(src).isDirectory());
}

export function main(argv: string[], io: Streams = std): number {
  let n = 0;
  try {
    for (const arg of argv) {
      const cut = arg.indexOf('=');
      if (cut < 0) throw new Error(`argument ${JSON.stringify(arg)}: erwartet quelle=ziel`);
      n += syncPath(arg.slice(0, cut), arg.slice(cut + 1));
    }
  } catch (e) {
    io.err(`❌ ${message(e)}\n`);
    return 1;
  }
  io.out(`✅ ${n} Dateien aktualisiert\n`);
  return 0;
}
