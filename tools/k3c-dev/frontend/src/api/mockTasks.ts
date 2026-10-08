import type { ConsoleLine, TaskCatalog, TaskGates, TaskGateState, TaskInfo, TaskNamespace, TaskRun } from './types';

// Erfundener Task-Katalog und Läufe für den Mock: Namen wie im Taskfile des Repos. `task:test` läuft ~3 s und endet
// grün, `lint` endet rot, `dev` läuft bis zum Stopp (Dauerläufer). Die Ausgabe liegt unter `task:<name>` wie in Go.

const ESC = '\u001b[';
const CAPACITY = 2000;

const t = (name: string, desc: string, file = 'Taskfile.yml', line = 1): TaskInfo => {
  const i = name.indexOf(':');
  return { name, namespace: i > 0 ? name.slice(0, i) : 'Workspace', leaf: i > 0 ? name.slice(i + 1) : name, desc, summary: '',
    aliases: [], file, line };
};

const TASKS: TaskInfo[] = [
  t('check', 'Lint, Typecheck und Tests (vor jedem Abschluss)', 'Taskfile.yml', 40),
  t('test', 'Vitest (Level-Generator, reine Logik)', 'Taskfile.yml', 52),
  t('lint', 'Oxlint über src/ und tests/', 'Taskfile.yml', 58),
  t('build', 'Typecheck + Produktions-Build nach dist/', 'Taskfile.yml', 66),
  t('dev', 'Dev-Server (auch im LAN erreichbar, Port 5173)', 'Taskfile.yml', 74),
  t('serve', 'Build + Go-Server (Port 8080)', 'Taskfile.yml', 82),
  t('check:go', 'go test + golangci-lint', 'Taskfile.yml', 96),
  t('check:dev', 'k3c-dev: Frontend, go test, golangci-lint', 'Taskfile.yml', 124),
  t('check:all', 'Alles inklusive Build', 'Taskfile.yml', 150),
  t('dev:frontend', 'Frontend von k3c-dev bauen', 'Taskfile.yml', 110),
  t('k3c-dev', 'Entwickler-Werkzeug k3c-dev als Fenster starten', 'Taskfile.yml', 132),
  t('k3c-dev:build', 'EXE von k3c-dev bauen', 'Taskfile.yml', 141),
];

function group(tasks: TaskInfo[]): TaskNamespace[] {
  const by = new Map<string, TaskInfo[]>();
  for (const x of tasks) by.set(x.namespace, [...(by.get(x.namespace) ?? []), x]);
  return [...by.entries()]
    .sort(([a], [b]) => (a === 'Workspace' ? -1 : b === 'Workspace' ? 1 : a < b ? -1 : 1))
    .map(([name, ts]) => ({ name, tasks: ts.sort((a, b) => (a.name < b.name ? -1 : 1)) }));
}

const SCRIPTS: Record<string, string[]> = {
  test: [
    `${ESC}2mtask: [test] npx vitest run${ESC}0m`,
    ` ${ESC}32m✓${ESC}0m src/world/levelGenerator.test.ts ${ESC}2m(500 tests)${ESC}0m`,
    ` ${ESC}32m✓${ESC}0m tests/planning.test.ts ${ESC}2m(34 tests)${ESC}0m`,
    ` ${ESC}1m${ESC}42m PASS ${ESC}0m Tests ${ESC}1m${ESC}32m175 passed${ESC}0m`,
  ],
  lint: [
    `${ESC}2mtask: [lint] npx oxlint${ESC}0m`,
    `${ESC}33m!${ESC}0m src/scenes/GameScene.ts:212 Funktion überschreitet 60 Zeilen`,
    `${ESC}31m×${ESC}0m src/online/clientConnection.ts:88 Verschachtelung 5 > 4`,
    `Found 1 error and 1 warning`,
  ],
  dev: [
    `${ESC}2mtask: [dev] npx vite${ESC}0m`,
    `${ESC}32m➜${ESC}0m  ${ESC}1mLocal${ESC}0m:   ${ESC}36mhttp://localhost:5173/${ESC}0m`,
  ],
};

/** Freigabe-Schloss wie in Go (taskgate): Datei plus Laufzeit-Schalter bis zum Beenden. */
export function mockGates() {
  const file = new Set(['check', 'test', 'lint', 'build', 'check:go', 'check:dev']);
  const runtime = new Map<string, boolean>();
  const state = (name: string): TaskGateState => {
    const set = runtime.get(name);
    if (set === undefined) return file.has(name) ? 'open' : 'closed';
    return set ? 'open-temp' : 'closed-temp';
  };
  const gates = (): TaskGates => ({ states: Object.fromEntries(TASKS.map((x) => [x.name, state(x.name)])), pending: runtime.size });
  return {
    taskGates: async () => gates(),
    taskGateToggle: async (name: string) => {
      const next = !state(name).startsWith('open');
      if (next === file.has(name)) runtime.delete(name);
      else runtime.set(name, next);
      return gates();
    },
    taskGateSave: async () => {
      for (const [name, on] of runtime) {
        if (on) file.add(name);
        else file.delete(name);
      }
      runtime.clear();
      return gates();
    },
  };
}

export function mockTasks(emit: (event: 'task:state', run: TaskRun) => void, emitLines: (l: ConsoleLine[]) => void) {
  let catalog: TaskCatalog = { namespaces: group(TASKS), count: TASKS.length, loadedAt: new Date().toISOString(), error: '' };
  const runs = new Map<string, TaskRun>();
  const buffers = new Map<string, ConsoleLine[]>();
  const timers = new Map<string, ReturnType<typeof setTimeout>[]>();
  const src = (name: string) => `task:${name}`;

  const add = (name: string, text: string, stream = 'stdout') => {
    const buf = buffers.get(src(name)) ?? [];
    const line: ConsoleLine = { source: src(name), stream, text, seq: (buf.at(-1)?.seq ?? 0) + 1 };
    buffers.set(src(name), [...buf, line].slice(-CAPACITY));
    emitLines([line]);
  };
  const update = (name: string, patch: Partial<TaskRun>) => {
    const run = { ...runs.get(name)!, ...patch };
    runs.set(name, run);
    emit('task:state', run);
    return run;
  };
  const finish = (name: string, state: TaskRun['state'], exitCode: number) => {
    const run = runs.get(name)!;
    update(name, { state, exitCode, endedAt: new Date().toISOString(),
      durationMs: Date.now() - new Date(run.startedAt).getTime() });
  };
  const later = (name: string, ms: number, fn: () => void) =>
    timers.set(name, [...(timers.get(name) ?? []), setTimeout(fn, ms)]);

  return {
    tasks: async () => catalog,
    tasksReload: async () => {
      await new Promise((r) => setTimeout(r, 400));
      catalog = { ...catalog, loadedAt: new Date().toISOString() };
      return catalog;
    },
    taskRuns: async () => [...runs.values()].sort((a, b) => (a.name < b.name ? -1 : 1)),
    taskStart: async (name: string, args: string[]): Promise<TaskRun> => {
      if (!TASKS.some((x) => x.name === name)) throw new Error(`Task "${name}" steht nicht im Katalog (task --list-all)`);
      if (runs.get(name)?.state === 'running') throw new Error(`Task läuft bereits: ${name}`);
      if (args.some((a) => /[&|<>^"%`;,]/.test(a))) throw new Error(`Argument "${args.find((a) => /[&|<>^"%`;,]/.test(a))}" enthält ein nicht erlaubtes Zeichen - abgelehnt statt bereinigt`);
      buffers.set(src(name), []);
      const run: TaskRun = { name, state: 'running', pid: 40000 + Math.floor(Math.random() * 9000), args,
        startedAt: new Date().toISOString(), endedAt: '', exitCode: 0, durationMs: 0, reason: '' };
      runs.set(name, run);
      emit('task:state', run);
      const lines = SCRIPTS[name] ?? [`${ESC}2mtask: [${name}]${ESC}0m`, 'fertig'];
      lines.forEach((text, i) => later(name, 300 + i * 600, () => add(name, text)));
      if (name !== 'dev') {
        later(name, 300 + lines.length * 600, () => finish(name, name === 'lint' ? 'failed' : 'succeeded', name === 'lint' ? 1 : 0));
      }
      return run;
    },
    taskStop: async (name: string): Promise<TaskRun> => {
      if (runs.get(name)?.state !== 'running') throw new Error(`Task läuft nicht: ${name}`);
      (timers.get(name) ?? []).forEach(clearTimeout);
      timers.delete(name);
      finish(name, 'cancelled', 1);
      return runs.get(name)!;
    },
    taskKnows: (source: string) => buffers.has(source),
    taskTail: (source: string) => buffers.get(source) ?? [],
  };
}
