import { describe, expect, it } from 'vitest';
import levelMsg from '../../../testdata/protocol/s2c-level.json';
import wirtschaft from '../../../testdata/protocol/s2c-snapshot-wirtschaft.json';
import type { BiomeConfig } from '../../model/biome';
import { BUILDINGS } from '../../model/data';
import type { HubUpgrade, LevelLayout, Player, Site, World } from '../../model/types';
import type { WorldState } from '../../online/clientProtocol';
import { createViewWorld } from '../../online/clientWorld';
import { offersHubUpgrade, offersPrice, priceTagSites } from './priceTagRules';

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

  it('gebaute Werkstatt mit vollem Bogen-Regal ist kein Kandidat', () => {
    const full = site({ id: 3, kind: 'workshop', state: 'built', bows: BUILDINGS.workshop.bowRack ?? 0, x: 10 });
    expect(offersPrice(full)).toBe(false);
    expect(priceTagSites(world([full], [player()])).size).toBe(0);
  });
});

// B-591/AC-01: Ausbau-Angebot vor der Burg aus echten Snapshot-Daten (Burg x 550, ID 1)
describe('priceTagSites: Ausbau-Angebot an der Burg (B-591/AC-01)', () => {
  const level = { depth: 0, layout: levelMsg.layout as unknown as LevelLayout, biome: {} as BiomeConfig };
  const snap = wirtschaft.s as unknown as WorldState;
  const { state: _state, ...rest } = snap.hubUpgrade as HubUpgrade;
  const open: HubUpgrade = { ...rest, paid: 0 }; // Gold offen, der Snapshot selbst wartet schon auf Material
  const real = (over: Record<string, unknown>, players: Player[]): World => ({ ...createViewWorld(level, { ...snap, ...over } as unknown as WorldState), players });

  it('Spieler vor der Burg, Gold offen: das Preisschild der Burg steht', () => {
    const w = real({ hubUpgrade: open, sites: [] }, [player({ x: 551 })]);
    expect(offersHubUpgrade(w)).toBe(true);
    expect([...priceTagSites(w)]).toEqual([w.castle.id]);
  });

  it('ein näherer Bauplatz gewinnt, außer Reichweite kein Schild', () => {
    const w = real({ hubUpgrade: open, sites: [] }, [player({ x: 553 })]);
    expect([...priceTagSites({ ...w, sites: [site({ id: 9, x: 554 })] })]).toEqual([9]);
    expect(priceTagSites({ ...w, players: [player({ x: 540 })] }).size).toBe(0);
  });

  it('Ausbau läuft (state aus dem Snapshot), letzte Hub-Stufe (kein hubUpgrade) oder Gold bezahlt: kein Angebot', () => {
    const p = [player({ x: 550 })];
    expect(priceTagSites(real({ sites: [] }, p)).size).toBe(0); // waitingMaterial
    expect(priceTagSites(real({ hubLevel: 5, hubUpgrade: undefined, sites: [] }, p)).size).toBe(0);
    expect(offersHubUpgrade({ hubUpgrade: { ...open, paid: open.gold } })).toBe(false);
  });
});
