// Auswahl und Schritte ohne Repo; portiert aus select_test.go (die Zeilen für Go-Pakete entfallen mit der Bahn go).

import { describe, expect, it } from 'vitest';
import { classify, emptyPlan, type Plan } from './select';
import { describe as line, jsSteps, rustSteps, type Step } from './steps';

/** Gesetzte Schalter: D Docs, R Rules, C Client, W WbVitest, S StudioVitest. */
function flags(p: Plan): string {
  const on: [boolean, string][] = [
    [p.docs, 'D'],
    [p.rules, 'R'],
    [p.client, 'C'],
    [p.wbVitest, 'W'],
    [p.studioVitest, 'S'],
  ];
  return on.filter(([set]) => set).map(([, name]) => name).join('');
}

const names = (steps: Step[]): string => steps.map((s) => s.name).join(' ');
const cmd = (s: Step): string => s.cmd.join(' ');
const rust = (...files: string[]): Step[] => rustSteps(classify(files));

describe('classify', () => {
  it.each([
    ['nur Doku: nur Planungs-Tests', ['docs/backlog/B-001-x.md', 'CLAUDE.md'], 'D'],
    ['Golden an der Wurzel: nur Regeln (RU41)', ['testdata/golden/sim-1.json.gz'], 'R'],
    ['data zählt als Client', ['data/enemies.json'], 'RC'],
    ['Taskfile und CI lösen keine Tests aus', ['Taskfile.yml', 'engine/Taskfile.yml', '.github/workflows/ci.yml'], 'R'],
    ['Workbench-Frontend: nur Vitest', ['tools/k3c-workbench/internal/planning/next.go', 'tools/k3c-workbench/frontend/src/App.tsx'], 'RW'],
    ['webkit: Vitest beider Apps', ['tools/webkit/atoms/Button.tsx'], 'RWS'],
    ['Studio: Frontend nur Vitest des Studios', ['tools/k3c-studio/frontend/src/App.tsx'], 'RS'],
    ['Daten in Unterordnern', ['data/biomes/wald.json', 'engine/balance/report/chart.json'], 'RC'],
    ['Lockfile zählt als Client', ['bun.lock'], 'RC'],
    ['checkdiff prüft sich selbst', ['tools/checkdiff/run.ts'], 'RC'],
    ['cleanup bekommt Lint, Typecheck und Vitest', ['tools/cleanup/cleanup.ts'], 'RC'],
    ['sync zählt als Client', ['tools/sync/sync.ts'], 'RC'],
    ['rustpaket zählt als Client', ['tools/rustpaket/pruefen.ts'], 'RC'],
    ['ruletable zählt als Client', ['tools/ruletable/table.ts'], 'RC'],
    ['soundatlas zählt als Client', ['tools/soundatlas/atlas.ts'], 'RC'],
    ['Wurzel: Tests und Konfiguration', ['tests/planning.test.ts', 'vitest.config.ts'], 'RC'],
  ])('%s', (_name, files, want) => {
    expect(flags(classify(files))).toBe(want);
  });
});

describe('jsSteps', () => {
  it('nur Doku: nur die Planungs-Tests', () => {
    const got = jsSteps({ ...emptyPlan(), docs: true }, 'HEAD');
    expect(names(got)).toBe('regeln');
    expect(cmd(got[0])).not.toContain('projectRules');
  });

  it('Client: Lint und Typecheck in Stufe 1, Vitest in Stufe 2', () => {
    const got = jsSteps({ ...emptyPlan(), rules: true, client: true }, 'HEAD');
    expect(names(got)).toBe('regeln lint typecheck vitest');
    expect(got[3].stage).toBe(2);
    expect(cmd(got[3])).toBe('bunx vitest run --changed HEAD --passWithNoTests --exclude tools/paper/golden/**');
  });

  it('Frontends: Vitest über task mit der Basis als einem Argument', () => {
    const got = jsSteps({ ...emptyPlan(), wbVitest: true, studioVitest: true }, 'abc');
    expect(got.map((s) => s.cmd)).toEqual([
      ['task', 'k3c-workbench:vitest', 'VITEST_ARGS=--changed abc --passWithNoTests'],
      ['task', 'k3c-studio:vitest', 'VITEST_ARGS=--changed abc --passWithNoTests'],
    ]);
  });
});

describe('rustSteps', () => {
  it('eine Crate: nur die geänderten Crates (RU1.2)', () => {
    const got = rust('rust/crates/k3c-sim/src/lib.rs', 'rust/crates/k3c-rng/Cargo.toml');
    expect(names(got)).toBe('rust-fmt rust-lint rust-test');
    expect([got[0].dir, got[0].stage, got[2].stage]).toEqual(['rust', 1, 2]);
    expect(cmd(got[1])).toBe('cargo clippy -p k3c-rng -p k3c-sim --all-targets -- -D warnings');
    expect(cmd(got[2])).toBe('cargo test -p k3c-rng -p k3c-sim');
    expect(line(got[0])).toBe('rust-fmt: cargo fmt --all --check (in rust)');
  });

  it.each(['rust/Cargo.toml', 'rust/Cargo.lock', 'rust/clippy.toml'])('%s wählt den ganzen Workspace', (file) => {
    const got = rust(file);
    expect(names(got)).toBe('rust-fmt rust-lint rust-test');
    expect(cmd(got[1])).toBe('cargo clippy --workspace --all-targets -- -D warnings');
    expect(cmd(got[2])).toBe('cargo test --workspace');
  });

  it.each([[['engine/sim/world.go']], [['rust/spike/src/main.rs', 'rust/Taskfile.yml']], [['docs/standards/rust-port.md']]])(
    'kein cargo für %j',
    (files) => {
      expect(rustSteps(classify(files))).toEqual([]);
    },
  );
});

describe('rustSteps der Werkzeuge (RU66.1)', () => {
  it('eine Crate in rust/tools', () => {
    const got = rust('rust/tools/crates/k3c-kit/src/lib.rs');
    expect(names(got)).toBe('tools-rust-fmt tools-rust-lint tools-rust-test');
    expect([got[0].dir, got[2].dir, got[0].lane, got[1].stage, got[2].stage]).toEqual(['rust/tools', 'rust/tools', 'rust', 1, 2]);
    expect(cmd(got[1])).toBe('cargo clippy -p k3c-kit --all-targets -- -D warnings');
    expect(cmd(got[2])).toBe('cargo test -p k3c-kit');
  });

  it.each(['rust/tools/Cargo.lock', 'rust/tools/Cargo.toml', 'rust/tools/clippy.toml'])('%s wählt den ganzen Workspace', (file) => {
    const got = rust(file);
    expect(names(got)).toBe('tools-rust-fmt tools-rust-lint tools-rust-test');
    expect(cmd(got[2])).toBe('cargo test --workspace');
  });

  it.each(['rust/tools/spike/src/main.rs', 'rust/tools/Taskfile.yml'])('%s: kein cargo', (file) => {
    expect(rust(file)).toEqual([]);
  });

  it('geerbte Toolchain wählt beide Workspaces', () => {
    const got = rust('rust/rust-toolchain.toml');
    expect(names(got)).toBe('rust-fmt rust-lint rust-test tools-rust-fmt tools-rust-lint tools-rust-test');
    expect(cmd(got[5])).toBe('cargo test --workspace');
    expect(names(rust('rust/Cargo.toml'))).toBe('rust-fmt rust-lint rust-test');
  });
});
