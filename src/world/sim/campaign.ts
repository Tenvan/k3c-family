import { biomeForDepth } from '../../model/biome';
import { newId } from './common';
import { cycleAt, globalDayNight } from './cycle';
import { TROOPS } from '../../model/data';
import { makeArcher, spawnVagrant } from './units';
import { addPlayer, createWorld } from './world';
import { SAVE_VERSION, type HubSave, type SaveGame, type World } from '../../model/types';

export { SAVE_VERSION, isSaveGame } from '../../model/types';
export type { HubSave, SaveGame } from '../../model/types';

/**
 * Kampagne = alle Stufen eines Spielstands. Jede Stufe hat ihren eigenen Hub, der beim Stufenwechsel erhalten bleibt.
 * Gespeichert wird laut GDD nur, was sich nicht aus dem Seed ergibt: Hubs, Truppen, Vorräte, Gold und was aus der
 * Welt schon entfernt wurde (gefällte Bäume, geöffnete Truhen). Gegner und Level-Layout nicht.
 */

export interface Campaign {
  id: string;
  seed: string;
  cycleSpeed: number;
  depth: number;
  unlockedDepth: number;
  /** Laufende Welten (Stufen, die in dieser Sitzung schon betreten wurden) */
  worlds: Map<number, World>;
  /** Geladene Hubs, deren Welt noch nicht erzeugt wurde */
  pendingHubs: Map<number, HubSave>;
  /** Gold pro Spielerplatz, auch für Spieler, die (noch) nicht beigetreten sind */
  playerGold: number[];
}

export interface CampaignOptions {
  id: string;
  cycleSpeed?: number;
  depth?: number;
}

const key = (o: { kind: string; x: number }) => `${o.kind}@${o.x.toFixed(2)}`;

export function createCampaign(seed: string, options: CampaignOptions): Campaign {
  const depth = options.depth ?? 0;
  const c: Campaign = {
    id: options.id,
    seed,
    cycleSpeed: options.cycleSpeed ?? 1,
    depth,
    unlockedDepth: depth,
    worlds: new Map(),
    pendingHubs: new Map(),
    playerGold: [],
  };
  c.worlds.set(depth, createWorld(biomeForDepth(depth), seed, { cycleSpeed: c.cycleSpeed }));
  return c;
}

export function currentWorld(c: Campaign): World {
  return c.worlds.get(c.depth)!;
}

/** Spieler beitreten lassen. Gold aus dem Spielstand gilt pro Spielerplatz. */
export function joinPlayer(c: Campaign): void {
  const w = currentWorld(c);
  const p = addPlayer(w);
  const saved = c.playerGold[p.index];
  if (saved !== undefined) p.gold = saved;
}

function worldFor(c: Campaign, depth: number, time: number, skillPoints: number): World {
  let w = c.worlds.get(depth);
  if (!w) {
    w = createWorld(biomeForDepth(depth), c.seed, { cycleSpeed: c.cycleSpeed, time });
    const hub = c.pendingHubs.get(depth);
    if (hub) applyHub(w, hub);
    c.pendingHubs.delete(depth);
    c.worlds.set(depth, w);
  }
  // Die Stufe stand still, während niemand dort war. Zeit und Zyklus an die Kampagne angleichen,
  // wartende Gegner behalten ihren Abstand.
  const shift = time - w.time;
  w.spawnQueue = w.spawnQueue.map((s) => ({ ...s, at: s.at + shift }));
  w.time = time;
  w.cycle = cycleAt(globalDayNight(), time * w.cycleSpeed);
  w.skillPoints = skillPoints;
  w.travel = null;
  return w;
}

/**
 * Wechselt die Stufe. Die Spieler nehmen Gold und Reihenfolge mit und stehen danach an der Burg der Zielstufe.
 * Truppen, Gebäude und Vorräte bleiben in ihrem Hub.
 */
export function travel(c: Campaign, toDepth: number): World {
  const from = currentWorld(c);
  const target = worldFor(c, toDepth, from.time, from.skillPoints);
  target.players = from.players.map((p) => ({
    ...p,
    id: newId(target), // ids sind nur innerhalb einer Welt eindeutig
    x: target.hubX + (p.index % 2 === 0 ? -3 : 3),
    vx: 0,
    hp: p.respawnIn > 0 ? p.maxHp : p.hp,
    respawnIn: 0,
    payCooldown: 0.5,
    paying: false,
  }));
  from.players = [];
  from.travel = null;
  c.depth = toDepth;
  c.unlockedDepth = Math.max(c.unlockedDepth, toDepth);
  target.events.push({ type: 'arrived', depth: toDepth, name: target.biome.name });
  return target;
}

// ---------- Speichern ----------

function hubSave(w: World, base: World): HubSave {
  const alive = (list: { kind: string; x: number }[]) => new Set(list.map(key));
  const nodesNow = alive(w.nodes);
  const pickupsNow = alive(w.pickups);
  return {
    depth: w.biome.depth,
    castleHp: w.castle.hp,
    stock: { ...w.stock },
    wave: w.wave,
    aggression: w.aggression,
    sites: w.sites.map((s) => ({
      kind: s.kind,
      x: s.x,
      // Ein angefangener Bau wird als "wartet auf Bauer" gespeichert, die Bauer kommen nach dem Laden wieder.
      state: s.state,
      paidGold: s.paidGold,
      buildProgress: s.buildProgress,
      hp: s.hp,
      bows: s.bows,
      bowPaidGold: s.bowPaidGold,
    })),
    troops: w.troops.filter((t) => t.kind !== 'vagrant').map((t) => ({ kind: t.kind, x: t.x, anchorX: t.anchorX })),
    nodesGone: base.nodes.map(key).filter((k) => !nodesNow.has(k)),
    nodesMarked: w.nodes.filter((n) => n.marked).map(key),
    pickupsTaken: base.pickups.map(key).filter((k) => !pickupsNow.has(k)),
  };
}

export function toSave(c: Campaign, savedAt: string): SaveGame {
  const w = currentWorld(c);
  const hubs: HubSave[] = [...c.pendingHubs.values()];
  for (const [depth, world] of c.worlds) {
    // Vergleichswelt frisch aus dem Seed: was dort ist, hier aber fehlt, wurde entfernt.
    hubs.push(hubSave(world, createWorld(biomeForDepth(depth), c.seed)));
  }
  const gold = [...c.playerGold];
  for (const p of w.players) gold[p.index] = p.gold;
  return {
    version: SAVE_VERSION,
    campaignId: c.id,
    savedAt,
    seed: c.seed,
    depth: c.depth,
    unlockedDepth: c.unlockedDepth,
    time: w.time,
    skillPoints: w.skillPoints,
    players: gold.map((g) => ({ gold: g ?? 0 })),
    hubs: hubs.sort((a, b) => a.depth - b.depth),
  };
}

// ---------- Laden ----------

export function fromSave(save: SaveGame, cycleSpeed = 1): Campaign {
  const c: Campaign = {
    id: save.campaignId,
    seed: save.seed,
    cycleSpeed,
    depth: save.depth,
    unlockedDepth: save.unlockedDepth,
    worlds: new Map(),
    pendingHubs: new Map(save.hubs.map((h) => [h.depth, h])),
    playerGold: save.players.map((p) => p.gold),
  };
  worldFor(c, save.depth, save.time, save.skillPoints);
  return c;
}

function applyHub(w: World, hub: HubSave): void {
  w.castle.hp = hub.castleHp;
  w.stock = { ...hub.stock };
  w.wave = hub.wave;
  if (w.aggression !== null && hub.aggression !== null) w.aggression = hub.aggression;

  for (const saved of hub.sites) {
    const site = w.sites.find((s) => key(s) === key(saved));
    if (!site) continue; // Bauplatz gibt es in dieser Version nicht mehr
    const state = saved.state === 'waitingWorker' || saved.state === 'waitingMaterial' || saved.state === 'built' ? saved.state : 'unpaid';
    Object.assign(site, { state, paidGold: saved.paidGold, buildProgress: saved.buildProgress, hp: saved.hp, bows: saved.bows, bowPaidGold: saved.bowPaidGold, workerId: null });
  }

  // Start-Truppen durch die gespeicherten ersetzen
  w.troops = w.troops.filter((t) => t.kind === 'vagrant');
  for (const t of hub.troops) {
    const troop = spawnVagrant(w, w.hubX, t.x);
    if (t.kind === 'archer') {
      makeArcher(w, troop);
      troop.anchorX = t.anchorX;
    } else if (t.kind === 'peasant') {
      Object.assign(troop, { kind: 'peasant', hp: TROOPS.peasant.hp, maxHp: TROOPS.peasant.hp, anchorX: w.hubX });
    }
  }

  const gone = new Set(hub.nodesGone);
  const marked = new Set(hub.nodesMarked);
  w.nodes = w.nodes.filter((n) => !gone.has(key(n)));
  for (const n of w.nodes) if (marked.has(key(n))) n.marked = true;
  const taken = new Set(hub.pickupsTaken);
  w.pickups = w.pickups.filter((p) => !taken.has(key(p)));
}
