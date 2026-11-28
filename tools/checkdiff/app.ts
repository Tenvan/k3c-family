// checkdiff prüft vor einem Commit nur, was die geänderten Dateien dieses Commits brauchen (task check:diff, B-518,
// B-730, B-742). Stufe 1: Regel-Tests, Lint, Typecheck, cargo fmt und clippy. Stufe 2, nur nach grüner Stufe 1: Vitest
// --changed und die Tests der geänderten Crates. Keine Sim-, Balance-, Gen- und Race-Läufe. Regeln:
// docs/arbeitsweise.md › Tempo im Lauf.
//
//   bun tools/checkdiff/main.ts [-base <ref>] [-plan] [-only js|rust]
//
// -base: Vergleichsbasis statt HEAD (z. B. origin/main für den ganzen Sprint; auch Umgebung CHECK_BASE).
// -plan: nur zeigen, was laufen würde. -only: nur eine Bahn (nach einem roten Lauf den betroffenen Teil wiederholen).
// Der Einstieg main.ts ruft nur `main` auf; die Logik liegt hier, damit Tests sie ohne Lauf importieren.

import { FlagError, parseFlags } from './flags';
import { changedFiles, gitLinesIn, type GitLines } from './repo';
import { runStage, toStdout, type Write } from './run';
import { classify } from './select';
import { describe, jsSteps, rustSteps, type Step } from './steps';

interface Options {
  base: string;
  only: string;
  plan: boolean;
}

const USAGE = `Usage: bun tools/checkdiff/main.ts [-base <ref>] [-plan] [-only js|rust]
  -base string   Vergleichsbasis statt HEAD
  -only string   nur eine Bahn: js oder rust
  -plan          nur zeigen, was laufen würde
`;

function parse(argv: string[], env: NodeJS.ProcessEnv): Options {
  const { values } = parseFlags(argv, { base: 'string', only: 'string', plan: 'bool' });
  return {
    base: String(values.base ?? env.CHECK_BASE ?? ''),
    only: String(values.only ?? ''),
    plan: values.plan === true,
  };
}

/** Bildet die Schritte des Laufs. */
function build(o: Options, git: GitLines): { base: string; steps: Step[] } {
  const { base, files } = changedFiles(o.base, git);
  const p = classify(files);
  const steps = [...jsSteps(p, base), ...rustSteps(p)];
  return { base, steps: steps.filter((s) => o.only === '' || s.lane === o.only) };
}

async function check(o: Options, write: Write, git: GitLines): Promise<number> {
  const start = Date.now();
  const seconds = () => ((Date.now() - start) / 1000).toFixed(0);
  let run: { base: string; steps: Step[] };
  try {
    run = build(o, git);
  } catch (e) {
    const reason = e instanceof Error ? e.message : String(e);
    write(`❌ check:diff: git scheitert (${reason}); Basis mit -base oder CHECK_BASE setzen\n`);
    return 1;
  }
  write(`🚦 check:diff Basis ${run.base} · ${run.steps.length} Schritte\n`);
  for (const s of run.steps) write(`   Stufe ${s.stage} · ${describe(s)}\n`);
  if (o.plan) return 0;
  for (const stage of [1, 2] as const) {
    const failed = await runStage(run.steps, stage, write);
    if (failed.length > 0) {
      write(`❌ check:diff fehlgeschlagen in Stufe ${stage}: ${failed.join(', ')} (${seconds()} s)\n`);
      return 1;
    }
  }
  write(`✅ check:diff grün (${seconds()} s)\n`);
  return 0;
}

export async function main(
  argv: string[],
  env: NodeJS.ProcessEnv = process.env,
  write: Write = toStdout,
  git: GitLines = gitLinesIn(),
): Promise<number> {
  let o: Options;
  try {
    o = parse(argv, env);
  } catch (e) {
    if (!(e instanceof FlagError)) throw e;
    process.stderr.write(e.message ? `${e.message}\n${USAGE}` : USAGE);
    return e.code;
  }
  // Ein anderer Wert filterte alle Schritte weg und meldete grün, ohne etwas zu prüfen.
  if (!['', 'js', 'rust'].includes(o.only)) {
    write(`❌ check:diff: die Bahn ${o.only} gibt es nicht; gültige Werte für -only: js, rust\n`);
    return 1;
  }
  return check(o, write, git);
}
