// Schritte: Aus der Auswahl werden Befehle. Stufe 1 sind die schnellen Prüfungen (Regel-Tests, Lint, Typecheck, cargo
// fmt und clippy), Stufe 2 die Tests; Stufe 2 startet nur nach grüner Stufe 1. Je Stufe laufen die Bahnen `js` und `rust`
// nebeneinander, die Schritte einer Bahn nacheinander. Befehle sind Argumentlisten, nie Shell-Zeilen.

import type { Plan, RustPlan } from './select';

export type Lane = 'js' | 'rust';

export interface Step {
  name: string;
  lane: Lane;
  /** 1 schnell, 2 Tests */
  stage: 1 | 2;
  /** Arbeitsordner relativ zur Repo-Wurzel */
  dir: string;
  cmd: string[];
}

/** Tests, die gerenderte Papierschnitt-Grafik brauchen (gen:paper läuft hier nicht, B-742). */
const VITEST_EXCLUDES = ['tools/paper/golden/**'];

const step = (name: string, lane: Lane, stage: 1 | 2, cmd: string[], dir = '.'): Step => ({ name, lane, stage, dir, cmd });

function clientSteps(base: string): Step[] {
  const changed = ['bunx', 'vitest', 'run', '--changed', base, '--passWithNoTests'];
  for (const x of VITEST_EXCLUDES) changed.push('--exclude', x);
  return [
    step('lint', 'js', 1, ['task', 'lint']),
    step('typecheck', 'js', 1, ['task', 'typecheck']),
    step('vitest', 'js', 2, changed),
  ];
}

export function jsSteps(p: Plan, base: string): Step[] {
  const out: Step[] = [];
  const scan: string[] = [];
  if (p.docs) scan.push('tests/planning.test.ts', 'tests/planningProjects.test.ts');
  if (p.rules) scan.push('tests/projectRules.test.ts');
  if (scan.length > 0) out.push(step('regeln', 'js', 1, ['bunx', 'vitest', 'run', ...scan]));
  if (p.client) out.push(...clientSteps(base));
  const args = `VITEST_ARGS=--changed ${base} --passWithNoTests`;
  if (p.wbVitest) out.push(step('wb-vitest', 'js', 2, ['task', 'k3c-workbench:vitest', args]));
  if (p.studioVitest) out.push(step('studio-vitest', 'js', 2, ['task', 'k3c-studio:vitest', args]));
  return out;
}

/** Die drei Schritte eines Workspace mit Namen <prefix>-fmt|lint|test im Ordner dir. */
function rustWorkspaceSteps(rp: RustPlan | undefined, prefix: string, dir: string): Step[] {
  if (!rp || (!rp.all && rp.crates.size === 0)) return [];
  const scope = rp.all ? ['--workspace'] : [...rp.crates].sort().flatMap((c) => ['-p', c]);
  return [
    step(`${prefix}-fmt`, 'rust', 1, ['cargo', 'fmt', '--all', '--check'], dir),
    step(`${prefix}-lint`, 'rust', 1, ['cargo', 'clippy', ...scope, '--all-targets', '--', '-D', 'warnings'], dir),
    step(`${prefix}-test`, 'rust', 2, ['cargo', 'test', ...scope], dir),
  ];
}

/** Je Workspace fmt und clippy in Stufe 1, Tests in Stufe 2, alle in der Bahn rust (`-only rust` nimmt beide). */
export function rustSteps(p: Plan): Step[] {
  return [
    ...rustWorkspaceSteps(p.rust, 'rust', 'rust'),
    ...rustWorkspaceSteps(p.rustTools, 'tools-rust', 'rust/tools'),
  ];
}

export function describe(s: Step): string {
  const dir = s.dir === '.' ? '' : ` (in ${s.dir})`;
  return `${s.name}: ${s.cmd.join(' ')}${dir}`;
}
