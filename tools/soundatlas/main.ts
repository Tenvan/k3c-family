// Einstieg von task gen:soundatlas: `bun tools/soundatlas/main.ts [-root <ordner>] [-ffmpeg <pfad>]` (Logik: atlas.ts).

import { FlagError, parseFlags } from '../checkdiff/flags';
import { run } from './atlas';

const USAGE = `Usage: bun tools/soundatlas/main.ts [-root <ordner>] [-ffmpeg <pfad>]
  -ffmpeg string
    \tPfad zu ffmpeg (default "ffmpeg")
  -root string
    \tProjektwurzel (default ".")
`;

function main(argv: string[]): number {
  try {
    const { values } = parseFlags(argv, { root: 'string', ffmpeg: 'string' });
    const n = run(String(values.root ?? '.'), String(values.ffmpeg ?? 'ffmpeg'));
    process.stdout.write(`soundatlas: ${n} Effekte nach public/audio/atlas.{ogg,mp3,json}\n`);
    return 0;
  } catch (e) {
    if (e instanceof FlagError) {
      process.stderr.write(e.message ? `${e.message}\n${USAGE}` : USAGE);
      return e.code;
    }
    process.stderr.write(`soundatlas: ${e instanceof Error ? e.message : String(e)}\n`);
    return 1;
  }
}

process.exitCode = main(process.argv.slice(2));
