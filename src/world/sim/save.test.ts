import { describe, expect, it } from 'vitest';
import { biomeForDepth } from '../biome';
import { ECONOMY } from './data';
import { isSaveData, loadWorld, toSave } from './save';
import { addPlayer, createWorld, step } from './world';
import type { PlayerCommand, World } from './types';

const forest = biomeForDepth(0);
const DT = 1 / 30;
const WALK: PlayerCommand = { moveX: 1, sprint: true, pay: false };

function run(w: World, seconds: number, commands: PlayerCommand[] = []): void {
  for (let t = 0; t < seconds; t += DT) step(w, commands, DT);
}

/** Welt mit etwas Fortschritt: Zeit vergangen, Gebäude bezahlt, Truhe weg, Ressource abgebaut. */
function playedWorld(): World {
  const w = createWorld(forest, 'save-test', { cycleSpeed: 8 });
  addPlayer(w);
  addPlayer(w);
  run(w, 20, [WALK, WALK]);
  w.players[0].gold = 17;
  w.players[1].gold = 4;
  w.stock = { wood: 12, stone: 3, copper: 1 };
  w.skillPoints = 2;
  w.wave = 3;
  w.castle.hp = 640;
  Object.assign(w.sites[0], { state: 'built', paidGold: 5, buildProgress: 1, hp: 111 });
  Object.assign(w.nodes[0], { marked: true, paidGold: 1, progress: 0.25 });
  w.nodes.splice(1, 1);
  w.pickups.splice(0, 1);
  return w;
}

/** Vergleichbarer Zustand ohne Ids, Laufziele und Aufträge (die werden beim Laden neu vergeben). */
function essence(w: World) {
  return {
    ...toSave(w, 'x'),
    cycle: w.cycle,
    sites: w.sites.map(({ id: _id, workerId: _w, ...s }) => s),
  };
}

describe('Speichern/Laden', () => {
  it('Laden ergibt denselben Spielstand', () => {
    const w = playedWorld();
    const loaded = loadWorld(forest, JSON.parse(JSON.stringify(toSave(w))), { cycleSpeed: 8 });
    expect(essence(loaded)).toEqual(essence(w));
    expect(loaded.troops.map((t) => t.kind)).toEqual(w.troops.map((t) => t.kind));
    expect(loaded.nodes.length).toBe(w.nodes.length);
    expect(loaded.pickups.length).toBe(w.pickups.length);
  });

  it('Spieler bekommen beim Beitritt ihr gespeichertes Gold, neue Plätze Startgold', () => {
    const loaded = loadWorld(forest, toSave(playedWorld()));
    expect(addPlayer(loaded).gold).toBe(17);
    expect(addPlayer(loaded).gold).toBe(4);
    expect(addPlayer(loaded).gold).toBe(ECONOMY.purse.startGold);
  });

  it('Gold noch nicht beigetretener Spieler geht beim nächsten Speichern nicht verloren', () => {
    const loaded = loadWorld(forest, toSave(playedWorld()));
    addPlayer(loaded).gold = 30;
    expect(toSave(loaded).playerGold).toEqual([30, 4]);
  });

  it('getragenes Material landet im Vorrat', () => {
    const w = playedWorld();
    w.troops[0].job = { type: 'carry', resource: 'wood', amount: 5 };
    expect(toSave(w).stock.wood).toBe(17);
  });

  it('geladene Welt läuft weiter, ohne sofort Tag/Nacht-Meldungen auszulösen', () => {
    const loaded = loadWorld(forest, toSave(playedWorld()), { cycleSpeed: 8 });
    step(loaded, [], DT);
    expect(loaded.events.filter((e) => e.type === 'dawn' || e.type === 'night' || e.type === 'dusk')).toEqual([]);
    expect(() => run(loaded, 30)).not.toThrow();
  });

  it('schnellerer Zyklus (?fast=1) ändert beim Laden nicht die Tageszeit', () => {
    const w = playedWorld();
    const loaded = loadWorld(forest, toSave(w), { cycleSpeed: 1 });
    expect(loaded.cycle).toEqual(w.cycle);
  });

  it('erkennt kaputte Stände', () => {
    expect(isSaveData(null)).toBe(false);
    expect(isSaveData({ version: 99 })).toBe(false);
    expect(isSaveData(toSave(playedWorld()))).toBe(true);
    expect(() => loadWorld(forest, { version: 1 } as never)).toThrow();
  });
});
