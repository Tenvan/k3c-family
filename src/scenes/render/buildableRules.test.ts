import { describe, expect, it } from 'vitest';
import { HUB } from '../../model/data';
import type { Site } from '../../model/types';
import { buildableSites } from './buildableRules';

const site = (id: number, kind: Site['kind'], x: number, state: Site['state'] = 'unpaid'): Site => ({
  id,
  kind,
  x,
  state,
  paidGold: 0,
  buildProgress: 0,
  hp: 1,
  maxHp: 1,
  workerId: null,
  bows: 0,
  bowPaidGold: 0,
});

describe('buildableSites', () => {
  const hubX = 500;
  const workshop = HUB.sites.find((h) => h.kind === 'workshop');
  const tavern = HUB.sites.find((h) => h.kind === 'tavern');

  it('Hub-Plätze erst ab ihrer Hub-Stufe', () => {
    expect(workshop && tavern).toBeTruthy();
    const sites = [site(1, 'workshop', hubX + workshop!.offsetUnits), site(2, 'tavern', hubX + tavern!.offsetUnits)];
    const lvl1 = buildableSites({ sites, hubX, hubLevel: 1 });
    expect(lvl1.has(1)).toBe(true);
    expect(lvl1.has(2)).toBe(tavern!.hubLevel <= 1);
    expect(buildableSites({ sites, hubX, hubLevel: tavern!.hubLevel }).has(2)).toBe(true);
  });

  it('bezahlte und gebaute Plätze zeigen kein Bauschild', () => {
    const sites = [site(1, 'workshop', hubX + workshop!.offsetUnits, 'built')];
    expect(buildableSites({ sites, hubX, hubLevel: 5 }).size).toBe(0);
  });

  it('Mauer 2 erst, wenn Mauer 1 derselben Seite steht', () => {
    const sites = [site(1, 'wall', hubX - 40, 'unpaid'), site(2, 'wall', hubX - 80)];
    const open = buildableSites({ sites, hubX, hubLevel: 2 });
    expect(open.has(1)).toBe(true);
    expect(open.has(2)).toBe(false);
    sites[0] = site(1, 'wall', hubX - 40, 'built');
    expect(buildableSites({ sites, hubX, hubLevel: 2 }).has(2)).toBe(true);
  });

  it('Mauer der Linie k erst ab Hub-Stufe k', () => {
    const sites = [site(1, 'wall', hubX + 40, 'built'), site(2, 'wall', hubX + 80)];
    expect(buildableSites({ sites, hubX, hubLevel: 1 }).has(2)).toBe(false);
    expect(buildableSites({ sites, hubX, hubLevel: 2 }).has(2)).toBe(true);
  });
});
