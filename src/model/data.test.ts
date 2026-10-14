import { describe, expect, it } from 'vitest';
import buildingsJson from '../../data/buildings.json';
import hubJson from '../../data/hub.json';
import { HUB } from './data';

// B-209: Jede Platz-Art aus data/hub.json muss der Client kennen (SiteKind = Gebäude außer Burg).
const known = new Set(Object.keys(buildingsJson).filter((k) => k !== 'castle'));
const raw = hubJson as unknown as Record<'sites' | 'islandSites', Record<string, unknown>[]>;

describe('HUB passt zu data/hub.json', () => {
  for (const list of ['sites', 'islandSites'] as const) {
    it(`${list}: jede kind ist bekannt, offsetUnits und hubLevel sind Zahlen`, () => {
      for (const site of raw[list]) {
        expect(known.has(String(site.kind)), `unbekannte Platz-Art: ${String(site.kind)}`).toBe(true);
        expect(typeof site.offsetUnits).toBe('number');
        expect(typeof site.hubLevel).toBe('number');
      }
    });
  }

  it('Typ deckt wallLines ab', () => {
    expect(HUB.wallLines.wallUnits.length).toBeGreaterThan(0);
    expect(HUB.wallLines.gateHubLevel).toBeGreaterThan(0);
  });
});
