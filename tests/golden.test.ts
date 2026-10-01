// Golden-Daten für den Go-Port (SP04, B-043): TS zeichnet auf, was RNG, Level-Generator und Simulation heute liefern.
// `npm test` prüft, dass TS die Dateien in testdata/golden/ unverändert erzeugt; `npm run golden` schreibt sie neu.
// Go liest dieselben Dateien und muss sie exakt nachbilden (engine/rng, engine/level, später engine/sim).
import { describe, expect, it } from 'vitest';
import { createRng, hashSeed } from '../src/core/rng';
import { BIOMES, type BiomeConfig } from '../src/world/biome';
import { generateLevel, validateLevel } from '../src/world/levelGenerator';
import { createCampaign, currentWorld, fromSave, joinPlayer, toSave, travel, type SaveGame } from '../src/world/sim/campaign';
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
  /** Abstand der Snapshots, Standard SNAPSHOT_EVERY (lange Läufe: 60) */
  snapshotEvery?: number;
  /** Start-Vorrat nach createWorld und addPlayer, damit Bauen ohne langes Sammeln erreichbar ist (SP06.3) */
  setup?: { stock: World['stock'] };
  /** Eingabe je Tick statt SCRIPT; aufgezeichnet als Segmente, Go spielt sie nur ab */
  bot?: (w: World, tick: number) => PlayerCommand[];
}

const GO = (dx: number): PlayerCommand => cmd(Math.sign(dx), Math.abs(dx) > 10);

/**
 * Ziel eines Aufbau-Spielers: Bauplätze seiner Seite bezahlen (ohne Treppen). Spieler 1 markiert danach Bäume,
 * Spieler 2 rekrutiert zwei Landstreicher und holt eine Truhe; beide kaufen Bögen bis zu 4 Bogenschützen.
 */
function builderGoal(w: World, p: World['players'][number], tick: number): { x: number; pay: boolean } | null {
  const side = p.index === 0 ? -1 : 1;
  const mine = w.sites.filter((s) => Math.sign(s.x - w.hubX) === side && !s.kind.startsWith('stairs'));
  // Spieler 2 bricht die erste Zahlung an seiner Mauer ab (Rückgabe der Münzen).
  const wall = mine.find((s) => s.kind === 'wall');
  if (side > 0 && wall && tick < 300) return { x: wall.x, pay: tick < 200 };
  const unpaid = mine.find((s) => s.state === 'unpaid');
  if (unpaid) return { x: unpaid.x, pay: true };
  const tree = w.nodes.filter((n) => !n.marked && n.kind === 'tree').sort((a, b) => Math.abs(a.x - p.x) - Math.abs(b.x - p.x))[0];
  if (side < 0 && tree && w.nodes.filter((n) => n.marked).length < 3) return { x: tree.x, pay: true };
  const vagrant = w.troops.find((t) => t.kind === 'vagrant');
  if (side > 0 && vagrant && w.troops.filter((t) => t.kind === 'peasant').length < 3) return { x: vagrant.x, pay: true };
  const shop = w.sites.find((s) => s.kind === 'workshop' && s.state === 'built');
  if (shop && shop.bows + w.troops.filter((t) => t.kind === 'archer').length < 4) return { x: shop.x, pay: true };
  const chest = w.pickups.find((pk) => pk.kind === 'chest');
  return side > 0 && chest ? { x: chest.x, pay: false } : null;
}

/** Beide Spieler stellen sich mit vollem Beutel an je ein Portal (Gegner klauen Gold, Monarchen fallen). */
function raider(w: World): PlayerCommand[] {
  return w.players.map((p) => {
    const dx = w.portals[p.index % 2 === 0 ? 0 : w.portals.length - 1] - p.x;
    return Math.abs(dx) > 0.5 ? GO(dx) : IDLE;
  });
}

function builder(w: World, tick: number): PlayerCommand[] {
  return w.players.map((p) => {
    const goal = builderGoal(w, p, tick);
    if (!goal) return IDLE;
    const dx = goal.x - p.x;
    return Math.abs(dx) > 0.5 ? GO(dx) : cmd(0, false, goal.pay);
  });
}

const SIM_RUNS: SimRun[] = [
  {
    name: 'forest-tag', biome: 'forest', seed: 'golden-1', cycleSpeed: 1, players: 2, ticks: 1800, expect: 'ohne Gegner',
  },
  {
    name: 'forest-ohne-spieler', biome: 'forest', seed: 'golden-1', cycleSpeed: 1, players: 0, ticks: 1800, expect: 'ohne Gegner',
  },
  {
    name: 'forest-nacht', biome: 'forest', seed: 'golden-2', cycleSpeed: 20, players: 2, ticks: 2700, expect: 'mit Welle',
  },
  {
    name: 'cave-aggression', biome: 'cave', seed: 'Käse🧀', cycleSpeed: 100, players: 1, ticks: 2700, expect: 'mit Welle',
  },
  // SP06.3: Läufe für die Abdeckung der Go-Simulation (B-074)
  {
    name: 'forest-aufbau', biome: 'forest', seed: 'golden-4', cycleSpeed: 5, players: 2, ticks: 6000, expect: 'mit Welle',
    snapshotEvery: 60, setup: { stock: { wood: 600, stone: 0, copper: 0 } }, bot: builder,
  },
  {
    name: 'cave-aufbau', biome: 'cave', seed: 'golden-8', cycleSpeed: 50, players: 2, ticks: 6000, expect: 'mit Welle',
    snapshotEvery: 60, setup: { stock: { wood: 600, stone: 0, copper: 0 } }, bot: builder,
  },
  {
    name: 'forest-raub', biome: 'forest', seed: 'golden-9', cycleSpeed: 20, players: 2, ticks: 1800, expect: 'mit Welle',
    snapshotEvery: 60, bot: raider,
  },
  {
    name: 'forest-sturm', biome: 'forest', seed: 'golden-5', cycleSpeed: 100, players: 2, ticks: 4500, expect: 'mit Welle',
    snapshotEvery: 60,
  },
  {
    name: 'cave-belagerung', biome: 'cave', seed: 'golden-6', cycleSpeed: 100, players: 1, ticks: 6000, expect: 'mit Welle',
    snapshotEvery: 60,
  },
  {
    name: 'mine-welle', biome: 'mine', seed: 'golden-7', cycleSpeed: 100, players: 2, ticks: 3000, expect: 'mit Welle',
    snapshotEvery: 60,
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
  if (run.setup) w.stock = { ...run.setup.stock };
  const every = run.snapshotEvery ?? SNAPSHOT_EVERY;
  const script = inputs(run.ticks, run.players);
  const segments: typeof script = [];
  const snapshots = [{ tick: 0, world: snapshot(w) }];
  for (let tick = 1, i = 0, n = 0; tick <= run.ticks; tick++) {
    let commands = script[i].commands;
    if (++n === script[i].ticks) (i++, (n = 0));
    if (run.bot) commands = run.bot(w, tick);
    const last = segments.at(-1);
    if (last && json(last.commands) === json(commands)) last.ticks++;
    else segments.push({ ticks: 1, commands });
    step(w, commands, DT);
    if (run.expect === 'ohne Gegner') expect(w.enemies.length + w.spawnQueue.length, `Tick ${tick}`).toBe(0);
    if (tick % every === 0) snapshots.push({ tick, world: snapshot(w) });
  }
  if (run.expect === 'mit Welle') expect(w.wave).toBeGreaterThan(0);
  return { segments: run.bot ? segments : script, snapshots, every };
}

const biome = (id: string) => BIOMES.find((b) => b.id === id)!;

/** Kampagnen-Lauf (SP06.2): 2 Spieler steigen über die Tiefen-Eingänge bis mine ab, in cave wird gespeichert und geladen. */
const CAMPAIGN = { name: 'campaign-abstieg', seed: 'golden-3', cycleSpeed: 1, players: 2, savedAt: '2026-01-01T00:00:00.000Z' };

/** Beide Spieler sprinten zum Tiefen-Eingang der aktuellen Welt und warten dort. */
function towardExit(w: World): PlayerCommand[] {
  const exit = w.level.entities.find((e) => e.kind === 'exit');
  return w.players.map((p) => (!exit || Math.abs(exit.x - p.x) <= 1 ? IDLE : cmd(Math.sign(exit.x - p.x), true)));
}

function campaignRun() {
  const { seed, cycleSpeed, players, savedAt } = CAMPAIGN;
  let c = createCampaign(seed, { id: 'golden', cycleSpeed });
  for (let i = 0; i < players; i++) joinPlayer(c);
  const segments: { ticks: number; commands: PlayerCommand[] }[] = [];
  const snap = (tick: number) => ({ tick, depth: c.depth, unlockedDepth: c.unlockedDepth, world: snapshot(currentWorld(c)) });
  const snapshots = [snap(0)];
  let tick = 0, saveAt = -1, inMine = -1;
  let save: SaveGame | null = null;
  while (inMine < 0 || tick < inMine + 300) {
    const w = currentWorld(c);
    const commands = towardExit(w);
    const last = segments.at(-1);
    if (last && json(last.commands) === json(commands)) last.ticks++;
    else segments.push({ ticks: 1, commands });
    step(w, commands, DT);
    tick++;
    if (w.travel && w.travel.progress >= 1) {
      travel(c, w.travel.toDepth);
      if (c.depth === 1 && saveAt < 0) saveAt = tick + 60;
      if (c.depth === 2) inMine = tick;
    }
    if (tick === saveAt) {
      save = JSON.parse(json(toSave(c, savedAt))) as SaveGame;
      c = fromSave(JSON.parse(json(save)) as SaveGame, cycleSpeed);
      for (let i = 0; i < players; i++) joinPlayer(c);
    }
    if (tick % SNAPSHOT_EVERY === 0) snapshots.push(snap(tick));
    expect(tick, 'Kampagne erreicht mine nicht').toBeLessThan(20000);
  }
  expect(save).not.toBeNull();
  return { segments, snapshots, saveAt, save, ticks: tick };
}

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
    const { segments, snapshots, every } = simulate(run, biome(run.biome));
    const { name, biome: id, seed, cycleSpeed, players, ticks, setup } = run;
    const head = { name, biome: id, seed, cycleSpeed, players, dt: DT, ticks, snapshotEvery: every, ...(setup && { setup }), inputs: segments };
    await expect(jsonFile(head, 'snapshots', snapshots)).toMatchFileSnapshot(`${DIR}/sim-${run.name}.json`);
  });

  it('campaign-abstieg.json', async () => {
    const { segments, snapshots, saveAt, save, ticks } = campaignRun();
    const { name, seed, cycleSpeed, players, savedAt } = CAMPAIGN;
    const head = { name, seed, cycleSpeed, players, dt: DT, ticks, snapshotEvery: SNAPSHOT_EVERY, saveAt, savedAt, save, inputs: segments };
    await expect(jsonFile(head, 'snapshots', snapshots)).toMatchFileSnapshot(`${DIR}/${name}.json`);
  });
});
