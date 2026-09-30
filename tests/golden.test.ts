// Golden-Daten für den Go-Port (SP04, B-043): TS zeichnet auf, was RNG, Level-Generator und Simulation heute liefern.
// `npm test` prüft, dass TS die Dateien in testdata/golden/ unverändert erzeugt; `npm run golden` schreibt sie neu.
// Go liest dieselben Dateien und muss sie exakt nachbilden (engine/rng, engine/level, später engine/sim).
import { describe, expect, it } from 'vitest';
import { createRng, hashSeed } from '../src/core/rng';
import { BIOMES, type BiomeConfig } from '../src/world/biome';
import { generateLevel, validateLevel } from '../src/world/levelGenerator';
import { addPlayer, createWorld, step } from '../src/world/sim/world';
import { IDLE, type PlayerCommand, type World } from '../src/world/sim/types';

const DIR = '../testdata/golden';

/** JSON.stringify, aber NaN/Infinity (würden zu null) brechen ab, damit die Golden-Daten nichts verschlucken. */
const json = (value: unknown) =>
  JSON.stringify(value, (key, v: unknown) => {
    if (typeof v === 'number' && !Number.isFinite(v)) throw new Error(`Nicht endliche Zahl in "${key}"`);
    return v;
  });

/** Kopf mit einem Feld pro Zeile, danach die Liste `key` mit einem Eintrag pro Zeile (lesbare Diffs). */
function jsonFile(head: Record<string, unknown>, key: string, items: unknown[]): string {
  const fields = Object.entries(head).map(([k, v]) => `  ${json(k)}: ${json(v)},\n`);
  return `{\n${fields.join('')}  ${json(key)}: [\n${items.map((i) => `    ${json(i)}`).join(',\n')}\n  ]\n}\n`;
}

const RNG_SEEDS = ['', 'a', 'familie-2026', 'Käse🧀', 'Ärger über Öl', '日本', '👨‍👩‍👧‍👦', 'forest:Käse🧀', 'k3c-'.repeat(30)];
const WEIGHTS: [string, number][] = [['a', 1], ['b', 3], ['c', 6]];
const PICK = ['x', 'y', 'z'];

function rngEntry(seed: string) {
  const times = <T>(n: number, f: () => T) => Array.from({ length: n }, f);
  const next = createRng(seed);
  const int = createRng(seed);
  const weighted = createRng(seed);
  const pick = createRng(seed);
  return {
    seed,
    hash: hashSeed(seed),
    next: times(50, () => next.next()),
    int: times(20, () => int.int(0, 9)),
    weighted: times(10, () => weighted.weighted(Object.fromEntries(WEIGHTS))),
    shuffle: createRng(seed).shuffle(Array.from({ length: 10 }, (_, i) => i)),
    pick: times(10, () => pick.pick(PICK)),
  };
}

const LEVEL_SEEDS = [
  ...Array.from({ length: 10 }, (_, i) => String(i)),
  'Käse🧀', 'familie-2026', 'Ärger', '日本', '👨‍👩‍👧‍👦', 'a', 'golden-1', 'golden-2', 'x'.repeat(50), 'k3c',
];

const DT = 1 / 30;
const SNAPSHOT_EVERY = 30;
const cmd = (moveX: number, sprint = false, pay = false): PlayerCommand => ({ moveX, sprint, pay });
const PAY = cmd(0, false, true);

/** Eingabe-Skript für zwei Spieler; wiederholt, bis der Lauf voll ist. Ein Spieler nimmt die erste Spalte. */
const SCRIPT: { ticks: number; commands: PlayerCommand[] }[] = [
  { ticks: 78, commands: [cmd(-1), cmd(1)] },
  { ticks: 120, commands: [PAY, cmd(1)] },
  { ticks: 90, commands: [cmd(-1, true), PAY] },
  { ticks: 60, commands: [PAY, PAY] },
  { ticks: 150, commands: [cmd(-1, true), cmd(1, true)] },
  { ticks: 60, commands: [PAY, PAY] },
  { ticks: 120, commands: [cmd(1), cmd(-0.5)] },
  { ticks: 45, commands: [IDLE, PAY] },
];

interface SimRun {
  name: string;
  biome: string;
  seed: string;
  cycleSpeed: number;
  players: number;
  ticks: number;
  /** `ohne Gegner`: in keinem Tick ein Gegner (Grundlage für SP05); `mit Welle`: am Ende lief mindestens eine Welle. */
  expect: 'ohne Gegner' | 'mit Welle';
}

const SIM_RUNS: SimRun[] = [
  {
    name: 'forest-tag', biome: 'forest', seed: 'golden-1', cycleSpeed: 1, players: 2, ticks: 1800, expect: 'ohne Gegner',
  },
  {
    name: 'forest-nacht', biome: 'forest', seed: 'golden-2', cycleSpeed: 20, players: 2, ticks: 2700, expect: 'mit Welle',
  },
  {
    name: 'cave-aggression', biome: 'cave', seed: 'Käse🧀', cycleSpeed: 100, players: 1, ticks: 2700, expect: 'mit Welle',
  },
];

/** Segmente des Skripts, gekürzt auf `ticks` und auf die Spielerzahl. */
function inputs(ticks: number, players: number) {
  const segments: { ticks: number; commands: PlayerCommand[] }[] = [];
  for (let done = 0, i = 0; done < ticks; i++) {
    const s = SCRIPT[i % SCRIPT.length];
    const n = Math.min(s.ticks, ticks - done);
    segments.push({ ticks: n, commands: s.commands.slice(0, players) });
    done += n;
  }
  return segments;
}

const snapshot = (w: World) => JSON.parse(json({ ...w, rng: undefined, biome: undefined, level: undefined })) as World;

function simulate(run: SimRun, biome: BiomeConfig) {
  const w = createWorld(biome, run.seed, { cycleSpeed: run.cycleSpeed });
  for (let i = 0; i < run.players; i++) addPlayer(w);
  const segments = inputs(run.ticks, run.players);
  const snapshots = [{ tick: 0, world: snapshot(w) }];
  let tick = 0;
  for (const segment of segments) {
    for (let n = 0; n < segment.ticks; n++) {
      step(w, segment.commands, DT);
      tick++;
      if (run.expect === 'ohne Gegner') expect(w.enemies.length + w.spawnQueue.length, `Tick ${tick}`).toBe(0);
      if (tick % SNAPSHOT_EVERY === 0) snapshots.push({ tick, world: snapshot(w) });
    }
  }
  if (run.expect === 'mit Welle') expect(w.wave).toBeGreaterThan(0);
  return { segments, snapshots };
}

const biome = (id: string) => BIOMES.find((b) => b.id === id)!;

describe('Golden-Daten (testdata/golden/)', () => {
  it('rng.json', async () => {
    const head = { params: { int: [0, 9], weights: WEIGHTS, pick: PICK, shuffle: 10 } };
    await expect(jsonFile(head, 'seeds', RNG_SEEDS.map(rngEntry))).toMatchFileSnapshot(`${DIR}/rng.json`);
  });

  it.each(BIOMES.map((b) => b.id))('level-%s.json', async (id) => {
    const levels = LEVEL_SEEDS.map((seed) => generateLevel(biome(id), seed));
    for (const level of levels) expect(validateLevel(level, biome(id)), `Seed ${String(level.seed)}`).toEqual([]);
    await expect(jsonFile({ biome: id }, 'levels', levels)).toMatchFileSnapshot(`${DIR}/level-${id}.json`);
  });

  it.each(SIM_RUNS)('sim-$name.json', async (run) => {
    const { segments, snapshots } = simulate(run, biome(run.biome));
    const { name, biome: id, seed, cycleSpeed, players, ticks } = run;
    const head = { name, biome: id, seed, cycleSpeed, players, dt: DT, ticks, snapshotEvery: SNAPSHOT_EVERY, inputs: segments };
    await expect(jsonFile(head, 'snapshots', snapshots)).toMatchFileSnapshot(`${DIR}/sim-${run.name}.json`);
  });
});
