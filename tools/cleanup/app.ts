// Aufruf des Aufräumens: Flags lesen, Lauf starten. Unbekannte Flags und lose Argumente brechen ab, bevor etwas
// geschieht: `task ops:aufraeumen -- dry` darf nie als scharfer Lauf durchgehen.

import { FlagError, parseFlags, type Flags } from '../checkdiff/flags';
import { run } from './cleanup';
import { ghMerged, gitWith } from './git';

const USAGE = `Usage: bun tools/cleanup/main.ts [-root <ordner>] [-dry]
  -dry           nur anzeigen, nichts ändern
  -root string   Projektwurzel (ein beliebiger Checkout des Repos, Vorgabe ".")
`;

type Write = (text: string) => void;

export interface Streams {
  out: Write;
  err: Write;
}

const std: Streams = {
  out: (text) => void process.stdout.write(text),
  err: (text) => void process.stderr.write(text),
};

export function main(argv: string[], env: NodeJS.ProcessEnv = process.env, io: Streams = std): number {
  let flags: Flags;
  try {
    flags = parseFlags(argv, { root: 'string', dry: 'bool' });
    if (flags.rest.length > 0) throw new FlagError(`unerwartetes Argument: ${flags.rest[0]}`);
  } catch (e) {
    if (!(e instanceof FlagError)) throw e;
    io.err(e.message ? `${e.message}\n${USAGE}` : USAGE);
    return e.code;
  }
  const root = String(flags.values.root ?? '.') || '.';
  try {
    run({ root, dry: flags.values.dry === true, git: gitWith(env), prMerged: ghMerged(root, env), out: io.out });
  } catch (e) {
    io.err(`aufraeumen: ${e instanceof Error ? e.message : String(e)}\n`);
    return 1;
  }
  return 0;
}
