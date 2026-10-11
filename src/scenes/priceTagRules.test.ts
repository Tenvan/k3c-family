import { describe, expect, it } from 'vitest';
import type { Player, Site, World } from '../model/types';
import { offersPrice, priceTagSites } from './priceTagRules';

const player = (over: Partial<Player> = {}): Player => ({ id: 0, index: 0, x: 10, respawnIn: 0, ...over }) as Player;
const site = (over: Partial<Site>): Site => ({ id: 1, kind: 'wall', x: 12, state: 'unpaid', paidGold: 0, buildProgress: 0, hp: 1, maxHp: 1, workerId: null, bows: 0, bowPaidGold: 0, ...over });
const world = (sites: Site[], players: Player[]): World => ({ sites, players }) as unknown as World;

describe('priceTagSites: nur das nächste Preisschild (B-395/AC-01)', () => {
  it('mehrere Preisschilder in Reichweite: nur das nächste', () => {
    const sites = [site({ id: 1, x: 14 }), site({ id: 2, x: 11 }), site({ id: 3, x: 8.5 })];
    expect([...priceTagSites(world(sites, [player({ x: 10 })]))]).toEqual([2]);
  });

  it('gleicher Abstand: die kleinere Nummer', () => {
    const sites = [site({ id: 7, x: 12 }), site({ id: 4, x: 8 })];
    expect([...priceTagSites(world(sites, [player({ x: 10 })]))]).toEqual([4]);
  });

  it('zwei Spieler an verschiedenen Plätzen: je ein Schild', () => {
    const sites = [site({ id: 1, x: 5 }), site({ id: 2, x: 30 })];
    const players = [player({ id: 0, x: 6 }), player({ id: 1, index: 1, x: 29 })];
    expect([...priceTagSites(world(sites, players))].sort()).toEqual([1, 2]);
  });

  it('außer Reichweite oder gefallener Monarch: kein Schild', () => {
    const sites = [site({ id: 1, x: 40 })];
    expect(priceTagSites(world(sites, [player({ x: 10 })])).size).toBe(0);
    expect(priceTagSites(world([site({ id: 2, x: 11 })], [player({ respawnIn: 5 })])).size).toBe(0);
  });

  it('bezahlte und gebaute Bauplätze sind keine Kandidaten, die Werkstatt mit freiem Regal schon', () => {
    const built = site({ id: 1, x: 10, state: 'built' });
    const shop = site({ id: 2, kind: 'workshop', state: 'built', bows: 0, x: 13 });
    expect(offersPrice(built)).toBe(false);
    expect(offersPrice(shop)).toBe(true);
    expect([...priceTagSites(world([built, shop], [player()]))]).toEqual([2]);
  });
});
