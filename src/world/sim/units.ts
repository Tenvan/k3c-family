import { approach, isDangerous, newId, outerWall } from './common';
import { BUILDINGS, ECONOMY, HUB, TROOPS } from './data';
import type { Troop, World } from './types';

/**
 * Eigene Truppen (KI):
 * - Landstreicher wandern um ihr Camp, bis ein Monarch sie mit einer Münze rekrutiert.
 * - Bauern holen Bögen, bauen bezahlte Gebäude und holen markierte Ressourcen. Bei Gefahr bleiben sie in der Burg.
 * - Bogenschützen besetzen Türme oder stehen hinter der äußersten Mauer und schießen automatisch.
 */

const ARRIVE = 0.3;
const ARROW_SPEED = 25;

export function spawnVagrant(w: World, campX: number, x = campX): Troop {
  const t: Troop = {
    id: newId(w),
    kind: 'vagrant',
    x,
    hp: TROOPS.vagrant.hp,
    maxHp: TROOPS.vagrant.hp,
    anchorX: campX,
    targetX: x,
    job: null,
    cooldown: 0,
    towerId: null,
    paidGold: 0,
  };
  w.troops.push(t);
  return t;
}

export function stepCamps(w: World, dt: number): void {
  const { maxVagrants, respawnSeconds, wanderUnits } = ECONOMY.recruitCamp;
  for (const camp of w.camps) {
    const count = w.troops.filter((t) => t.kind === 'vagrant' && t.anchorX === camp.x).length;
    if (count >= maxVagrants) {
      camp.respawnIn = respawnSeconds;
      continue;
    }
    camp.respawnIn -= dt;
    if (camp.respawnIn <= 0) {
      spawnVagrant(w, camp.x, camp.x + (w.rng.next() - 0.5) * wanderUnits);
      camp.respawnIn = respawnSeconds;
    }
  }
}

export function stepTroops(w: World, dt: number): void {
  for (const t of w.troops) {
    if (t.kind === 'vagrant') wander(w, t, t.anchorX, ECONOMY.recruitCamp.wanderUnits, dt);
    else if (t.kind === 'peasant') stepPeasant(w, t, dt);
    else stepArcher(w, t, dt);
  }
}

function wander(w: World, t: Troop, center: number, radius: number, dt: number): void {
  const speed = TROOPS[t.kind].speed * 0.5;
  if (Math.abs(t.targetX - center) > radius) t.targetX = center;
  if (Math.abs(t.x - t.targetX) < ARRIVE) {
    t.cooldown -= dt;
    if (t.cooldown <= 0) {
      t.targetX = center + (w.rng.next() * 2 - 1) * radius;
      t.cooldown = 1 + w.rng.next() * 3;
    }
    return;
  }
  t.x = approach(t.x, t.targetX, speed, dt);
}

function walkTo(t: Troop, x: number, dt: number): boolean {
  t.x = approach(t.x, x, TROOPS[t.kind].speed, dt);
  return Math.abs(t.x - x) < ARRIVE;
}

function stepPeasant(w: World, t: Troop, dt: number): void {
  if (!t.job) t.job = findJob(w, t);
  const danger = isDangerous(w);
  const job = t.job;

  // Bei Gefahr: Arbeit außerhalb der Mauern liegen lassen und in die Burg.
  if (danger && job && job.type === 'gather') {
    releaseJob(w, t);
  }

  if (!t.job) {
    if (danger) walkTo(t, w.hubX + ((t.id % 5) - 2), dt);
    else wander(w, t, w.hubX, HUB.homeRadiusUnits, dt);
    return;
  }

  switch (t.job.type) {
    case 'fetchBow': {
      const site = w.sites.find((s) => s.id === (t.job as { siteId: number }).siteId);
      if (!site || site.state !== 'built') return void (t.job = null);
      if (!walkTo(t, site.x, dt)) return;
      t.job = null;
      if (site.bows > 0) {
        site.bows--;
        makeArcher(w, t);
        w.events.push({ type: 'armed' });
      }
      return;
    }
    case 'build': {
      const site = w.sites.find((s) => s.id === (t.job as { siteId: number }).siteId);
      if (!site || site.state !== 'waitingWorker') return void (t.job = null);
      if (!walkTo(t, site.x, dt)) return;
      site.buildProgress += dt / Math.max(0.1, BUILDINGS[site.kind].buildSeconds);
      if (site.buildProgress >= 1) {
        Object.assign(site, { state: 'built', buildProgress: 1, hp: BUILDINGS[site.kind].hp, maxHp: BUILDINGS[site.kind].hp, workerId: null });
        t.job = null;
        w.events.push({ type: 'built', kind: site.kind });
      }
      return;
    }
    case 'gather': {
      const nodeId = t.job.nodeId;
      const node = w.nodes.find((n) => n.id === nodeId);
      if (!node) return void (t.job = null);
      if (!walkTo(t, node.x, dt)) return;
      const g = ECONOMY.gatherables[node.kind];
      node.progress += dt / g.workSeconds;
      if (node.progress >= 1) {
        w.nodes = w.nodes.filter((n) => n !== node);
        t.job = { type: 'carry', resource: g.resource, amount: g.amount };
      }
      return;
    }
    case 'carry': {
      if (!walkTo(t, w.hubX, dt)) return;
      w.stock[t.job.resource] += t.job.amount;
      w.events.push({ type: 'gathered', resource: t.job.resource, amount: t.job.amount });
      if (w.aggression !== null && w.biome.cycle.type === 'aggressionPool') {
        w.aggression = Math.min(100, w.aggression + w.biome.cycle.percentPerGather);
      }
      t.job = null;
      return;
    }
  }
}

function findJob(w: World, t: Troop): Troop['job'] {
  const taken = (pred: (o: Troop) => boolean) => w.troops.filter((o) => o !== t && pred(o)).length;

  for (const s of w.sites) {
    if (s.kind !== 'workshop' || s.state !== 'built' || s.bows === 0) continue;
    const fetching = taken((o) => o.job?.type === 'fetchBow' && o.job.siteId === s.id);
    if (fetching < s.bows) return { type: 'fetchBow', siteId: s.id };
  }
  const site = w.sites
    .filter((s) => s.state === 'waitingWorker' && (s.workerId === null || !w.troops.some((o) => o.id === s.workerId)))
    .sort((a, b) => Math.abs(a.x - t.x) - Math.abs(b.x - t.x))[0];
  if (site) {
    site.workerId = t.id;
    return { type: 'build', siteId: site.id };
  }
  if (isDangerous(w)) return null;
  const node = w.nodes
    .filter((n) => n.marked && (n.workerId === null || !w.troops.some((o) => o.id === n.workerId)))
    .sort((a, b) => Math.abs(a.x - w.hubX) - Math.abs(b.x - w.hubX))[0];
  if (node) {
    node.workerId = t.id;
    return { type: 'gather', nodeId: node.id };
  }
  return null;
}

export function releaseJob(w: World, t: Troop): void {
  const job = t.job;
  if (!job) return;
  if (job.type === 'gather') {
    const node = w.nodes.find((n) => n.id === job.nodeId);
    if (node && node.workerId === t.id) node.workerId = null;
  }
  if (job.type === 'build') {
    const site = w.sites.find((s) => s.id === job.siteId);
    if (site && site.workerId === t.id) site.workerId = null;
  }
  t.job = null;
}

/** Befördert zum Bogenschützen. Die Seite (anchorX) wird so gewählt, dass beide Seiten gleich stark besetzt sind. */
export function makeArcher(w: World, t: Troop): void {
  const left = w.troops.filter((o) => o !== t && o.kind === 'archer' && o.anchorX < w.hubX).length;
  const right = w.troops.filter((o) => o !== t && o.kind === 'archer' && o.anchorX >= w.hubX).length;
  const side = left <= right ? -1 : 1;
  Object.assign(t, { kind: 'archer', hp: TROOPS.archer.hp, maxHp: TROOPS.archer.hp, cooldown: 0, job: null, anchorX: w.hubX + side });
}

export function isOnTower(w: World, t: Troop): boolean {
  if (t.towerId === null) return false;
  const tower = w.sites.find((s) => s.id === t.towerId);
  return !!tower && Math.abs(tower.x - t.x) < ARRIVE;
}

function stepArcher(w: World, t: Troop, dt: number): void {
  // Posten: freier Platz auf einem Turm, sonst hinter der äußersten Mauer seiner Seite.
  if (t.towerId === null) {
    const tower = w.sites
      .filter((s) => s.kind === 'tower' && s.state === 'built' && Math.sign(s.x - w.hubX) === Math.sign(t.anchorX - w.hubX))
      .filter((s) => w.troops.filter((o) => o.towerId === s.id).length < (BUILDINGS.tower.archerSlots ?? 0))
      .sort((a, b) => Math.abs(a.x - t.x) - Math.abs(b.x - t.x))[0];
    if (tower) t.towerId = tower.id;
  }
  const tower = t.towerId === null ? null : w.sites.find((s) => s.id === t.towerId);
  const side: -1 | 1 = t.anchorX < w.hubX ? -1 : 1;
  const wall = outerWall(w, side);
  const post = tower ? tower.x : wall ? wall.x - side * 2 : w.hubX + side * 10;
  walkTo(t, post, dt);

  t.cooldown = Math.max(0, t.cooldown - dt);
  const range = TROOPS.archer.range + (isOnTower(w, t) ? (BUILDINGS.tower.rangeBonus ?? 0) : 0);
  if (t.cooldown > 0) return;
  let target: World['enemies'][number] | null = null;
  for (const e of w.enemies) {
    if (Math.abs(e.x - t.x) > range) continue;
    if (!target || Math.abs(e.x - t.x) < Math.abs(target.x - t.x)) target = e;
  }
  if (!target) return;
  w.projectiles.push({ id: newId(w), x: t.x, targetId: target.id, team: 'player', damage: TROOPS.archer.damage, speed: ARROW_SPEED });
  t.cooldown = 1 / TROOPS.archer.attacksPerSecond;
}
