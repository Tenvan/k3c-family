// gen:paper (Task in tools/Taskfile.yml): bun tools/paper/bin/gen.ts [wurzel | --nur-kopieren]: ohne Wurzel .work/grafik → public/grafik (B-737)
// Antwort wie die Render-CLI: ein JSON-Objekt; Exit 0 ok, 1 abgelehnt ({ok: false, fehler}), 2 Aufruf-/Systemfehler.
import { existsSync } from 'node:fs';
import { join } from 'node:path';
import { konsoleUndLog } from '../cli/fortschritt';
import { genParallel } from '../cli/genParallel';
import { genMitWork, nurKopieren, ohneManifest, WORK } from '../cli/work';
import { Abgelehnt } from '../render/fehler';
import { ROOT } from '../render/stil';

// `--fehlt public|work`: Exit 1, wenn dort zu einer Spec das Manifest fehlt (Status-Prüfung der Tasks, B-748); gibt die fehlenden aus
if (process.argv[2] === '--fehlt') {
  const fehlt = ohneManifest(process.argv[3] === 'work' ? WORK : join(ROOT, 'public/grafik'));
  if (fehlt.length) process.stderr.write(`Manifest fehlt: ${fehlt.slice(0, 5).join(', ')}${fehlt.length > 5 ? ` … (${fehlt.length})` : ''}
`);
  process.exit(fehlt.length ? 1 : 0);
}
const kopieren = process.argv[2] === '--nur-kopieren'; // ohne Rendern: Ergebnisse aus .work/grafik nach public/grafik (B-748)
const eigeneWurzel = kopieren ? undefined : process.argv[2];
const wurzel = eigeneWurzel ?? join(ROOT, 'public/grafik');
try {
  if (!existsSync(wurzel)) throw new Error(`Wurzel fehlt: ${wurzel}`);
  // Ohne Argument im Arbeitsordner `.work/grafik` rendern und Geändertes nach public/grafik kopieren (B-737); mit Wurzel direkt dort.
  // Fortschritt je Schritt auf stderr und in .work/grafik/gen.log (B-740); stdout bleibt das eine JSON-Objekt.
  // Rendern parallel auf `K3C_PAPER_JOBS` Workern (Standard: Kerne minus 1, höchstens 8), im Arbeitsordner mit Cache je Asset (B-751).
  const melde = konsoleUndLog(join(WORK, 'gen.log'));
  const ergebnis = kopieren ? nurKopieren(wurzel, WORK, melde) : eigeneWurzel ? await genParallel(wurzel, { melde }) : await genMitWork(wurzel, WORK, melde);
  process.stdout.write(JSON.stringify({ ok: true, ...ergebnis }) + '\n');
} catch (e) {
  if (e instanceof Abgelehnt) {
    process.stdout.write(JSON.stringify({ ok: false, fehler: e.fehler }) + '\n');
    process.exitCode = 1;
  } else {
    process.stderr.write((e as Error).message + '\n');
    process.exitCode = 2;
  }
}
