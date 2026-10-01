import { BUILDINGS, HUB, MONARCH } from '../../model/data';
import type { World } from '../../model/types';

export const newId = (w: World): number => w.nextId++;

/** Bewegt `x` mit `speed` auf `target` zu, ohne darüber hinauszuschießen. */
export function approach(x: number, target: number, speed: number, dt: number): number {
  const step = speed * dt;
  return Math.abs(target - x) <= step ? target : x + Math.sign(target - x) * step;
}

export const isAlive = (p: { respawnIn: number }): boolean => p.respawnIn <= 0;

export const isNightTime = (w: World): boolean => w.cycle.phase === 'night';

/** Gefahr: nachts oder solange Gegner in der Welt sind bleiben Bauern im Hub. */
export const isDangerous = (w: World): boolean => isNightTime(w) || w.enemies.length > 0;

/** Intakte Mauer auf der Seite `side` (-1 links, 1 rechts), die am weitesten außen steht. */
export function outerWall(w: World, side: -1 | 1) {
  let best: World['sites'][number] | null = null;
  for (const s of w.sites) {
    if (s.kind !== 'wall' || s.state !== 'built' || Math.sign(s.x - w.hubX) !== side) continue;
    if (!best || Math.abs(s.x - w.hubX) > Math.abs(best.x - w.hubX)) best = s;
  }
  return best;
}

/** Schaden auf irgendein Ziel (per id). Tod/Zerstörung wird hier bzw. im Aufräumen des Ticks behandelt. */
export function applyDamage(w: World, targetId: number, damage: number): void {
  const player = w.players.find((p) => p.id === targetId);
  if (player) {
    if (!isAlive(player)) return;
    player.hp -= Math.max(1, damage - MONARCH.base.defense);
    if (player.hp <= 0) {
      player.hp = 0;
      player.respawnIn = MONARCH.respawnSeconds;
      player.vx = 0;
      w.events.push({ type: 'playerDown', player: player.index });
    }
    return;
  }
  const troop = w.troops.find((t) => t.id === targetId);
  if (troop) return void (troop.hp -= damage);
  const enemy = w.enemies.find((e) => e.id === targetId);
  if (enemy) return void (enemy.hp -= damage);
  if (w.castle.id === targetId) return void (w.castle.hp -= damage);
  const site = w.sites.find((s) => s.id === targetId);
  if (site && site.state === 'built') {
    site.hp -= damage;
    if (site.hp <= 0) destroySite(w, site);
  }
}

/** Gebäude zerstört: Bauplatz ist wieder leer und muss neu bezahlt werden. */
export function destroySite(w: World, site: World['sites'][number]): void {
  const wasBuilt = site.state === 'built';
  Object.assign(site, { state: 'unpaid', paidGold: 0, buildProgress: 0, hp: 0, workerId: null, bows: 0, bowPaidGold: 0 });
  site.maxHp = BUILDINGS[site.kind].hp;
  for (const t of w.troops) {
    if (t.towerId === site.id) t.towerId = null;
    if (t.job && 'siteId' in t.job && t.job.siteId === site.id) t.job = null;
  }
  if (wasBuilt) w.events.push({ type: 'destroyed', kind: site.kind });
}

export const castleRadius = (): number => HUB.castleRadiusUnits;
