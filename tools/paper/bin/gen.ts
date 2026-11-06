// gen:paper (Task in tools/Taskfile.yml): bun tools/paper/bin/gen.ts [wurzel = public/grafik]
// Antwort wie die Render-CLI: ein JSON-Objekt; Exit 0 ok, 1 abgelehnt ({ok: false, fehler}), 2 Aufruf-/Systemfehler.
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { gen } from '../cli/gen';
import { Abgelehnt } from '../render/fehler';
import { ROOT } from '../render/stil';

const wurzel = process.argv[2] ?? join(ROOT, 'public/grafik');
try {
  if (!existsSync(wurzel)) throw new Error(`Wurzel fehlt: ${wurzel}`);
  process.stdout.write(JSON.stringify({ ok: true, ...gen(wurzel) }) + '\n');
} catch (e) {
  if (e instanceof Abgelehnt) {
    process.stdout.write(JSON.stringify({ ok: false, fehler: e.fehler }) + '\n');
    process.exitCode = 1;
  } else {
    process.stderr.write((e as Error).message + '\n');
    process.exitCode = 2;
  }
}
