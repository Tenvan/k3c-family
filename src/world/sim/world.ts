import { createRng } from '../../core/rng';
import type { BiomeConfig } from '../biome';
import { generateLevel } from '../levelGenerator';
import { newId } from './common';
import { cycleAt, globalDayNight } from './cycle';
import { BUILDINGS, ECONOMY, HUB, MONARCH, TROOPS } from './data';
import { payDawnIncome, stepPlayers, stepSites } from './economy';
import { removeDeadEnemies, sendEnemiesHome, stepEnemies, stepProjectiles, stepSpawns } from './enemies';
import { makeArcher, releaseJob, spawnVagrant, stepCamps, stepTroops } from './units';
import { hasDepth, stepTravel } from './travel';
import { planWave } from './waves';
import type { Player, PlayerCommand, Site, World } from './types';

/**
 * Die Simulation einer Stufe: createWorld() baut den Startzustand aus Biom + Seed, step() rechnet einen Tick.
 * Deterministisch: gleicher Seed + gleiche Eingaben => gleicher Zustand. Kein Phaser, kein Math.random().
 */

export interface WorldOptions {
  /** Beschleunigt den Tag/Nacht-Zyklus (Tests, Dev: ?fast=1) */
  cycleSpeed?: number;
  /** Startzeit (Sekunden), damit der globale Tag/Nacht-Zyklus beim Stufenwechsel weiterläuft */
  time?: number;
}

export function createWorld(biome: BiomeConfig, seed: string, options: WorldOptions = {}): World {
  const level = generateLevel(biome, seed);
  const hubX = level.hubCenterUnits;
  const time = options.time ?? 0;
  const cycleSpeed = options.cycleSpeed ?? 1;
  const w: World = {
    seed,
    biome,
    level,
    rng: createRng(`${biome.id}:${seed}:sim`),
    time,
    cycleSpeed,
    nextId: 1,
    widthUnits: level.widthUnits,
    hubX,
    cycle: cycleAt(globalDayNight(), time * cycleSpeed),
    aggression: biome.cycle.type === 'aggressionPool' ? 0 : null,
    wave: 0,
    players: [],
    coins: [],
    troops: [],
    nodes: [],
    sites: [],
    castle: { id: 0, x: hubX, hp: BUILDINGS.castle.hp, maxHp: BUILDINGS.castle.hp },
    enemies: [],
    projectiles: [],
    pickups: [],
    camps: [],
    portals: [],
    spawnQueue: [],
    stock: { wood: 0, stone: 0, copper: 0 },
    skillPoints: 0,
    travel: null,
    events: [],
  };
  w.castle.id = newId(w);

  for (const e of level.entities) {
    if (ECONOMY.gatherables[e.kind]) {
      w.nodes.push({ id: newId(w), kind: e.kind, x: e.x, paidGold: 0, marked: false, workerId: null, progress: 0 });
    } else if (e.kind === 'chest' || e.kind === 'skillPoint') {
      w.pickups.push({ id: newId(w), kind: e.kind, x: e.x });
    } else if (e.kind === 'recruitCamp') {
      w.camps.push({ x: e.x, respawnIn: ECONOMY.recruitCamp.respawnSeconds });
      for (let i = 0; i < ECONOMY.recruitCamp.maxVagrants; i++) spawnVagrant(w, e.x, e.x + (i - 0.5) * 3);
    } else if (e.kind === 'portal') {
      w.portals.push(e.x);
    }
  }
  for (const s of HUB.sites) {
    if (biome.depth < (s.fromDepth ?? 0) || (s.needsDeeper && !hasDepth(biome.depth + 1))) continue;
    w.sites.push(emptySite(w, s.kind, hubX + s.offsetUnits));
  }
  for (let i = 0; i < (HUB.startTroops.peasant ?? 0); i++) Object.assign(spawnVagrant(w, hubX, hubX + 2 + i), { kind: 'peasant', hp: TROOPS.peasant.hp, maxHp: TROOPS.peasant.hp });
  for (let i = 0; i < (HUB.startTroops.archer ?? 0); i++) makeArcher(w, spawnVagrant(w, hubX));
  return w;
}

function emptySite(w: World, kind: Site['kind'], x: number): Site {
  return { id: newId(w), kind, x, state: 'unpaid', paidGold: 0, buildProgress: 0, hp: 0, maxHp: BUILDINGS[kind].hp, workerId: null, bows: 0, bowPaidGold: 0 };
}

/** Couch-Koop: neuer Monarch an der Burg. */
export function addPlayer(w: World): Player {
  const index = w.players.length;
  const p: Player = {
    id: newId(w),
    index,
    x: w.hubX + (index % 2 === 0 ? -3 : 3),
    vx: 0,
    facing: index % 2 === 0 ? -1 : 1,
    gold: ECONOMY.purse.startGold,
    hp: MONARCH.base.hp,
    maxHp: MONARCH.base.hp,
    respawnIn: 0,
    // Der Beitritts-Tastendruck soll nicht gleich eine Münze ausgeben.
    payCooldown: 0.5,
    paying: false,
    payKey: null,
    payAmount: 0,
  };
  w.players.push(p);
  return p;
}

/** Ein Simulationsschritt. `commands[i]` gehört zu `players[i]`. */
export function step(w: World, commands: readonly PlayerCommand[], dt: number): void {
  w.events = [];
  w.time += dt;
  stepCycle(w, dt);
  stepSpawns(w);
  stepPlayers(w, commands, dt);
  stepCamps(w, dt);
  stepSites(w);
  stepTroops(w, dt);
  stepEnemies(w, dt);
  stepProjectiles(w, dt);
  removeDeadEnemies(w);
  w.troops = w.troops.filter((t) => {
    if (t.hp > 0) return true;
    releaseJob(w, t);
    return false;
  });
  if (w.castle.hp <= 0) castleFallen(w);
  stepTravel(w, dt);
}

function stepCycle(w: World, dt: number): void {
  const before = w.cycle;
  w.cycle = cycleAt(globalDayNight(), w.time * w.cycleSpeed);
  const changed = before.phase !== w.cycle.phase;
  const dayNight = w.biome.cycle.type === 'dayNight';

  if (changed && w.cycle.phase === 'dusk') w.events.push({ type: 'dusk' });
  if (changed && w.cycle.phase === 'night') {
    w.events.push({ type: 'night', day: w.cycle.day });
    if (dayNight) startWave(w);
  }
  if (changed && w.cycle.phase === 'day') {
    w.events.push({ type: 'dawn', day: w.cycle.day });
    if (dayNight) sendEnemiesHome(w);
    payDawnIncome(w);
  }

  if (w.aggression !== null && w.biome.cycle.type === 'aggressionPool') {
    w.aggression = Math.min(100, w.aggression + (w.biome.cycle.percentPerMinute / 60) * dt * w.cycleSpeed);
    if (w.aggression >= 100) {
      w.aggression = 0;
      startWave(w);
    }
  }
}

export function startWave(w: World): void {
  w.wave++;
  const plan = planWave(w.biome, w.wave, w.rng, w.portals, w.cycle.phase === 'night');
  for (const s of plan) w.spawnQueue.push({ kind: s.kind, x: s.x, at: w.time + s.delay });
  w.events.push({ type: 'wave', wave: w.wave, count: plan.length });
}

/**
 * Niederlage laut GDD: Respawn am Hub, Gebäude bleiben zerstört, 50% der Ressourcen und alle Truppen verloren.
 * Die Burg selbst steht danach wieder (sonst wäre das Spiel vorbei).
 */
function castleFallen(w: World): void {
  w.events.push({ type: 'castleFallen' });
  w.castle.hp = w.castle.maxHp;
  for (const s of w.sites) Object.assign(s, emptySite(w, s.kind, s.x), { id: s.id });
  for (const r of ['wood', 'stone', 'copper'] as const) w.stock[r] = Math.floor(w.stock[r] / 2);
  for (const p of w.players) {
    p.gold = Math.floor(p.gold / 2);
    Object.assign(p, { respawnIn: 0, hp: p.maxHp, vx: 0, x: w.hubX + (p.index % 2 === 0 ? -3 : 3) });
  }
  for (const n of w.nodes) n.workerId = null;
  w.troops = w.troops.filter((t) => t.kind === 'vagrant');
  w.enemies = [];
  w.spawnQueue = [];
  w.projectiles = [];
}
