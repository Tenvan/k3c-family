import { isAlive, newId } from './common';
import { BUILDINGS, ECONOMY, MONARCH, TROOPS, type Cost, type ResourceKind } from './data';
import { IDLE, type Player, type PlayerCommand, type ResourceNode, type Site, type Troop, type World } from './types';

/**
 * Monarchen, Münzen und alles, was man bezahlt (K2C-Prinzip: eine Taste für alles).
 * A halten gibt im Takt Münzen an das nächste bezahlbare Ziel in Reichweite. Ohne Ziel fällt die Münze
 * auf den Boden (so gibt man dem anderen Spieler Gold).
 */

type PayTarget = { kind: 'site'; site: Site } | { kind: 'vagrant'; troop: Troop } | { kind: 'node'; node: ResourceNode };

const RESOURCES: ResourceKind[] = ['wood', 'stone', 'copper'];

export function stepPlayers(w: World, commands: readonly PlayerCommand[], dt: number): void {
  const speed = MONARCH.base.speed;
  for (const p of w.players) {
    const cmd = commands[p.index] ?? IDLE;
    p.payCooldown = Math.max(0, p.payCooldown - dt);

    if (!isAlive(p)) {
      p.paying = false;
      refundPending(w, p);
      p.respawnIn -= dt;
      if (p.respawnIn <= 0) respawn(w, p);
      continue;
    }

    const target = cmd.moveX * speed * (cmd.sprint ? MONARCH.sprintMultiplier : 1);
    p.vx += (target - p.vx) * (1 - Math.exp(-MONARCH.acceleration * dt));
    p.x = Math.min(w.widthUnits, Math.max(0, p.x + p.vx * dt));
    if (Math.abs(p.vx) > 0.05) p.facing = p.vx > 0 ? 1 : -1;

    p.paying = cmd.pay;
    if (cmd.pay && p.payCooldown <= 0 && p.gold > 0) {
      payOneCoin(w, p);
      p.payCooldown = ECONOMY.payIntervalSeconds;
    }
    // Aufgehört zu halten (oder das Ziel gewechselt), bevor der Betrag voll war: Münzen kommen zurück.
    if (p.payKey && (!cmd.pay || keyOf(findPayTarget(w, p)) !== p.payKey)) refundPending(w, p);
  }
  collectCoins(w);
  collectPickups(w);
}

export function respawn(w: World, p: Player): void {
  p.respawnIn = 0;
  p.hp = p.maxHp;
  p.vx = 0;
  p.x = w.hubX + (p.index % 2 === 0 ? -3 : 3);
}

/** Was würde eine Münze gerade bezahlen? (Auch für die Anzeige von Preisschildern.) */
export function findPayTarget(w: World, p: Player): PayTarget | null {
  const range = ECONOMY.payRangeUnits;
  const near = <T extends { x: number }>(items: T[], ok: (t: T) => boolean): T | null => {
    let best: T | null = null;
    for (const it of items) {
      if (Math.abs(it.x - p.x) > range || !ok(it)) continue;
      if (!best || Math.abs(it.x - p.x) < Math.abs(best.x - p.x)) best = it;
    }
    return best;
  };
  const site = near(w.sites, (s) => sitePayable(s));
  if (site) return { kind: 'site', site };
  const troop = near(w.troops, (t) => t.kind === 'vagrant');
  if (troop) return { kind: 'vagrant', troop };
  const node = near(w.nodes, (n) => !n.marked && ECONOMY.gatherables[n.kind] !== undefined);
  if (node) return { kind: 'node', node };
  return null;
}

export function sitePayable(s: Site): boolean {
  if (s.state === 'unpaid') return true;
  if (s.kind === 'workshop' && s.state === 'built') {
    return s.bows < (BUILDINGS.workshop.bowRack ?? 0) && s.bowPaidGold < (TROOPS.archer.cost?.gold ?? 0);
  }
  return false;
}

function keyOf(t: PayTarget | null): string | null {
  if (!t) return null;
  return t.kind === 'site' ? `site:${t.site.id}` : t.kind === 'vagrant' ? `vagrant:${t.troop.id}` : `node:${t.node.id}`;
}

/** Gezahltes, aber nicht vollendetes Gold zurück auf den Boden werfen und beim Ziel abziehen. */
function refundPending(w: World, p: Player): void {
  const key = p.payKey;
  const amount = p.payAmount;
  p.payKey = null;
  p.payAmount = 0;
  if (!key || amount <= 0) return;
  const [kind, idText] = key.split(':');
  const id = Number(idText);
  let back = 0;
  const take = (paid: number) => Math.min(amount, paid);
  if (kind === 'site') {
    const s = w.sites.find((x) => x.id === id);
    if (s && sitePayable(s)) {
      if (s.state === 'unpaid') (back = take(s.paidGold)), (s.paidGold -= back);
      else (back = take(s.bowPaidGold)), (s.bowPaidGold -= back);
    }
  } else if (kind === 'vagrant') {
    const t = w.troops.find((x) => x.id === id);
    if (t && t.kind === 'vagrant') (back = take(t.paidGold)), (t.paidGold -= back);
  } else {
    const n = w.nodes.find((x) => x.id === id);
    if (n && !n.marked) (back = take(n.paidGold)), (n.paidGold -= back);
  }
  for (let i = 0; i < back; i++) w.coins.push({ id: newId(w), x: p.x, blockedPlayerId: null, blockedUntil: 0 });
}

function payOneCoin(w: World, p: Player): void {
  const target = findPayTarget(w, p);
  // Am fertig bezahlten Bauplatz nichts fallen lassen (sonst verliert man beim Festhalten Münzen).
  if (!target && w.sites.some((s) => Math.abs(s.x - p.x) <= ECONOMY.payRangeUnits)) return;
  p.gold--;
  if (!target) {
    w.coins.push({ id: newId(w), x: p.x, blockedPlayerId: p.id, blockedUntil: w.time + ECONOMY.dropPickupDelaySeconds });
    return;
  }
  const key = keyOf(target);
  if (p.payKey !== key) refundPending(w, p);
  p.payKey = key;
  p.payAmount++;
  switch (target.kind) {
    case 'site': {
      const s = target.site;
      if (s.state === 'unpaid') {
        s.paidGold++;
        if (s.paidGold >= (BUILDINGS[s.kind].cost.gold ?? 0)) s.state = 'waitingMaterial';
      } else {
        s.bowPaidGold++;
      }
      if (!sitePayable(s)) clearPending(p);
      break;
    }
    case 'vagrant': {
      const t = target.troop;
      t.paidGold++;
      if (t.paidGold >= (TROOPS.vagrant.recruitCost?.gold ?? 1)) {
        Object.assign(t, { kind: 'peasant', hp: TROOPS.peasant.hp, maxHp: TROOPS.peasant.hp, anchorX: w.hubX, targetX: t.x, job: null, paidGold: 0 });
        w.events.push({ type: 'recruited', player: p.index });
        clearPending(p);
      }
      break;
    }
    case 'node': {
      const n = target.node;
      n.paidGold++;
      if (n.paidGold >= ECONOMY.gatherables[n.kind].markCost) (n.marked = true), clearPending(p);
      break;
    }
  }
}

function clearPending(p: Player): void {
  p.payKey = null;
  p.payAmount = 0;
}

function collectCoins(w: World): void {
  const range = ECONOMY.pickupRangeUnits;
  w.coins = w.coins.filter((c) => {
    const taker = w.players.find(
      (p) =>
        isAlive(p) &&
        p.gold < ECONOMY.purse.maxGold &&
        Math.abs(p.x - c.x) <= range &&
        !(c.blockedPlayerId === p.id && w.time < c.blockedUntil),
    );
    if (taker) taker.gold++;
    return !taker;
  });
}

function collectPickups(w: World): void {
  const range = ECONOMY.pickupRangeUnits * 1.5;
  w.pickups = w.pickups.filter((pk) => {
    const p = w.players.find((pl) => isAlive(pl) && Math.abs(pl.x - pk.x) <= range);
    if (!p) return true;
    if (pk.kind === 'chest') {
      const gold = w.rng.int(...ECONOMY.chestGold);
      giveGold(w, p, gold, pk.x);
      w.events.push({ type: 'chest', player: p.index, gold });
    } else {
      w.skillPoints++;
      w.events.push({ type: 'skillPoint', player: p.index, total: w.skillPoints });
    }
    return false;
  });
}

/** Gold in den Beutel, was nicht mehr passt, fällt als Münzen auf den Boden. */
export function giveGold(w: World, p: Player, amount: number, x = p.x): void {
  const fits = Math.min(amount, ECONOMY.purse.maxGold - p.gold);
  p.gold += fits;
  scatterCoins(w, x, amount - fits);
}

export function scatterCoins(w: World, x: number, count: number): void {
  for (let i = 0; i < count; i++) {
    const cx = Math.min(w.widthUnits, Math.max(0, x + (w.rng.next() - 0.5) * 3));
    w.coins.push({ id: newId(w), x: cx, blockedPlayerId: null, blockedUntil: 0 });
  }
}

export function canAfford(stock: World['stock'], cost: Cost): boolean {
  return RESOURCES.every((r) => stock[r] >= (cost[r] ?? 0));
}

function spend(stock: World['stock'], cost: Cost): void {
  for (const r of RESOURCES) stock[r] -= cost[r] ?? 0;
}

/** Bezahlte Bauplätze ziehen das Baumaterial aus dem Hub-Vorrat, sobald genug da ist. Werkstatt fertigt Bögen. */
export function stepSites(w: World): void {
  for (const s of w.sites) {
    if (s.state === 'waitingMaterial' && canAfford(w.stock, BUILDINGS[s.kind].cost)) {
      spend(w.stock, BUILDINGS[s.kind].cost);
      s.state = 'waitingWorker';
    }
    const bow = TROOPS.archer.cost ?? {};
    if (s.kind === 'workshop' && s.state === 'built' && s.bowPaidGold >= (bow.gold ?? 0) && canAfford(w.stock, bow)) {
      spend(w.stock, bow);
      s.bowPaidGold = 0;
      s.bows++;
    }
  }
}

/** Morgens: Steuern für jeden Monarchen. */
export function payDawnIncome(w: World): void {
  for (const p of w.players) if (isAlive(p)) giveGold(w, p, ECONOMY.dawnGoldPerPlayer);
}
