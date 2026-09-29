import { describe, expect, it } from 'vitest';
import { createRng } from '../../core/rng';
import { BIOMES, biomeForDepth } from '../biome';
import { cycleAt, globalDayNight } from './cycle';
import { BUILDINGS, ECONOMY, ENEMIES, TROOPS, WAVES } from './data';
import { findPayTarget } from './economy';
import { spawnEnemy } from './enemies';
import { spawnVagrant } from './units';
import { planWave, waveRow } from './waves';
import { addPlayer, createWorld, startWave, step } from './world';
import { IDLE, type PlayerCommand, type World } from './types';

const forest = biomeForDepth(0);
const DT = 1 / 30;
const PAY: PlayerCommand = { moveX: 0, sprint: false, pay: true };

function run(w: World, seconds: number, commands: PlayerCommand[] = []): void {
  for (let t = 0; t < seconds; t += DT) step(w, commands, DT);
}

/** Welt ohne Landstreicher/Portale in Hubnähe, damit Tests nur das prüfen, was sie aufbauen. */
function quietWorld(seed = 'test'): World {
  const w = createWorld(forest, seed);
  w.troops = [];
  w.camps = [];
  w.pickups = [];
  return w;
}

const snapshot = (w: World) => JSON.stringify({ ...w, rng: undefined, biome: undefined, level: undefined });

describe('Tag/Nacht-Zyklus', () => {
  const cfg = { dayMinutes: 10, twilightMinutes: 1, nightMinutes: 5 };
  it('wechselt Tag → Dämmerung → Nacht → Tag', () => {
    expect(cycleAt(cfg, 0)).toMatchObject({ phase: 'day', day: 1 });
    expect(cycleAt(cfg, 10 * 60)).toMatchObject({ phase: 'dusk', day: 1 });
    expect(cycleAt(cfg, 11 * 60)).toMatchObject({ phase: 'night', day: 1 });
    expect(cycleAt(cfg, 16 * 60)).toMatchObject({ phase: 'day', day: 2 });
  });

  it('der globale Zyklus kommt aus der Oberwelt', () => {
    expect(globalDayNight()).toEqual(forest.cycle);
  });
});

describe('Wellen', () => {
  it('folgen der GDD-Tabelle', () => {
    expect(waveRow(1).standard).toEqual([5, 10]);
    expect(waveRow(7).elite).toEqual([1, 2]);
    expect(waveRow(30).fromWave).toBe(11);
  });

  it('haben die richtige Größe, nutzen nur Biom-Gegner und sind deterministisch', () => {
    for (const biome of BIOMES) {
      for (let wave = 1; wave <= 12; wave++) {
        const plan = planWave(biome, wave, createRng(`w${wave}`), [100, 900], true);
        const row = waveRow(wave);
        const elites = plan.filter((s) => ENEMIES[s.kind].tier === 'elite').length;
        expect(plan.length - elites).toBeGreaterThanOrEqual(row.standard[0]);
        expect(plan.length - elites).toBeLessThanOrEqual(row.standard[1]);
        expect(elites).toBeLessThanOrEqual(row.elite[1]);
        for (const s of plan) expect([...biome.enemies.portal, ...biome.enemies.night]).toContain(s.kind);
        expect(planWave(biome, wave, createRng(`w${wave}`), [100, 900], true)).toEqual(plan);
      }
    }
  });
});

describe('Welt', () => {
  it('startet mit Burg, Bauplätzen, Ressourcen und Landstreichern aus dem Level', () => {
    const w = createWorld(forest, 'k3c');
    expect(w.castle.x).toBe(w.hubX);
    expect(w.sites.map((s) => s.kind).sort()).toEqual(['tower', 'tower', 'wall', 'wall', 'workshop']);
    expect(w.nodes.some((n) => n.kind === 'tree')).toBe(true);
    expect(w.portals.length).toBe(forest.portals.count);
    expect(w.troops.filter((t) => t.kind === 'vagrant').length).toBe(w.camps.length * ECONOMY.recruitCamp.maxVagrants);
  });

  it('ist deterministisch: gleicher Seed + gleiche Eingaben => gleicher Zustand', () => {
    const play = () => {
      const w = createWorld(forest, 'familie', { cycleSpeed: 20 });
      addPlayer(w);
      addPlayer(w);
      for (let i = 0; i < 3000; i++) step(w, [{ moveX: Math.sin(i / 50), sprint: i % 200 < 50, pay: i % 90 < 10 }, IDLE], DT);
      return snapshot(w);
    };
    expect(play()).toEqual(play());
  });

  it('bewegt Monarchen nur horizontal und innerhalb der Welt', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    run(w, 2, [{ moveX: 1, sprint: false, pay: false }]);
    expect(p.x).toBeGreaterThan(w.hubX);
    p.x = 0.5;
    run(w, 2, [{ moveX: -1, sprint: true, pay: false }]);
    expect(p.x).toBe(0);
  });
});

describe('Gold', () => {
  it('ohne Ziel fällt die Münze, der andere Spieler hebt sie auf, der Geber nicht sofort', () => {
    const w = quietWorld();
    const [a, b] = [addPlayer(w), addPlayer(w)];
    a.x = b.x = w.hubX; // Mitte des Hubs, dort ist kein Bauplatz
    w.players.forEach((p) => (p.payCooldown = 0));
    b.x = w.hubX + 10;
    const goldA = a.gold;
    step(w, [PAY, IDLE], DT);
    expect(a.gold).toBe(goldA - 1);
    expect(w.coins).toHaveLength(1);
    run(w, 0.5);
    expect(w.coins).toHaveLength(1); // Geber ist noch gesperrt
    b.x = a.x;
    const goldB = b.gold;
    step(w, [], DT);
    expect(b.gold).toBe(goldB + 1);
    expect(w.coins).toHaveLength(0);
  });

  it('Beutel hat ein Limit, Überschuss bleibt liegen', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    p.gold = ECONOMY.purse.maxGold;
    w.coins.push({ id: 999, x: p.x, blockedPlayerId: null, blockedUntil: 0 });
    step(w, [], DT);
    expect(p.gold).toBe(ECONOMY.purse.maxGold);
    expect(w.coins).toHaveLength(1);
  });

  it('Truhen geben Gold, Skill-Punkte zählen für alle', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    p.gold = 0;
    w.pickups.push({ id: 900, kind: 'chest', x: p.x }, { id: 901, kind: 'skillPoint', x: p.x });
    step(w, [], DT);
    expect(p.gold).toBeGreaterThanOrEqual(ECONOMY.chestGold[0]);
    expect(w.skillPoints).toBe(1);
    expect(w.events.map((e) => e.type).sort()).toEqual(['chest', 'skillPoint']);
  });
});

describe('Rekrutieren & Arbeiten', () => {
  it('Landstreicher wird für eine Münze zum Bauer', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    p.payCooldown = 0;
    const v = spawnVagrant(w, p.x + 1);
    v.cooldown = 99; // steht still
    step(w, [PAY], DT);
    expect(v.kind).toBe('peasant');
    expect(p.gold).toBe(ECONOMY.purse.startGold - (TROOPS.vagrant.recruitCost?.gold ?? 1));
  });

  it('markierter Baum wird gefällt, das Holz landet im Hub', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    spawnVagrant(w, w.hubX).kind = 'peasant';
    const tree = w.nodes.filter((n) => n.kind === 'tree').sort((a, b) => Math.abs(a.x - w.hubX) - Math.abs(b.x - w.hubX))[0];
    p.x = tree.x;
    p.payCooldown = 0;
    step(w, [PAY], DT);
    expect(tree.marked).toBe(true);
    run(w, 200);
    expect(w.nodes).not.toContain(tree);
    expect(w.stock.wood).toBe(ECONOMY.gatherables.tree.amount);
  });

  it('Mauer: Gold bezahlen, Holz aus dem Vorrat, Bauer baut', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    spawnVagrant(w, w.hubX).kind = 'peasant';
    const wall = w.sites.find((s) => s.kind === 'wall')!;
    const cost = BUILDINGS.wall.cost;
    p.x = wall.x;
    p.gold = 30;
    run(w, 3, [PAY]);
    expect(wall.state).toBe('waitingMaterial');
    expect(p.gold).toBe(30 - (cost.gold ?? 0)); // nicht mehr bezahlt als nötig
    expect(findPayTarget(w, p)?.kind).not.toBe('site');
    w.stock.wood = cost.wood ?? 0;
    run(w, BUILDINGS.wall.buildSeconds + 20);
    expect(wall.state).toBe('built');
    expect(wall.hp).toBe(BUILDINGS.wall.hp);
    expect(w.stock.wood).toBe(0);
  });

  it('Werkstatt fertigt Bögen, Bauer wird Bogenschütze und besetzt den Turm', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    spawnVagrant(w, w.hubX).kind = 'peasant';
    const workshop = w.sites.find((s) => s.kind === 'workshop')!;
    const tower = w.sites.find((s) => s.kind === 'tower')!;
    for (const s of [workshop, tower]) Object.assign(s, { state: 'built', hp: s.maxHp });
    w.stock.wood = 1000;
    p.x = workshop.x;
    p.gold = 40;
    run(w, 10, [PAY]);
    run(w, 40);
    const archer = w.troops.find((t) => t.kind === 'archer');
    expect(archer).toBeDefined();
    expect(archer!.towerId).toBe(tower.id);
    expect(Math.abs(archer!.x - tower.x)).toBeLessThan(0.5);
  });
});

describe('Nacht & Kampf', () => {
  it('zur Nacht öffnen sich die Portale, bei Tagesanbruch fliehen die Gegner', () => {
    const w = createWorld(forest, 'nacht', { cycleSpeed: 60 });
    addPlayer(w);
    const types: string[] = [];
    let sawEnemies = false;
    for (let i = 0; i < 30 * 30 && !types.includes('dawn'); i++) {
      step(w, [], DT);
      types.push(...w.events.map((e) => e.type));
      sawEnemies ||= w.enemies.length > 0;
    }
    expect(types).toEqual(expect.arrayContaining(['dusk', 'night', 'wave', 'dawn']));
    expect(sawEnemies).toBe(true);
    expect(w.enemies.every((e) => e.fleeing)).toBe(true);
  });

  it('eine Mauer hält Gegner auf und wird angegriffen', () => {
    const w = quietWorld();
    const wall = w.sites.find((s) => s.kind === 'wall' && s.x < w.hubX)!;
    Object.assign(wall, { state: 'built', hp: wall.maxHp });
    const e = spawnEnemy(w, 'goblin', wall.x - 10);
    run(w, 6);
    expect(e.x).toBeLessThan(wall.x);
    expect(wall.hp).toBeLessThan(wall.maxHp);
  });

  it('Greed klaut Gold und flieht, erlegt droppt er es wieder', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    p.gold = 20;
    p.x = w.hubX - 20;
    const e = spawnEnemy(w, 'greed', p.x - 3);
    e.homeX = p.x - 50;
    run(w, 2);
    expect(p.gold).toBe(20 - WAVES.stealGold);
    expect(e.fleeing).toBe(true);
    expect(e.carriedGold).toBe(WAVES.stealGold);
    e.hp = 0;
    step(w, [], DT);
    expect(w.coins.length).toBeGreaterThanOrEqual(WAVES.stealGold + ENEMIES.greed.gold[0]);
  });

  it('Bogenschützen erlegen Gegner in Reichweite', () => {
    const w = quietWorld();
    const archer = spawnVagrant(w, w.hubX - 10);
    Object.assign(archer, { kind: 'archer', hp: TROOPS.archer.hp });
    const e = spawnEnemy(w, 'greed', archer.x - TROOPS.archer.range + 1);
    Object.assign(e, { speed: 0, hp: TROOPS.archer.damage });
    run(w, 2);
    expect(w.enemies).not.toContain(e);
  });

  it('Monarch fällt und kehrt ohne Strafe an die Burg zurück', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    p.gold = 0;
    p.x = w.hubX - 30;
    const e = spawnEnemy(w, 'goblin', p.x - 1);
    e.speed = 0;
    let down = false;
    for (let i = 0; i < 30 * 60 && !down; i++) {
      step(w, [], DT);
      down = w.events.some((ev) => ev.type === 'playerDown');
    }
    expect(down).toBe(true);
    w.enemies = [];
    run(w, 6);
    expect(p.respawnIn).toBe(0);
    expect(p.hp).toBe(p.maxHp);
    expect(Math.abs(p.x - w.hubX)).toBeLessThan(5);
  });

  it('fällt die Burg: Gebäude zerstört, halbe Ressourcen, Truppen weg', () => {
    const w = quietWorld();
    const p = addPlayer(w);
    p.gold = 20;
    w.stock.wood = 40;
    const wall = w.sites[0];
    Object.assign(wall, { state: 'built', hp: wall.maxHp });
    spawnVagrant(w, w.hubX).kind = 'peasant';
    w.castle.hp = 1;
    spawnEnemy(w, 'goblin', w.hubX - 5);
    run(w, 2);
    expect(w.sites.every((s) => s.state === 'unpaid')).toBe(true);
    expect(w.stock.wood).toBe(20);
    expect(p.gold).toBe(10);
    expect(w.troops.filter((t) => t.kind !== 'vagrant')).toHaveLength(0);
    expect(w.castle.hp).toBe(w.castle.maxHp);
  });
});

describe('Aggressionspool (unter Tage)', () => {
  it('löst bei 100% eine Welle aus und startet wieder bei 0', () => {
    const w = createWorld(biomeForDepth(1), 'hoehle');
    expect(w.aggression).toBe(0);
    w.aggression = 99.99;
    run(w, 1);
    expect(w.wave).toBe(1);
    expect(w.aggression).toBeLessThan(5);
  });

  it('Kills erhöhen den Pool', () => {
    const w = createWorld(biomeForDepth(1), 'hoehle');
    const e = spawnEnemy(w, 'bat', w.hubX - 200);
    e.hp = 0;
    step(w, [], DT);
    expect(w.aggression).toBeGreaterThanOrEqual(5);
  });

  it('startWave nutzt die Portale der Stufe', () => {
    const w = createWorld(biomeForDepth(2), 'mine');
    startWave(w);
    expect(w.spawnQueue.length).toBeGreaterThan(0);
    for (const s of w.spawnQueue) expect(w.portals).toContain(s.x);
  });
});

describe('Robustheit', () => {
  it.each(BIOMES.map((b) => [b.id, b] as const))('%s: mehrere Zyklen mit zufälligen Eingaben bleiben stabil', (_id, biome) => {
    const w = createWorld(biome, 'chaos', { cycleSpeed: 30 });
    addPlayer(w);
    addPlayer(w);
    w.stock.wood = w.stock.stone = w.stock.copper = 500;
    const input = createRng('eingaben');
    let cmds: PlayerCommand[] = [IDLE, IDLE];
    for (let i = 0; i < 30 * 120; i++) {
      if (i % 20 === 0) cmds = cmds.map(() => ({ moveX: input.next() * 2 - 1, sprint: input.next() < 0.3, pay: input.next() < 0.3 }));
      if (w.aggression !== null && i % 600 === 0) w.aggression = 100; // unter Tage Wellen erzwingen
      step(w, cmds, DT);
      const positions = [...w.players, ...w.troops, ...w.enemies, ...w.coins, ...w.projectiles].map((o) => o.x);
      for (const x of positions) {
        expect(Number.isFinite(x)).toBe(true);
        expect(x).toBeGreaterThanOrEqual(-1);
        expect(x).toBeLessThanOrEqual(w.widthUnits + 1);
      }
      for (const p of w.players) expect(p.gold).toBeGreaterThanOrEqual(0);
    }
    expect(w.wave).toBeGreaterThan(0);
  });
});

describe('Balancing (Startaufstellung)', () => {
  it('mit beiden Mauern übersteht die Startaufstellung die erste Nacht', () => {
    for (const seed of ['a', 'b', 'c', 'd']) {
      const w = createWorld(forest, seed);
      addPlayer(w);
      addPlayer(w);
      for (const s of w.sites) if (s.kind === 'wall') Object.assign(s, { state: 'built', hp: s.maxHp });
      w.time = forest.cycle.type === 'dayNight' ? (forest.cycle.dayMinutes + forest.cycle.twilightMinutes) * 60 - 1 : 0;
      let fallen = false;
      while (w.cycle.phase !== 'day' || w.wave === 0) {
        step(w, [], DT);
        fallen ||= w.events.some((e) => e.type === 'castleFallen');
      }
      expect(fallen, `Seed ${seed}`).toBe(false);
    }
  });
});
