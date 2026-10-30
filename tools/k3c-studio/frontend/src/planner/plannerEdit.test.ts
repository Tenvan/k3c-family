import { describe, expect, it } from 'vitest';
import { mockHubPlan } from '../api/mockHub';
import {
  addSite, candidateText, moveSite, numberOf, parseHub, removeSite, setMerchant, setSiteField, setSiteKind, setZone1Tower, setZone1Wall, siteOf,
} from './plannerEdit';

// WZ9.3: Änderungen am Kandidaten; geprüft wird im Go-Backend, hier nur geändert.

const hub = () => parseHub(mockHubPlan().raw);

describe('Gebäude-Planer: Kandidat ändern', () => {
  it('ändert Felder eines Platzes, ohne das Original anzufassen', () => {
    const before = hub();
    const next = setSiteField(setSiteField(before, 'sites', 7, 'hubLevel', 3), 'sites', 7, 'needsDeeper', true);
    expect(next.sites[7]).toEqual({ kind: 'armory', offsetUnits: 40, hubLevel: 3, needsDeeper: true });
    expect(before.sites[7]!.hubLevel).toBe(7);
  });

  it('fromDepth 0 und needsDeeper false entfallen wie in der Datei', () => {
    const next = setSiteField(setSiteField(hub(), 'sites', 1, 'fromDepth', 0), 'sites', 2, 'needsDeeper', false);
    expect('fromDepth' in next.sites[1]!).toBe(false);
    expect('needsDeeper' in next.sites[2]!).toBe(false);
  });

  it('Baugrund der Zone 1 und Händler', () => {
    const next = setMerchant(setZone1Tower(setZone1Wall(hub(), 99), 98), 'onlyDepth', 1);
    expect(next.sitePlacement.zone1WallUnits).toBe(99);
    expect(next.sitePlacement.zone1TowerUnits).toBe(98);
    expect(next.merchant.onlyDepth).toBe(1);
    expect(hub().sitePlacement.zone1WallUnits).toBe(60);
  });

  it('Platz hinzufügen und entfernen', () => {
    const added = addSite(hub(), 'islandSites', 'healer', -60, 2);
    expect(added.islandSites.at(-1)).toEqual({ kind: 'healer', offsetUnits: -60, hubLevel: 2 });
    const removed = removeSite(hub(), 'sites', 3);
    expect(removed.sites.map((s) => s.kind)).not.toContain('healer');
    expect(siteOf(removed, { source: 'sites', index: 3 })!.kind).toBe('tavern');
    expect(siteOf(removed, { source: 'merchant', index: 0 })).toBeUndefined();
  });

  it('Kandidat als Text behält alle übrigen Felder', () => {
    const text = candidateText(setZone1Wall(hub(), 62));
    expect(JSON.parse(text).levels).toEqual(parseHub(mockHubPlan().raw).levels);
  });

  it('Zahl aus dem Eingabefeld', () => {
    expect(numberOf('-40')).toBe(-40);
    expect(numberOf('')).toBeNull();
    expect(numberOf('x')).toBeNull();
  });
});

describe('Gebäude-Planer: Art und Liste im Inspektor (B-670)', () => {
  it('ändert die Art, die übrigen Felder bleiben', () => {
    const before = hub();
    expect(setSiteKind(before, 'sites', 7, 'healer').sites[7]).toEqual({ kind: 'healer', offsetUnits: 40, hubLevel: 7 });
    expect(before.sites[7]!.kind).toBe('armory');
  });

  it('verschiebt einen Platz in die andere Liste, hinten angehängt', () => {
    const before = hub();
    const next = moveSite(before, 'sites', 7, 'islandSites');
    expect(next.sites).toHaveLength(before.sites.length - 1);
    expect(next.islandSites.at(-1)).toEqual(before.sites[7]);
    expect(moveSite(before, 'sites', 7, 'sites')).toBe(before);
  });
});
