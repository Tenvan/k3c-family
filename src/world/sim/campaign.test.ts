import { describe, expect, it } from 'vitest';
import { createCampaign, currentWorld, fromSave, isSaveGame, joinPlayer, toSave, travel, type Campaign } from './campaign';
import { cycleAt, globalDayNight } from './cycle';
import { travelPoints } from './travel';
import { step } from './world';
import type { World } from './types';

const DT = 1 / 30;
const roundtrip = (c: Campaign) => JSON.parse(JSON.stringify(toSave(c, '2026-01-01T00:00:00.000Z')));

function build(w: World, kind: string): World['sites'][number] {
  const site = w.sites.find((s) => s.kind === kind)!;
  Object.assign(site, { state: 'built', hp: site.maxHp, buildProgress: 1 });
  return site;
}

/** Alle Spieler zum Punkt stellen und warten, bis die Kampagne wechseln würde. */
function walkTo(w: World, x: number): void {
  for (const p of w.players) p.x = x;
  for (let i = 0; i < 200 && (w.travel?.progress ?? 0) < 1; i++) step(w, [], DT);
}

describe('Kampagne: Stufenwechsel', () => {
  it('Tiefen-Eingang führt nach unten, Gold und Zeit kommen mit, der Hub oben bleibt', () => {
    const c = createCampaign('abstieg', { id: 'a' });
    joinPlayer(c);
    joinPlayer(c);
    const top = currentWorld(c);
    top.players[0].gold = 33;
    top.stock.wood = 77;
    build(top, 'wall');
    const exit = travelPoints(top).find((p) => p.via === 'exit')!;
    expect(exit.toDepth).toBe(1);

    walkTo(top, exit.x);
    expect(top.travel?.progress).toBe(1);
    const time = top.time;
    const cave = travel(c, 1);

    expect(c.depth).toBe(1);
    expect(c.unlockedDepth).toBe(1);
    expect(cave.biome.id).toBe('cave');
    expect(cave.players.map((p) => p.gold)).toEqual([33, expect.any(Number)]);
    expect(cave.players.every((p) => Math.abs(p.x - cave.hubX) < 5)).toBe(true);
    expect(cave.time).toBe(time);
    expect(cave.cycle).toEqual(cycleAt(globalDayNight(), time));
    expect(cave.events.map((e) => e.type)).toContain('arrived');
    expect(top.players).toHaveLength(0);
    expect(top.stock.wood).toBe(77);
  });

  it('gebaute Treppe hoch führt zurück in denselben Hub', () => {
    const c = createCampaign('treppe', { id: 't' });
    joinPlayer(c);
    const top = currentWorld(c);
    const wall = build(top, 'wall');
    travel(c, 1);
    const cave = currentWorld(c);
    expect(travelPoints(cave).some((p) => p.via === 'stairsUp')).toBe(false); // noch nicht gebaut
    const stairs = build(cave, 'stairsUp');
    walkTo(cave, stairs.x);
    expect(cave.travel).toMatchObject({ toDepth: 0, via: 'stairsUp' });
    const back = travel(c, 0);
    expect(back).toBe(top);
    expect(wall.state).toBe('built');
    expect(back.players).toHaveLength(1);
  });

  it('Treppen gibt es nur, wo sie hinführen können, die tiefste Stufe hat keinen Ausgang', () => {
    const forest = currentWorld(createCampaign('x', { id: 'x' }));
    expect(forest.sites.map((s) => s.kind)).not.toContain('stairsUp');
    expect(forest.sites.map((s) => s.kind)).toContain('stairsDown');
    const mine = currentWorld(createCampaign('x', { id: 'x', depth: 2 }));
    expect(mine.sites.map((s) => s.kind)).toContain('stairsUp');
    expect(mine.sites.map((s) => s.kind)).not.toContain('stairsDown');
    expect(travelPoints(mine)).toEqual([]);
  });

  it('ohne Spieler oder wenn einer fehlt, wird nicht gewechselt', () => {
    const c = createCampaign('allein', { id: 'a' });
    joinPlayer(c);
    joinPlayer(c);
    const w = currentWorld(c);
    const exit = travelPoints(w)[0];
    w.players[0].x = exit.x;
    for (let i = 0; i < 120; i++) step(w, [], DT);
    expect(w.travel).toBeNull();
  });
});

describe('Kampagne: Speichern & Laden', () => {
  it('stellt Hub, Truppen, Vorrat, Gold und die veränderte Welt wieder her', () => {
    const c = createCampaign('speicher', { id: 'spiel-1' });
    joinPlayer(c);
    joinPlayer(c);
    const w = currentWorld(c);
    w.players[0].gold = 21;
    w.players[1].gold = 3;
    w.stock = { wood: 12, stone: 5, copper: 0 };
    w.skillPoints = 2;
    w.wave = 4;
    w.castle.hp = 640;
    build(w, 'wall').hp = 111;
    Object.assign(w.sites.find((s) => s.kind === 'workshop')!, { state: 'unpaid', paidGold: 7 });
    const felled = w.nodes[0];
    const marked = w.nodes[1];
    marked.marked = true;
    w.nodes = w.nodes.filter((n) => n !== felled);
    const chest = w.pickups[0];
    w.pickups = w.pickups.filter((p) => p !== chest);
    const troops = w.troops.filter((t) => t.kind !== 'vagrant').map((t) => t.kind).sort();
    for (let i = 0; i < 30 * 5; i++) step(w, [], DT);

    const save = roundtrip(c);
    expect(isSaveGame(save)).toBe(true);
    const loaded = fromSave(save);
    joinPlayer(loaded);
    joinPlayer(loaded);
    const l = currentWorld(loaded);

    expect(l.players.map((p) => p.gold)).toEqual([21, 3]);
    expect(l.stock).toEqual({ wood: 12, stone: 5, copper: 0 });
    expect(l.skillPoints).toBe(2);
    expect(l.wave).toBe(4);
    expect(l.castle.hp).toBe(640);
    expect(l.time).toBeCloseTo(w.time);
    expect(l.sites.find((s) => s.kind === 'wall')).toMatchObject({ state: 'built', hp: 111 });
    expect(l.sites.find((s) => s.kind === 'workshop')).toMatchObject({ state: 'unpaid', paidGold: 7 });
    expect(l.nodes.some((n) => n.kind === felled.kind && n.x === felled.x)).toBe(false);
    expect(l.nodes.find((n) => n.x === marked.x)?.marked).toBe(true);
    expect(l.pickups.some((p) => p.x === chest.x)).toBe(false);
    expect(l.troops.filter((t) => t.kind !== 'vagrant').map((t) => t.kind).sort()).toEqual(troops);
    expect(loaded.id).toBe('spiel-1');
  });

  it('speichert alle besuchten Stufen, geladene Hubs werden beim Betreten wiederhergestellt', () => {
    const c = createCampaign('mehrere', { id: 'm' });
    joinPlayer(c);
    currentWorld(c).stock.wood = 50;
    travel(c, 1);
    currentWorld(c).stock.stone = 40;
    build(currentWorld(c), 'stairsUp');

    const save = roundtrip(c);
    expect(save.hubs.map((h: { depth: number }) => h.depth)).toEqual([0, 1]);
    expect(save.depth).toBe(1);

    const loaded = fromSave(save);
    joinPlayer(loaded);
    expect(currentWorld(loaded).stock.stone).toBe(40);
    const top = travel(loaded, 0);
    expect(top.stock.wood).toBe(50);
    // erneut speichern verliert nichts
    expect(roundtrip(loaded).hubs).toHaveLength(2);
  });

  it('Gold von Spielern, die noch nicht beigetreten sind, bleibt erhalten', () => {
    const c = createCampaign('p2', { id: 'p' });
    joinPlayer(c);
    joinPlayer(c);
    currentWorld(c).players[1].gold = 30;
    const loaded = fromSave(roundtrip(c));
    joinPlayer(loaded);
    expect(roundtrip(loaded).players.map((p: { gold: number }) => p.gold)[1]).toBe(30);
  });

  it('erkennt fremde Daten', () => {
    expect(isSaveGame(null)).toBe(false);
    expect(isSaveGame({ version: 99 })).toBe(false);
    expect(isSaveGame({ ...roundtrip(createCampaign('v', { id: 'v' })), hubs: 'kaputt' })).toBe(false);
  });
});
