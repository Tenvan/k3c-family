import { applyDamage, castleRadius, isAlive, newId } from './common';
import { ECONOMY, ENEMIES, WAVES, type ResourceKind } from '../../model/data';
import { scatterCoins } from './economy';
import { isOnTower } from './units';
import type { Enemy, Site, World } from '../../model/types';

/**
 * Gegner laufen vom Portal geradeaus auf die Burg zu (keine Sprünge).
 * Eine intakte Mauer hält sie auf (außer `ignoresWalls`), dann greifen sie an, was in Reichweite ist.
 * `stealsGold`: klaut Gold vom Monarchen und flieht damit zum Portal. `fleesAtHalfHp`: flieht bei halber HP.
 */

const BODY = 0.6;

export function spawnEnemy(w: World, kind: string, x: number): Enemy {
  const d = ENEMIES[kind];
  const depth = w.biome.depth;
  const s = WAVES.depthScaling;
  const hp = Math.round(d.hp * s.hp ** depth);
  const e: Enemy = {
    id: newId(w),
    kind,
    x,
    hp,
    maxHp: hp,
    damage: Math.round(d.damage * s.damage ** depth),
    speed: d.speed * s.speed ** depth,
    range: d.range,
    traits: d.traits,
    cooldown: 0,
    fleeing: false,
    carriedGold: 0,
    homeX: x,
  };
  w.enemies.push(e);
  return e;
}

export function stepSpawns(w: World): void {
  const due = w.spawnQueue.filter((s) => s.at <= w.time);
  if (due.length === 0) return;
  w.spawnQueue = w.spawnQueue.filter((s) => s.at > w.time);
  for (const s of due) spawnEnemy(w, s.kind, s.x);
}

/** Bei Tagesanbruch fliehen alle Gegner zurück zu ihren Portalen (K2C). */
export function sendEnemiesHome(w: World): void {
  for (const e of w.enemies) e.fleeing = true;
  w.spawnQueue = [];
}

type Target = { id: number; x: number; kind: 'player' | 'troop' | 'wall' | 'castle' | 'site'; player?: World['players'][number] };

export function stepEnemies(w: World, dt: number): void {
  for (const e of w.enemies) {
    e.cooldown = Math.max(0, e.cooldown - dt);
    if (e.traits.includes('fleesAtHalfHp') && e.hp < e.maxHp / 2) e.fleeing = true;

    if (e.fleeing) {
      e.x += Math.sign(e.homeX - e.x) * Math.min(Math.abs(e.homeX - e.x), e.speed * 1.2 * dt);
      continue;
    }

    const dir = Math.sign(w.hubX - e.x) || 1;
    const wall = e.traits.includes('ignoresWalls') ? null : blockingWall(w, e, dir);
    const target = chooseTarget(w, e, dir, wall);

    if (target) {
      if (e.cooldown <= 0) attack(w, e, target);
      continue;
    }

    let x = e.x + dir * e.speed * dt;
    const stop = wall ? wall.x - dir * BODY : w.hubX - dir * castleRadius();
    x = dir > 0 ? Math.min(x, stop) : Math.max(x, stop);
    e.x = x;
  }
  // Geflohene Gegner verschwinden im Portal (mitsamt geklautem Gold).
  w.enemies = w.enemies.filter((e) => !(e.fleeing && Math.abs(e.x - e.homeX) < 0.3));
}

function blockingWall(w: World, e: Enemy, dir: number): Site | null {
  let best: Site | null = null;
  for (const s of w.sites) {
    if (s.kind !== 'wall' || s.state !== 'built') continue;
    const ahead = dir > 0 ? s.x >= e.x - BODY && s.x < w.hubX : s.x <= e.x + BODY && s.x > w.hubX;
    if (ahead && (!best || Math.abs(s.x - e.x) < Math.abs(best.x - e.x))) best = s;
  }
  return best;
}

function chooseTarget(w: World, e: Enemy, dir: number, wall: Site | null): Target | null {
  const ranged = e.traits.includes('ranged');
  // Nahkämpfer erreichen nichts hinter der Mauer.
  const reachable = (x: number) => ranged || !wall || (dir > 0 ? x <= wall.x : x >= wall.x);
  const inRange = (x: number, extra = 0) => Math.abs(x - e.x) <= e.range + BODY + extra && reachable(x);

  const candidates: Target[] = [];
  for (const p of w.players) if (isAlive(p) && inRange(p.x)) candidates.push({ id: p.id, x: p.x, kind: 'player', player: p });
  for (const t of w.troops) {
    if (t.kind === 'vagrant' || (!ranged && isOnTower(w, t))) continue;
    if (inRange(t.x)) candidates.push({ id: t.id, x: t.x, kind: 'troop' });
  }
  if (wall && Math.abs(wall.x - e.x) <= e.range + BODY + 0.1) candidates.push({ id: wall.id, x: wall.x, kind: 'wall' });
  if (ranged) {
    for (const s of w.sites) if (s.kind === 'tower' && s.state === 'built' && inRange(s.x)) candidates.push({ id: s.id, x: s.x, kind: 'site' });
  }
  if (Math.abs(w.castle.x - e.x) <= e.range + castleRadius() + 0.1 && reachable(w.castle.x)) {
    candidates.push({ id: w.castle.id, x: w.castle.x, kind: 'castle' });
  }
  if (candidates.length === 0) return null;

  const preferred = (c: Target) =>
    (e.traits.includes('prefersBuildings') && (c.kind === 'wall' || c.kind === 'castle' || c.kind === 'site')) ||
    (e.traits.includes('prefersTowers') && c.kind === 'site') ||
    (e.traits.includes('prefersTroops') && c.kind === 'troop') ||
    ((e.traits.includes('prefersMonarch') || e.traits.includes('stealsGold')) && c.kind === 'player');
  const pool = candidates.some(preferred) ? candidates.filter(preferred) : candidates;
  return pool.reduce((a, b) => (Math.abs(b.x - e.x) < Math.abs(a.x - e.x) ? b : a));
}

function attack(w: World, e: Enemy, target: Target): void {
  e.cooldown = 1 / WAVES.attacksPerSecond;
  const p = target.player;
  if (p && e.traits.includes('stealsGold') && p.gold > 0) {
    const amount = Math.min(p.gold, WAVES.stealGold);
    p.gold -= amount;
    e.carriedGold += amount;
    e.fleeing = true;
    w.events.push({ type: 'goldStolen', player: p.index, amount });
    return;
  }
  if (e.traits.includes('ranged')) {
    w.projectiles.push({ id: newId(w), x: e.x, targetId: target.id, team: 'enemy', damage: e.damage, speed: 20 });
  } else {
    applyDamage(w, target.id, e.damage);
  }
}

/** Positionen aller Ziele, die ein Geschoss verfolgen kann. */
function targetX(w: World, id: number): number | null {
  for (const list of [w.enemies, w.players, w.troops, w.sites] as { id: number; x: number }[][]) {
    const found = list.find((o) => o.id === id);
    if (found) return found.x;
  }
  return w.castle.id === id ? w.castle.x : null;
}

export function stepProjectiles(w: World, dt: number): void {
  w.projectiles = w.projectiles.filter((pr) => {
    const x = targetX(w, pr.targetId);
    if (x === null) return false;
    if (Math.abs(x - pr.x) <= pr.speed * dt) {
      applyDamage(w, pr.targetId, pr.damage);
      return false;
    }
    pr.x += Math.sign(x - pr.x) * pr.speed * dt;
    return true;
  });
}

/** Besiegte Gegner droppen Gold (plus geklautes Gold), manchmal auch die Stufen-Ressource. */
export function removeDeadEnemies(w: World): void {
  w.enemies = w.enemies.filter((e) => {
    if (e.hp > 0) return true;
    scatterCoins(w, e.x, w.rng.int(...ENEMIES[e.kind].gold) + e.carriedGold);
    const drop = ECONOMY.enemyResourceDrop;
    const resource = w.biome.primaryResource as ResourceKind;
    if (w.rng.next() < drop.chance && resource in w.stock) {
      w.stock[resource] += drop.amount;
      w.events.push({ type: 'gathered', resource, amount: drop.amount });
    }
    if (w.aggression !== null && w.biome.cycle.type === 'aggressionPool') {
      w.aggression = Math.min(100, w.aggression + w.biome.cycle.percentPerKill);
    }
    return false;
  });
}
