// gen:paper (Task in tools/Taskfile.yml): bun tools/paper/bin/gen.ts [wurzel]: ohne Wurzel .work/grafik → public/grafik (B-737)
// Antwort wie die Render-CLI: ein JSON-Objekt; Exit 0 ok, 1 abgelehnt ({ok: false, fehler}), 2 Aufruf-/Systemfehler.
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { gen } from '../cli/gen';
import { konsoleUndLog } from '../cli/fortschritt';
import { genMitWork, WORK } from '../cli/work';
import { Abgelehnt } from '../render/fehler';
import { ROOT } from '../render/stil';

const wurzel = process.argv[2] ?? join(ROOT, 'public/grafik');
try {
  if (!existsSync(wurzel)) throw new Error(`Wurzel fehlt: ${wurzel}`);
  // Ohne Argument im Arbeitsordner `.work/grafik` rendern und Geändertes nach public/grafik kopieren (B-737); mit Wurzel direkt dort.
  // Fortschritt je Schritt auf stderr und in .work/grafik/gen.log (B-740); stdout bleibt das eine JSON-Objekt.
  const melde = konsoleUndLog(join(WORK, 'gen.log'));
  process.stdout.write(JSON.stringify({ ok: true, ...(process.argv[2] ? gen(wurzel, { melde }) : genMitWork(wurzel, WORK, melde)) }) + '\n');
} catch (e) {
  if (e instanceof Abgelehnt) {
    process.stdout.write(JSON.stringify({ ok: false, fehler: e.fehler }) + '\n');
    process.exitCode = 1;
  } else {
    process.stderr.write((e as Error).message + '\n');
    process.exitCode = 2;
  }
}
