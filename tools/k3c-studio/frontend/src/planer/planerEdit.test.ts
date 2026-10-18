import { describe, expect, it } from 'vitest';
import { mockHubPlan } from '../api/mockHub';
import {
  addSite, candidateText, numberOf, parseHub, removeSite, setGateHubLevel, setMerchant, setSiteField, setWallUnit, siteOf,
} from './planerEdit';

// WZ9.3: Änderungen am Kandidaten; geprüft wird im Go-Backend, hier nur geändert.

const hub = () => parseHub(mockHubPlan().raw);

describe('Gebäude-Planer: Kandidat ändern', () => {
  it('ändert Felder eines Platzes, ohne das Original anzufassen', () => {
    const before = hub();
    const next = setSiteField(setSiteField(before, 'sites', 8, 'hubLevel', 3), 'sites', 8, 'needsDeeper', true);
    expect(next.sites[8]).toEqual({ kind: 'armory', offsetUnits: 40, hubLevel: 3, needsDeeper: true });
    expect(before.sites[8]!.hubLevel).toBe(7);
  });

  it('fromDepth 0 und needsDeeper false entfallen wie in der Datei', () => {
    const next = setSiteField(setSiteField(hub(), 'sites', 1, 'fromDepth', 0), 'sites', 2, 'needsDeeper', false);
    expect('fromDepth' in next.sites[1]!).toBe(false);
    expect('needsDeeper' in next.sites[2]!).toBe(false);
  });

  it('Mauerlinie, Tor und Händler', () => {
    const next = setMerchant(setGateHubLevel(setWallUnit(hub(), 4, 99), 3), 'onlyDepth', 1);
    expect(next.wallLines.wallUnits[4]).toBe(99);
    expect(next.wallLines.gateHubLevel).toBe(3);
    expect(next.merchant.onlyDepth).toBe(1);
    expect(setWallUnit(hub(), 99, 1)).toEqual(hub());
  });

  it('Platz hinzufügen und entfernen', () => {
    const added = addSite(hub(), 'islandSites', 'healer', -60, 2);
    expect(added.islandSites.at(-1)).toEqual({ kind: 'healer', offsetUnits: -60, hubLevel: 2 });
    const removed = removeSite(hub(), 'sites', 4);
    expect(removed.sites.map((s) => s.kind)).not.toContain('healer');
    expect(siteOf(removed, { source: 'sites', index: 4 })!.kind).toBe('tavern');
    expect(siteOf(removed, { source: 'merchant', index: 0 })).toBeUndefined();
  });

  it('Kandidat als Text behält alle übrigen Felder', () => {
    const text = candidateText(setGateHubLevel(hub(), 3));
    expect(JSON.parse(text).levels).toEqual(parseHub(mockHubPlan().raw).levels);
  });

  it('Zahl aus dem Eingabefeld', () => {
    expect(numberOf('-40')).toBe(-40);
    expect(numberOf('')).toBeNull();
    expect(numberOf('x')).toBeNull();
  });
});
