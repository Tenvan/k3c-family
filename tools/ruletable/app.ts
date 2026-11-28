// Schreibt den Block „Hub-Stufe → Gebäude“ aus data/hub.json in docs/rules/materialien-gebaeude.md (§ 3, zwischen
// den Marken). Aufruf nur über den Task `gen:rules`; gleiche Daten ergeben byte-gleichen Text (WZ15.2, B-462).

import { randomBytes } from 'node:crypto';
import { readFileSync, renameSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { FlagError, parseFlags, type Flags } from '../checkdiff/flags';
import { block, replaceBlock } from './table';

const USAGE = `Usage: bun tools/ruletable/main.ts [-root <ordner>]
  -root string
    \tProjektwurzel (default ".")
`;

const message = (e: unknown): string => (e instanceof Error ? e.message : String(e));

/** run ersetzt den Block und schreibt die Datei nur bei einer Änderung (atomar über eine temporäre Datei). */
export function run(root: string): boolean {
  const hub = readFileSync(path.join(root, 'data', 'hub.json'), 'utf8');
  const file = path.join(root, 'docs', 'rules', 'materialien-gebaeude.md');
  const doc = readFileSync(file, 'utf8');
  const wanted = block(hub);
  let next: string;
  try {
    next = replaceBlock(doc, wanted);
  } catch (e) {
    throw new Error(`${file}: ${message(e)}`);
  }
  if (next === doc) return false;
  const tmp = path.join(path.dirname(file), `${randomBytes(6).toString('hex')}.tmp`);
  try {
    writeFileSync(tmp, next, { flag: 'wx' });
    renameSync(tmp, file);
  } finally {
    rmSync(tmp, { force: true }); // bleibt nur bei einem Fehler übrig
  }
  return true;
}

export interface Streams {
  out: (text: string) => void;
  err: (text: string) => void;
}

const std: Streams = {
  out: (text) => void process.stdout.write(text),
  err: (text) => void process.stderr.write(text),
};

export function main(argv: string[], io: Streams = std): number {
  let flags: Flags;
  try {
    flags = parseFlags(argv, { root: 'string' });
  } catch (e) {
    if (!(e instanceof FlagError)) throw e;
    io.err(e.message ? `${e.message}\n${USAGE}` : USAGE);
    return e.code;
  }
  try {
    const changed = run(String(flags.values.root ?? '.'));
    io.out(changed ? '✅ regeltabelle: Block aktualisiert\n' : '✅ regeltabelle: Block ist aktuell\n');
  } catch (e) {
    io.err(`❌ regeltabelle: ${message(e)}\n`);
    return 1;
  }
  return 0;
}
