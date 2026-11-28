// Aufruf von rustpaket: Unterbefehl und Flags lesen, Lauf starten. `lizenzen` schreibt LIZENZEN.txt nach stdout
// (B-1121), `pruefen` prüft einen Paketordner gegen die feste Liste (B-1137).

import { FlagError, parseFlags, type FlagKind, type Flags } from '../checkdiff/flags';
import { lizenzen, type Write } from './lizenzen';
import { pruefen } from './pruefen';

const USAGE = `Aufruf: rustpaket lizenzen -exe <k3c-game.exe> [-rust rust] [-target x86_64-pc-windows-msvc]
        rustpaket pruefen [-max-exe-mib 40] [-lokal] <paketordner>
`;

const LIZENZEN_USAGE = `Usage of lizenzen:
  -exe licenses
    \tk3c-game.exe, gibt mit licenses den Kopf aus
  -rust string
    \tOrdner des Rust-Workspace (default "rust")
  -target string
    \tZielplattform der EXE (default "x86_64-pc-windows-msvc")
`;

const PRUEFEN_USAGE = `Usage of pruefen:
  -lokal
    \tsaves/, reports/ und trace/ im Paketordner dulden (Entwicklungsordner nach dem Spielen)
  -max-exe-mib int
    \tgrößte erlaubte EXE in MiB (Schwelle G1; 0 = keine Grenze) (default 40)
`;

export interface Streams {
  out: Write;
  err: (text: string) => void;
}

const std: Streams = {
  out: (chunk) => void process.stdout.write(chunk),
  err: (text) => void process.stderr.write(text),
};

function flagsOf(argv: string[], spec: Record<string, FlagKind>, usage: string, io: Streams): Flags {
  try {
    return parseFlags(argv, spec);
  } catch (e) {
    if (!(e instanceof FlagError)) throw e;
    io.err(e.message ? `${e.message}\n${usage}` : usage);
    throw new Error(e.message || 'flag: help requested');
  }
}

function cmdLizenzen(argv: string[], io: Streams): void {
  const { values } = flagsOf(argv, { exe: 'string', rust: 'string', target: 'string' }, LIZENZEN_USAGE, io);
  const exe = String(values.exe ?? '');
  if (!exe) throw new Error('-exe fehlt');
  const rust = String(values.rust ?? 'rust');
  const target = String(values.target ?? 'x86_64-pc-windows-msvc');
  lizenzen({ exe, rust, target }, io.out);
}

function maxExeMib(value: string, io: Streams): number {
  if (/^[+-]?\d+$/.test(value)) return Number(value);
  const text = `invalid value ${JSON.stringify(value)} for flag -max-exe-mib: parse error`;
  io.err(`${text}\n${PRUEFEN_USAGE}`);
  throw new Error(text);
}

function cmdPruefen(argv: string[], io: Streams): void {
  const { values, rest } = flagsOf(argv, { 'max-exe-mib': 'string', lokal: 'bool' }, PRUEFEN_USAGE, io);
  const max = maxExeMib(String(values['max-exe-mib'] ?? '40'), io);
  if (rest.length !== 1) throw new Error('genau ein Paketordner erwartet');
  pruefen(rest[0], max, values.lokal === true, io.out);
}

export function main(argv: string[], io: Streams = std): number {
  const command = argv[0] === 'lizenzen' ? cmdLizenzen : argv[0] === 'pruefen' ? cmdPruefen : undefined;
  if (!command) {
    io.err(USAGE);
    return 2;
  }
  try {
    command(argv.slice(1), io);
  } catch (e) {
    io.err(`❌ rustpaket: ${e instanceof Error ? e.message : String(e)}\n`);
    return 1;
  }
  return 0;
}
