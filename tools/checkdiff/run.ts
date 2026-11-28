// Lauf: Schritte einer Stufe ausführen, Bahnen nebeneinander, Schritte einer Bahn nacheinander bis zum ersten Fehler.
// Die Marker `⏱▶ name` und `⏱■ name exit=n` zeitstempelt check_run der Workbench (B-739). Die Ausgabe eines Schritts
// erscheint nur bei einem Fehler, am Stück.

import { spawn } from 'node:child_process';
import { notFound, resolveExe } from './exe';
import { describe, type Step } from './steps';

export type Write = (data: string | Buffer) => void;

interface Result {
  code: number;
  out: Buffer;
}

export const toStdout: Write = (data) => void process.stdout.write(data);

/** Führt den Befehl ohne Shell aus und sammelt stdout und stderr in der Reihenfolge ihres Eintreffens. */
function capture(s: Step): Promise<Result> {
  return new Promise((done) => {
    const chunks: Buffer[] = [];
    const failed = (message: string) => done({ code: 127, out: Buffer.concat([...chunks, Buffer.from(`${message}\n`)]) });
    const [name, ...args] = s.cmd;
    const exe = resolveExe(name);
    if (!exe) return failed(notFound(name));
    let startError: Error | undefined;
    try {
      const child = spawn(exe, args, { cwd: s.dir, stdio: ['ignore', 'pipe', 'pipe'], windowsHide: true });
      child.stdout.on('data', (d: Buffer) => chunks.push(d));
      child.stderr.on('data', (d: Buffer) => chunks.push(d));
      child.on('error', (e) => {
        startError = e;
        if (child.pid === undefined) failed(e.message);
      });
      child.on('close', (code) => {
        if (startError) return failed(startError.message);
        done({ code: code ?? -1, out: Buffer.concat(chunks) });
      });
    } catch (e) {
      failed(e instanceof Error ? e.message : String(e));
    }
  });
}

export async function runStep(s: Step, write: Write = toStdout): Promise<boolean> {
  write(`⏱▶ ${s.name}\n`);
  const start = Date.now();
  const { code, out } = await capture(s);
  if (code !== 0) {
    write(`❌ ${describe(s)} (${((Date.now() - start) / 1000).toFixed(1)} s)\n`);
    write(out);
  }
  write(`⏱■ ${s.name} exit=${code}\n`);
  return code === 0;
}

/** Führt die Schritte einer Stufe aus und liefert die Namen der gescheiterten Schritte. */
export async function runStage(steps: Step[], stage: Step['stage'], write: Write = toStdout): Promise<string[]> {
  const lanes = new Map<string, Step[]>();
  for (const s of steps.filter((x) => x.stage === stage)) lanes.set(s.lane, [...(lanes.get(s.lane) ?? []), s]);
  const failed: string[] = [];
  await Promise.all(
    [...lanes.values()].map(async (lane) => {
      for (const s of lane) {
        if (!(await runStep(s, write))) {
          failed.push(s.name);
          return;
        }
      }
    }),
  );
  return failed;
}
